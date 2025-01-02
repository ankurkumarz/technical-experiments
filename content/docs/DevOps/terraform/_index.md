---
title: Terraform
---

## Terraform

- Terraform is HashiCorp's infrastructure as code tool. It lets you define resources and infrastructure in human-readable, declarative configuration files, and manages your infrastructure's lifecycle. 
- It supports multiple cloud providers - AWS, Azure, GCP, Oracle Cloud, and others (1K+ providers).
- [Click here](https://learn.hashicorp.com/tutorials/terraform/infrastructure-as-code?in=terraform/aws-get-started) to access the tutorial.

## Notes
- [Terraform Registry](https://registry.terraform.io/) for providers, modules and libraries.
- [Terraform Language](https://www.terraform.io/language)- sing a JSON-like configuration language called HCL (HashiCorp Configuration Language), its backend is written in Golang.
- Use [VS Code extension](https://marketplace.visualstudio.com/items?itemName=HashiCorp.terraform) for syntax highlights
- When you applied your configuration, Terraform writes data into a [state file](https://www.terraform.io/language/state) called *terraform.tfstate*. It uses JSON format to store data. **State** can be stored in Terraform Cloud or Cloud-providers like **AWS S3 or Google Cloud Storage**. [Click here](https://www.terraform.io/language/state/remote-state-data) to read more about remote state.

## Local Installation

```
brew tap hashicorp/tap
brew update
brew install hashicorp/tap/terraform
terraform -help
terraform -install-autocomplete
terraform fmt
terraform validate
```

## Run Terraform to create a Docker container
- Create a file "main.tf"
```
terraform {
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 2.13.0"
    }
  }
}

provider "docker" {}

resource "docker_image" "nginx" {
  name         = "nginx:latest"
  keep_locally = false
}

resource "docker_container" "nginx" {
  image = docker_image.nginx.latest
  name  = "tutorial"
  ports {
    internal = 80
    external = 8000
  }
}

```
- Run the following command:
```
terraform init
terraform plan
terraform apply
terraform destroy
```