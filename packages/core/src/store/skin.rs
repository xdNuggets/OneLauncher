use std::{fs::{self}, io::Error};
use uuid::Uuid;
use base64::encode;

use crate::ErrorKind;

use super::MinecraftSkin;

#[derive(Debug, Clone)]
pub struct SkinController {
	pub skins: Vec<MinecraftSkin>,
	pub current_skin: Option<MinecraftSkin>,
}

impl SkinController {
	#[tracing::instrument]
	pub async fn initialize() -> Result<SkinController, Error> {
		// Get all existing skins from the database; else create a new vector.
		let skins_dir = std::path::Path::new("skins");
		if !skins_dir.exists() {
			fs::create_dir(skins_dir)?;
			Ok(SkinController {
				skins: Vec::new(),
				current_skin: None,
			})

		} else {
			// Do stuff if it exists. Fill the cache with all files
			let skins = fs::read_dir(skins_dir)?;
			let mut skins_vec = Vec::new();

			for skin_entry in skins {
				let entry = skin_entry?;
				let file_name = entry.file_name().into_string().unwrap();
				let split = file_name.split('_').collect::<Vec<&str>>();
				let mc_skin = MinecraftSkin {
					id: Uuid::parse_str(split.first().unwrap()).unwrap(),
					name: split.last().unwrap().to_string(),
					src: encode(fs::read(entry.path())?), // Deprecated but im too lazy to use a better method
				};
				skins_vec.push(mc_skin);
			}

			let current_skin = skins_vec.iter().find(|skin| skin.name.contains("current_skin")).cloned();

			Ok(SkinController { skins: skins_vec, current_skin })
		}

	}

	#[tracing::instrument]
	pub async fn get_skins(&self) -> crate::Result<Vec<MinecraftSkin>> {
		Ok(self.skins.clone())
	}

	#[tracing::instrument]
	pub async fn get_skin(&self, uuid: Uuid) -> crate::Result<MinecraftSkin> {
		let skin = self.skins.iter().find(|skin| skin.id == uuid);
		match skin {
			Some(skin) => Ok(skin.clone()),
			None => Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
		}
	}

	#[tracing::instrument]
	pub async fn add_skin(&mut self, skin: MinecraftSkin) -> crate::Result<()> {
		self.skins.push(skin);
		Ok(())
	}

	#[tracing::instrument]
	pub async fn set_skin(&mut self, skin: MinecraftSkin) -> crate::Result<()> {
		let index = self.skins.iter().position(|s: &MinecraftSkin| s.id == skin.id);
		match index {
			Some(_index) => {
				self.current_skin = Some(skin);
				Ok(())
			}
			None => Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
		}
	}

	pub async fn remove_skin(&mut self, uuid: Uuid) -> crate::Result<()> {
		let index = self.skins.iter().position(|skin| skin.id == uuid);
		match index {
			Some(index) => {
				self.skins.remove(index);
				Ok(())
			}
			None => Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
		}
	}
}
