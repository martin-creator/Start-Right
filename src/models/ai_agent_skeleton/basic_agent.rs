use crate::models::ai_agent_skeleton::basic_traits::BasicTraits;
use crate::models::general_http_content::llm_content_structure::Message;

#[derive(Debug, PartialEq)]
/// The state of the agent
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
/// The basic agent skeleton
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}


/// Implementation of the basic traits for the basic agent
impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([]),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
