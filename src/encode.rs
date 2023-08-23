use opus::{Encoder, Channels, Application};
use tokio::{sync::mpsc, net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};

use crate::frame::{ Frame, self };

pub async fn encode_music(mut en_recv: mpsc::Receiver<Vec<Frame>>){
	println!("encodin!");

	let guild_id: u64 = match std::env::args().nth(1) {
		Some(id) => u64::from_str_radix(&id, 10)
			.expect("Please provide an integer for the server ID"),
		None => {
			eprintln!("Please provide a guild_id!");
			return
		}
	};

	let mut frame_buf: Vec<Frame> = Vec::new();
	let mut encoder = Encoder::new(48_000, Channels::Stereo, Application::Audio).expect("Err encoder");

	//TODO: Erorr Handle this...
	let mut stream = TcpStream::connect("127.0.0.1:4242".to_string()).await.expect("Error connecting!");

	//Send the guild_id.
	stream.write(&guild_id.to_be_bytes()).await.expect("GuildID Write Error");
	stream.flush().await.expect("GuildID Flush Error");

	loop {
		frame_buf.append(&mut en_recv.recv().await.unwrap_or_default());

		let (x, chunks) = chunkenize(frame_buf, 2880);
		frame_buf = x;

		// let mut encoded: Vec<u8> = Vec::new();

		for chunk in chunks {
			//the chunks sent in must be of size 120, 240, 480, 960, 1920, or 2880 per channel.
			// let enc_chunk = encoder.encode_vec_float(&chunk, 5760 as usize).expect("HIH");
			// encoded.extend((enc_chunk.len() as i16).to_le_bytes().to_vec());
			// encoded.extend(enc_chunk);

			let encoded = encoder.encode_vec_float(&chunk, 5760 as usize).expect("HIH");
			println!("Encoded Len: {}, Frame remainder Len: {}", encoded.len(), frame_buf.len());
			let _ = stream.write(&encoded).await;
			let _ = stream.flush().await;

			loop {
				match stream.read_u8().await {
					Ok(0) => break,
					Ok(1) => break,
					Ok(c) => eprintln!("Unknown acknowledgement code: {}", c),
					Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => continue,
					Err(_) => panic!("Acknowledgement read error! Remove this panic later"),
				}
			}
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

pub fn _resample(input: Vec<Vec<f32>>) -> (Vec<f32>, Vec<f32>) {
	use rubato::{
		Resampler, 
		SincFixedIn, 
		SincInterpolationType, 
		SincInterpolationParameters, 
		WindowFunction};

	let i_params = SincInterpolationParameters {
		sinc_len: 240,
		f_cutoff: 0.95,
		oversampling_factor: 160,
		interpolation: SincInterpolationType::Nearest,
		window: WindowFunction::BlackmanHarris2
	};
	
	let mut resampler = SincFixedIn::<f32>::new(
		48000 as f64 / 44100 as f64,
		2.0,
		i_params,
		input.get(0).unwrap().len(),
		2
	).unwrap();

	let result = match resampler.process(&input, None) {
		Ok(r) => r,
		Err(e) => {
			eprintln!("Resampling error: {}", e);
			input
		}
	};

	// TODO: Handle the unwrap.
	(result.get(0).unwrap().to_vec(), result.get(1).unwrap().to_vec())
}
