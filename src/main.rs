use std::fs::File;
mod frame;
mod music;

#[tokio::main]
async fn main() {
    println!("Howdy.");

	music::decode_music(File::open("audio_samples/skyrim from past to present.mp3").expect("File open error"));

	let client = reqwest::Client::new();

	let _res = client.post("http://127.0.0.1:4242")
		.body("PACKAGE!").send().await.expect("Something went wrong here...");
}

