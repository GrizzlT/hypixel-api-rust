use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct KeyReply {
    success: bool,
    record: KeyData,
}

#[derive(Debug, Copy, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyData {
    queries_in_past_min: i32,
    key: Uuid,
    owner: Uuid,
    limit: i32,
    total_queries: i32,
}