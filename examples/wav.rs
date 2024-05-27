fn main() {
    let mut reader = hound::WavReader::open("audio.wav").unwrap();
    println!("{:?}", reader.spec());
    let audio: Vec<i16> = reader.samples().filter_map(|s| s.ok()).collect();
    let api = inference::api::Api::new().unwrap();
    let session = api.silero_vad().unwrap();
    session.process(&audio[..160])
}
