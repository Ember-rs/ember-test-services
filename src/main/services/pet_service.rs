use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

use scafra::prelude::*;

use crate::models::pet::{Pet, PetRequest, PetStatus};

#[service]
pub struct PetService {
    store: Arc<PetStore>,
}

pub struct PetStore {
    pets: RwLock<BTreeMap<u64, Pet>>,
    next_id: AtomicU64,
}

static ACTIVE_PET_STORES: AtomicUsize = AtomicUsize::new(0);

#[bean]
pub fn pet_store() -> PetStore {
    ACTIVE_PET_STORES.fetch_add(1, Ordering::AcqRel);
    PetStore {
        pets: RwLock::new(BTreeMap::new()),
        next_id: AtomicU64::new(0),
    }
}

async fn pet_store_ready() -> bool {
    ACTIVE_PET_STORES.load(Ordering::Acquire) > 0
}

// Scafra's health hooks are process-wide, so this reports whether any live
// PetStore exists in the process rather than identifying a specific graph.
register_health_check!("pet-store", pet_store_ready);

impl Drop for PetStore {
    fn drop(&mut self) {
        ACTIVE_PET_STORES.fetch_sub(1, Ordering::AcqRel);
    }
}

impl PetService {
    pub fn list(&self, status: Option<PetStatus>, tags: &[String]) -> Result<Vec<Pet>> {
        let pets = self
            .store
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
            .store
            .pets
            .read()
            .map_err(|_| AppError::Internal("pet store unavailable".to_owned()))?;
        pets.get(&id).cloned().ok_or(AppError::NotFound)
    }

    pub fn create(&self, request: PetRequest) -> Result<Pet> {
        validate_name(&request.name)?;
        let id = self.store.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let pet = Pet {
            id,
            name: request.name,
            photo_urls: request.photo_urls,
            tags: request.tags,
            status: request.status,
        };
        self.store
            .pets
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
            .store
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
            .store
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

#[cfg(test)]
mod tests {
    use super::{pet_store, pet_store_ready};

    #[tokio::test]
    async fn pet_store_readiness_tracks_live_instances() {
        assert!(!pet_store_ready().await);

        let store = pet_store();
        assert!(pet_store_ready().await);

        drop(store);
        assert!(!pet_store_ready().await);
    }
}
