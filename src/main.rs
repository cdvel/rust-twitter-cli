extern crate config;
extern crate twitter_stream;
extern crate json;

use std::collections::HashMap;
use twitter_stream::{Token, TwitterStreamBuilder};
use twitter_stream::rt::{self, Future, Stream};

fn main() {

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let secrets = settings.try_into::<HashMap<String, String>>().unwrap();

    let token = Token::new( secrets.get("consumer_key").unwrap().to_owned(), 
                            secrets.get("consumer_secret").unwrap().to_owned(),
                            secrets.get("access_key").unwrap().to_owned(),
                            secrets.get("access_secret").unwrap().to_owned() );

    let future = TwitterStreamBuilder::filter(token)
        .track(Some("@Twitter"))
        .listen()
    .unwrap()
        .flatten_stream()
        .for_each(|tweet| {
            let payload = json::parse(&tweet).unwrap();
            println!("\n🐦 {}\n", payload["text"]);
            println!("@{} | {} | {}\n", 
                        payload["user"]["screen_name"], 
                        payload["created_at"], 
                        payload["user"]["location"]);
            Ok(())
        })
        .map_err(|e| println!("{}", e));

    rt::run(future);
}
