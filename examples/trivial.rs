fn main() {
    let api = inference::api::Api::new().unwrap();
    let session = api.session().unwrap();
    println!("Session ready {:?}", session);
}
