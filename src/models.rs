use mongodb::bson::oid::ObjectId;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

pub type Cents = u32;

#[derive(Serialize, Deserialize)]
#[derive(Object)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Object)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    member_ids: Vec<ObjectId>,
    expenses: Vec<Expense>,
}

#[derive(Serialize, Deserialize)]
#[derive(Object)]
pub struct Expense {
    name: String,
    price: Cents,
    payer_id: ObjectId,
}