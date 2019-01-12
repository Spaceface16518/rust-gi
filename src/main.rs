use self::uri::IntoUri;
use hyper::{
    rt::{self, Future, Stream},
    Client,
};
use hyper_tls::HttpsConnector;
use log::{error, info};
use std::{
    env::args,
    io::{self, Write},
    ops::Deref,
};

mod uri;

fn main() {
    pretty_env_logger::init();

    // Get the URI from command line arguments
    let uri = args().skip(1).collect::<Vec<String>>().into_uri();

    rt::run(rt::lazy(|| {
        // Build a client configured for HTTP connection
        let client = Client::builder().build::<_, hyper::Body>(
            // Use 4 DNS threads
            HttpsConnector::new(4).expect("TLS initialization failed"),
        );
        info!("Client connected");

        client
            // Preform GET request
            .get(uri)
            .and_then(|res| {
                // Convert response into hyper::Body
                res.into_body().for_each(|chunk| {
                    // handle each chunk (basically a &[u8])
                    handler(&chunk).map_err(|e| {
                        // Since corrupted output is unacceptable for this kind
                        // of program, a panic should be triggered if an error
                        // occurs in this stage
                        panic!("Error writing to stdout, error={}", e)
                    })
                })
            })
            // Less important error handling
            .map_err(|err| {
                error!("{}", err);
            })
    }));
    info!("Success");
}

/// Just writes all the bytes given to stdout
#[inline]
fn handler<T: Deref<Target = [u8]>>(bytes: &T) -> io::Result<()> {
    io::stdout().write_all(bytes)
}
