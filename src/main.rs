use std::process::Command;

fn main() {
    // 获取操作系统和架构信息
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    println!("Current OS: {}, architecture: {}", os, arch);

    // 根据操作系统和架构选择对应的二进制
    let binary_name = match (os, arch) {
        ("macos", "aarch64") => "mac_aarch64",
        ("macos", "x86_64") => "mac_x86_64",
        ("windows", "x86_64") => "windows_x86_64",
        ("linux", "x86_64") => "linux_x86_64",
        ("linux", "aarch64") => "linux_aarch64",
        _ => {
            eprintln!("Unsupported OS/architecture combination: {}/{}", os, arch);
            std::process::exit(1);
        }
    };

    // 运行对应的二进制
    let status = Command::new(format!("target/debug/{}", binary_name))
        .status()
        .unwrap_or_else(|_| {
            // 如果直接运行失败，尝试用 cargo run 构建并运行
            Command::new("cargo")
                .args(["run", "--bin", binary_name])
                .status()
                .expect("Failed to execute command")
        });

    // 使用相同的退出码
    std::process::exit(status.code().unwrap_or(1));
}
