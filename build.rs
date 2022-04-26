extern crate protoc_rust;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/n8w8.capnp")
        //.file("schema/bar.capnp")
        .output_path("src")
        .run().expect("CapnProto failed to compile schema.");
    protoc_rust::Codegen::new()
        .out_dir("src/models")
        .inputs(&["schema/n8w8.proto"])
        //.include("protos")
        .run()
        .expect("Running protoc failed.");
}
