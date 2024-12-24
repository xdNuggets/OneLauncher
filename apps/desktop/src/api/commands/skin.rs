use std::collections::HashMap;

use onelauncher::{skin, store::MinecraftSkin};
use uuid::Uuid;

#[tauri::command]
#[specta::specta]
pub async fn get_skins() -> Result<HashMap<Uuid, Vec<MinecraftSkin>>, String> {
	Ok(skin::get_skins().await?)
}

#[specta::specta]
#[tauri::command]
pub async fn get_skin(profile_id: Uuid, uuid: Uuid) -> Result<MinecraftSkin, String> {
	Ok(skin::get_skin(profile_id, uuid).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn add_skin(profile_id: Uuid, skin: MinecraftSkin) -> Result<(), String> {
	Ok(skin::add_skin(profile_id, skin).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn remove_skin(profile_id: Uuid, uuid: Uuid) -> Result<(), String> {
	Ok(skin::remove_skin(profile_id, uuid).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn set_skin(profile_id: Uuid, skin: MinecraftSkin) -> Result<(), String> {
	Ok(skin::set_skin(profile_id, skin).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn save_skins() -> Result<(), String> {
	Ok(skin::save_skins().await?)
}

#[specta::specta]
#[tauri::command]
pub async fn get_current_skin(profile_id: Uuid) -> Result<Option<MinecraftSkin>, String> {
	Ok(skin::get_current_skin(profile_id).await?)
}

#[specta::specta]
#[tauri::command]
pub async fn get_profile_skins(profile_id: Uuid) -> Result<Vec<MinecraftSkin>, String> {
	Ok(skin::get_profile_skins(profile_id).await?)
}