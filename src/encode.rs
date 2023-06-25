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
	let mut encoder = Encoder::new(48000, Channels::Stereo, Application::Audio).expect("Err encoder");

	loop {
		frame_buf.append(&mut en_recv.recv().await.unwrap_or_default());


		println!("Frames Len: {}", frame_buf.len());

		let (x, chunks) = chunkenize(frame_buf, 2880);
		frame_buf = x;

		for chunk in chunks {
			//the chunks sent in must be of size 120, 240, 480, 960, 1920, or 2880 per channel.
			let mut encoded: Vec<u8> = encoder.encode_vec_float(&chunk, 5760 as usize).expect("HIH");
			let mut buf = (encoded.len() as i16).to_le_bytes().to_vec();
			buf.append(&mut encoded);
			
			let client = reqwest::Client::new();		
			let _res = client.post("http://127.0.0.1:4242/")
				.header("guild_id", guild_id)
				.body(buf).send().await.expect("Something went wrong here...");
		}
	}
}


// TODO: Better, more descriptive name lol.
pub fn chunkenize(audio_data: Vec<Frame>, chunk_size: usize) -> (Vec<Frame>, impl Iterator<Item = Vec<f32>>) {
	let mut data: Vec<Vec<f32>> = Vec::new();
	
	let chunk_iter = audio_data.chunks_exact(chunk_size);
	let remainder = chunk_iter.remainder();

	for chunk in chunk_iter {
		// Resample from 44100 to 48000. Add a check in the future to do this ONLY if needed.
		// let (left, right) = resample(vec![frame::lefts(chunk), frame::rights(chunk)]);
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

pub fn resample(input: Vec<Vec<f32>>) -> (Vec<f32>, Vec<f32>) {
	use rubato::{
		Resampler, 
		SincFixedIn, 
		InterpolationType, 
		InterpolationParameters, 
		WindowFunction};

	let i_params = InterpolationParameters {
		sinc_len: 240,
		f_cutoff: 0.95,
		oversampling_factor: 160,
		interpolation: InterpolationType::Nearest,
		window: WindowFunction::BlackmanHarris2
	};
	
	let mut resampler = SincFixedIn::<f32>::new(
		48000 as f64 / 44100 as f64,
		i_params,
		input.get(0).unwrap().len(),
		2
	);

	let result = match resampler.process(&input) {
		Ok(r) => r,
		Err(e) => {
			eprintln!("Resampling error: {}", e);
			input
		}
	};

	// TODO: Handle the unwrap.
	(result.get(0).unwrap().to_vec(), result.get(1).unwrap().to_vec())
}
