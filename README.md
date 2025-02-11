# llmd
llmd is a LLMs daemonset, it provide model manager and get up and running large language models, it can use llama.cpp or vllm or sglang to running large language models.

## Features
- Model manager, pull、push、list、cp、rm model.
- Model runner, run model with llama.cpp or vllm or sglang.
- Model monitor, monitor model status and logs.
- Model web, web ui to manage models and logs.

## Usage

### Quick Start
```bash
# Deploy llmd to your Kubernetes cluster
kubectl apply -f https://raw.githubusercontent.com/llmd/llmd/main/deploy/llmd.yaml

# Check the deployment status
kubectl get pods -n llmd-system
```

### Local Development
```bash
# Clone the repository
git clone https://github.com/llmd/llmd.git
cd llmd

# Apply the local deployment file
kubectl apply -f deploy/llmd.yaml
```


## llmd-server

### 启动服务器
现在你可以通过以下方式启动服务器：
使用默认配置文件：./llmd-server
指定配置文件：./llmd-server -c path/to/config.yaml 或 ./llmd-server --config path/to/config.yaml