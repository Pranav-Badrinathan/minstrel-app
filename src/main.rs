use std::fs::File;
	
mod frame;
mod decode;
mod encode;

#[tokio::main]
async fn main() {
    println!("Howdy.");

	let decode_task = tokio::spawn(
		decode::decode_music(
			File::open("audio_samples/skyrim from past to present.mp3")
				.expect("File open error")
	));

	let encode_task = tokio::spawn(
		encode::encode_music()
	);

	tokio::try_join!(decode_task, encode_task).expect("Decode/Encode & Send concurrency failed");

	let client = reqwest::Client::new();

	// let _res = client.post("http://127.0.0.1:4242")
	// 	.body("PACKAGE!").send().await.expect("Something went wrong here...");
}
