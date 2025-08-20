---
bookFlatSection: true
title: N8N
---

## Context

- [N8N](https://n8n.io/) has emerged as one of the most popular workflow-based GenAI solution to build automation for ITOps, SecOps, DevOps, and business-focused orchestration pipelines.
- It has [Enterprise and Community](https://docs.n8n.io/hosting/community-edition-features/) Version but the licensing for community is not truly Open-source license like Apache. It has fair-code license, which has limitations such as usage only for non-commercial or personal use.
- It has 1000+ integrations available with tools in Analytics, CRM, AI, and many more. [List of Integrations](https://n8n.io/integrations/).
- It is built using NodeJS.
- [Blog on Building AI Agents with N8N](https://blog.n8n.io/ai-agents/)
- [Building AI Agentic Workflows with N8N](https://blog.n8n.io/ai-agentic-workflows/)

## Key Differentiators

- In n8n, a workflow automation tool, you can create all three types of workflows:
  - **Traditional workflow automation**: *Pros:* Follows predefined, rigid steps with structured data. *Cons:* Limited ability to adapt to new situations + Requires manual updates to make changes.
  - **AI-enhanced workflows**: *Pros:* Use AI for specific tasks within a predefined workflow with structured and unstructured data. *Cons*: Limited decision-making capabilities + Still follow a largely linear process.
  - **AI agentic workflows**: *Pros:* Dynamically adapt the workflow depending on context and goals, Handle both structured and unstructured data, Manage multi-step, non-linear processes. Make complex decisions autonomously, Can learn and improve over time.
- In n8n, everything is organized as workflows - sets of interconnected nodes that process and transfer data. While simple workflows can handle basic automations, more sophisticated workflows can act as AI agents when combined with LangChain and other AI-powered nodes.
- n8n takes it a step further by providing a low-code interface to LangChain. In n8n, you can simply drag and drop LangChain nodes onto the canvas and configure them. n8n supports the JavaScript implementation of LangChain.
- In n8n, you can create multi-agent systems by connecting multiple workflows, each representing a specialized agent.

## Local Setup

- All config stored in folder: /Users/ankkumar/.n8n
- It has a default SQLite Database, supports PostgreSQL for Prod. DB stores workflow data (definition, execution, history), Users and RBAC, Tokens and API Keys, LLM config and credentials data, and other workflow analytics.

```
nvm use v22.16.0
npm update -g n8n  
n8n start   
```