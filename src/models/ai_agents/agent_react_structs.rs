use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// To define what stage the frontend developer is at
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FrontendBuildMode {
  Infrastructure,
  PageComponents,
  Completion
}


/// For decoding the serde_json api routes for a given page
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct APIAssignment {
  pub api_route: String,
  pub method: String,
  pub route_type: String,
}


/// Used for creating a type to be used for decoding shorthand
pub type PageRoutes = HashMap<String, Vec<APIAssignment>>;


/// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageAPIAssign {
  pub page: Vec<APIAssignment>
}


/// Used for decoding page names and suggested content
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SitePages {
  pub page_name: String,
  pub suggested_content_sections: serde_json::Value
}


/// Used for DesignBuildSheet
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DesignBuildSheet {
  pub pages: Option<Vec<String>>,
  pub pages_descriptons: Option<Vec<SitePages>>,
  pub api_assignments: Option<PageRoutes>,
  pub brand_colours: Option<Vec<String>>,
  pub build_mode: FrontendBuildMode
}