extern crate protobuf_codegen;

fn main() {
    protobuf_codegen::Codegen::new()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
        .out_dir("src")
        .includes(&["."])
        .inputs(&["n8w8.proto"])
        .run_from_script();
}
