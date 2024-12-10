use uuid::Uuid;

use crate::{store::MinecraftSkin, State};

#[tracing::instrument]
pub async fn get_skins() -> crate::Result<Vec<MinecraftSkin>> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_skins().await?)
}

#[tracing::instrument]
pub async fn get_skin(uuid: Uuid) -> crate::Result<MinecraftSkin> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_skin(uuid).await?)
}

#[tracing::instrument]
pub async fn add_skin(skin: MinecraftSkin) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.add_skin(skin).await?)
}

#[tracing::instrument]
pub async fn remove_skin(uuid: Uuid) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.remove_skin(uuid).await?)
}

#[tracing::instrument]
pub async fn set_skin(skin: MinecraftSkin) -> crate::Result<()> {
	let state = State::get().await?;
	let mut skins = state.skin.write().await;
	Ok(skins.set_current_skin(skin).await?)
}

#[tracing::instrument]
pub async fn get_current_skin() -> crate::Result<Option<MinecraftSkin>> {
	let state = State::get().await?;
	let skins = state.skin.read().await;
	Ok(skins.get_current_skin().await?)
}