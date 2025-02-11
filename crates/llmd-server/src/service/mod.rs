use tonic::{Request, Response, Status};

use crate::model::{CopyManager, ListManager, PullManager, PushManager, RemoveManager};

// First, declare the module that contains the generated proto code
pub mod model_service {
    tonic::include_proto!("modelservice");
}

// Then update the imports to use the local model_service module
use model_service::model_service_server::ModelService as ModelServiceTrait;
use model_service::*;

#[derive(Default)]
pub struct ModelService {
    pull_manager: PullManager,
    push_manager: PushManager,
    list_manager: ListManager,
    copy_manager: CopyManager,
    remove_manager: RemoveManager,
}

#[tonic::async_trait]
impl ModelServiceTrait for ModelService {
    async fn pull_model(
        &self,
        request: Request<PullModelRequest>,
    ) -> Result<Response<PullModelResponse>, Status> {
        let req = request.into_inner();
        match self.pull_manager.pull_model(&req.model_name).await {
            Ok(success) => Ok(Response::new(PullModelResponse {
                success,
                message: "Model pulled successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(PullModelResponse {
                success: false,
                message: e.to_string(),
            })),
        }
    }

    async fn push_model(
        &self,
        request: Request<PushModelRequest>,
    ) -> Result<Response<PushModelResponse>, Status> {
        let req = request.into_inner();
        match self
            .push_manager
            .push_model(&req.model_name, req.model_data)
            .await
        {
            Ok(success) => Ok(Response::new(PushModelResponse {
                success,
                message: "Model pushed successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(PushModelResponse {
                success: false,
                message: e.to_string(),
            })),
        }
    }

    async fn list_models(
        &self,
        _request: Request<ListModelsRequest>,
    ) -> Result<Response<ListModelsResponse>, Status> {
        match self.list_manager.list_models().await {
            Ok(model_names) => Ok(Response::new(ListModelsResponse { model_names })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn copy_model(
        &self,
        request: Request<CopyModelRequest>,
    ) -> Result<Response<CopyModelResponse>, Status> {
        let req = request.into_inner();
        match self
            .copy_manager
            .copy_model(&req.source_model, &req.target_model)
            .await
        {
            Ok(success) => Ok(Response::new(CopyModelResponse {
                success,
                message: "Model copied successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(CopyModelResponse {
                success: false,
                message: e.to_string(),
            })),
        }
    }

    async fn remove_model(
        &self,
        request: Request<RemoveModelRequest>,
    ) -> Result<Response<RemoveModelResponse>, Status> {
        let req = request.into_inner();
        match self.remove_manager.remove_model(&req.model_name).await {
            Ok(success) => Ok(Response::new(RemoveModelResponse {
                success,
                message: "Model removed successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(RemoveModelResponse {
                success: false,
                message: e.to_string(),
            })),
        }
    }
} 