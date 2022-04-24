# Flux CD
- [Flux](https://fluxcd.io/) is a set of continuous and progressive delivery solutions for Kubernetes that are open and extensible.
- Flux is a tool for keeping Kubernetes clusters in sync with sources of configuration (like Git repositories), and automating updates to configuration when there is new code to deploy.
- Flux v2 is constructed with the GitOps Toolkit, a set of composable APIs and specialized tools for building Continuous Delivery on top of Kubernetes.


Flux Links

- **GitHub**: https://github.com/fluxcd/flux2


# Local setup
- Guide: https://fluxcd.io/docs/get-started/
- Install using brew
```
brew install fluxcd/tap/fluxbrew install fluxcd/tap/flux
export GITHUB_TOKEN=<PUT_TOKEN>
export GITHUB_USER=ankurkumarz
flux check --pre
```
- Enable Flux for a project repository
```

flux bootstrap github \
--owner=$GITHUB_USER\
--repository=fleet-infra \
--branch=main \
--path=./clusters/my-cluster \
--personal

```