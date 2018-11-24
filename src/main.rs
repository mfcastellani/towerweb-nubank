#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Clone, Debug)]
struct ArgResource;

/// This will be the JSON response
#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl HelloWorld {
        #[get("/")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse { 
                message: "hello nubank, let's Rust",
            })
        }
    }


    impl ArgResource {
       #[get("/hello/:param")]
        fn path_str(&self, param: String) -> Result<String, ()> {
            Ok(format!("Hello {}, be welcome", param))
        }

        #[get("/hello2/:foo/:bar")]
        fn path_multi_str(&self, foo: String, bar: String) -> Result<String, ()> {
            Ok(format!("Hello {} and {}", foo, bar))
        }

        #[get("/number/:num")]
        fn path_num(&self, num: u32) -> Result<String, ()> {
            Ok(format!("The number is -> {}", num))
        }

        #[get("/query-string")]
        fn hello_query_string(&self, query_string: String) -> Result<String, ()> {
            Ok(format!("We received the query {:?}", query_string))
        }

        #[post("/request-body")]
        fn request_body(&self, body: Vec<u8>) -> Result<String, ()> {
            Ok(format!("The BODY {} bytes", body.len()))
        }

        #[get("/headers")]
        fn headers(&self, x_required: String, x_optional: Option<String>) -> Result<String, ()> {
            Ok(format!("We received: x-required = {}; x-optional = {:?}", x_required, x_optional))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .resource(ArgResource)
        .run(&addr)
        .unwrap();
}
