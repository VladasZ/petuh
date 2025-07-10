use serde::{Deserialize, Serialize};
use store::OnDisk;

pub static SAVED_RESPONSES: OnDisk<Vec<SavedResponse>> = OnDisk::new("");

#[derive(Serialize, Deserialize)]
pub struct SavedResponse {
    query:    String,
    response: String,
}
