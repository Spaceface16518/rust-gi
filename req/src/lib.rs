extern crate log;

use log::{error, info, trace};
use std::{
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
};

/// Preforms a GET request on the provided IP, using the arguments provided.
/// These arguments will be formatted properly and appended to the URL for use
/// with the API.
pub fn get<I: ToSocketAddrs>(ip: I, args: Vec<String>) -> io::Result<Vec<u8>> {
    // Try to connect
    let mut stream = connect(ip);

    // If this is logged, the connection was successful
    trace!("Connected to the provided IP");

    // Try to write to the stream
    match write_request(&mut stream, args) {
        // If it was successful, try to read from the stream for the response
        Ok(bytes_written) => {
            info!("Success! {} bytes written to stream. Attempting to read from stream...", bytes_written);

            // Read in the response to a buffer
            let buf = read_all(stream);

            // Log the total bytes written; remember that the `read_all` method
            // logs each loop as well
            info!("Success! {} bytes read in from stream in total", buf.len());

            // Return the buffer
            Ok(buf)
        },

        // If there was an error, don't try to read in the response, just bubble
        // up the error
        Err(err) => {
            error!("An error occurred while attempting to write to the stream");
            Err(err)
        },
    }
}

/// Reads the contents of a stream into a vector and returns it. This is used
/// for reading in the HTTP response.
#[inline]
fn read_all<R: Read>(mut stream: R) -> Vec<u8> {
    // Initialize storage
    let mut buf: Vec<u8> = Vec::new();

    // Read on a loop
    loop {
        // Read 32 bytes at a time
        let mut tmp: [u8; 32] = [0; 32];

        // Attempt read
        let bytes_read =
            stream.read(&mut tmp).expect("Error reading from TCP stream");

        // Stop trying if we didn't read any bytes; this shouldn't have that
        // much overhead because an empty read won't take that much time
        if bytes_read == 0 {
            break
        }

        // Send the number of bytes read upstream
        trace!("{} bytes read", bytes_read);

        // Append the read bytes to our storage
        buf.extend_from_slice(&tmp);
    }
    buf
}

#[inline]
fn connect<S: ToSocketAddrs>(ip: S) -> TcpStream {
    // TODO: some custom error handling
    TcpStream::connect(ip).expect("Could not connect to provided IP address")
}

#[inline]
fn write_request<S: Write>(
    stream: &mut S,
    api_arguments: Vec<String>,
) -> io::Result<usize> {
    let buf = format_request(to_arg_string(api_arguments));
    stream.write(buf.as_bytes())
}

/// Merges a list of arguments into one argument string, ready for writing to
/// the request
#[inline]
fn to_arg_string(a: Vec<String>) -> String { a.join(",") }

/// Formats a `String` of API arguments (pre-formatted) into a full request,
/// ready to be written to the stream
#[inline]
fn format_request(formatted_api_arguments: String) -> String {
    format!(
        "GET /api/{} HTTP/1.1
User-Agent: rust-gi net (v{})
Host: gitignore.io
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive",
        formatted_api_arguments,
        env!("CARGO_PKG_VERSION")
    )
}

#[cfg(test)]
mod req_tests {
    use super::*;

    #[test]
    fn test_to_arg_string() {
        let args: Vec<String> = vec!["arg1", "arg2", "arg3"]
            .into_iter()
            .map(|s| String::from(s))
            .collect();
        assert_eq!(to_arg_string(args), "arg1,arg2,arg3".to_string());
    }

    #[test]
    fn test_format_request() {
        assert_eq!(
            format_request("arg1,arg2,arg3".to_string()),
            format!(
                "GET /api/arg1,arg2,arg3 HTTP/1.1
User-Agent: rust-gi net (v{})
Host: gitignore.io
Accept-Language: en-us
Accept-Encoding: gzip, deflate
Connection: Keep-Alive",
                env!("CARGO_PKG_VERSION") /* This test is not to see if this
                                           * works */
            )
        );
    }
}
