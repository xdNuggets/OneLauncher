use std::{fs::{self, File}, io::Write};
use uuid::Uuid;

use crate::ErrorKind;

use super::{Directories, MinecraftSkin};

#[derive(Debug, Clone)]
pub struct SkinController {
	pub skins: Vec<MinecraftSkin>,
	pub current_skin: Option<MinecraftSkin>,
}

impl SkinController {
	#[tracing::instrument]
	pub async fn initialize() -> Result<SkinController, crate::Error> {
		// Get all existing skins from the database; else create a new vector.
		let skins_path = Directories::init_skins_file()?;
		let file_exists = skins_path.exists();
		if !file_exists {
			let mut file = File::create(&skins_path)?;
			file.write(b"[]")?;
			Ok(SkinController {
				skins: Vec::new(),
				current_skin: None,
			})



		} else {
			// Do stuff if it exists. Fill the cache with all files
			let skins_content = fs::read_to_string(skins_path)?;
			let skins_vec: Vec<MinecraftSkin> = serde_json::from_str(&skins_content)?;

			let mut skins = Vec::new();
			for skin_entry in skins_vec {
				skins.push(skin_entry);
			}
			let current_skin = skins.iter().find(|skin| skin.current).cloned();

			Ok(SkinController { skins, current_skin })
		}

	}

	#[tracing::instrument]
	pub async fn get_skins(&self) -> crate::Result<Vec<MinecraftSkin>> {
		Ok(self.skins.clone())
	}

	pub async fn save(&self) -> crate::Result<()> {
		let json_obj = serde_json::to_string_pretty(&self.skins).unwrap();
		let skins_path = Directories::init_skins_file()?;
		fs::write(skins_path, json_obj).unwrap();
		Ok(())
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
		self.save().await?;
		Ok(())
	}

	#[tracing::instrument]
	pub async fn set_skin(&mut self, skin: MinecraftSkin) -> crate::Result<()> {
		// unset the current saved skin
		if let Some(current) = self.skins.iter_mut().find(|s| s.current) {
			current.current = false;
		}

		// update the target skin
		if let Some(target) = self.skins.iter_mut().find(|s| s.id == skin.id) {
			target.current = true;
			self.save().await?;
			Ok(())
		} else {
			Err(ErrorKind::SkinError("Skin not found".to_string()).into())
		}
	}

	pub async fn remove_skin(&mut self, uuid: Uuid) -> crate::Result<()> {
		let skin_to_remove = self.skins.iter().find(|skin| skin.id == uuid);
		match skin_to_remove {
			Some(skin) => {
				self.skins.retain(|skin| skin.id != uuid);
				self.save().await?;
				Ok(())
			}
			None => Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
		}
	}

	pub async fn get_current_skin(&mut self) -> crate::Result<Option<MinecraftSkin>> {
		let skin = self.skins.iter_mut().find(|s| s.current);
		Ok(skin.cloned())
	}

}
