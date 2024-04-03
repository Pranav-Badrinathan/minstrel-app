use iced::{
	executor,
	widget::{
		button, column, container, horizontal_space, pick_list, row, scrollable, vertical_space,
	},
	Application, Command, Theme,
};

// Struct for holding the app state.
#[derive(Default)]
pub(crate) struct Minstrel {
	target_guild: Option<u64>,
	saved_guilds: Vec<u64>,
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
	Connect,
	Connected,
	SelectGuild(u64),
}

impl Application for Minstrel {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Self {
				saved_guilds: vec![0, 1, 2, 3, 4, 5],
				..Default::default()
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		String::from("Minstrel")
	}

	fn update(&mut self, msg: Self::Message) -> iced::Command<Self::Message> {
		match msg {
			Message::Connect => Command::perform(crate::play(), |_| Message::Connected),
			Message::Connected => Command::none(),
			Message::SelectGuild(guild) => {
				self.target_guild = Some(guild);
				Command::none()
			}
		}
	}

	fn view(&self) -> iced::Element<Self::Message> {
		// The connection set.
		let con_btn = button("Connect!").on_press(Message::Connect);
		let guild_lst = pick_list(
			self.saved_guilds.as_slice(),
			self.target_guild,
			Message::SelectGuild,
		);

		let menu_bar = row![horizontal_space(), guild_lst, con_btn]
			.padding(10)
			.spacing(5);
		let side_bar = column![vertical_space()];
		let board = scrollable(vertical_space());

		let main_view = row![side_bar, board];

		let page = column![menu_bar, main_view];

		container(page).into()
	}
}
