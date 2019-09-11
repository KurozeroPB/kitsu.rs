//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`KitsuRequester`].
//!
//! [`KitsuRequester`]: trait.KitsuRequester.html

use std::io::Read;
use reqwest::{Client as ReqwestClient, RequestBuilder, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json;
use ::builder::Search;
use ::{API_URL, Error, Result};
use ::model::{Anime, Character, Manga, Producer, Response, User};

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implemenation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use kitsu::KitsuReqwestRequester;
/// ```
///
/// At this point, the methods will be on your Reqwest Client.
pub trait KitsuRequester {
    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_id = 1;
    ///
    ///     // Get the anime.
    ///     let anime = client.get_anime(anime_id)
    ///         .expect("Error getting anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_anime(&self, id: u64) -> Result<Response<Anime>>;

    /// Gets a character using its id.
    fn get_character(&self, id: u64) -> Result<Response<Character>>;

    /// Gets a manga using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_id = 1;
    ///
    ///     // Get the manga.
    ///     let manga = client.get_anime(manga_id)
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_manga(&self, id: u64) -> Result<Response<Manga>>;

    /// Gets a producer using its id
    /// 
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let producer_id = 1;
    ///
    ///     // Get the anime.
    ///     let anime = client.get_producer(producer_id)
    ///         .expect("Error getting producer");
    ///
    ///     // Do something with producer
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_producer(&self, id: u64) -> Result<Response<Producer>>;

    /// Gets a user using their id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_id = 1;
    ///
    ///     // Get the user.
    ///     let user = client.get_anime(user_id)
    ///         .expect("Error getting user");
    ///
    ///     // Do something with user
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn get_user(&self, id: u64) -> Result<Response<User>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let anime_name = "Your Lie in April";
    ///
    ///     // Search for the anime.
    ///     let anime = client.search_anime(|f| f.filter("text", anime_name))
    ///         .expect("Error searching for anime");
    ///
    ///     // Do something with anime
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<Anime>>>;

    /// Searches for a character.
    fn search_characters<F: FnOnce(Search) -> Search>(&self, f: F)
        -> Result<Response<Vec<Character>>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let manga_name = "Say I Love You";
    ///
    ///     // Search for the manga.
    ///     let manga = client.search_manga(|f| f.filter("text", manga_name))
    ///         .expect("Error getting manga");
    ///
    ///     // Do something with manga
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<Manga>>>;

    /// Gets an anime using its id.
    ///
    /// # Examples
    ///
    ///
    ///
    /// ```rust,no_run
    /// extern crate kitsu;
    /// extern crate reqwest;
    ///
    /// use kitsu::KitsuReqwestRequester;
    /// use reqwest::Client;
    ///
    /// fn main() {
    ///     // Create the reqwest Client.
    ///     let client = Client::new();
    ///
    ///     let user_name = "Billy";
    ///
    ///     // Search for the user.
    ///     let user = client.search_users(|f| f.filter("name", user_name))
    ///         .expect("Error searching for user");
    ///
    ///     // Do something with users
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error parsing the response
    /// body.
    ///
    /// Returns [`Error::ReqwestBad`] if the request was otherwise bad for some
    /// reason, containing the response.
    ///
    /// Returns [`Error::ReqwestInvalid`] if the response was a non-OK (status
    /// code 200) response, containing the response.
    ///
    /// Returns [`Error::ReqwestParse`] if there was an error parsing the image
    /// parameters into a valid URL.
    ///
    /// Returns [`Error::ReqwestUnauthorized`] if the authorization token was
    /// invalid.
    ///
    /// [`Error::Json`]: ../enum.Error.html#variant.Json
    /// [`Error::NoParamsSpecified`]: ../enum.Error.html#variant.NoParamsSpecified
    /// [`Error::ReqwestBad`]: ../enum.Error.html#variant.ReqwestBad
    /// [`Error::ReqwestInvalid`]: ../enum.Error.html#variant.ReqwestInvalid
    /// [`Error::ReqwestParse`]: ../enum.Error.html#variant.ReqwestParse
    /// [`Error::ReqwestUnauthorized`]: ../enum.Error.html#variant.ReqwestUnauthorized
    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<User>>>;
}

impl KitsuRequester for ReqwestClient {
    fn get_anime(&self, id: u64) -> Result<Response<Anime>> {
        let uri = Url::parse(&format!("{}/anime/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Anime>>(uri)
    }

    fn get_character(&self, id: u64) -> Result<Response<Character>> {
        let uri = Url::parse(&format!("{}/characters/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Character>>(uri)
    }

    fn get_manga(&self, id: u64) -> Result<Response<Manga>> {
        let uri = Url::parse(&format!("{}/manga/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Manga>>(uri)
    }

    fn get_user(&self, id: u64) -> Result<Response<User>> {
        let uri = Url::parse(&format!("{}/users/{}", API_URL, id.to_string()))?;

        handle_request::<Response<User>>(uri)
    }

    fn get_producer(&self, id: u64) -> Result<Response<Producer>> {
        let uri = Url::parse(&format!("{}/producers/{}", API_URL, id.to_string()))?;

        handle_request::<Response<Producer>>(uri)
    }

    fn search_anime<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<Anime>>> {
        let params = f(Search::default()).0;
        let uri = Url::parse(&format!("{}/anime?{}", API_URL, params))?;

        handle_request::<Response<Vec<Anime>>>(uri)
    }

    fn search_characters<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<Character>>> {
        let params = f(Search::default()).0;
        let uri = Url::parse(&format!("{}/characters?{}", API_URL, params))?;

        handle_request::<Response<Vec<Character>>>(uri)
    }

    fn search_manga<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<Manga>>> {
        let params = f(Search::default()).0;
        let uri = Url::parse(&format!("{}/manga?{}", API_URL, params))?;

        handle_request::<Response<Vec<Manga>>>(uri)
    }

    fn search_users<F: FnOnce(Search) -> Search>(&self, f: F) ->
        Result<Response<Vec<User>>> {
        let params = f(Search::default()).0;
        let uri = Url::parse(&format!("{}/users?{}", API_URL, params))?;

        handle_request::<Response<Vec<User>>>(uri)
    }
}

fn handle_request<T: DeserializeOwned>(uri: reqwest::Url) -> Result<T> {
    let client = reqwest::Client::new();
    let response = client.get(uri).send()?;

    match response.status() {
        StatusCode::OK => {},
        StatusCode::BAD_REQUEST => {
            return Err(Error::ReqwestBad(Box::new(response)));
        },
        StatusCode::UNAUTHORIZED => {
            return Err(Error::ReqwestUnauthorized(Box::new(response)));
        },
        _ => return Err(Error::ReqwestInvalid(Box::new(response))),
    }

    from_reader(response)
}

#[inline]
fn from_reader<T: DeserializeOwned, U: Read>(reader: U) -> Result<T> {
    serde_json::from_reader(reader).map_err(From::from)
}
