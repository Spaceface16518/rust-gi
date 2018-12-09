extern crate futures;
extern crate hyper;
extern crate log;

mod uri;
mod client;

use futures::Future;
use hyper::{Body, Client, Uri};
use log::{error, info, trace};
use crate::client::https_connector;
use crate::client::client;
use crate::uri::uri_from;
use futures::Stream;


/// Performs a GET request, using the arguments provided.
/// These arguments will be formatted properly and appended to the URL for use
/// with the API.
pub fn get(args: Vec<String>) -> impl Future { 
    client().get(uri_from(args)).and_then(|res| {
        info!("Response recieved! Status: {}", res.status());
        res.into_body().concat2()
    })
 }
