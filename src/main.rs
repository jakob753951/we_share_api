mod models;
mod endpoints;

use mongodb::Client;
use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::AddData;
use poem_openapi::OpenApiService;
use endpoints::Api;
use std::env;

#[tokio::main]
async fn main() {
	let mongodb = Client::with_uri_str(env::var("MONGODB_URI").unwrap())
		.await
		.unwrap()
		.database("we_share");

	let url = env::var("HOST_URL").unwrap();

	let api_service =
		OpenApiService::new(Api, "WeShare", "1.0.0")
			.server(env::var("DOCUMENTATION_TARGET_URL").unwrap());
	let openapi = api_service.openapi_explorer();
	let swagger = api_service.swagger_ui();
	let app = Route::new()
		.nest("/", api_service)
		.nest("/openapi", openapi)
		.nest("/swagger", swagger)
		.with(AddData::new(mongodb));

	println!("Running server on {url}");
	Server::new(TcpListener::bind(url))
		.run(app)
		.await
		.expect("Correct IO");
}