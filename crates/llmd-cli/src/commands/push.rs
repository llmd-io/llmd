use tonic::transport::Channel;
use crate::proto::modelservice::{PushModelRequest, model_service_client::ModelServiceClient};

pub async fn execute(model: &str, channel: &Channel) {
    let mut client = ModelServiceClient::new(channel.clone());
    
    let request = PushModelRequest {
        model_name: model.to_string(),
        model_data: vec![], // TODO: Add actual model data
    };
    
    let response = client.push_model(request).await.unwrap();
    println!("Response: {:?}", response);
}
