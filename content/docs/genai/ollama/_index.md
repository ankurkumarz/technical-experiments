---
title: Ollama
---

## Context

- [Ollama](https://ollama.com/) is one of the quickest way to run LLMs on your local system.
- Best of running SLMs (Small Language Models)locally.
- Ollama provides a default REST API for running the managed models using OpenAI-compatible interface.
- For Web Interface, use [Open WebUI](https://github.com/open-webui/open-webui). It supports various LLM runners, including Ollama and OpenAI-compatible APIs.
- [Library](https://ollama.com/library) has 100+ models listed.
- See [Examples](https://github.com/ollama/ollama/tree/main/examples)
- Like Docker concept, Ollama helps to create a model from a Modelfile.
- [GitHub](https://github.com/ollama/ollama)
- Not recommended for production. It is better to use either Cloud-provider deployed version (e.g. Vertex AI, Bedrock, Azure AI) or use [vllm](https://github.com/vllm-project/vllm), or similar solutions.


## Running Locally on Mac

```
ollama list
ollama run phi3
```

## Create a Local Model

- Create a Model file. 

```
FROM llama3.2
PARAMETER temperature 1
SYSTEM """
You are Mario from super mario bros, acting as an assistant.
"""
```

- Run the below command to create a model:

```
ollama create mymodel -f ./Modelfile
```

## Access a REST API

```
curl http://localhost:11434/api/chat -d '{
  "model": "llama3.2",
  "messages": [
    { "role": "user", "content": "why is the sky blue?" }
  ]
}'
```

## Install Web UI

```
pip install open-webui
open-webui serve
```