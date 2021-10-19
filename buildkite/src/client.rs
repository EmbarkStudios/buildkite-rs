use crate::agent::AgentService;
use crate::build::BuildService;
use crate::http::HttpClient;
use crate::organization::OrganizationService;
use crate::pipeline::PipelineService;

use std::rc::Rc;

/// Client is the Buildkite API client
pub struct Client {
    /// The buildkite client
    client: Rc<HttpClient>,
}

impl Client {
    /// new returns a new buildkite client
    pub fn new(token: &str) -> Self {
        Client {
            client: Rc::new(HttpClient::new(token.to_string())),
        }
    }

    pub fn organization(&self) -> OrganizationService {
        OrganizationService::new(Rc::clone(&self.client))
    }

    pub fn agent(&self) -> AgentService {
        AgentService::new(Rc::clone(&self.client))
    }

    pub fn build(&self) -> BuildService {
        BuildService::new(Rc::clone(&self.client))
    }

    pub fn pipeline(&self) -> PipelineService {
        PipelineService::new(Rc::clone(&self.client))
    }
}
