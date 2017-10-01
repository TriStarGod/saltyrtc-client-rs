extern crate native_tls;
extern crate saltyrtc_client;
extern crate tokio_core;
extern crate websocket;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use native_tls::{TlsConnector, Certificate, Protocol};
use tokio_core::reactor::Core;
use websocket::futures::{Future, Stream, Sink};
use websocket::Message;

use saltyrtc_client::errors::{Error as SaltyrtcError, ErrorKind};


fn main() {
    let mut core = Core::new().unwrap();

    // Read server certificate bytes
    let mut server_cert_bytes: Vec<u8> = vec![];
    File::open(&Path::new("saltyrtc.der"))
        .unwrap()
        .read_to_end(&mut server_cert_bytes)
        .unwrap();

    // Parse server certificate
    let server_cert = Certificate::from_der(&server_cert_bytes)
        .unwrap_or_else(|e| {
            panic!("Problem with CA cert: {}", e);
        });

    // Create TLS connector instance
    let mut tls_builder = TlsConnector::builder()
        .unwrap_or_else(|e| panic!("Could not initialize TlsConnector builder: {}", e));
    tls_builder.supported_protocols(&[Protocol::Tlsv11, Protocol::Tlsv11, Protocol::Tlsv10])
        .unwrap_or_else(|e| panic!("Could not set TLS protocols: {}", e));
    tls_builder.add_root_certificate(server_cert)
        .unwrap_or_else(|e| panic!("Could not add root certificate: {}", e));
    let tls_connector = tls_builder.build()
        .unwrap_or_else(|e| panic!("Could not initialize TlsConnector: {}", e));

    let client = saltyrtc_client::connect(
            "wss://localhost:8765",
            Some(tls_connector),
            &core.handle()
        ).unwrap();

    let answer = client
        .and_then(|client| client.send(Message::text("hallo").into()))
        .and_then(|s| s.into_future().map_err(|e| e.0))
        .map(|(m, _)| {
            println!("Received answer: {:?}", m);
            assert_eq!(m, Some(Message::text("hallo").into()))
        })
        .map_err(|e| SaltyrtcError::from_kind(format!("Error while processing server answer: {}", e).into()));

    match core.run(answer) {
        Ok(x) => println!("Success: {:?}", x),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(cause) = e.cause() {
                println!("Cause: {}", cause);
            }
        },
    };
}
