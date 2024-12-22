---
title: Warp
---

## Context

- [Warp](https://www.warp.dev/) is an AI-assisted terminal supported on Linux & MacOS. It can be configured to use underlying terminal such as Bash, Zsh, or others.
- Warp AI is an AI-powered assistant that looks up commands, walks you through multi-step workflows, auto-completes Workflows, and proactively fix bugs that's fully integrated into the terminal.
- Agent Mode is a mode in Warp that lets you perform any terminal task with natural language.

## Local Installation

```
brew install --cask warp
```

## Usage

- Keeping multiple terminals open along with Warp AI.
- Using Warp Drive to store the workflows and commonly-used commands.
- Install other productivity tools such as below for additional advantages:
- Keyboards shortcuts such as **Command \** to launch Warp drive.
- Type # on the command line input to Ask Warp AI for command suggestions.

```
brew install zoxide bat fzf
z wmx
zi
fzf --preview 'bat --style=numbers --color=always --line-range :500 {}'
```
