use anyhow::Result;

#[derive(Default)]
pub struct ListManager;

impl ListManager {
    pub async fn list_models(&self) -> Result<Vec<String>> {
        // TODO: Implement actual model listing logic
        Ok(vec!["model1".to_string(), "model2".to_string()])
    }
} 