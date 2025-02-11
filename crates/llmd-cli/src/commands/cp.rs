use tonic::transport::Channel;
use crate::proto::modelservice::{CopyModelRequest, model_service_client::ModelServiceClient};

pub async fn execute(source: &str, destination: &str, channel: &Channel) {
    let mut client = ModelServiceClient::new(channel.clone());
    
    let request = CopyModelRequest {
        source_model: source.to_string(),
        target_model: destination.to_string(),
    };
    
    let response = client.copy_model(request).await.unwrap();
    println!("Response: {:?}", response);
}
