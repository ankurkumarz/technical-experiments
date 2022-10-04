---
title: Jenkins
---
# Jenkins
- [Jenkins](https://www.jenkins.io/) is the one of the leading opensource CI/CD (or automation) tool prevalent in the industry.
- Most widely used and the massieve support available from community.

## Improvement Areas

While there are many advantages of Jenkins, these are well-known limitation or improvement areas:
- User experience though there are improvements using plugins like Blue Ocean
- Cloud-native nature in the age of alternatives like [Tekton](https://tekton.dev/).

## Local Installation
- Installation instructions [available here](https://www.jenkins.io/doc/book/installing/).

```
brew install jenkins-lts
brew services start jenkins-lts
```
- Dashboard URL (Local): http://localhost:8080/
- Password on MacOS in file: **/Users/ankkumar/.jenkins/secrets/initialAdminPassword**
- Generated a GitHub token and connected it with Jenkins

## Key Plugins
Apart from default plugins, these are relevant while it is contextual:
- Blue Ocean - for redefined user experience
- Docker - for docker integration
- Audit Log - for details of who, when and what actions
- Logstask / Kafka Logs / Syslog - for sending logs to centralized logging server
- Terraform - wrapper for Terraform
- SonarQube - for code quality. Configure as per [instructions here](https://docs.sonarqube.org/latest/analysis/scan/sonarscanner-for-jenkins/). Manage Jenkins -> Configure system
- Snyk Security - [Installation Instructions](https://github.com/jenkinsci/snyk-security-scanner-plugin)