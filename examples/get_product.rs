extern crate gdax_rs;
extern crate tokio_core;

use tokio_core::reactor::Core;

use gdax_rs::RESTClient;
use gdax_rs::products::GetProducts;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut test_client = RESTClient::default(&handle);
    let products = core.run(test_client.send_request(&GetProducts::new()))
        .unwrap();

    println!("{:?}", products);
}
