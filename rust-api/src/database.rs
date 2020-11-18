use crate::models::Post;

pub struct Database{
    post: Vec<Post>,
}

impl Database{
    pub fn new()->Database{
        Database{
            post: vec![],
        }
    }

    pub fn add_post(&mut self, post: Post){
        self.post.push(post);
    }

    pub fn posts(&self) -> &Vec<Post>{
        &self.post
    }
}