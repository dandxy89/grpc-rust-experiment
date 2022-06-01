use std::pin::Pin;

use grpc_rust_experiment::solar_system_info;
use grpc_rust_experiment::solar_system_info::solar_system_server::SolarSystemServer;
use grpc_rust_experiment::solar_system_info::{
    solar_system_server::SolarSystem, Planet, PlanetRequest, PlanetResponse, PlanetsListResponse,
};
use tokio::sync::mpsc;
use tokio_stream::Stream;
use tokio_stream::StreamExt;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Default)]
struct SolarSystemInfoService {}

#[tonic::async_trait]
impl SolarSystem for SolarSystemInfoService {
    type GetPlanetsStream =
        Pin<Box<dyn Stream<Item = Result<PlanetResponse, Status>> + Send + Sync + 'static>>;

    async fn get_planets_list(
        &self,
        request: Request<()>,
    ) -> Result<Response<PlanetsListResponse>, Status> {
        println!("Got a request {:?}", request);
        let dummy_data = (1..=100)
            .into_iter()
            .map(|seq_id| format!("{}", seq_id))
            .collect();
        let reply = PlanetsListResponse { list: dummy_data };
        Ok(Response::new(reply))
    }

    async fn get_planets(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::GetPlanetsStream>, Status> {
        println!("Got a request {:?}", request);
        let (tx, rx) = mpsc::channel(4);
        let planets: Vec<Planet> = (1..=1000)
            .into_iter()
            .map(|seq_id| Planet {
                id: 1,
                name: format!("DemoPlanet{seq_id}"),
                r#type: 1,
                mean_radius: 31.1,
                mass: 99.1,
                satellites: vec![],
                image: vec![],
            })
            .collect();

        tokio::spawn(async move {
            let mut stream = tokio_stream::iter(&planets);

            while let Some(planet) = stream.next().await {
                tx.send(Ok(PlanetResponse {
                    planet: Some(planet.clone()),
                }))
                .await
                .unwrap();
            }
        });

        Ok(Response::new(Box::pin(
            tokio_stream::wrappers::ReceiverStream::new(rx),
        )))
    }

    async fn get_planet(
        &self,
        request: Request<PlanetRequest>,
    ) -> Result<Response<PlanetResponse>, Status> {
        println!("Got a request {:?}", request);
        let planet = Planet {
            id: 1,
            name: request.into_inner().name,
            r#type: 1,
            mean_radius: 31.1,
            mass: 99.1,
            satellites: vec![],
            image: vec![],
        };

        let reply = PlanetResponse {
            planet: Some(planet),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Solar System info server");

    let solar_system_info = SolarSystemInfoService::default();
    let solar_system_svc = SolarSystemServer::new(solar_system_info);

    let reflection_svc = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(solar_system_info::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    Server::builder()
        .add_service(solar_system_svc)
        .add_service(reflection_svc)
        .serve(addr)
        .await?;

    Ok(())
}
