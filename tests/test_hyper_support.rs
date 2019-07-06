#![cfg(feature = "hyper-support")]

extern crate futures;
extern crate hyper;
extern crate hyper_rustls;
extern crate kitsu;
extern crate tokio;

use futures::Future;
use hyper::Client;
use hyper_rustls::HttpsConnector;
use kitsu::KitsuHyperRequester;

#[ignore]
#[test]
fn test_get_anime() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.get_anime(1).map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_get_character() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.get_character(1).map(|_| ()).map_err(|why| {
        panic!("{:?}", why);
    });

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_get_manga() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.get_manga(1).map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_get_producer() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.get_producer(1).map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_get_user() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.get_user(1).map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_search_anime() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.search_anime(|f| f.filter("text", "non non biyori"))
        .map(|_| ())
        .map_err(|why| {
            panic!("{:?}", why);
        });

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_search_manga() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.search_manga(|f| f.filter("text", "orange"))
        .map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}

#[ignore]
#[test]
fn test_search_users() {
    let connector = HttpsConnector::new(1);
    let client = Client::builder().build(connector);

    let runner = client.search_users(|f| f.filter("name", "vikhyat"))
        .map(|_| ()).map_err(|_| ());

    tokio::run(runner);
}
