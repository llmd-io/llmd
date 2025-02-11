use tonic::transport::Channel;
use crate::proto::modelservice::{ListModelsRequest, model_service_client::ModelServiceClient};

pub async fn execute(channel: &Channel) {
    let mut client = ModelServiceClient::new(channel.clone());
    
    let request = ListModelsRequest {};
    
    let response = client.list_models(request).await.unwrap();
    println!("Models: {:?}", response);
}
