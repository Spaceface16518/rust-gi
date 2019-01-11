use hyper::{
    rt::{self, Future, Stream},
    Client,
};
use hyper_tls::HttpsConnector;
use std::{
    env::args,
    io::{self, Write},
};
use log::{error, info};
use std::ops::Deref;

mod uri;

fn main() {
    pretty_env_logger::init();
    let args = args().skip(1).collect::<Vec<String>>();
    let uri = uri::uri_from(args);

    rt::run(rt::lazy(|| {
        let client = Client::builder().build::<_, hyper::Body>(
            HttpsConnector::new(4).expect("TLS initialization failed"),
        );
        info!("Client connected");
        client
            .get(uri)
            .and_then(|res| {
                res.into_body()
                    .for_each(|chunk| {
                        handler(&chunk).map_err(|e| {
                            panic!("Error writing to stdout, error={}", e)
                        })
                    })
            })
            .map_err(|err| {
                error!("{}", err);
            })
    }));
    info!("Success");
}

#[inline]
fn handler<T: Deref<Target = [u8]>>(bytes: &T) -> io::Result<()> {
    io::stdout().write_all(bytes)
}
