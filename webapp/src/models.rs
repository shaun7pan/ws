use serde::{Deserialize, Serialize};

//view models
#[derive(Serialize, Deserialize, Debug)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TearcherResponse {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String,
}
