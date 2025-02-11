use tonic::transport::Channel;
use crate::proto::modelservice::{RemoveModelRequest, model_service_client::ModelServiceClient};

pub async fn execute(model: &str, channel: &Channel) {
    let mut client = ModelServiceClient::new(channel.clone());
    
    let request = RemoveModelRequest {
        model_name: model.to_string(),
    };
    
    let response = client.remove_model(request).await.unwrap();
    println!("Response: {:?}", response);
}
