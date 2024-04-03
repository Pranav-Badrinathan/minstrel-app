use iced::{
	executor,
	widget::{button, container, horizontal_space, row},
	Application, Command, Theme,
};

// Struct for holding the app state.
pub(crate) struct Minstrel;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
	Connect,
	Connected,
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
			Message::Connect => Command::perform(crate::play(), |_| Message::Connected),
			Message::Connected => Command::none(),
		}
	}

	fn view(&self) -> iced::Element<Self::Message, Self::Theme, iced::Renderer> {
		let con_btn = button("Connect!").on_press(Message::Connect);

		let menu_bar = row![horizontal_space(), con_btn].padding(10);

		container(menu_bar).into()
	}
}
