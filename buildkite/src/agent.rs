use crate::http;
use crate::http::HttpClient;
use crate::types::{Agent, Result};
use serde::{Deserialize, Serialize};

const AGENTS_PER_PAGE: u32 = 25;

pub struct AgentService<'a> {
    pub client: &'a HttpClient,
}

impl<'a> AgentService<'a> {
    pub fn new(client: &'a HttpClient) -> AgentService<'a> {
        AgentService { client }
    }

    pub fn iter_for(&self, org: &str) -> AgentServiceIterator<'a> {
        AgentServiceIterator::new(self.client, org)
    }

    pub fn list(&self, org: &str) -> Result<Vec<Agent>> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents", base_url);
        self.client.get_response(url.as_str())
    }

    pub fn get(&self, org: &str, agent_id: &str) -> Result<Vec<Agent>> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents/{}", base_url, agent_id);
        self.client.get_response(url.as_str())
    }

    pub fn stop(&self, org: &str, agent_id: &str, force: bool) -> Result<()> {
        let base_url = http::org_url(org);
        let url = format!("{}/agents/{}", base_url, agent_id);
        let request = StopAgentRequest { force: force };
        self.client.put(url.as_str(), &request)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StopAgentRequest {
    force: bool,
}

pub struct AgentServiceIterator<'a> {
    pub client: &'a HttpClient,
    org: String,
    agents: Vec<Agent>,
    current_page: u32,
    current_index: u32,
}

impl<'a> AgentServiceIterator<'a> {
    pub fn new(client: &'a HttpClient, org: &str) -> AgentServiceIterator<'a> {
        AgentServiceIterator {
            client,
            org: org.to_string(),
            agents: vec![],
            current_page: 1,
            current_index: 0u32,
        }
    }

    fn next_page(&mut self) -> Result<Vec<Agent>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/agents", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &self.current_page.to_string())]);

        self.current_page += 1;

        result
    }
}

impl<'a> Iterator for AgentServiceIterator<'a> {
    type Item = Agent;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.current_index % AGENTS_PER_PAGE) as usize;
        self.current_index += 1;

        if idx == 0 {
            match self.next_page() {
                Ok(l) => self.agents = l,
                Err(e) => {
                    println!("Err: {:?}", e);
                    return None;
                }
            }
        }

        if self.agents.len() > idx {
            Some(self.agents[idx].clone())
        } else {
            None
        }
    }
}
