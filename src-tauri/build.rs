use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=../index.html");
    println!("cargo:rerun-if-changed=../package.json");
    println!("cargo:rerun-if-changed=../pnpm-lock.yaml");
    println!("cargo:rerun-if-changed=../src");
    println!("cargo:rerun-if-changed=../tsconfig.json");
    println!("cargo:rerun-if-changed=../vite.config.ts");

    if std::env::var_os("CARGO_FEATURE_BUILD_FRONTEND").is_some() {
        let status = Command::new("pnpm")
            .arg("build")
            .current_dir("..")
            .status()
            .expect("无法执行 pnpm build，请确认已安装 pnpm");

        if !status.success() {
            panic!("pnpm build 执行失败");
        }
    }

    tauri_build::build()
}
