use crate::models::ai_agent_skeleton::basic_agent::BasicAgent;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// The state of the agent
pub struct RouteObject {
    pub is_route_dynamic: String,
    pub method: String,
    pub request_body: serde_json::Value,
    pub response: serde_json::Value,
    pub route: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
/// Project scope contains information about the project
pub struct ProjectScope {
    pub is_crud_required: bool,
    pub is_user_login_and_logout: bool,
    pub is_external_urls_required: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
/// Fact sheet for the project
pub struct FactSheet {
    pub project_description: String,
    pub project_scope: Option<ProjectScope>,
    pub external_urls: Option<Vec<String>>,
    pub backend_code: Option<String>,
    pub frontend_code: Option<String>,
    pub api_endpoint_schema: Option<Vec<RouteObject>>,
}

#[async_trait]
/// Special functions trait for the agent used to execute logic
pub trait SpecialFunctions: Debug {
    // Used to that manager can get attributes from Agents
    fn get_attributes_from_agent(&self) -> &BasicAgent;

    // This function will allow agents to execute their logic
    async fn execute(
        &mut self,
        factsheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
