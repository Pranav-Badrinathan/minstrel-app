use opus::{Encoder, Channels, Application};
use tokio::sync::mpsc;

pub async fn encode_music(mut en_recv: mpsc::Receiver<Vec<f32>>){
	println!("encodin!");

	loop {
		let frames = en_recv.recv().await.unwrap_or_default();

		let mut encoder = Encoder::new(48000, Channels::Stereo, Application::Audio).expect("Err encoder");

		println!("Frames Len: {}", frames.len());


		//Erros for whatever reason. don't know.
		let encoded = encoder.encode_vec_float(&frames[0..5760].to_vec(), 5760 as usize).expect("HIH");

		if frames.len() > 0 {
			let client = reqwest::Client::new();		
			let _res = client.post("http://127.0.0.1:4242")
				.body(encoded).send().await.expect("Something went wrong here...");
		}
	}
}
