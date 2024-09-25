use serde::Serialize;

#[derive(Serialize)]
pub struct BaseHttpResponse<T> {
    pub status: u16,
    pub success: bool,
    pub data: T,
}
