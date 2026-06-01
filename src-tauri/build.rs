fn main() {
    println!("cargo:rerun-if-changed=../index.html");
    println!("cargo:rerun-if-changed=../package.json");
    println!("cargo:rerun-if-changed=../pnpm-lock.yaml");
    println!("cargo:rerun-if-changed=../src");
    println!("cargo:rerun-if-changed=../tsconfig.json");
    println!("cargo:rerun-if-changed=../vite.config.ts");

    tauri_build::build()
}
