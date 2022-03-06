# ArgoCD

[Argo CD](https://argoproj.github.io/) is a declarative, GitOps continuous delivery tool for Kubernetes.

## Links
- [Doccumentation](https://argo-cd.readthedocs.io/)
- [Project Home Page](https://argoproj.github.io/)
- [GitHub Repo](https://github.com/argoproj/argo-workflows)


> Reviewed in Mar 2022

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

## Key Observations
- [Intuit](https://blog.argoproj.io/introducing-argo-cd-declarative-continuous-delivery-for-kubernetes-da2a73a780cd) started ArgoCD and currently using to run its production workload of 100+ nodes cluster
- Argo was accepted to CNCF on Apr, 2020 and is at the Incubating project maturity level.
- **Companies** using Red Hat, Google, Ticketmaster, WordPress, NVIDIA, Tesla, Adobe
- ArgoCD provides GUI in comparison to Flux CD
- Supports raw Kubernetes manifests (YAML), Kustomize, and Helm.
- Argo CD has RBAC-based access control and it is independent of Kubernetes.
- **Deployment**: Argo CD can be configured in a “pull” (instance of Argo CD on each cluster) or “push” (one Argo CD instance connecting to many clusters).

## Compare ArgoCD vs. Flux
- Read the article: https://thenewstack.io/gitops-on-kubernetes-deciding-between-argo-cd-and-flux

![ArgoCD vs. Flux](images/ArgoCD-flux.png)