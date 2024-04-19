mod hello_world {
    include!("hello_world.rs");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Client starting...");
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    for i in 0..10 {
        let request = tonic::Request::new(HelloRequest {
            name: format!("Tonic-{}", i).into(),
        });

        let response = client.say_hello(request).await?;

        println!("RESPONSE={:?}", response.into_inner());
    }

    Ok(())
}
