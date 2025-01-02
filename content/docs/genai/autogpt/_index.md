---
title: AutoGPT
---

## Context

- [AutoGPT](https://agpt.co/), developed by a gaming company Significant Gravitas, is a platform built by to create, deploy, and manage continuous AI agents that automate complex workflows.
- Initially supported only OpenAI's GPT 3 & 4 models, now it supports multiple LLM Providers: **Anthropic, Groq, Llama**
- It has a hybrid licensing model - mixed of MIT and Polyform Shield License - so not truly an open source.
- [GitHub Repo](https://github.com/Significant-Gravitas/AutoGPT?)

## Platform Architecture

- Primarily, have **AutoGPT Server** and **AutoGPT Frontend**
- Platform Components: 
  - **Agents and Workflows**: customized workflows to build agents for various tasks such as data processing, task management, integration, etc.
  - **Blocks as Integrations**: connecting with external services, data processing, AI model invocations, conditional logic and custom scripts, and more.
- AutoGPT employs the [**agent protocol**](https://agentprotocol.ai/) standard by the AI Engineer Foundation. This standardizes the communication pathways from your agent to the frontend and benchmark.
- Built [Benchmark](https://pypi.org/project/agbenchmark/) for measuring agent's performnance.
- Technologies: Supabase/PostgreSQL as Vector database, Kong as gateway, Redis for Caching, Next.js for Frontend, Docker for Containers

## Local Setup

Running backend (read [instructions here](https://docs.agpt.co/platform/getting-started/#cloning-the-repository)):

```
git clone https://github.com/Significant-Gravitas/AutoGPT.git
cd AutoGPT
git submodule update --init --recursive
cd autogpt_platform
cp supabase/docker/.env.example .env
docker compose up -d --build
```
Running frontend:

```
cd sandbox/genai/AutoGPT/autogpt_platform/frontend
cp .env.example .env
nvm use 18.17.0
npm install
npm run dev
```