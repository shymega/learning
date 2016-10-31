#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;
extern crate jsonrpc_core;

#[derive(Serialize, Deserialize, Debug)]
struct Hello_Request {
    jsonrpc: String,
    method: String,
    params: Vec<i32>,
    id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hello_Response<'a> {
    jsonrpc: String,
    result: String,
    id: usize,
}

use jsonrpc_core::*;

struct SayHello;
impl AsyncMethodCommand for SayHello {
    fn execute(&self, _params: Params, ready: Ready) {
        ready.ready(Ok(Value::String("hello".to_string())))
    }
}

fn main() {
    let io = IoHandler::new();
    io.add_async_method("say_hello", SayHello);

    let request_struct = Hello_Request {
        jsonrpc: "2.0".to_string(),
        method: "say_hello".to_string(),
        params: vec![42, 23],
        id: 1,
    };

    let request_json = serde_json::to_string(&request_struct).unwrap();
    let request: &str = request_json.as_str();

    //    let response =
    //    r#"{"jsonrpc":"2.0","result":"hello","id":1}"#;

    let response_struct = Hello_Response {
        jsonrpc: "2.0".to_string(),
        result: "hello".to_string(),
        id: 1,
    };

    let response_json = serde_json::to_string(&response_struct).unwrap();
    let response: &str = response_json.as_str();

    io.handle_request(request).map(|async_response| {
        async_response.on_result(move |res| {
            if res == response.to_string() {
                println!("It's the same!");
            };
            assert_eq!(res, response.to_string());
        })
    });
}
