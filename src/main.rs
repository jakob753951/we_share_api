use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use poem_openapi::param::Query;

struct Api;

#[OpenApi]
impl Api {
	/// Greeting
	#[oai(path = "/", method = "get")]
	async fn greeting(&self, name: Query<Option<String>>) -> PlainText<String> {
		match name.0 {
			Some(name) => PlainText(format!("Hello, {name}!")),
			None => PlainText("Hi!".to_string())
		}
	}
}

#[tokio::main]
async fn main() {
	let api_service =
		OpenApiService::new(Api, "WeShare", "1.0.0")
			.server("http://localhost:3000");
	let openapi = api_service.openapi_explorer();
	let swagger = api_service.swagger_ui();
	let rapidoc = api_service.rapidoc();
	let redoc = api_service.redoc();
	let app = Route::new()
		.nest("/", api_service)
		.nest("/openapi", openapi)
		.nest("/swagger", swagger)
		.nest("/rapidoc", rapidoc)
		.nest("/redoc", redoc);
	
	Server::new(TcpListener::bind("127.0.0.1:3000"))
		.run(app)
		.await
		.expect("Correct IO");
}