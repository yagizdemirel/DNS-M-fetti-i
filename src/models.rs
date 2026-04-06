use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsLog {
    pub id: usize,
    pub timestamp: String,
    pub domain: String,
    pub record_type: String,
    pub status: String,
}