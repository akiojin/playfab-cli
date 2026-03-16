---
name: title
description: Use playfab-cli to manage title-level configuration, catalog settings, and global entity policies. Use when the user needs to inspect or modify title configuration, enable/disable Economy v2, or set global policies. Do not use for player-specific or item-specific operations.
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: config
---

# Title Configuration Operations

Use playfab-cli to manage title-level configuration and policies.

## Use When

- The user needs to inspect or modify title-level configuration.
- The user wants to enable or disable Economy v2 catalog.
- The user needs to view or set global entity policies.
- The task involves title-wide settings that apply to all players.

## Do Not Use When

- The operation targets a specific player or entity (use player skill).
- The operation manages catalog items (use catalog skill).

## Available Tools

| Tool | Description |
|------|-------------|
| `get_catalog_config` | Retrieve the catalog configuration for the title |
| `update_catalog_config` | Update the catalog configuration |
| `set_global_policy` | Set the global policy for all entities |
| `get_global_policy` | Get the global policy |

## Examples

```bash
# Get catalog configuration
playfab-cli tool call get_catalog_config --json '{}'

# Update catalog configuration
playfab-cli tool call update_catalog_config --json '{
  "config": {
    "IsCatalogEnabled": true,
    "Platforms": ["Steam", "GooglePlay", "AppleAppStore"],
    "AdminEntities": [{"Id": "...", "Type": "title"}]
  }
}'

# Get global policy
playfab-cli tool call get_global_policy --json '{}'

# Set global policy
playfab-cli tool call set_global_policy --json '{
  "permissions": [
    {
      "action": "Read",
      "effect": "Allow",
      "resource": "pfrn:data--*!*/Profile/*",
      "principal": "*"
    }
  ]
}'
```

## Common Workflows

### 1. Inspect title configuration

```bash
playfab-cli system ping
playfab-cli config show
playfab-cli tool call get_catalog_config --json '{}'
playfab-cli tool call get_global_policy --json '{}'
```

### 2. Configure Economy v2

```bash
# Check current config
playfab-cli tool call get_catalog_config --json '{}'
# Update config to enable catalog
playfab-cli tool call update_catalog_config --json '{"config": {"IsCatalogEnabled": true}}'
```

## Notes

- Title configuration changes affect all players and entities under the title.
- Global policies define default access control for all entity profiles.
- Economy v2 must be enabled via `update_catalog_config` before using catalog/inventory tools.
