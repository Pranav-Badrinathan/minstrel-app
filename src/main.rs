use symphonia::core::{probe::Hint, io::MediaSourceStream, codecs::*, errors::Error};
use std::fs::File;

#[tokio::main]
async fn main() {
    println!("Howdy.");


	let client = reqwest::Client::new();

	let _res = client.post("http://127.0.0.1:4242")
		.body("PACKAGE!").send().await.expect("Something went wrong here...");
}

// TODO: Error pls it's horrendus.
async fn decode_music(){
	let src = File::open("audio_samples/skyrim from past to present.mp3").expect("File open error");
	
	let mss = MediaSourceStream::new(Box::new(src), Default::default());
	let format_reader = symphonia::default::get_probe().format(
																&Hint::new(), 
																mss, 
																&Default::default(), 
																&Default::default()
	).expect("Unsupported Format!").format;

	// TODO: Remove the unwrap and implement error handling.
	let decoder = symphonia::default::get_codecs().make(
				&format_reader.default_track().unwrap().codec_params, 
				&Default::default()
	).expect("Decoder not working");

	let mut frames = vec![];
	    // The decode loop.
    loop {
        // Get the next packet from the media format.
		let packet = match format_reader.next_packet() {
            Ok(packet) => packet,
            Err(err) => {
                panic!("{}", err);
            }
        };

        match decoder.decode(&packet) {
            Ok(_decoded) => {
                // Consume the decoded audio samples (see below).
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
                // An unrecoverable error occured, halt decoding.
                panic!("{}", err);
            }
        }
    }
}
