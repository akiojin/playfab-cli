# Title Configuration Operations

Use playfab-cli to manage title-level configuration and policies.

## Available Tools

| Tool | Description |
|------|-------------|
| `get_catalog_config` | Retrieve the catalog configuration for the title |
| `update_catalog_config` | Update the catalog configuration |
| `set_global_policy` | Set the global policy for all entities |
| `get_global_policy` | Get the global policy |

## Examples

### Get catalog configuration

```bash
playfab-cli tool call get_catalog_config --json '{}'
```

### Update catalog configuration

```bash
playfab-cli tool call update_catalog_config --json '{
  "config": {
    "IsCatalogEnabled": true,
    "Platforms": ["Steam", "GooglePlay", "AppleAppStore"],
    "AdminEntities": [{"Id": "...", "Type": "title"}]
  }
}'
```

### Get global policy

```bash
playfab-cli tool call get_global_policy --json '{}'
```

### Set global policy

```bash
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
