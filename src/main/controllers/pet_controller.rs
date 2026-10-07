use scafra::prelude::*;

use crate::{
    models::pet::{Pet, PetRequest, StatusQuery, TagsQuery},
    services::pet_service::PetService,
};

#[controller("/")]
pub struct PetController {
    service: PetService,
}

#[routes]
impl PetController {
    #[get("/pet")]
    pub async fn list(&self, query: Query<StatusQuery>) -> Result<Json<Vec<Pet>>> {
        self.service.list(query.status, &[]).map(Json)
    }

    #[get("/pet/findByStatus")]
    pub async fn find_by_status(
        &self,
        query: Query<StatusQuery>,
    ) -> Result<Json<Vec<Pet>>> {
        self.service.list(query.status, &[]).map(Json)
    }

    #[get("/pet/findByTags")]
    pub async fn find_by_tags(&self, query: Query<TagsQuery>) -> Result<Json<Vec<Pet>>> {
        self.service.list(None, &query.tags).map(Json)
    }

    #[get("/pet/{id}")]
    pub async fn get(&self, id: Path<u64>) -> Result<Json<Pet>> {
        self.service.find(id.0).map(Json)
    }

    #[post("/pet")]
    pub async fn create(&self, payload: JsonBody<PetRequest>) -> Result<(StatusCode, Json<Pet>)> {
        self.service
            .create(payload.into_inner())
            .map(|pet| (StatusCode::CREATED, Json(pet)))
    }

    #[put("/pet/{id}")]
    pub async fn update(
        &self,
        id: Path<u64>,
        payload: JsonBody<PetRequest>,
    ) -> Result<Json<Pet>> {
        self.service.update(id.0, payload.into_inner()).map(Json)
    }

    #[delete("/pet/{id}")]
    pub async fn delete(&self, id: Path<u64>) -> Result<StatusCode> {
        self.service.delete(id.0).map(|()| StatusCode::NO_CONTENT)
    }
}
