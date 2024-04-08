use std::{fmt::Display, num::NonZeroU64};

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
	target_guild: Option<GuildConnection>,
	saved_guilds: Vec<GuildConnection>,
}

#[derive(Clone, Debug)]
pub(crate) enum Message {
	Connect,
	Connected,
	SelectGuild(GuildConnection),
}

impl Application for Minstrel {
	type Executor = executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
		(
			Self {
				saved_guilds: vec![GuildConnection::new("name", 100u64)],
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
			self.target_guild.clone(),
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

#[derive(PartialEq, Debug, Clone)]
pub struct GuildConnection {
	pub name: String,
	pub guild_id: NonZeroU64,
}

impl GuildConnection {
	pub fn new(name: &str, guild_id: u64) -> Self {
		GuildConnection {
			name: name.to_string(),
			guild_id: NonZeroU64::new(guild_id).unwrap(),
		}
	}
}

impl Display for GuildConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.pad(&self.name)
    }
}
