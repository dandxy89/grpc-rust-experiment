use grpc_rust_experiment::{
    error::SolarSystemError, solar_system_info::solar_system_client::SolarSystemClient,
};
use tonic::{transport::Channel, Request};

async fn create_grpc_client() -> SolarSystemClient<Channel> {
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await
        .expect("Can't create a channel");

    SolarSystemClient::new(channel)
}

async fn get_planets_list(
    mut grpc_client: SolarSystemClient<Channel>,
    sequence_id: usize,
) -> Result<String, SolarSystemError> {
    let response = grpc_client.get_planets_list(Request::new(())).await;

    return match response {
        Ok(response) => {
            let _message = response
                .into_inner()
                .list
                .into_iter()
                .map(|planet_name| {
                    format!(
                        "{} (use <code>/planet {}</code>)",
                        planet_name,
                        planet_name.to_lowercase()
                    )
                })
                .collect::<Vec<String>>()
                .join(" | ");
            Ok(format!("Got ID {}", sequence_id))
        }
        Err(_) => Err(SolarSystemError::new(
            "Internal error while handling list of planets",
        )),
    };
}

async fn get_planets(
    mut grpc_client: SolarSystemClient<Channel>,
    sequence_id: usize,
) -> Result<String, SolarSystemError> {
    let response = grpc_client.get_planets(Request::new(())).await;

    return match response {
        Ok(response) => {
            let mut planets_stream = response.into_inner();
            while let Some(response) = planets_stream.message().await.expect("Can't get a planet") {
                println!("{:?}", response.planet);
            }

            Ok(format!("Got ID {}", sequence_id))
        }
        Err(_) => Err(SolarSystemError::new(
            "Internal error while handling list of planets",
        )),
    };
}

#[tokio::main]
async fn main() {
    let grpc_client = create_grpc_client().await;
    let mut handles = vec![];

    for seq_id in 0..1_000 {
        let client = grpc_client.clone();
        handles.push(if seq_id % 2 == 0 {
            tokio::spawn(async move { get_planets_list(client, seq_id).await })
        } else {
            tokio::spawn(async move { get_planets(client, seq_id).await })
        });
    }

    for handle in handles {
        let s = handle.await.unwrap().unwrap();
        println!("S {}", s);
    }
}
