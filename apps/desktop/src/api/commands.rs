use std::path::PathBuf;
use std::str::FromStr;

use interpulse::api::minecraft::Version;
use onelauncher::constants::{NATIVE_ARCH, TARGET_OS, VERSION};
use onelauncher::data::{Loader, ManagedPackage, MinecraftCredentials, PackageData, Settings};
use onelauncher::package::content;
use onelauncher::store::{Cluster, ClusterPath};
use onelauncher::{cluster, minecraft, processor, settings};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use uuid::Uuid;

// todo: use rspc for commands that don't actually require `tauri` as a dependency
#[macro_export]
macro_rules! collect_commands {
	() => {{
		use $crate::api::commands::*;
		use $crate::ext::updater::*;
		tauri_specta::ts::builder()
			.config(specta::ts::ExportConfig::default().bigint(specta::ts::BigIntExportBehavior::BigInt))
			.commands(tauri_specta::collect_commands![
				// User
				auth_login,
				get_users,
				get_user,
				get_default_user,
				set_default_user,
				remove_user,
				// Cluster
				create_cluster,
				remove_cluster,
				get_cluster,
				get_clusters,
				run_cluster,
				// Processor
				get_running_clusters,
				get_processes_by_path,
				kill_process,
				// Settings
				get_settings,
				set_settings,
				// Metadata
				get_minecraft_versions,
				// Package
				random_mods,
				get_mod,
				download_mod,
				// Other
				get_program_info,
				reload_webview,
				set_menu_bar_item_state,
				// Updater
				check_for_update,
				install_update,
			])
	}};
}

#[derive(Serialize, Deserialize, Type)]
pub struct CreateCluster {
	name: String,
	mc_version: String,
	mod_loader: Loader,
	loader_version: Option<String>,
	icon: Option<PathBuf>,
	icon_url: Option<String>,
	package_data: Option<PackageData>,
	skip: Option<bool>,
	skip_watch: Option<bool>,
}

#[tauri::command(async)]
#[specta::specta]
pub async fn set_menu_bar_item_state(window: tauri::Window, event: crate::ext::menu::MenuEvent, enabled: bool) {
	let menu = window.menu().expect("failed to get menu for current window");
	crate::ext::menu::set_enabled(&menu, event, enabled);
}

#[tauri::command(async)]
#[specta::specta]
pub async fn reload_webview(handle: tauri::AppHandle) {
	handle
		.get_webview_window("main")
		.expect("failed to get window handle")
		.with_webview(reload_webview_inner)
		.expect("failed to reload webview");
}

pub fn reload_webview_inner(webview: tauri::webview::PlatformWebview) {
	#[cfg(target_os = "macos")]
	{
		unsafe {
			onelauncher_macos::reload_webview(&webview.inner().cast());
		}
	}

	#[cfg(target_os = "linux")]
	{
		use webkit2gtk::WebViewExt;
		webview.inner().reload();
	}

	#[cfg(target_os = "windows")]
	unsafe {
		webview
			.controller()
			.CoreWebView2()
			.expect("failed to get inner webview handle")
			.Reload()
			.expect("failed to reload webview");
	}
}

#[specta::specta]
#[tauri::command(async)]
pub async fn create_cluster(props: CreateCluster) -> crate::api::Result<Option<Uuid>> {
	let path = cluster::create::create_cluster(
		props.name,
		props.mc_version,
		props.mod_loader,
		props.loader_version,
		props.icon,
		props.icon_url,
		props.package_data,
		props.skip,
		props.skip_watch,
	)
	.await?;

	if let Some(cluster) = cluster::get(&path, None).await? {
		Ok(Some(cluster.uuid))
	} else {
		Ok(None)
	}
}

#[specta::specta]
#[tauri::command(async)]
pub async fn remove_cluster(uuid: Uuid) -> crate::api::Result<()> {
	let path = ClusterPath::find_by_uuid(uuid).await?;
	Ok(cluster::remove(&path).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn run_cluster(uuid: Uuid) -> crate::api::Result<(Uuid, u32)> {
	let path = ClusterPath::find_by_uuid(uuid).await?;
	let c_lock = cluster::run(&path).await?;

	let p_uuid = c_lock.read().await.uuid;
	let p_pid = c_lock
		.read()
		.await
		.current_child
		.read()
		.await
		.id()
		.unwrap_or(0);

	Ok((p_uuid, p_pid))
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_running_clusters() -> crate::api::Result<Vec<Cluster>> {
	Ok(processor::get_running_clusters().await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_processes_by_path(path: ClusterPath) -> crate::api::Result<Vec<Uuid>> {
	Ok(processor::get_uuids_by_cluster_path(path).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn kill_process(uuid: Uuid) -> crate::api::Result<()> {
	processor::kill_by_uuid(uuid).await?;
	Ok(())
}

// #[specta::specta]
// #[tauri::command]
// pub fn update_cluster(cluster: Cluster) -> Result<(), String> {

// }

#[specta::specta]
#[tauri::command(async)]
pub async fn get_cluster(uuid: Uuid) -> crate::api::Result<Option<Cluster>> {
	Ok(cluster::get_by_uuid(uuid, None).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_clusters() -> crate::api::Result<Vec<Cluster>> {
	Ok(cluster::list(None).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_minecraft_versions() -> crate::api::Result<Vec<Version>> {
	Ok(onelauncher::api::metadata::get_minecraft_versions()
		.await?
		.versions)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_settings() -> crate::api::Result<Settings> {
	Ok(settings::get().await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn set_settings(settings: Settings) -> crate::api::Result<()> {
	Ok(settings::set(settings).await?)
}

#[derive(Serialize, Deserialize, Type)]
pub struct ProgramInfo {
	launcher_version: String,
	webview_version: String,
	tauri_version: String,
	dev_build: bool,
	platform: String,
	arch: String,
}

#[specta::specta]
#[tauri::command(async)]
pub fn get_program_info() -> ProgramInfo {
	let webview_version = tauri::webview_version().unwrap_or("UNKNOWN".into());
	let tauri_version = tauri::VERSION;
	let dev_build = tauri::is_dev();

	ProgramInfo {
		launcher_version: VERSION.into(),
		webview_version,
		tauri_version: tauri_version.into(),
		dev_build,
		platform: TARGET_OS.into(),
		arch: NATIVE_ARCH.into(),
	}
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_users() -> crate::api::Result<Vec<MinecraftCredentials>> {
	Ok(minecraft::users().await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_user(uuid: Uuid) -> crate::api::Result<MinecraftCredentials> {
	Ok(minecraft::get_user(uuid).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_default_user() -> crate::api::Result<Option<MinecraftCredentials>> {
	let uuid = minecraft::get_default_user().await?;

	match uuid {
		Some(uuid) => Ok(Some(minecraft::get_user(uuid).await?)),
		None => Ok(None),
	}
}

#[specta::specta]
#[tauri::command(async)]
pub async fn set_default_user(uuid: Uuid) -> crate::api::Result<()> {
	minecraft::set_default_user(uuid).await?;
	Ok(())
}

#[specta::specta]
#[tauri::command(async)]
pub async fn auth_login(handle: AppHandle) -> crate::api::Result<Option<MinecraftCredentials>> {
	let flow = minecraft::begin().await?;
	let timestamp = chrono::Utc::now();

	if let Some(window) = handle.get_webview_window("signin") {
		window.close()?;
	}

	tracing::info!("init webview mod {}", flow.redirect_uri);
	let url = tauri::Url::from_str(&flow.redirect_uri).unwrap();
	let url = tauri::WebviewUrl::External(url);
	let window = tauri::WebviewWindowBuilder::new(&handle, "signin", url)
		.title("Log into OneLauncher")
		.always_on_top(true)
		.center()
		.build()
		.unwrap();

	tracing::info!("requests user attention");
	window.request_user_attention(Some(tauri::UserAttentionType::Critical))?;

	tracing::info!("beginning to check for updates");
	while (chrono::Utc::now() - timestamp) < chrono::Duration::minutes(10) {
		if window.title().is_err() {
			return Ok(None);
		}

		if window.url()?.as_str().starts_with("https://login.live.com/oauth20_desktop.srf") {
			if let Some((_, code)) = window.url()?.query_pairs().find(|x| x.0 == "code") {
				window.close()?;
				let value = minecraft::finish(&code.clone(), flow).await?;

				return Ok(Some(value));
			}
		}

		tokio::time::sleep(std::time::Duration::from_millis(50)).await;
	}

	window.close()?;

	Ok(None)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn remove_user(uuid: Uuid) -> crate::api::Result<()> {
	Ok(minecraft::remove_user(uuid).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn random_mods() -> crate::api::Result<Vec<ManagedPackage>> {
	let provider = content::Providers::Modrinth;
	Ok(provider.list().await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn get_mod(project_id: String) -> crate::api::Result<ManagedPackage> {
	let provider = content::Providers::Modrinth;
	Ok(provider.get(&project_id).await?)
}

#[specta::specta]
#[tauri::command(async)]
pub async fn download_mod(cluster_id: Uuid, version_id: String) -> crate::api::Result<()> {
	let cluster = cluster::get_by_uuid(cluster_id, None)
		.await?
		.ok_or(anyhow::anyhow!("cluster not found").into())?;
	let provider = content::Providers::Modrinth;
	let game_version = cluster.meta.mc_version.clone();

	provider
		.get_version_for_game_version(&version_id, &game_version)
		.await?
		.files
		.first()
		.ok_or(anyhow::anyhow!("no files found").into())?
		.download_to_cluster(&cluster)
		.await?;

	Ok(())
}
