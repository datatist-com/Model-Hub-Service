use serde::Deserialize;

/// Common pagination params embedded via `#[serde(flatten)]`
/// in every list query struct.
///
/// `page` and `page_size` are stored as `Option<String>` because HTTP query
/// strings are always text; parsing them ourselves avoids serde type-mismatch
/// errors ("invalid type: string '1', expected i64") that occur when using
/// integer types with `#[serde(flatten)]` across different serde backends.
#[derive(Debug, Deserialize, Clone)]
pub struct PaginationParams {
    pub page: Option<String>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<String>,
}

impl PaginationParams {
    pub fn page(&self) -> i64 {
        self.page
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(1)
            .max(1)
    }

    pub fn page_size(&self) -> i64 {
        self.page_size
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(20)
            .clamp(1, 200)
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1).saturating_mul(self.page_size())
    }
}
