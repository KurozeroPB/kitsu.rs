//! Bridge to provide a client implementation for the `hyper` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`KitsuRequester`].
//!
//! [`KitsuRequester`]: trait.KitsuRequester.html

use futures::future::{self, Future};
use futures::Stream;
use hyper::client::{Client as HyperClient, Connect};
use hyper::error::Error as HyperError;
use hyper::Uri;
use serde_json;
use std::str::FromStr;
use ::builder::Search;
use ::model::*;
use ::{API_URL, Error};

macro_rules! try_uri {
    ($uri:ident) => {
        match Uri::from_str($uri) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        }
    }
}

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `hyper` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use kitsu_io::KitsuHyperRequester;
/// ```
///
/// At this point, the methods will be on your Hyper Client.
pub trait KitsuRequester {
    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    /// Get an anime with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let anime_id = 1;
    ///
    /// let runner = client.get_anime(anime_id)
    ///     .map(|anime| {
    ///         println!(
    ///             "The anime's name is '{}'",
    ///             anime.data.attributes.canonical_title,
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_anime(&self, id: u64)
        -> Box<Future<Item = Response<Anime>, Error = Error>>;

    /// Gets a manga using its id.
    ///
    /// # Examples
    ///
    /// Get a manga with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let manga_id = 1;
    ///
    /// let runner = client.get_manga(manga_id)
    ///     .map(|manga| {
    ///         println!(
    ///             "The manga's name is '{}'",
    ///             manga.data.attributes.canonical_title,
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_manga(&self, id: u64)
        -> Box<Future<Item = Response<Manga>, Error = Error>>;

    /// Gets a user using their id.
    ///
    /// # Examples
    ///
    /// Get a user with the id of 1:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let user_id = 1;
    ///
    /// let runner = client.get_user(user_id)
    ///     .map(|user| {
    ///         println!(
    ///             "The user's name is '{}'",
    ///             user.data.attributes.name,
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn get_user(&self, id: u64)
        -> Box<Future<Item = Response<User>, Error = Error>>;

    /// Searches for an anime using the passed [Search] builder.
    ///
    /// # Examples
    ///
    /// Search for an anime with the name "Beyond the Boundary":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let anime_name = "Beyond the Boundary";
    ///
    /// let runner = client.search_anime(|f| f.filter("text", anime_name))
    ///     .map(|resp| {
    ///         println!(
    ///             "There are {} results",
    ///             resp.data.len(),
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<Anime>>, Error = Error>>;

    /// Searches for a manga using the passed [Search] builder.
    ///
    /// # Examples
    ///
    /// Search for a manga with the name "Orange":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let manga_name = "Orange";
    ///
    /// let runner = client.search_manga(|f| f.filter("text", manga_name))
    ///     .map(|resp| {
    ///         println!(
    ///             "There are {} results",
    ///             resp.data.len(),
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<Manga>>, Error = Error>>;

    /// Searches for a user using the passed [`Search`] builder.
    ///
    /// # Examples
    ///
    /// Search for a user with the name "Bob":
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate kitsu_io;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use kitsu_io::KitsuHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(1, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let user_name = "Bob";
    ///
    /// let runner = client.search_users(|f| f.filter("name", user_name))
    ///     .map(|resp| {
    ///         println!(
    ///             "There are {} results",
    ///             resp.data.len(),
    ///         );
    ///     })
    ///     .map_err(|why| {
    ///         println!("Error with the request: {:?}", why);
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    /// [`Search`]: ../builder/struct.Search.html
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `02_hyper` and should
    // roughly match it to ensure accuracy.
    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<User>>, Error = Error>>;
}

impl<B, C: Connect> KitsuRequester for HyperClient<C, B>
    where B: Stream<Error = HyperError> + 'static, B::Item: AsRef<[u8]> {
    fn get_anime(&self, id: u64)
        -> Box<Future<Item = Response<Anime>, Error = Error>> {
        let url = format!("{}/anime/{}", API_URL, id);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_manga(&self, id: u64)
        -> Box<Future<Item = Response<Manga>, Error = Error>> {
        let url = format!("{}/manga/{}", API_URL, id);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn get_user(&self, id: u64)
        -> Box<Future<Item = Response<User>, Error = Error>> {
        let url = format!("{}/users/{}", API_URL, id);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<Anime>>, Error = Error>> {
        let params = f(Search::default()).0;

        let url = format!("{}/anime?{}", API_URL, params);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<Manga>>, Error = Error>> {
        let params = f(Search::default()).0;

        let url = format!("{}/manga?{}", API_URL, params);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }

    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Box<Future<Item = Response<Vec<User>>, Error = Error>> {
        let params = f(Search::default()).0;

        let url = format!("{}/users?{}", API_URL, params);
        let c = &url;
        let uri = try_uri!(c);

        Box::new(self.get(uri)
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }
}
