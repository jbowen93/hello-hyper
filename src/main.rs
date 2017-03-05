#![deny(warnings)]
extern crate env_logger;
extern crate hyper;
extern crate futures;
extern crate tokio_core;

use futures::future::FutureResult;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

use hyper::Client;

static PHRASE: &'static [u8] = b"Hello World!";

#[derive(Clone, Copy)]
struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {

        //Just forward to nginx
        let url = "http://127.0.0.1:8080".parse().unwrap();

        //Create the request client
        let mut core = tokio_core::reactor::Core::new().unwrap();
        let handle = core.handle();
        let client = Client::new(&handle);

        //Create the request
        // let mut client_req = Client::Request::new(req.method().clone(), url);
        // client_req.set_body(req.body());

        let work = client.get(url);
        core.run(work).unwrap();
        
        //Make response
        futures::future::ok(
            Response::new()
                .with_header(ContentLength(PHRASE.len() as u64 ))
                .with_header(ContentType::plaintext())
                .with_body(PHRASE)
        )
    }
}


fn main() {
    env_logger::init().expect("Failed to start logger");
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Hello)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();

}