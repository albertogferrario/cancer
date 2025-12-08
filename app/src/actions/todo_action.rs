//! Todo actions

use kit::database::{Model, ModelMut};
use kit::injectable;
use sea_orm::Set;

use crate::models::todo;

#[injectable]
pub struct CreateRandomTodoAction;

impl CreateRandomTodoAction {
    pub async fn execute(&self) -> Result<todo::Model, kit::error::FrameworkError> {
        let random_num = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() % 10000;

        let new_todo = todo::ActiveModel {
            title: Set(format!("Todo #{}", random_num)),
            description: Set(Some(format!("This is a random todo created at timestamp {}", random_num))),
            ..Default::default()
        };

        todo::Entity::insert_one(new_todo).await
    }
}

#[injectable]
pub struct ListTodosAction;

impl ListTodosAction {
    pub async fn execute(&self) -> Result<Vec<todo::Model>, kit::error::FrameworkError> {
        todo::Entity::all().await
    }
}
