---
name: cloudscript
description: Use playfab-cli to manage and execute CloudScript functions (Azure Functions integration). Use when the user needs to register, execute, or manage CloudScript Azure Functions, HTTP/queue-triggered functions, or post function execution results. Do not use for admin-level CloudScript revision management (use admin skill).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: scripting
---

# CloudScript and Azure Functions

Use playfab-cli to manage and execute CloudScript functions (Azure Functions integration).

## Use When

- The user needs to register or unregister Azure Functions with PlayFab.
- The user wants to execute a CloudScript function.
- The user needs to list registered functions (HTTP, queue-triggered, or general).
- The user wants to post function execution results.
- The task involves serverless function management.

## Do Not Use When

- The user needs admin-level CloudScript revision management (use admin skill for `admin_update_cloud_script`, `admin_set_published_revision`, etc.).
- The user needs server-side CloudScript execution (use server skill for `server_execute_cloud_script`).

## Available Tools

| Tool | Description |
|------|-------------|
| `execute_function` | Execute a CloudScript Azure Function by name |
| `list_functions` | List all registered CloudScript Azure Functions |
| `register_function` | Register a CloudScript Azure Function |
| `list_http_functions` | List all registered HTTP-triggered Azure Functions |
| `register_http_function` | Register an HTTP-triggered Azure Function |
| `unregister_function` | Unregister a CloudScript Azure Function |
| `list_queued_functions` | List all registered queue-triggered Azure Functions |
| `register_queued_function` | Register a queue-triggered Azure Function |
| `unregister_queued_function` | Unregister a queue-triggered Azure Function |
| `post_function_result_for_entity_triggered_action` | Post result for entity-triggered execution |
| `post_function_result_for_function_execution` | Post result for function execution |
| `post_function_result_for_player_triggered_action` | Post result for player-triggered execution |
| `post_function_result_for_scheduled_task` | Post result for scheduled task execution |
| `execute_entity_cloud_script` | Execute entity CloudScript (legacy revision-based) |

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

## Notes

- Azure Functions integration is the recommended approach over legacy CloudScript revisions.
- HTTP-triggered functions respond to HTTP requests; queue-triggered functions process messages.
- `post_function_result_*` tools are for reporting results back from function executions.
- `execute_entity_cloud_script` is the legacy revision-based approach -- prefer `execute_function`.
