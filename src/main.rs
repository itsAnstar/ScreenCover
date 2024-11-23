use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use rdev::{listen, Event as RdevEvent, EventType, Key};

struct App {
    window: Option<Window>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // 创建窗口并设置全屏
        let mut window = event_loop.create_window(Window::default_attributes()).unwrap();
        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        window.set_decorations(false);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Window close requested");
                event_loop.exit();
            },
            _ => (),
        }
    }
}

fn main() {
    // 创建快捷键监听线程
    std::thread::spawn(|| {
        if let Err(error) = listen(handle_event) {
            eprintln!("Error: {:?}", error);
        }
    });

    // 创建事件循环
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);  // 使用 Wait 模式以节省资源

    // 运行应用
    let mut app = App::default();
    event_loop.run_app(&mut app).expect("Error running application");
}

fn handle_event(event: RdevEvent) {
    match event.event_type {
        EventType::KeyPress(Key::Escape) => {
            println!("ESC pressed. Exiting...");
            std::process::exit(0);
        }
        EventType::KeyPress(Key::F11) => {
            println!("F11 pressed. Rendering screen cover...");
            // TODO: 触发渲染窗口的逻辑
        }
        _ => {}
    }
}
