use std::fs::File;
use symphonia::core::{
	io::MediaSourceStream, 
	probe::Hint, 
	errors::Error, 
	audio::{ AudioBuffer, AudioBufferRef, Signal }, 
	sample::Sample, 
	conv::{	FromSample, IntoSample }
};
use tokio::sync::mpsc;

// TODO: Error pls it's horrendus.
pub async fn decode_music(src: File, de_send: mpsc::Sender<Vec<f32>>){
	let mss = MediaSourceStream::new(Box::new(src), Default::default());
	let mut format_reader = symphonia::default::get_probe().format(
																&Hint::new(), 
																mss, 
																&Default::default(), 
																&Default::default()
	).expect("Unsupported Format!").format;

	// TODO: Remove the unwrap and implement error handling.
	let mut decoder = symphonia::default::get_codecs().make(
				&format_reader.default_track().unwrap().codec_params, 
				&Default::default()
	).expect("Decoder not working");

	println!("Sample Rate: {}", decoder.codec_params().sample_rate.unwrap());


	let mut frames: Vec<f32> = vec![];

    // The decode loop.
    loop {
        // Get the next packet from the media format.
		let packet = match format_reader.next_packet() {
            Ok(packet) => packet,
            Err(err) => {
				// Unrecoverable error. TODO: Add non-panic handling.
                panic!("{}", err);
            }
        };

        match decoder.decode(&packet) {
            Ok(decoded) => {
				frames.append(&mut load_frames_from_buffer_ref(&decoded));
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error, skip the packet.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode due to invalid data, skip the packet.
                continue;
            }
            Err(err) => {
                // An unrecoverable error occured, halt decoding. TODO: Add non-panic handling.
                panic!("{}", err);
            }
        }
		
		let time: f32 = frames.len() as f32 / decoder.codec_params().sample_rate.unwrap() as f32;

		if time >= 0.3 {
			de_send.send(frames).await.expect("Error sending!");
			frames = Vec::new();
		}
    }
}

pub fn load_frames_from_buffer_ref(buffer: &AudioBufferRef) -> Vec<f32> {
	match buffer {
		AudioBufferRef::U8(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::U16(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::U24(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::U32(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::S8(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::S16(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::S24(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::S32(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::F32(buffer) => load_frames_from_buffer(buffer),
		AudioBufferRef::F64(buffer) => load_frames_from_buffer(buffer),
	}
}

// The where means the sample will return an f32 when FromSample<S> on it?
pub fn load_frames_from_buffer<S: Sample>(buffer: &AudioBuffer<S>) -> Vec<f32>
	where f32: FromSample<S>
{
	match buffer.spec().channels.count() {
		1 => buffer
			.chan(0)
			.into_iter()
			.map(|a| (*a).into_sample())
			.collect(),

		2 => interleave(
			buffer.chan(0).into_iter().map(|a| (*a).into_sample()).collect(), 
			buffer.chan(1).into_iter().map(|a| (*a).into_sample()).collect()
		),
		// TODO: Error handle. Return a Result.
		_ => panic!("TODO ERROR. Unsupported channel configuration")
	}
}

pub fn interleave<T>(a: T, b: T) -> T
	where T: IntoIterator + FromIterator<T::Item>
{
	a.into_iter()
        .zip(b.into_iter()) 
        .flat_map(|(a, b)| [a, b].into_iter())
        .collect::<T>()
}
