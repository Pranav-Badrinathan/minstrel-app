use opus::{Encoder, Channels, Application};
use tokio::sync::mpsc;

use crate::frame::{ Frame, self };

pub async fn encode_music(mut en_recv: mpsc::Receiver<Vec<Frame>>){
	println!("encodin!");

	let guild_id = match std::env::args().nth(1) {
		Some(id) => u64::from_str_radix(&id, 10)
			.expect("Please provide an integer for the server ID"),
		None => {
			eprintln!("Please provide a guild_id!");
			return
		}
	};

	let mut frame_buf: Vec<Frame> = Vec::new();

	loop {
		frame_buf.append(&mut en_recv.recv().await.unwrap_or_default());

		let mut encoder = Encoder::new(48000, Channels::Stereo, Application::Audio).expect("Err encoder");

		println!("Frames Len: {}", frame_buf.len());

		let (x, chunks) = chunkenize(frame_buf, 960);
		frame_buf = x;

		for chunk in chunks {
			//the chunks sent in must be of size 120, 240, 480, 960, 1920, or 2880 per channel.
			let encoded = encoder.encode_vec_float(&chunk, 1920 as usize).expect("HIH");
			
			let client = reqwest::Client::new();		
			let _res = client.post("http://127.0.0.1:4242/")
				.header("guild_id", guild_id)
				.body(encoded).send().await.expect("Something went wrong here...");
		}
	}
}


// TODO: Better, more descriptive name lol.

pub fn chunkenize(audio_data: Vec<Frame>, chunk_size: usize) -> (Vec<Frame>, impl Iterator<Item = Vec<f32>>) {
	let mut data: Vec<Vec<f32>> = Vec::new();
	
	let chunk_iter = audio_data.chunks_exact(chunk_size);
	let remainder = chunk_iter.remainder();

	for chunk in chunk_iter {
		let f32_chunk = interleave(frame::lefts(chunk), frame::rights(chunk));
		data.push(f32_chunk);
	}

	(remainder.to_vec(), data.into_iter())
}

pub fn interleave<T>(a: T, b: T) -> T
	where T: IntoIterator + FromIterator<T::Item>
{
	a.into_iter()
        .zip(b.into_iter()) 
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect::<T>()
}
