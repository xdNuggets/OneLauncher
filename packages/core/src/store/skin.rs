use std::{collections::HashMap, fs::{self, File}, io::Write};
use uuid::Uuid;

use crate::ErrorKind;

use super::{Directories, MinecraftSkin};

#[derive(Debug, Clone)]
pub struct SkinController {
	pub skins: HashMap<Uuid, Vec<MinecraftSkin>>,
}

impl SkinController {
	#[tracing::instrument]
	pub async fn initialize() -> Result<SkinController, crate::Error> {
		// Get all existing skins from the database; else create a new vector.
		let skins_path = Directories::init_skins_file()?;
		let file_exists = skins_path.exists();
		if !file_exists {
			let mut file = File::create(&skins_path)?;
			file.write(b"{}")?;
			Ok(SkinController {
				skins: HashMap::new(),
			})

		} else {
			// Do stuff if it exists. Fill the cache with all files
			let skins_content = fs::read_to_string(skins_path)?;
			let skins_lib: HashMap<Uuid, Vec<MinecraftSkin>> = serde_json::from_str(&skins_content)?;
			let mut skins = HashMap::new();
			for profile in skins_lib.keys() {
				let profile_skins = skins_lib.get(profile).unwrap();
				let mut skins_vec = Vec::new();
				for skin in profile_skins {
					skins_vec.push(skin.clone());
				}
				skins.insert(profile.clone(), skins_vec);
			}


			Ok(SkinController { skins })
		}

	}

	/// Gets all profile libraries from the cache.
	#[tracing::instrument]
	pub async fn get_skins(&self) -> crate::Result<HashMap<Uuid, Vec<MinecraftSkin>>> {
		Ok(self.skins.clone())
	}

	#[tracing::instrument]
	pub async fn add_profile(&mut self, profile_id: Uuid) -> crate::Result<()> {
		self.skins.insert(profile_id, Vec::new());
		self.save().await?;
		Ok(())
	}

	#[tracing::instrument]
	pub async fn get_profile_skins(&self, profile_id: Uuid) -> crate::Result<Vec<MinecraftSkin>> {
		let skins = self.skins.get(&profile_id);
		match skins {
			Some(skins) => Ok(skins.clone()),
			None => Err(ErrorKind::SkinError("Profile not found".to_string()).into()),
		}
	}

	pub async fn save(&self) -> crate::Result<()> {
		let json_obj = serde_json::to_string_pretty(&self.skins).unwrap();
		let skins_path = Directories::init_skins_file()?;
		fs::write(skins_path, json_obj).unwrap();
		Ok(())
	}

	#[tracing::instrument]
	pub async fn get_skin(&self, profile_id: Uuid, uuid: Uuid) -> crate::Result<MinecraftSkin> {
		let profile_skins = self.skins.iter().find(|profile| profile.0.clone() == profile_id);
		for (_key, values) in profile_skins {
			let skin = values.iter().find(|skin| skin.id == uuid);
			match skin {
				Some(skin) => return Ok(skin.clone()),
				None => return Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
			}
		}

		Err(ErrorKind::SkinError("Skin not found".to_string()).into())
	}

	#[tracing::instrument]
	pub async fn add_skin(&mut self, profile_id: Uuid, skin: MinecraftSkin) -> crate::Result<()> {
		let profile_skins = self.skins.iter_mut().find(|profile| profile.0.clone() == profile_id);
		profile_skins.unwrap().1.push(skin);
		self.save().await?;
		Ok(())
	}

	#[tracing::instrument]
	pub async fn set_skin(&mut self, profile_id: Uuid, skin: MinecraftSkin) -> crate::Result<()> {
		// Unset the current skin for the profile
		let mut skins = self.skins.clone();
		let profile_skins = skins.iter_mut().find(|profile| profile.0.clone() == profile_id);
		for (_key, values) in profile_skins {
			for skin in values.iter_mut() {
				skin.current = false;
			}

			// Set the new skin as the current skin
			let skin = values.iter_mut().find(|skin| skin.id == skin.id);

			match skin {
				Some(skin) => {
					skin.current = true;
					self.skins.insert(profile_id, values.clone());
					self.save().await?;
					return Ok(());
				},
				None => return Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
			}

		}
		Err(ErrorKind::SkinError("Error saving skin".to_string()).into())

	}

	pub async fn remove_skin(&mut self, profile_id: Uuid, uuid: Uuid) -> crate::Result<()> {
		let mut skins = self.skins.clone();
		let profile_skins = skins.iter_mut().find(|profile| profile.0.clone() == profile_id);
		for(_key, values) in profile_skins {
			let skin = values.iter().find(|skin| skin.id == uuid);
			match skin {
				Some(skin) => {
					let index = values.iter().position(|s| s.id == skin.id).unwrap();
					values.remove(index);
					self.skins.insert(profile_id, values.clone());
					self.save().await?;
					return Ok(());
				},
				None => return Err(ErrorKind::SkinError("Skin not found".to_string()).into()),
			}
		}
		Err(ErrorKind::SkinError("Error removing skin".to_string()).into())
	}

	pub async fn get_current_skin(&mut self, profile_id: Uuid) -> crate::Result<Option<MinecraftSkin>> {
		let profile_skins = self.skins.iter().find(|profile| profile.0.clone() == profile_id);
		for (_key, values) in profile_skins {
			let current_skin = values.iter().find(|skin| skin.current);
			return Ok(current_skin.map(|skin| skin.clone()));
		}
		Err(ErrorKind::SkinError("Error getting current skin".to_string()).into())
	}

}
