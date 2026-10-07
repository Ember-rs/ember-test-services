use std::{
    collections::BTreeMap,
    sync::{atomic::{AtomicU64, Ordering}, Arc, RwLock},
};

use scafra::prelude::*;

use crate::models::pet::{Pet, PetRequest, PetStatus};

#[service]
pub struct PetService {
    pets: Arc<RwLock<BTreeMap<u64, Pet>>>,
    next_id: Arc<AtomicU64>,
}

impl PetService {
    pub fn list(&self, status: Option<PetStatus>, tags: &[String]) -> Result<Vec<Pet>> {
        let pets = self
            .pets
            .read()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?;

        Ok(pets
            .values()
            .filter(|pet| status.is_none_or(|wanted| pet.status == wanted))
            .filter(|pet| tags.iter().all(|tag| pet.tags.contains(tag)))
            .cloned()
            .collect())
    }

    pub fn find(&self, id: u64) -> Result<Pet> {
        let pets = self
            .pets
            .read()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?;
        pets.get(&id).cloned().ok_or(AppError::NotFound)
    }

    pub fn create(&self, request: PetRequest) -> Result<Pet> {
        validate_name(&request.name)?;
        let id = self.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let pet = Pet {
            id,
            name: request.name,
            photo_urls: request.photo_urls,
            tags: request.tags,
            status: request.status,
        };
        self.pets
            .write()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?
            .insert(id, pet.clone());
        Ok(pet)
    }

    pub fn update(&self, id: u64, request: PetRequest) -> Result<Pet> {
        validate_name(&request.name)?;
        let pet = Pet {
            id,
            name: request.name,
            photo_urls: request.photo_urls,
            tags: request.tags,
            status: request.status,
        };
        let mut pets = self
            .pets
            .write()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?;
        if !pets.contains_key(&id) {
            return Err(AppError::NotFound);
        }
        pets.insert(id, pet.clone());
        Ok(pet)
    }

    pub fn delete(&self, id: u64) -> Result<()> {
        let removed = self
            .pets
            .write()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?
            .remove(&id)
            .is_some();
        if removed {
            Ok(())
        } else {
            Err(AppError::NotFound)
        }
    }
}

fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        return Err(AppError::Validation("name must not be empty".to_owned()));
    }
    if name.chars().count() > 120 {
        return Err(AppError::Validation(
            "name must be at most 120 characters".to_owned(),
        ));
    }
    Ok(())
}
