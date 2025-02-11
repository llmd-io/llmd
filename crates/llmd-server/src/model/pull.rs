use anyhow::Result;
use log::info;
use hf_hub::{api::sync::ApiBuilder,Cache, Repo, RepoType};
use std::path::PathBuf;

#[derive(Default)]
pub struct PullManager;

impl PullManager {
    pub async fn pull_model(&self, model_name: &str) -> Result<bool> {
        info!("Starting to pull model: {}", model_name);

        let _proxy = std::env::var("https_proxy").expect("This example expects a HTTPS_PROXY environment variable to be defined to test that the routing happens correctly. Starts a socks servers and use point HTTPS_PROXY to that server to see the routing in action.");
        
        // Initialize the Hugging Face API client
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let path = home_dir.join(".llmd").join("models");
        let cache = Cache::new(path);
        let api = ApiBuilder::from_cache(cache).with_progress(true).build().unwrap();

        // Get the model repository
        let repo = api.model(model_name.to_string());
        
        // List all files in the repository
        let files = repo.info().unwrap().siblings;
        info!("Found {} files in model repository", files.len());

        // Download each file
        for file in files {
            info!("Downloading: {:?}", file);
            // let local_path = repo.get_path(&file.rfilename);
            // if local_path.exists() {
            //     println!("File {} already exists, skipping download", file.rfilename);
            //     continue;
            // }
            repo.download(&file.rfilename).unwrap();
        }
        
        Ok(true)
    }
} 