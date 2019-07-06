extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate kitsu;
extern crate tokio;

use futures::Future;
use futures::stream::Stream;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use kitsu::KitsuHyperRequester;
use std::io::{self, Write};

fn main() {
    // Read an anime name to search for from the users input.
    let mut input = String::new();
    print!("Enter an anime name to search for:\n>");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input_trimmed = input.trim();

    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    // Search for the anime and return the response.
    let runner = client.search_anime(|f| f.filter("text", input_trimmed))
        .expect("Error making request")
        .and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("\n\nDone")
        });

    tokio::run(runner);
}
