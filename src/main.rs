#![allow(non_snake_case)]

use std::fs::File;
use dioxus::prelude::*;

mod frame;
mod decode;
mod encode;

// For Dioxus Components

#[tokio::main]
async fn main() {
    println!("Howdy.");

	dioxus_desktop::launch(App);
}

async fn play() {
	let (de_send, en_recv) = tokio::sync::mpsc::channel(1);

	let decode_task = tokio::spawn(
		decode::decode_music(
			File::open("audio_samples/skyrim from past to present 48khz.mp3")
				.expect("File open error"), de_send
	));

	let encode_task = tokio::spawn(
		encode::encode_music(en_recv)
	);

	let (_a, _b) = (decode_task.await, encode_task.await);
}

fn App(cx: Scope) -> Element {
	cx.render(rsx! {
		button {
			onclick: move |_| play(),
			"Play"
		}
	})
}
