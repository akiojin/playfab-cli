# CloudScript and Azure Functions

Use playfab-cli to manage and execute CloudScript functions (Azure Functions integration).

## Available Tools

### Function Execution

| Tool | Description |
|------|-------------|
| `execute_function` | Execute a CloudScript Azure Function by name |
| `execute_entity_cloud_script` | Execute entity CloudScript (legacy revision-based) |

### Function Registration

| Tool | Description |
|------|-------------|
| `register_function` | Register a CloudScript Azure Function |
| `unregister_function` | Unregister a CloudScript function |
| `list_functions` | List all registered CloudScript functions |
| `register_http_function` | Register an HTTP-triggered Azure Function |
| `list_http_functions` | List all HTTP-triggered functions |
| `register_queued_function` | Register a queue-triggered Azure Function |
| `unregister_queued_function` | Unregister a queue-triggered function |
| `list_queued_functions` | List all queue-triggered functions |

### Function Results

| Tool | Description |
|------|-------------|
| `post_function_result_for_entity_triggered_action` | Post result for entity-triggered execution |
| `post_function_result_for_function_execution` | Post result for function execution |
| `post_function_result_for_player_triggered_action` | Post result for player-triggered execution |
| `post_function_result_for_scheduled_task` | Post result for scheduled task execution |

## Examples

### List all registered functions

```bash
playfab-cli tool call list_functions --json '{}'
```

### Register a new function

```bash
playfab-cli tool call register_function --json '{
  "functionName": "ProcessPurchase",
  "functionUrl": "https://my-game-functions.azurewebsites.net/api/ProcessPurchase"
}'
```

### Execute a function

```bash
playfab-cli tool call execute_function --json '{
  "functionName": "ProcessPurchase",
  "functionParameter": {
    "itemId": "sword-001",
    "quantity": 1
  },
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "generatePlayStreamEvent": true
}'
```

### Execute legacy entity CloudScript

```bash
playfab-cli tool call execute_entity_cloud_script --json '{
  "functionName": "GrantDailyReward",
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "functionParameter": {"day": 7},
  "revisionSelection": "Live"
}'
```

### Register an HTTP function

```bash
playfab-cli tool call register_http_function --json '{
  "functionName": "WebhookHandler",
  "functionUrl": "https://my-game-functions.azurewebsites.net/api/WebhookHandler"
}'
```

### Register a queued function

```bash
playfab-cli tool call register_queued_function --json '{
  "functionName": "ProcessBatchJob",
  "queueName": "batch-jobs"
}'
```

### Unregister a function

```bash
playfab-cli tool call unregister_function --json '{"functionName": "OldFunction"}'
```

## Common Workflows

### 1. Deploy and register a new function

```bash
# Deploy Azure Function (outside CLI)
# Register with PlayFab
playfab-cli tool call register_function --json '{"functionName": "MyFunction", "functionUrl": "https://..."}'
# Verify registration
playfab-cli tool call list_functions --json '{}'
# Test execution
playfab-cli tool call execute_function --json '{"functionName": "MyFunction", "functionParameter": {"test": true}}'
```

### 2. Audit registered functions

```bash
playfab-cli tool call list_functions --json '{}'
playfab-cli tool call list_http_functions --json '{}'
playfab-cli tool call list_queued_functions --json '{}'
```

### 3. Replace a function endpoint

```bash
# Unregister old
playfab-cli tool call unregister_function --json '{"functionName": "MyFunction"}'
# Register new endpoint
playfab-cli tool call register_function --json '{"functionName": "MyFunction", "functionUrl": "https://new-endpoint..."}'
```
