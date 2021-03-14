#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::RwLock;

use tauri::{AppBuilder, Webview};
extern crate webview_official;

mod show;
mod settings;

mod appdata;
use appdata::AppData;
use appdata::AppDataPlugin;

mod commands;

mod workshop;
use transactions::Transactions;
use workshop::Workshop;

mod base64_image;
pub(crate) use base64_image::Base64Image;

mod game_addons;
use game_addons::GameAddons;

mod transactions;

pub(crate) mod lib;

use lazy_static::lazy_static;
lazy_static! {
	pub(crate) static ref WORKSHOP: RwLock<Workshop> = RwLock::new(match Workshop::init() {
		Ok(workshop) => workshop,
		Err(error) => {
			show::panic(format!("Couldn't initialize the Steam API! Is Steam running?\nError: {:#?}", error));
			panic!();
		},
	});

	pub(crate) static ref APP_DATA: RwLock<AppData> = RwLock::new(match AppData::init(WORKSHOP.read().unwrap().get_user()) {
		Ok(app_data) => app_data,
		Err(error) => {
			show::panic(format!("{:#?}", error));
			panic!();
		}
	});

	pub(crate) static ref GAME_ADDONS: RwLock<GameAddons> = RwLock::new(GameAddons::init());
	
	pub(crate) static ref TRANSACTIONS: RwLock<Transactions> = RwLock::new(Transactions::init());
}

fn main() {
	// TODO use steam api to get gmod dir instead of steamlocate

	let window_size = APP_DATA.read().unwrap().settings.window_size.clone();
	let mut first_setup = true;
	let setup = move |webview: &mut Webview, _: String| {
		webview.set_title(&format!("gmpublisher v{}", env!("CARGO_PKG_VERSION")));

		if first_setup {
			webview.set_size(500, 500, webview_official::SizeHint::MIN);
			webview.set_size(std::cmp::max(window_size.0, 500), std::cmp::max(window_size.1, 500), webview_official::SizeHint::NONE);

			drop(window_size);
			first_setup = false;
		}
	};

	AppBuilder::new()
		.setup(setup)
		.plugin(AppDataPlugin::init())
		.invoke_handler(commands::invoke_handler())
		.build().run();
}