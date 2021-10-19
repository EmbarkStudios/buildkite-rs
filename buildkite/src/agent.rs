use crate::http;
use crate::http::HttpClient;
use crate::iterator::{BuildkiteIterator, Paginator};
use crate::types::{Agent, Result};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::rc::Rc;

pub struct AgentService {
    pub client: Rc<HttpClient>,
}

impl AgentService {
    pub fn new(client: Rc<HttpClient>) -> AgentService {
        AgentService { client }
    }

    pub fn iter_for<T: for<'de> Deserialize<'de>>(&self, org: &str) -> BuildkiteIterator<T> {
        BuildkiteIterator::new(Box::new(AgentServiceIterator::new(
            Rc::clone(&self.client),
            org,
        )))
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
        let request = StopAgentRequest { force };
        self.client.put(url.as_str(), &request)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct StopAgentRequest {
    force: bool,
}

pub struct AgentServiceIterator {
    pub client: Rc<HttpClient>,
    org: String,
}

impl<T> Paginator<T> for AgentServiceIterator
where
    T: DeserializeOwned,
{
    fn get_page(&self, page: u32) -> Result<Vec<T>> {
        let base_url = http::org_url(&self.org);
        let url = format!("{}/agents", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &page.to_string())]);

        result
    }
}

impl AgentServiceIterator {
    pub fn new(client: Rc<HttpClient>, org: &str) -> AgentServiceIterator {
        AgentServiceIterator {
            client,
            org: org.to_string(),
        }
    }
}
