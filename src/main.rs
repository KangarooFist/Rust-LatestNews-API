use std::error::Error;
use serde::Deserialize;
use colour::{dark_blue, yellow};

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response= ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(articles)
}

fn render_articles(articles: &Articles) {
    for i in &articles.articles {
        dark_blue!("> {}\n", i.title);
        yellow!("- {}\n\n", i.url);
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=d89db64c8dc94d489f128e6cd726c626";
    let articles= get_articles(url)?;
    
    render_articles(&articles);

    Ok(())
}

