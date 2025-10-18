use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Service {
    pub name: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateService {
    pub name: String,
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceResponse {
    pub name: String,
}

impl Service {
    pub fn new(create_data: CreateService) -> Self {
        Self {
            name: create_data.name,
            link: create_data.link,
        }
    }
    pub fn to_response(&self) -> ServiceResponse {
        ServiceResponse {
            name: self.name.clone(),
        }
    }
}
