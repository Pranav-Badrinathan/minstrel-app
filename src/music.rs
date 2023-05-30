use std::fs::File;

use symphonia::core::{io::MediaSourceStream, probe::Hint, errors::Error};

use crate::frame::Frame;

// TODO: Error pls it's horrendus.
pub async fn decode_music(src: File){
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

	let mut frames: Vec<Frame> = vec![];

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
            Ok(_decoded) => {

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
    }
}
