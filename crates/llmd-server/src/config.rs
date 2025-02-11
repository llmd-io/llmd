use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub model: ModelConfig,
    pub engine: EngineConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ModelConfig {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct EngineConfig {
    pub name: String,
    pub config: serde_yaml::Value,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_valid_config() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary config file
        let mut temp_file = NamedTempFile::new()?;
        let config_content = r#"
server:
  host: "127.0.0.1"
  port: 8080
model:
  name: "gpt-3.5-turbo"
engine:
  name: "vllm"
  config:
    num_gpus: 1
    gpu_memory_utilization: 0.8
    max_model_len: 131072
"#;
        temp_file.write_all(config_content.as_bytes())?;

        // Load and verify config
        let config = Config::load(temp_file.path().to_str().unwrap())?;
        
        assert_eq!(config.server.host, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.model.name, "gpt-3.5-turbo");
        assert_eq!(config.engine.name, "vllm");
        assert_eq!(config.engine.config["num_gpus"], 1);
        assert_eq!(config.engine.config["gpu_memory_utilization"], 0.8);
        assert_eq!(config.engine.config["max_model_len"], 131072);

        Ok(())
    }

    #[test]
    fn test_server_addr() {
        let config = Config {
            server: ServerConfig {
                host: "localhost".to_string(),
                port: 3000,
            },
            model: ModelConfig {
                name: "test-model".to_string(),
            },
            engine: EngineConfig {
                name: "test-engine".to_string(),
                config: serde_yaml::Value::Null,
            },
        };

        assert_eq!(config.server_addr(), "localhost:3000");
    }

    #[test]
    fn test_load_invalid_config() {
        let result = Config::load("nonexistent_file.yaml");
        assert!(result.is_err());
    }
} 

