use tonic::transport::Channel;
use crate::proto::modelservice::{PullModelRequest, model_service_client::ModelServiceClient};

pub async fn execute(model: &str, channel: &Channel) {
    let mut client = ModelServiceClient::new(channel.clone());
    
    let request = PullModelRequest {
        model_name: model.to_string(),
    };
    
    let response = client.pull_model(request).await.unwrap();
    if response.get_ref().success {
        println!("Successfully pulled model: {}", model);
    } else {
        println!("Failed to pull model: {}", model);
    }
}
