extern crate hyper_tls;
extern crate hyper;

use hyper::Client;

type Connector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

const MAX_DNS_THREADS: usize = 4;

pub fn https_connector(threads: usize) -> Connector {
    hyper_tls::HttpsConnector::new(threads).expect("Could not generate https connector")
}

pub fn client() -> Client<Connector, hyper::Body> {
    Client::builder().retry_canceled_requests(true).keep_alive(true).set_host(true).build(https_connector(MAX_DNS_THREADS))
}
