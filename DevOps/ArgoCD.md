# ArgoCD

Argo CD is a declarative, GitOps continuous delivery tool for Kubernetes.The documentation is available at below links:
- https://argo-cd.readthedocs.io/en/stable/

# Argo CD Architecture

![Argo CD Architecture](https://argo-cd.readthedocs.io/en/stable/assets/argocd_architecture.png)
(Source: https://argo-cd.readthedocs.io/en/stable/)


# Local Setup
- Instructions available at the [Getting Started guide](https://argo-cd.readthedocs.io/en/stable/) helps to setup locally
- Step 1: Setup Kubernetes (Minikube, Rancher Desktop, Docker Desktop)
- Step 2: Create namespace & deploy to Kubernetes cluster:
```
kubectl create namespace argocd
kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
```
- Step 3: Installed CLI on Mac:
```
brew install argocd
```
- Step 4: Added the Load Balancer and Port Forwarded for local access:
```
kubectl patch svc argocd-server -n argocd -p '{"spec": {"type": "LoadBalancer"}}'

kubectl port-forward --address 0.0.0.0 svc/argocd-server -n argocd 8082:443
```
Don't forget to add *0.0.0.0* address otherwise you can't use IP Address (alternative is to use localhost)

# First Deployment
- Example repository is available here: https://github.com/argoproj/argocd-example-apps
- User Id is *admin* (automatically setup initially)
- Password needs to be retrieved using:
```
kubectl -n argocd get secret argocd-initial-admin-secret -o jsonpath="{.data.password}" | base64 -d; echo
```

- Login to Console: https://localhost:8082 
- Login to CLI: argocd login localhost:8082
- Deploy the app using CLI or use Dashboard:
```
argocd app create guestbook --repo https://github.com/argoproj/argocd-example-apps.git --path guestbook --dest-server https://kubernetes.default.svc --dest-namespace default`
```

## Local Setup Snapshot

![Local Dashboard](images/argocd-local.png)

