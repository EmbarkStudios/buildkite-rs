use crate::http;
use crate::http::HttpClient;
use crate::types::{Organization, Result};

const ORGANIZATIONS_PER_PAGE: u32 = 25;

pub struct OrganizationService<'a> {
    /// The buildkite client
    pub client: &'a HttpClient,
}

impl<'a> OrganizationService<'a> {
    pub fn new(client: &'a HttpClient) -> OrganizationService {
        OrganizationService { client }
    }

    pub fn iter(&self) -> OrganizationServiceIterator<'a> {
        OrganizationServiceIterator::new(self.client)
    }

    pub fn list(&self) -> Result<Vec<Organization>> {
        let base_url = http::base_url();
        let url = format!("{}/organizations", base_url);
        self.client.get_response(url.as_str())
    }

    pub fn get(&self, org: &str) -> Result<Organization> {
        let base_url = http::org_url(org);
        let url = format!("{}", base_url);
        self.client.get_response(url.as_str())
    }
}

pub struct OrganizationServiceIterator<'a> {
    pub client: &'a HttpClient,
    organizations: Vec<Organization>,
    current_page: u32,
    current_index: u32,
}

impl<'a> OrganizationServiceIterator<'a> {
    pub fn new(client: &'a HttpClient) -> OrganizationServiceIterator<'a> {
        OrganizationServiceIterator {
            client,
            organizations: vec![],
            current_page: 1,
            current_index: 0u32,
        }
    }

    fn next_page(&mut self) -> Result<Vec<Organization>> {
        let base_url = http::base_url();
        let url = format!("{}/organizations", base_url);
        let result = self
            .client
            .get_response_with_query(url.as_str(), &[("page", &self.current_page.to_string())]);

        self.current_page += 1;

        result
    }
}

impl<'a> Iterator for OrganizationServiceIterator<'a> {
    type Item = Organization;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.current_index % ORGANIZATIONS_PER_PAGE) as usize;
        self.current_index += 1;

        if idx == 0 {
            match self.next_page() {
                Ok(l) => self.organizations = l,
                Err(e) => {
                    println!("Err: {:?}", e);
                    return None;
                }
            }
        }

        if self.organizations.len() > idx {
            Some(self.organizations[idx].clone())
        } else {
            None
        }
    }
}
