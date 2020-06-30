use async_graphql::{EmptyMutation, EmptySubscription, Schema, Deferred};
use std::thread::{sleep};
use std::time::Duration;

pub type MySchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct Comment {
  id: i32,
  text: String
}

#[async_graphql::Object]
impl Comment {
  async fn text(&self) -> &str {
    &self.text
  }
}

pub struct Post {
  id: i32,
  title: String,
  text: String
}

#[async_graphql::Object]
impl Post {
  async fn title(&self) -> &str {
    &self.title
  }
  async fn text(&self) -> &str {
    &self.text
  }
  async fn comments(&self) -> Deferred<Vec<Comment>> {
    match self.id {
      0 => {
        sleep(Duration::from_millis(5000));
        vec![
          Comment {
            id: 0,
            text: String::from("Comment on first")
          }
        ].into()
      }
      _ => {
        vec![
          Comment {
            id: 1,
            text: String::from("Comment on others")
          }
        ].into()
      }
    }
  }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
  async fn hello(&self) -> String {
    String::from("Hello world!")
  }
  
  async fn posts(&self) -> Vec<Post> {
    vec![
      Post {
        id: 0,
        title: String::from("First post"),
        text: String::from("First post content")
      },
      Post {
        id: 1,
        title: String::from("Second post"),
        text: String::from("Second post content")
      }
    ]
  }
}