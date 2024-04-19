fn main() {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .out_dir("src") // you can change the generated code's location
        .compile(
            &["proto/hello_world/v1/hello_world.proto"],
            &["proto/hello_world"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
