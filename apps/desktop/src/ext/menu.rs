use tauri::menu::{Menu, MenuItemKind};
use tauri::{Manager, Wry};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, specta::Type, Deserialize, strum::EnumString, strum::AsRefStr, strum::Display)]
pub enum MenuEvent {
	NewCluster,
	AddMod,
	OpenMods,
	OpenScreenshots,
	OpenSettings,
	ReloadWebview,
	ToggleDeveloperTools,
}

const CLUSTER_LOCKED_MENU_IDS: &[MenuEvent] = &[
	MenuEvent::AddMod,
	MenuEvent::OpenMods,
	MenuEvent::OpenScreenshots,
];

pub fn setup_menu(handle: &tauri::AppHandle) -> tauri::Result<Menu<Wry>> {
	handle.on_menu_event(move |handle, event| {
		if let Ok(event) = MenuEvent::from_str(&event.id().0) {
			handle_menu_event(event, handle);
		} else {
			println!("unknown menuevent: {}", event.id().0);
		}
	});

	#[cfg(not(target_os = "macos"))]
	{
		Menu::new(handle)
	}

	#[cfg(target_os = "macos")]
	{
		use tauri::menu::{AboutMetadataBuilder, MenuBuilder, MenuItemBuilder, SubmenuBuilder};

		let app_menu = SubmenuBuilder::new(handle, "OneLauncher")
			.about(Some(
				AboutMetadataBuilder::new()
					.authors(Some(vec!["Polyfrost".to_string()]))
					.license(Some(onelauncher::constants::VERSION))
					.version(Some(onelauncher::constants::VERSION))
					.website(Some("https://polyfrost.org/"))
					.website_label(Some("Polyfrost.org"))
					.build(),
			))
			.separator()
			.item(
				&MenuItemBuilder::with_id(MenuEvent::NewCluster, "New Cluster")
					.accelerator("Cmd+Shift+C")
					.build(handle)?,
			)
			.separator()
			.hide()
			.hide_others()
			.show_all()
			.separator()
			.quit()
			.build()?;

		let view_menu = SubmenuBuilder::new(handle, "View")
			.item(
				&MenuItemBuilder::with_id(MenuEvent::OpenMods, "Mods")
					.accelerator("CmdOrCtrl+M")
					.build(handle)?,
			)
			.item(
				&MenuItemBuilder::with_id(MenuEvent::OpenScreenshots, "Screenshots")
					.accelerator("CmdOrCtrl+S")
					.build(handle)?,
			)
			.item(
				&MenuItemBuilder::with_id(MenuEvent::OpenSettings, "Settings")
					.accelerator("CmdOrCtrl+Comma")
					.build(handle)?,
			);

		#[cfg(debug_assertions)]
		let view_menu = view_menu.separator().item(
			&MenuItemBuilder::with_id(MenuEvent::ToggleDeveloperTools, "Toggle Developer Tools")
				.accelerator("CmdOrCtrl+Shift+Alt+I")
				.build(handle)?,
		);

		let view_menu = view_menu.build()?;

		let window_menu = SubmenuBuilder::new(handle, "Window")
			.minimize()
			.close_window()
			.fullscreen()
			.item(
				&MenuItemBuilder::with_id(MenuEvent::ReloadWebview, "Reload Webview")
					.accelerator("CmdOrCtrl+Shift+R")
					.build(handle)?,
			)
			.build()?;

		let menu = MenuBuilder::new(handle)
			.item(&app_menu)
			.item(&view_menu)
			.item(&window_menu)
			.build()?;

		for event in CLUSTER_LOCKED_MENU_IDS {
			set_enabled(&menu, *event, false);
		}

		Ok(menu)
	}
}

pub fn handle_menu_event(event: MenuEvent, handle: &tauri::AppHandle) {
	let webview = handle.get_webview_window("main").expect("failed to find window");

	// todo: use tauri specta instead of this
	match event {
		MenuEvent::NewCluster => webview.emit("keybind", "new_cluster").unwrap(),
		MenuEvent::AddMod => webview.emit("keybind", "add_mod").unwrap(),
		MenuEvent::OpenMods => webview.emit("keybind", "open_mods").unwrap(),
		MenuEvent::OpenScreenshots => webview.emit("keybind", "open_screenshots").unwrap(),
		MenuEvent::OpenSettings => webview.emit("keybind", "open_settings").unwrap(),
		MenuEvent::ReloadWebview => webview.with_webview(crate::api::commands::reload_webview_inner).expect("failed to reload webview"),
		MenuEvent::ToggleDeveloperTools => {
			#[cfg(feature = "devtools")]
			if webview.is_devtool_open() {
				webview.close_devtools();
			} else {
				webview.open_devtools();
			}
		},
	}
}

pub fn refresh_menu_bar(handle: &tauri::AppHandle, enabled: bool) {
	let menu = handle
		.get_webview_window("main")
		.expect("unable to find window")
		.menu()
		.expect("unable to get menu");

	for event in CLUSTER_LOCKED_MENU_IDS {
		set_enabled(&menu, *event, enabled);
	}
}

pub fn set_enabled(menu: &Menu<Wry>, event: MenuEvent, enabled: bool) {
	let result = match menu.get(event.as_ref()) {
		Some(MenuItemKind::MenuItem(i)) => i.set_enabled(enabled),
		Some(MenuItemKind::Submenu(i)) => i.set_enabled(enabled),
		Some(MenuItemKind::Predefined(_)) => return,
		Some(MenuItemKind::Check(i)) => i.set_enabled(enabled),
		Some(MenuItemKind::Icon(i)) => i.set_enabled(enabled),
		None => {
			tracing::error!("failed to get menu item: {event:?}");
			return;
		}
	};

	if let Err(e) = result {
		tracing::error!("failed to set menu item state: {e:#?}");
	}
}
