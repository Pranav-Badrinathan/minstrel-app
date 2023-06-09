use opus::{Encoder, Channels, Application};
use tokio::sync::mpsc;

use crate::frame::{Frame, lefts, rights};

pub async fn encode_music(mut en_recv: mpsc::Receiver<Vec<Frame>>){
	println!("encodin!");

	loop {
		let frames = en_recv.recv().await.unwrap_or_default();

		let mut encoder = Encoder::new(48000, Channels::Stereo, Application::Audio).expect("Err encoder");

		println!("Frames Len: {}", frames.len());

		for chunk in chunkenize(frames, 2880) {
			//the frames sent in must be of size 120, 240, 480, 960, 1920, or 2880
			let encoded = encoder.encode_vec_float(&chunk, 5760 as usize).expect("HIH");

			let client = reqwest::Client::new();		
			let _res = client.post("http://127.0.0.1:4242")
				.body(encoded).send().await.expect("Something went wrong here...");
		}

	}
}


// TODO: Better, more descriptive name lol.

pub fn chunkenize(audio_data: Vec<Frame>, chunk_size: usize) -> impl Iterator<Item = Vec<f32>> {
	let mut data: Vec<Vec<f32>> = Vec::new();
	
	for chunk in audio_data.chunks(chunk_size) {
		let mut f32_chunk = interleave(lefts(chunk), rights(chunk));

		if f32_chunk.len() < (2 * chunk_size) {
			f32_chunk.resize(2 * chunk_size, 0.0);
		}

		data.push(f32_chunk);
	}

	data.into_iter()
}

pub fn interleave<T>(a: T, b: T) -> T
	where T: IntoIterator + FromIterator<T::Item>
{
	a.into_iter()
        .zip(b.into_iter()) 
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect::<T>()
}
