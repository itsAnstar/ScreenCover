use winit::{
    event::WindowEvent,
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Window},
};
use rdev::{listen, Event as RdevEvent, EventType, Key};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

// 用于在线程间共享窗口状态
struct WindowState {
    window: Option<Window>,
}

fn create_fullscreen_window(window_target: &winit::event_loop::EventLoopWindowTarget<()>) -> Window {
    WindowBuilder::new()
        .with_decorations(false)  // 无边框
        .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))  // 全屏
        .with_transparent(true)  // 支持透明
        .build(window_target)
        .expect("Failed to create window")
}

fn main() {
    // 创建事件循环
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    
    // 创建通道，用于线程间通信
    let (tx, rx) = mpsc::channel();
    
    // 创建共享窗口状态
    let window_state = Arc::new(Mutex::new(WindowState { window: None }));
    let window_state_clone = window_state.clone();

    // 创建快捷键监听线程
    std::thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            handle_event(event, &tx);
        }) {
            eprintln!("Error: {:?}", error);
        }
    });

    // 设置事件循环控制流
    event_loop.set_control_flow(ControlFlow::Wait);

    // 运行事件循环
    let _ = event_loop.run(move |event, window_target| {
        // 检查是否有来自快捷键线程的消息
        if let Ok(command) = rx.try_recv() {
            match command {
                "show" => {
                    let mut state = window_state.lock().unwrap();
                    if state.window.is_none() {
                        let window = create_fullscreen_window(window_target);
                        window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
                        state.window = Some(window);
                    }
                }
                "hide" => {
                    let mut state = window_state.lock().unwrap();
                    if let Some(window) = state.window.take() {
                        drop(window);  // 关闭窗口
                    }
                }
                _ => {}
            }
        }

        match event {
            winit::event::Event::WindowEvent { 
                event: WindowEvent::CloseRequested,
                ..
            } => {
                let mut state = window_state_clone.lock().unwrap();
                state.window = None;  // 清除窗口引用
            },
            winit::event::Event::Resumed => {
                // 在程序启动时创建初始窗口
                let mut state = window_state.lock().unwrap();
                if state.window.is_none() {
                    let window = create_fullscreen_window(window_target);
                    window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
                    state.window = Some(window);
                }
            },
            _ => (),
        }
    });
}

fn handle_event(event: RdevEvent, tx: &mpsc::Sender<&'static str>) {
    match event.event_type {
        EventType::KeyPress(Key::KeyL) => {
            // 检查 Shift 是否被按下
            if event.modifiers.shift {
                println!("Shift+L pressed. Showing window...");
                tx.send("show").unwrap();
            }
        }
        EventType::KeyPress(Key::Escape) => {
            println!("ESC pressed. Hiding window...");
            tx.send("hide").unwrap();
        }
        _ => {}
    }
}