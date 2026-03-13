use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Toast {
    pub id: u32,
    pub name: String,
    pub logo_uri: String,
    pub title: String,
    pub message: String,
    pub hero_image_uri: String,
    pub inline_images: Vec<String>,
    pub tag: String,
    pub group: String,
    pub creation_time: String,
    pub fingerprint: String,
    pub fingerprint_without_time: String,
}
