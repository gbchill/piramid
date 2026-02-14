//! Types for range search requests and responses.
//! This module defines the data structures used for handling range search requests and responses in the API. It includes the `RangeSearchRequest` struct, which represents the parameters for a range search operation, such as the query vector, minimum score threshold, distance metric, and other search parameters. These types are used to deserialize incoming JSON requests for range search operations and to structure the data for processing the search
use serde::Deserialize;

fn default_k() -> usize { 10 }

#[derive(Deserialize)]
pub struct RangeSearchRequest {
    pub vector: Vec<f32>,
    pub min_score: f32,
    #[serde(default)]
    pub metric: Option<String>,
    #[serde(default = "default_k")]
    pub k: usize,
    #[serde(default)]
    pub ef: Option<usize>,
    #[serde(default)]
    pub nprobe: Option<usize>,
    #[serde(default)]
    pub overfetch: Option<usize>,
    #[serde(default)]
    pub preset: Option<String>,
}
