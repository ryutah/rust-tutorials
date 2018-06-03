extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-chaged={}", proto_root);
    protoc_grpcio::compile_grpc_protos(&["example/diner.proto"], &[proto_root], &proto_root)
        .unwrap();
}
