fn main() {
    let api = inference::api::Api::new().unwrap();
    let session = api.silero_vad().unwrap();
    println!("Session ready {:#?}", session);
}
