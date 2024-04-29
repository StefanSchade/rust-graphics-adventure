use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use log::info;

use port::adapters_inbound::web_server::WebServer;
use port::adapters_outbound::service_container::ServiceContainer;

use crate::web::option_01_actixweb::app_state::AppState;
use crate::web::option_01_actixweb::controllers::player_controller;

#[derive(Debug)]
pub struct ActixWebServer {
    pub service_container: ServiceContainer,
}

impl WebServer for ActixWebServer {
    // pub fn start_server(sc: ServiceContainer) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>> {
    fn start_server(&self) -> Pin<Box<dyn Future<Output=Result<(), std::io::Error>> + Send>> {
        info!("starting actix");

        let app_state = AppState {
            move_player_domain_story: self.service_container.move_player(),
        };

        let server_future = async move {
            let server = HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(Arc::new(app_state.clone())))
                    .configure(Self::configure_routes)
                //.service(web::resource("/player/move").route(web::post().to(player_controller::move_player))) alternative cfg
            })
                .bind("localhost:8080")?;
            server.run()
                .await
        };

        Box::pin(server_future)
    }
}


impl ActixWebServer {
    pub(crate) fn configure_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/player/move").route(web::post().to(player_controller::move_player))
        );
    }

    pub fn new(container: ServiceContainer) -> Arc<Self> {
        Arc::new(ActixWebServer {
            service_container: container,
        })
    }
}