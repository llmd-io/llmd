use anyhow::Result;

#[derive(Default)]
pub struct RemoveManager;

impl RemoveManager {
    pub async fn remove_model(&self, model_name: &str) -> Result<bool> {
        // TODO: Implement actual model removal logic
        println!("Removing model: {}", model_name);
        Ok(true)
    }
} 