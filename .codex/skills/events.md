# PlayStream Events and Telemetry

Use playfab-cli to write events and manage telemetry keys.

## Available Tools

### Event Writing

| Tool | Description |
|------|-------------|
| `write_events` | Write one or more PlayStream events |
| `write_telemetry_events` | Write telemetry events to the telemetry pipeline |

### Telemetry Key Management

| Tool | Description |
|------|-------------|
| `create_telemetry_key` | Create a new telemetry key |
| `delete_telemetry_key` | Delete a telemetry key |
| `get_telemetry_key` | Get details of a telemetry key |
| `list_telemetry_keys` | List all telemetry keys |
| `set_telemetry_key_active` | Activate or deactivate a telemetry key |

## Examples

### Write a PlayStream event

```bash
playfab-cli tool call write_events --json '{
  "events": [
    {
      "name": "player_level_up",
      "eventNamespace": "custom.gameplay",
      "payload": {"newLevel": 10, "previousLevel": 9},
      "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
    }
  ]
}'
```

### Write multiple events

```bash
playfab-cli tool call write_events --json '{
  "events": [
    {
      "name": "item_purchased",
      "eventNamespace": "custom.economy",
      "payload": {"itemId": "sword-001", "price": 500, "currency": "gold"}
    },
    {
      "name": "currency_spent",
      "eventNamespace": "custom.economy",
      "payload": {"currency": "gold", "amount": 500, "reason": "purchase"}
    }
  ]
}'
```

### Write telemetry events

```bash
playfab-cli tool call write_telemetry_events --json '{
  "events": [
    {
      "name": "session_start",
      "eventNamespace": "custom.telemetry",
      "payload": {"platform": "iOS", "appVersion": "1.2.3", "deviceModel": "iPhone15"}
    }
  ]
}'
```

### Create a telemetry key

```bash
playfab-cli tool call create_telemetry_key --json '{"keyName": "mobile-client-key"}'
```

### List telemetry keys

```bash
playfab-cli tool call list_telemetry_keys --json '{}'
```

### Get telemetry key details

```bash
playfab-cli tool call get_telemetry_key --json '{"keyName": "mobile-client-key"}'
```

### Activate/deactivate a key

```bash
playfab-cli tool call set_telemetry_key_active --json '{"keyName": "mobile-client-key", "active": false}'
```

### Delete a telemetry key

```bash
playfab-cli tool call delete_telemetry_key --json '{"keyName": "old-key"}'
```

## Common Workflows

### 1. Set up telemetry pipeline

```bash
# Create key for client
playfab-cli tool call create_telemetry_key --json '{"keyName": "game-client"}'
# Verify
playfab-cli tool call get_telemetry_key --json '{"keyName": "game-client"}'
# Test event writing
playfab-cli tool call write_telemetry_events --json '{"events": [{"name": "test", "eventNamespace": "custom.test", "payload": {"test": true}}]}'
```

### 2. Rotate telemetry keys

```bash
# Create new key
playfab-cli tool call create_telemetry_key --json '{"keyName": "game-client-v2"}'
# Deactivate old key
playfab-cli tool call set_telemetry_key_active --json '{"keyName": "game-client-v1", "active": false}'
# Delete old key after migration
playfab-cli tool call delete_telemetry_key --json '{"keyName": "game-client-v1"}'
```

### 3. Track custom game events

```bash
playfab-cli tool call write_events --json '{
  "events": [
    {"name": "match_completed", "eventNamespace": "custom.gameplay", "payload": {"matchId": "m-123", "duration": 300, "winner": "PLAYER_A"}},
    {"name": "xp_earned", "eventNamespace": "custom.progression", "payload": {"amount": 150, "source": "match"}, "entity": {"Id": "PLAYER_A", "Type": "title_player_account"}}
  ]
}'
```
