use std::fs::File;

use iced::{Application, Settings};

mod decode;
mod encode;
mod frame;
mod ui;

fn main() -> iced::Result {
	println!("Howdy.");
	ui::Minstrel::run(Settings::default())
}

pub async fn play() {
	let (de_send, en_recv) = tokio::sync::mpsc::channel(1);

	let decode_task = tokio::spawn(decode::decode_music(
		File::open("audio_samples/skyrim_one_they_fear.mp3")
			.expect("File open error"),
		de_send,
	));

	let encode_task = tokio::spawn(encode::encode_music(en_recv));

	let (_a, _b) = (decode_task.await, encode_task.await);
}
