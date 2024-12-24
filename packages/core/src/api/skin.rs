use std::collections::HashMap;

use uuid::Uuid;

use crate::{store::MinecraftSkin, State};

#[tracing::instrument]
pub async fn get_skins() -> crate::Result<HashMap<Uuid, Vec<MinecraftSkin>>> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_skins().await?)
}

#[tracing::instrument]
pub async fn get_skin(profile_id: Uuid, uuid: Uuid) -> crate::Result<MinecraftSkin> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_skin(profile_id, uuid).await?)
}

#[tracing::instrument]
pub async fn add_skin(profile_id: Uuid, skin: MinecraftSkin) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.add_skin(profile_id, skin).await?)
}

#[tracing::instrument]
pub async fn remove_skin(profile_id: Uuid, uuid: Uuid) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.remove_skin(profile_id, uuid).await?)
}

#[tracing::instrument]
pub async fn set_skin(profile_id: Uuid, skin: MinecraftSkin) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.set_skin(profile_id, skin).await?)
}

#[tracing::instrument]
pub async fn save_skins() -> crate::Result<()> {
	let state = State::get().await?;
	let skins = state.skin.write().await;
	Ok(skins.save().await?)
}

#[tracing::instrument]
pub async fn get_current_skin(profile_id: Uuid) -> crate::Result<Option<MinecraftSkin>> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	let skin = skins.get_current_skin(profile_id).await?;
	Ok(skin)
}

#[tracing::instrument]
pub async fn get_profile_skins(profile_id: Uuid) -> crate::Result<Vec<MinecraftSkin>> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_profile_skins(profile_id).await?)
}