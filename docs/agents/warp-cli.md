## Warp CLI

This agent runs Warp's terminal-based AI workflows using the official CLI (`warp agent run`).

### Requirements

- Warp desktop app with CLI support enabled
- Installed CLI (from Warp → Command Palette → "Install CLI")
- Authenticated agent profiles and configured MCP servers

### Config Fields

| Field | Description |
| --- | --- |
| `profile` | Agent profile to use (`--profile`) |
| `mcp_servers` | List of MCP servers (`--mcp-server`) |
| `extra_flags` | Any additional flags (e.g. `--gui`) |
| `binary` | Override binary (`warp-preview`, etc.) |

### How it works

Vibe Kanban shells out to:

```bash
warp agent run --prompt "<your prompt>" [--profile <id>] [--mcp-server <id>]...
```

Warp CLI streams output back into the Vibe Kanban interface in real time.
