use serde::Deserialize;

/// Common pagination + sort params embedded via `#[serde(flatten)]`
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
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
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

    pub fn sort_by(&self) -> &str {
        self.sort_by.as_deref().unwrap_or("createdAt")
    }

    pub fn sort_order(&self) -> &str {
        self.sort_order.as_deref().unwrap_or("desc")
    }

    pub fn offset(&self) -> i64 {
        (self.page() - 1) * self.page_size()
    }

    /// Map a camelCase or snake_case sort field to a whitelisted DB column.
    /// `allowed` is a list of `(input_alias, db_column)` pairs.
    /// Falls back to `default_col` if no match.
    pub fn safe_sort_col<'a>(
        &self,
        allowed: &[(&'a str, &'a str)],
        default_col: &'a str,
    ) -> &'a str {
        let key = self.sort_by();
        allowed
            .iter()
            .find(|(alias, _)| alias.eq_ignore_ascii_case(key))
            .map(|(_, col)| *col)
            .unwrap_or(default_col)
    }

    pub fn safe_order(&self) -> &'static str {
        if self.sort_order().eq_ignore_ascii_case("asc") {
            "ASC"
        } else {
            "DESC"
        }
    }
}
