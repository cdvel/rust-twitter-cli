extern crate config;
extern crate json;
extern crate twitter_stream;

use std::collections::HashMap;
use twitter_stream::rt::{self, Future, Stream};
use twitter_stream::{Token, TwitterStreamBuilder};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        panic!("Invalid number of arguments. Use quotation marks if needed.");
    }
    let term: &str = match args[1].as_str() {
        "search" => args[2].as_ref(),
        _ => panic!("{} is not a valid flag", args[1]),
    };

    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let secrets = settings.try_into::<HashMap<String, String>>().unwrap();

    let token = Token::new(
        secrets.get("consumer_key").unwrap().to_owned(),
        secrets.get("consumer_secret").unwrap().to_owned(),
        secrets.get("access_key").unwrap().to_owned(),
        secrets.get("access_secret").unwrap().to_owned(),
    );

    let future = TwitterStreamBuilder::filter(token)
        .track(Some(term))
        .listen()
        .unwrap()
        .flatten_stream()
        .for_each(|tweet| {
            let payload = json::parse(&tweet).unwrap();
            println!("\n🐦 {}\n", payload["text"]);
            println!(
                "@{} | {} | {}\n",
                payload["user"]["screen_name"], payload["created_at"], payload["user"]["location"]
            );
            Ok(())
        })
        .map_err(|e| println!("{}", e));

    rt::run(future);
}
