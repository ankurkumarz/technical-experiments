# DAPR

- The Distributed Application Runtime (Dapr) provides APIs that simplify microservice connectivity.
- Dapr’s sidecar take care of the complex challenges such as service discovery, message broker integration, encryption, observability, and secret management

# Architecture

![Architecture](https://dapr.io/images/service-invocation.png)

# Sample Reference Application

![Reference Application](https://github.com/dapr/quickstarts/blob/v1.6.0/hello-kubernetes/img/Architecture_Diagram.png)

# Local Installation (using Kubernetes)

- Step 1: Install DAPR CLI library. [Click here](https://docs.dapr.io/getting-started/install-dapr-cli/) to read more.

```
brew install dapr/tap/dapr-cli
```

- Setup DAPR on K8S Cluster (creates a namespace). [Click here](https://docs.dapr.io/operations/hosting/kubernetes/kubernetes-deploy/) for detailed instructions for different K8S setup (AKS, EKS, GKE).
```
dapr init --kubernetes --wait
```

- 