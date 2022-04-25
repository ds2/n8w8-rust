fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .file("schema/schema.capnp")
        //.file("schema/bar.capnp")
        .output_path("src")
        .run().expect("schema compiler command");
}
