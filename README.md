# snapd-rs: Rust bindings for snapd

A Canonical AI Hackathon project providing Rust bindings for the snapd daemon. Interact with snap packages programmatically instead of shelling out to CLI commands.

## What are snapd bindings?

Rust wrappers around the snapd REST API. Instead of making HTTP calls by hand, use type-safe functions to query snaps, install/remove packages, and interact with the snapd system.

## What is workshop?

[Workshop](https://github.com/canonical/workshop) manages development environments and integrates with AI assistants. It handles setup, provides quick-access commands, and containerizes your workspace.

## Setup

### Prerequisites

Update LXD to version 6 and install workshop:
```bash
sudo snap refresh --channel=6/stable lxd
sudo snap install workshop --channel=latest/edge
```

### Initialize the project

```bash
workshop launch
```

### Using the workshop shell

Enter an interactive shell:
```bash
workshop shell
> copilot
```

Or run commands directly:
```bash
# Run copilot interactively
workshop run copilot

# Run copilot with a prompt
workshop run copilot-prompt <prompt>

# Example
workshop run copilot-prompt how many times does the letter p occur in raspberry?
```

## Quick Reference

For available workshop commands, see `workshop.yaml`.
