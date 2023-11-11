use std::str::FromStr;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson, Client, Database};
use poem::{EndpointExt, listener::TcpListener, Route, Server};
use poem::middleware::AddData;
use poem::web::{Data, Path};
use poem_openapi::{Object, OpenApi, OpenApiService, payload::Json};
use serde::{Deserialize, Serialize};
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use poem_openapi::payload::PlainText;

pub type Cents = u32;

#[derive(Serialize, Deserialize)]
#[derive(Object)]
#[derive(Clone)]
pub struct User {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	name: String,
	phone: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Object)]
#[derive(Clone)]
pub struct Group {
	#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
	id: Option<ObjectId>,
	name: String,
	member_ids: Vec<ObjectId>,
	expenses: Vec<Expense>,
}

#[derive(Serialize, Deserialize)]
#[derive(Object)]
#[derive(Clone)]
pub struct Expense {
	name: String,
	price: Cents,
	payer_id: ObjectId,
}

struct Api;

#[OpenApi]
impl Api {
	/// Get all Users
	#[oai(path = "/users", method = "get")]
	async fn get_users(&self, database: Data<&Database>) -> Json<Vec<User>> {
		let collection = database.collection::<User>("users");
		let cursor = collection.find(None, None).await.unwrap();
		let users = cursor.try_collect::<Vec<User>>().await.unwrap();
		Json(users)
	}

	/// Get user by ID
	#[oai(path = "/users/:id", method = "get")]
	async fn get_user(&self, database: Data<&Database>, user_id: Path<String>) -> Json<User> {
		let collection = database.collection::<User>("users");
		let object_id = ObjectId::from_str(user_id.as_str()).unwrap();
		let user  = collection.find_one(doc!{"_id": object_id}, None).await.unwrap().unwrap();
		Json(user)
	}

	/// Create a user
	#[oai(path = "/users", method = "post")]
	async fn add_user(&self, database: Data<&Database>, user: Json<User>) -> Json<User> {
		let collection = database.collection::<User>("users");
		let result = collection.insert_one(user.0, None).await.unwrap();
		let result = collection.find_one(doc!{"_id": result.inserted_id}, None).await.unwrap().unwrap();
		Json(result)
	}
	
	/// Delete a user
	#[oai(path = "/users/:user_id", method = "delete")]
	async fn delete_user(&self, database: Data<&Database>, user_id: Path<String>) -> PlainText<String> {
		let collection = database.collection::<User>("users");
		let object_id = ObjectId::from_str(user_id.as_str()).unwrap();
		let result  = collection.delete_one(doc!{"_id": object_id}, None).await.unwrap();
		let deleted = result.deleted_count;
		
		PlainText(format!("{deleted} user(s) deleted"))
	}
	
	/// Get all Groups
	#[oai(path = "/groups", method = "get")]
	async fn get_groups(&self, database: Data<&Database>) -> Json<Vec<Group>> {
		let collection = database.collection::<Group>("group");
		let cursor = collection.find(None, None).await.unwrap();
		let groups = cursor.try_collect::<Vec<Group>>().await.unwrap();
		Json(groups)
	}

	/// Get group by ID
	#[oai(path = "/groups/:group_id", method = "get")]
	async fn get_group(&self, database: Data<&Database>, group_id: Path<String>) -> Json<Option<Group>> {
		let collection = database.collection::<Group>("groups");
		let object_id = ObjectId::from_str(group_id.as_str()).unwrap();
		let group = collection.find_one(doc!{"_id": object_id}, None).await.unwrap();
		Json(group)
	}

	/// Create a group
	#[oai(path = "/groups", method = "post")]
	async fn add_group(&self, database: Data<&Database>, group: Json<Group>) -> Json<Group> {
		let collection = database.collection::<Group>("groups");
		let result = collection.insert_one(group.0, None).await.unwrap();
		let result = collection.find_one(doc!{"_id": result.inserted_id}, None).await.unwrap().unwrap();
		Json(result)
	}
	
	/// Delete a group
	#[oai(path = "/users/:group_id", method = "delete")]
	async fn delete_group(&self, database: Data<&Database>, group_id: Path<String>) -> PlainText<String> {
		let collection = database.collection::<Group>("groups");
		let object_id = ObjectId::from_str(group_id.as_str()).unwrap();
		let result  = collection.delete_one(doc!{"_id": object_id}, None).await.unwrap();
		let deleted = result.deleted_count;
		PlainText(format!("{deleted} group(s) deleted"))
	}
	
	/// Add a user to a group
	#[oai(path = "/groups/:group_id/members/:user_id", method = "put")]
	async fn add_group_member(&self, database: Data<&Database>, group_id: Path<String>, user_id: Path<String>) -> Json<Group> {
		let collection = database.collection::<Group>("groups");
		let result = collection.update_one(
			doc!{"_id" : group_id.0},
			doc!{"$addToSet": {"members": user_id.0}},
			None
		).await.unwrap();
		let result = collection.find_one(doc!{"_id": result.upserted_id}, None).await.unwrap().unwrap();
		Json(result)
	}
	
	/// Remove a user from a group
	#[oai(path = "/groups/:group_id/members/:user_id", method = "delete")]
	async fn remove_group_member(&self, database: Data<&Database>, group_id: Path<String>, user_id: Path<String>) -> Json<Group> {
		let collection = database.collection::<Group>("groups");
		let result = collection.update_one(
			doc!{"_id" : group_id.0},
			doc!{"$push": {"members": user_id.0}},
			None
		).await.unwrap();
		let result = collection.find_one(doc!{"_id": result.upserted_id}, None).await.unwrap().unwrap();
		Json(result)
	}

	/// Add expense to a group
	#[oai(path = "/groups/:group_id/expenses", method = "put")]
	async fn add_expense(&self, database: Data<&Database>, group_id: Path<String>, expense: Json<Expense>) -> Json<Group> {
		let collection = database.collection::<Group>("groups");
		let serialized_expense = bson::to_bson(&expense.0).unwrap();
		let document = serialized_expense.as_document().unwrap();
		let result = collection.update_one(
				doc!{"_id" : group_id.0},
				doc!{"$addToSet": {"expenses": document}},
				None
			).await.unwrap();
		let result = collection.find_one(doc!{"_id": result.upserted_id}, None).await.unwrap().unwrap();
		Json(result)
	}
}

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
	
	Server::new(TcpListener::bind("127.0.0.1:3000"))
		.run(app)
		.await
		.expect("Correct IO");
}