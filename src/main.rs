mod models;
mod endpoints;

use mongodb::Client;
use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::AddData;
use poem_openapi::OpenApiService;
use endpoints::Api;

#[tokio::main]
async fn main() {
	let mongodb = Client::with_uri_str("mongodb+srv://jakoblm:61lQ5Il8jf0n4BaO@cluster0.vh1fcbg.mongodb.net/?retryWrites=true&w=majority")
		.await
		.unwrap()
		.database("we_share");

	let api_service =
		OpenApiService::new(Api, "WeShare", "1.0.0")
			.server("http://localhost:3000");
	let openapi = api_service.openapi_explorer();
	let swagger = api_service.swagger_ui();
	let app = Route::new()
		.nest("/", api_service)
		.nest("/openapi", openapi)
		.nest("/swagger", swagger)
		.with(AddData::new(mongodb));

	println!("Running server on localhost:3000");
	Server::new(TcpListener::bind("localhost:3000"))
		.run(app)
		.await
		.expect("Correct IO");
}