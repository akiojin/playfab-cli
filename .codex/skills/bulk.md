# Batch and Bulk Operations

Use playfab-cli to perform batch and bulk operations across multiple entities or items.

## Available Tools

| Tool | Description |
|------|-------------|
| `batch_create_draft_items` | Create multiple draft catalog items in a single request |
| `grant_items_to_users` | Grant items to multiple users at once |
| `execute_inventory_operations` | Execute multiple inventory operations atomically |
| `execute_transfer_operations` | Execute multiple transfer operations atomically |

## Batch Command

The CLI also provides a dedicated `batch` subcommand for running multiple tool calls:

```bash
playfab-cli batch --json '[
  {"tool": "search_items", "params": {"search": "sword"}},
  {"tool": "search_items", "params": {"search": "shield"}}
]'
```

## Examples

### Batch create draft items

```bash
playfab-cli tool call batch_create_draft_items --json '{
  "items": [
    {
      "Type": "catalogItem",
      "Title": {"NEUTRAL": "Iron Sword"},
      "Description": {"NEUTRAL": "A sturdy iron sword"}
    },
    {
      "Type": "catalogItem",
      "Title": {"NEUTRAL": "Iron Shield"},
      "Description": {"NEUTRAL": "A sturdy iron shield"}
    }
  ]
}'
```

### Grant items to multiple users

```bash
playfab-cli tool call grant_items_to_users --json '{
  "items": [
    {
      "PlayFabId": "PLAYER_A",
      "items": [{"Id": "reward-001", "Amount": 1}, {"Id": "gold-coin", "Amount": 500}]
    },
    {
      "PlayFabId": "PLAYER_B",
      "items": [{"Id": "reward-001", "Amount": 1}, {"Id": "gold-coin", "Amount": 500}]
    },
    {
      "PlayFabId": "PLAYER_C",
      "items": [{"Id": "reward-001", "Amount": 1}, {"Id": "gold-coin", "Amount": 500}]
    }
  ]
}'
```

### Atomic inventory operations

```bash
playfab-cli tool call execute_inventory_operations --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "operations": [
    {"Subtract": {"Item": {"Id": "gold-coin"}, "Amount": 1000}},
    {"Add": {"Item": {"Id": "premium-sword"}, "Amount": 1}},
    {"Add": {"Item": {"Id": "health-potion"}, "Amount": 5}}
  ]
}'
```

### Atomic transfer operations

```bash
playfab-cli tool call execute_transfer_operations --json '{
  "givingEntity": {"Id": "PLAYER_A", "Type": "title_player_account"},
  "receivingEntity": {"Id": "PLAYER_B", "Type": "title_player_account"},
  "operations": [
    {"Item": {"Id": "sword-001"}, "Amount": 1},
    {"Item": {"Id": "shield-001"}, "Amount": 1}
  ]
}'
```

### Batch from file

```bash
playfab-cli batch --json "$(cat batch_operations.json)"
```

### Batch from stdin

```bash
cat batch_operations.json | playfab-cli batch --stdin
```

## Common Workflows

### 1. Season reward distribution

```bash
# Grant season rewards to all qualifying players
playfab-cli tool call grant_items_to_users --json '{
  "items": [
    {"PlayFabId": "PLAYER_1", "items": [{"Id": "season-reward", "Amount": 1}]},
    {"PlayFabId": "PLAYER_2", "items": [{"Id": "season-reward", "Amount": 1}]}
  ]
}'
```

### 2. Catalog bulk setup

```bash
# Create all items at once
playfab-cli tool call batch_create_draft_items --json '{"items": [...]}'
# Then publish each one
playfab-cli tool call publish_draft_item --json '{"id": "item-001"}'
playfab-cli tool call publish_draft_item --json '{"id": "item-002"}'
```

## Rate Limits

Be aware of PlayFab rate limits when performing bulk operations:

- `execute_inventory_operations`: 60 requests per 90 seconds (per player entity)
- `grant_items_to_users`: Follows standard title entity rate limits
- Consider adding delays between large batches
