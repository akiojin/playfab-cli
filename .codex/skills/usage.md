# PlayFab CLI Usage

General usage guide for the playfab-cli command-line tool.

## Prerequisites

Set the following environment variables before using the CLI:

```bash
export PLAYFAB_TITLE_ID=your_title_id
export PLAYFAB_DEV_SECRET_KEY=your_secret_key
```

## Global Options

| Option | Description |
|--------|-------------|
| `--output text\|json` | Output format (default: text) |
| `-v, --verbose` | Increase verbosity (use -vv for debug) |
| `--dry-run` | Preview the request without executing |

## Commands

### Tool Operations

```bash
# List all available tools
playfab-cli tool list

# Show JSON schema for a tool
playfab-cli tool schema <tool_name>

# Call a tool with JSON parameters
playfab-cli tool call <tool_name> --json '{...}'

# Call a tool with parameters from a file
playfab-cli tool call <tool_name> --params-file params.json
```

### System Commands

```bash
# Check connectivity to PlayFab
playfab-cli system ping
```

### Configuration

```bash
# Show current configuration
playfab-cli config show

# Set a configuration value
playfab-cli config set <key> <value>
```

### Batch Operations

```bash
# Execute batch operations from JSON
playfab-cli batch --json '[{"tool": "search_items", "params": {...}}, ...]'

# Execute batch operations from stdin
cat operations.json | playfab-cli batch --stdin
```

### CLI Management

```bash
# Install or update the CLI binary
playfab-cli cli install
playfab-cli cli install --force

# Check CLI health
playfab-cli cli doctor
```

## Common Workflows

### 1. Verify Setup

```bash
playfab-cli system ping
playfab-cli config show
```

### 2. Explore Available Tools

```bash
playfab-cli tool list
playfab-cli tool schema search_items
```

### 3. Use JSON Output for Scripting

```bash
playfab-cli --output json tool call search_items --json '{"search": "sword"}'
```

### 4. Dry Run Before Executing

```bash
playfab-cli --dry-run tool call delete_item --json '{"id": "item-123"}'
```
