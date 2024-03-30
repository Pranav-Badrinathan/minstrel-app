use std::fs::File;

use iced::{executor, widget::button, Application, Command, Settings, Theme};

mod frame;
mod decode;
mod encode;

// Struct for holding the app state.
struct Minstrel;

#[derive(Clone, Copy, Debug)]
enum Message {
	Connect,
	Connected
}

impl Application for Minstrel {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        String::from("Minstrel")
    }

	fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
		match message {
			Message::Connect => Command::perform(play(), |_| Message::Connected),
			Message::Connected => Command::none(),
		}
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
		button("Connect!").on_press(Message::Connect).into()
    }
}

fn main() -> iced::Result {
    println!("Howdy.");
	Minstrel::run(Settings::default())
}

async fn play() {
	let (de_send, en_recv) = tokio::sync::mpsc::channel(1);

	let decode_task = tokio::spawn(
		decode::decode_music(
			File::open("audio_samples/skyrim_one_they_fear.mp3")
				.expect("File open error"), de_send
	));

	let encode_task = tokio::spawn(
		encode::encode_music(en_recv)
	);

	let (_a, _b) = (decode_task.await, encode_task.await);
}

