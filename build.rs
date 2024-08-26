use prost_build::compile_protos;
use std::env;
use std::fs::copy;
use std::path::Path;

fn compile_and_copy_protos(proto_files: &[&str], proto_include_dirs: &[&str], output_file: &str) {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    println!("out_dir: {}", out_dir);
    compile_protos(proto_files, proto_include_dirs).expect("Failed to compile protos");

    // 自动生成模块文件的路径
    let module_path = Path::new(&out_dir).join("_.rs");

    // 检查生成的文件是否存在
    if !module_path.exists() {
        panic!("Generated file does not exist: {:?}", module_path);
    }

    // 将生成的模块文件复制到指定的输出文件
    let output_path = Path::new("src").join(output_file);
    copy(&module_path, &output_path).expect("Failed to copy generated proto file");
}

fn main() {
    // 编译并复制 stark proto
    let stark_proto_files = [
        "proto3/test/b.proto", // works fine if comment this line
        "proto3/test/c.proto",
    ];
    let stark_include_dirs = ["proto3", "proto3/test"];
    compile_and_copy_protos(
        &stark_proto_files,
        &stark_include_dirs,
        "proto_generated.rs",
    );
    // 重新编译触发机制
    println!("cargo:rerun-if-changed=proto3/");
    println!("cargo:rerun-if-changed=src/lib.rs");
}
