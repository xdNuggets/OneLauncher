use onelauncher::{skin, store::MinecraftSkin};
use uuid::Uuid;

#[tauri::command]
#[specta::specta]
pub async fn get_skins() -> Result<Vec<MinecraftSkin>, String> {
	Ok(skin::get_skins().await?)
}

#[specta::specta]
#[tauri::command]
pub async fn get_skin(uuid: Uuid) -> Result<MinecraftSkin, String> {
	Ok(skin::get_skin(uuid).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn add_skin(skin: MinecraftSkin) -> Result<(), String> {
	Ok(skin::add_skin(skin).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn remove_skin(uuid: Uuid) -> Result<(), String> {
	Ok(skin::remove_skin(uuid).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn set_skin(skin: MinecraftSkin) -> Result<(), String> {
	Ok(skin::set_skin(skin).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn save_skins() -> Result<(), String> {
	Ok(skin::save_skins().await?)
}

#[specta::specta]
#[tauri::command]
pub async fn get_current_skin() -> Result<Option<MinecraftSkin>, String> {
	Ok(skin::get_current_skin().await?)
}