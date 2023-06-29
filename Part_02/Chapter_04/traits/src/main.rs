pub trait Summarizable  {
    fn summary(&self) -> String {
        String::from("Read more...");
    }
}
pub struct NewsArticle  {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle   {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet    {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet  { //Here we implement the summarizable trait on the tweet type.
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    println!("Hello, world!");
}
