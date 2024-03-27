pub mod error;
pub mod generation;
pub mod models;

#[derive(Debug, Clone)]
pub struct Ollama {
    pub(crate) host: String,
    pub(crate) port: Option<u16>,
    pub(crate) reqwest_client: reqwest::Client,
}

impl Ollama {
    pub fn new(host: String, port: Option<u16>) -> Self {
        Self {
            host,
            port,
            ..Default::default()
        }
    }

    /// Returns the http URI of the Ollama instance
    pub fn uri(&self) -> String {
        if let Some(port) = self.port {
            format!("{}:{}", self.host, port)
        } else {
            self.host.clone()
        }
    }
}

impl Default for Ollama {
    /// Returns a default Ollama instance with the host set to `http://127.0.0.1:11434`.
    fn default() -> Self {
        Self {
            host: "http://127.0.0.1".to_string(),
            port: Some(11434),
            reqwest_client: reqwest::Client::new(),
        }
    }
}
