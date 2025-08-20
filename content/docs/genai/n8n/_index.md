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

### Start N8N securely with HTTPS and PostgreSQL:

- Generate Certs

```
openssl genrsa -out client.key 2048
openssl req -new -key client.key -out client.csr
openssl req -x509 -new -nodes -key client.key -sha256 -days 1024 -out rootCA.crt
openssl x509 -req -in client.csr -signkey client.key -out client.crt -days 365
```

- Create .env

```
DB_TYPE=postgresdb
DB_POSTGRESDB_DATABASE="n8n"
DB_POSTGRESDB_HOST=<<DB_HOST>>
DB_POSTGRESDB_PORT=5432
DB_POSTGRESDB_USER=<<N8N_ADMIN - HOST using Neon.tech>>
DB_POSTGRESDB_PASSWORD=<<N8N_PASSWORD>>
DB_POSTGRESDB_SCHEMA="n8n"
N8N_ENFORCE_SETTINGS_FILE_PERMISSIONS=true
DB_POSTGRESDB_SSL_ENABLED=true
DB_POSTGRESDB_SSL_REJECT_UNAUTHORIZED=true
N8N_SECURE_COOKIE=true
N8N_SSL_KEY=./client.key
N8N_SSL_CERT=./client.crt
N8N_PROTOCOL=https
```

-
```
cd /Users/<user_id>/sandbox/genai/n8n
export $(grep -v '^#' .env | xargs) && npx n8n
```

- Registered using Gmail and Password


## Sample Workflow for Chat using Local Ollama Model (DeepSeek)

- Run Ollama

```
ollama run deepseek-r1:7b
```

```
{
  "name": "LocalDeepSeek",
  "nodes": [
    {
      "parameters": {
        "options": {}
      },
      "id": "4680bb87-fb84-4361-9924-b61a27b4337a",
      "name": "When chat message received",
      "type": "@n8n/n8n-nodes-langchain.chatTrigger",
      "position": [
        700,
        20
      ],
      "webhookId": "ebdeba3f-6b4f-49f3-ba0a-8253dd226161",
      "typeVersion": 1.1
    },
    {
      "parameters": {
        "model": "=deepseek-r1:7b",
        "options": {}
      },
      "id": "b88f2048-c822-4dcc-bf35-f4564e2febad",
      "name": "Ollama Chat Model",
      "type": "@n8n/n8n-nodes-langchain.lmChatOllama",
      "position": [
        900,
        240
      ],
      "typeVersion": 1,
      "credentials": {
        "ollamaApi": {
          "id": "gx6gKTRZF0VzIRnP",
          "name": "Ollama account"
        }
      }
    },
    {
      "parameters": {
        "content": "## Chat with local LLMs using n8n and Ollama\nThis n8n workflow allows you to seamlessly interact with your self-hosted Large Language Models (LLMs) through a user-friendly chat interface. By connecting to Ollama, a powerful tool for managing local LLMs, you can send prompts and receive AI-generated responses directly within n8n.\n\n### How it works\n1. When chat message received: Captures the user's input from the chat interface.\n2. Chat LLM Chain: Sends the input to the Ollama server and receives the AI-generated response.\n3. Delivers the LLM's response back to the chat interface.\n\n### Set up steps\n* Make sure Ollama is installed and running on your machine before executing this workflow.\n* Edit the Ollama address if different from the default.\n",
        "height": 473,
        "width": 485
      },
      "id": "68755986-5872-4f2c-bf8e-190f4ba416d4",
      "name": "Sticky Note",
      "type": "n8n-nodes-base.stickyNote",
      "position": [
        160,
        -360
      ],
      "typeVersion": 1
    },
    {
      "parameters": {
        "content": "## Ollama setup\n* Connect to your local Ollama, usually on http://localhost:11434\n* If running in Docker, make sure that the n8n container has access to the host's network in order to connect to Ollama. You can do this by passing `--net=host` option when starting the n8n Docker container",
        "height": 258,
        "width": 368,
        "color": 6
      },
      "id": "49cac5e5-559f-422c-b7b0-9cd855bd02fc",
      "name": "Sticky Note1",
      "type": "n8n-nodes-base.stickyNote",
      "position": [
        1040,
        220
      ],
      "typeVersion": 1
    },
    {
      "parameters": {},
      "id": "1ed5f762-9b32-40df-b584-a357d057abb9",
      "name": "Chat LLM Chain",
      "type": "@n8n/n8n-nodes-langchain.chainLlm",
      "position": [
        920,
        20
      ],
      "typeVersion": 1.4
    }
  ],
  "pinData": {},
  "connections": {
    "Ollama Chat Model": {
      "ai_languageModel": [
        [
          {
            "node": "Chat LLM Chain",
            "type": "ai_languageModel",
            "index": 0
          }
        ]
      ]
    },
    "When chat message received": {
      "main": [
        [
          {
            "node": "Chat LLM Chain",
            "type": "main",
            "index": 0
          }
        ]
      ]
    }
  },
  "active": false,
  "settings": {
    "executionOrder": "v1"
  },
  "versionId": "8127a00c-1e69-447d-a209-f3c490706468",
  "meta": {
    "templateCredsSetupCompleted": true,
    "instanceId": "422be22b3dbdefcf0e5a2f7170e1c9426e37a2a34a9c580e427a97e11c79b02a"
  },
  "id": "Grcwml77RcXW5Hjl",
  "tags": []
}
```