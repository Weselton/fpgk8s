fn main() {
    tonic_build::configure()
        .build_server(true)
        .compile_protos(&["proto/v1beta1.proto", "proto/v1alpha.proto"], &["proto"])
        .unwrap();
}
