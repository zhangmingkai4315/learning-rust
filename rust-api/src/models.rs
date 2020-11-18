use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post{
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}

impl Post{
    pub fn new(title: &str, body: &str, author: &str, datetime: DateTime<Utc>, uuid: Uuid) ->Post{
        Post{
            title: title.to_string(),
            body: body.to_string(),
            author: body.to_string(),
            datetime,
            uuid,
        }
    }

    pub fn uuid(&self)->Uuid{
        self.uuid
    }



}
