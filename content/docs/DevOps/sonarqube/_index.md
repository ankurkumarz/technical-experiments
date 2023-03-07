---
title: Sonarqube
---
# Sonarqube
- [SonarQube](https://www.sonarqube.org/), is the most widely used automatic code review tool that systematically helps deliver Clean Code.
- It supports Kubernetes and Docker for installation.
- It provides DevOps platform integration with GitHub, GitLab, Bitbucket, Azure DevOps, and others.
![Sonar Quality](https://docs.sonarqube.org/9.6/images/dev-cycle.png)

## Local Installation
- Installation instructions:
```
brew install sonarqube
brew services start sonarqube
```
- Dashboard URL (Local): http://localhost:9000/ - default userid password (admin/admin).
- For enterprise installation, needs to ensure scalability as it has 3 components - Server (Web server, Search server, Compute engine), Database, and Scanner (one or more based on the projects). [Read more](https://docs.sonarqube.org/latest/setup/install-server/) about installation in an enterprise.
![Cluster Setup](https://docs.sonarqube.org/9.6/images/SQ-instance-components.png)