use std::fs::File;
	
mod frame;
mod decode;
mod encode;

#[tokio::main]
async fn main() {
    println!("Howdy.");

	// "Symphonia" crate is not async. Because it is blocking, we need to simulate async
	// by setting channel limit to 1. After 1 iteration, symphonia's green thread yields 
	// for other threads to async-ly do their bit. Not the solution, but works for now.
	let (de_send, en_recv) = tokio::sync::mpsc::channel(1);

	let decode_task = tokio::spawn(
		decode::decode_music(
			File::open("audio_samples/skyrim from past to present.mp3")
				.expect("File open error"), de_send
	));

	let encode_task = tokio::spawn(
		encode::encode_music(en_recv)
	);

	// tokio::try_join!(decode_task, encode_task).expect("Decode/Encode & Send concurrency failed");
	
	let (_a, _b) = (decode_task.await, encode_task.await);

	let _client = reqwest::Client::new();

	// let _res = client.post("http://127.0.0.1:4242")
	// 	.body("PACKAGE!").send().await.expect("Something went wrong here...");
}
