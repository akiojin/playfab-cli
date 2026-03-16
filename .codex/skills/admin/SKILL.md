---
name: admin
description: Use playfab-cli for Admin API operations including player banning, account management, title data, legacy v1 economy, CloudScript revision management, content delivery, task scheduling, statistics, push notifications, and matchmaker configuration. Use when the user needs admin-level operations that require the developer secret key. Do not use for Economy v2 operations (use catalog/inventory) or for server-side game logic (use server skill).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: admin
---

# Admin API Operations

Use playfab-cli for administrative operations that require the developer secret key.

## Use When

- The user needs to ban, revoke bans, or manage player account access.
- The user wants to manage player accounts (delete, export, lookup).
- The user needs to manage title data, internal data, or policies.
- The user wants to use legacy v1 economy APIs (catalog items, virtual currencies, stores).
- The user needs CloudScript revision management (upload, publish revisions).
- The user wants to manage scheduled tasks or content delivery.
- The user needs to manage player statistics definitions or push notifications.

## Do Not Use When

- The operation uses Economy v2 APIs (use catalog, inventory, or economy skill).
- The operation is server-side game logic (use server skill).
- The user needs entity-based group management (use groups skill).
- The user needs matchmaking or multiplayer server management (use multiplayer skill).

## Available Tools

### Player Account Management

| Tool | Description |
|------|-------------|
| `admin_ban_users` | Bans users by PlayFab ID with optional IP/MAC address bans |
| `admin_revoke_bans` | Revokes one or more bans by ban ID |
| `admin_get_user_bans` | Gets all bans for a user |
| `admin_update_bans` | Updates information of a list of existing bans |
| `admin_get_user_account_info` | Retrieves the user's PlayFab account details |
| `admin_delete_player` | Removes a user's player account from the title |
| `admin_get_player_profile` | Retrieves the player's profile |
| `admin_lookup_user_account_info` | Retrieves user details using various identifiers |
| `admin_send_account_recovery_email` | Sends account recovery email to the user |
| `admin_reset_password` | Reset a player's password for a given title |
| `admin_update_user_title_display_name` | Updates the title-specific display name |
| `admin_delete_master_player_account` | Removes a master player account entirely from all titles |
| `admin_export_master_player_data` | Exports all associated data of a master player account |

### Segments

| Tool | Description |
|------|-------------|
| `admin_get_players_in_segment` | Retrieves the list of players in a given segment |
| `admin_get_all_segments` | Retrieves an array of player segment definitions |

### Player Data

| Tool | Description |
|------|-------------|
| `admin_get_user_data` | Retrieves the title-specific custom data for the user |
| `admin_update_user_data` | Updates the title-specific custom data for the user |
| `admin_get_user_internal_data` | Retrieves title-specific custom internal data |
| `admin_update_user_internal_data` | Updates title-specific custom internal data |
| `admin_get_user_read_only_data` | Retrieves title-specific custom read-only data |
| `admin_update_user_read_only_data` | Updates title-specific custom read-only data |
| `admin_get_user_publisher_data` | Retrieves publisher-specific custom data |
| `admin_update_user_publisher_data` | Updates publisher-specific custom data |
| `admin_get_user_publisher_internal_data` | Retrieves publisher-specific custom internal data |
| `admin_update_user_publisher_internal_data` | Updates publisher-specific custom internal data |
| `admin_get_user_publisher_read_only_data` | Retrieves publisher-specific custom read-only data |
| `admin_update_user_publisher_read_only_data` | Updates publisher-specific custom read-only data |

### Title Data & Policies

| Tool | Description |
|------|-------------|
| `admin_get_title_data` | Retrieves the key-value store of custom title settings |
| `admin_set_title_data` | Updates the key-value store of custom title settings |
| `admin_get_title_internal_data` | Retrieves internal title settings |
| `admin_set_title_internal_data` | Updates internal title settings |
| `admin_get_policy` | Gets the requested policy |
| `admin_update_policy` | Changes a policy for a title |

### Legacy Economy v1

| Tool | Description |
|------|-------------|
| `admin_get_catalog_items` | Retrieves the title's catalog of virtual goods (legacy v1) |
| `admin_set_catalog_items` | Creates/updates catalog configuration (legacy v1) |
| `admin_get_store_items` | Retrieves store items (legacy v1) |
| `admin_set_store_items` | Sets all items in a virtual store (legacy v1) |
| `admin_add_virtual_currency_types` | Adds virtual currency types (legacy v1) |
| `admin_list_virtual_currency_types` | Lists virtual currency types (legacy v1) |
| `admin_remove_virtual_currency_types` | Removes virtual currency types (legacy v1) |
| `admin_add_user_virtual_currency` | Increments virtual currency (legacy v1) |
| `admin_subtract_user_virtual_currency` | Decrements virtual currency (legacy v1) |
| `admin_get_user_inventory` | Retrieves user's inventory (legacy v1) |
| `admin_grant_items_to_users` | Adds items to user inventories (legacy v1) |
| `admin_revoke_inventory_item` | Revokes access to an inventory item (legacy v1) |
| `admin_revoke_inventory_items` | Revokes access to items across users (legacy v1) |
| `admin_modify_item_uses` | Modifies remaining uses of an item (legacy v1) |
| `admin_check_limited_edition_item_availability` | Checks limited edition availability (legacy v1) |

### CloudScript

| Tool | Description |
|------|-------------|
| `admin_update_cloud_script` | Creates a new CloudScript revision |
| `admin_get_cloud_script_versions` | Lists all CloudScript versions |
| `admin_get_cloud_script_revision` | Gets a specific CloudScript revision |
| `admin_set_published_revision` | Sets the published revision |

### Content Delivery

| Tool | Description |
|------|-------------|
| `admin_get_content_list` | Retrieves the content list |
| `admin_get_content_upload_url` | Gets a pre-authorized upload URL |
| `admin_delete_content` | Deletes a content file |

### Task Scheduling

| Tool | Description |
|------|-------------|
| `admin_create_actions_on_players_in_segment_task` | Creates a task for segment actions |
| `admin_get_actions_on_players_in_segment_task_instance` | Gets task instance details |
| `admin_get_tasks` | Lists configured tasks |
| `admin_run_task` | Runs a task immediately |
| `admin_abort_task_instance` | Aborts a running task |
| `admin_update_task` | Updates an existing task |

### Random Result Tables

| Tool | Description |
|------|-------------|
| `admin_get_random_result_tables` | Retrieves random drop table configuration |
| `admin_update_random_result_tables` | Updates random drop table configuration |

### Reports & Statistics

| Tool | Description |
|------|-------------|
| `admin_get_data_report` | Retrieves a download URL for a report |
| `admin_get_player_statistics_definitions` | Gets all player statistic configurations |
| `admin_create_player_statistic_definition` | Adds a new player statistic |
| `admin_update_player_statistic_definition` | Updates a player statistic definition |
| `admin_increment_player_statistic_version` | Resets a statistic (backs up old values) |
| `admin_get_player_statistic_versions` | Gets statistic version information |

### Push Notifications

| Tool | Description |
|------|-------------|
| `admin_send_push_notification` | Sends a push notification to a user |
| `admin_send_push_notification_from_template` | Sends push notification from template |
| `admin_set_push_notification` | Sets title push notification settings |

### Player Shared Secrets

| Tool | Description |
|------|-------------|
| `admin_create_player_shared_secret` | Creates a new player shared secret key |
| `admin_delete_player_shared_secret` | Deletes a player shared secret key |
| `admin_get_player_shared_secrets` | Gets all player shared secret keys |
| `admin_set_player_secret` | Sets or resets the player's secret |

### Identity

| Tool | Description |
|------|-------------|
| `admin_get_play_fab_id_from_facebook_instant_games_id` | Gets PlayFab ID from Facebook Instant Games ID |

### Legacy Matchmaker

| Tool | Description |
|------|-------------|
| `admin_get_matchmaker_game_info` | Gets completed session details (legacy) |
| `admin_get_matchmaker_game_modes` | Gets game modes for server executable (legacy) |
| `admin_modify_matchmaker_game_modes` | Updates game server mode details (legacy) |
| `admin_delete_matchmaker_game_modes` | Removes game server executable (legacy) |
| `admin_register_game` | Registers a new game server |
| `admin_deregister_game` | Deregisters a game server |
| `admin_set_game_server_mode` | Sets game server mode (active/inactive) |

## Examples

### Ban a player

```bash
playfab-cli tool call admin_ban_users --json '{
  "bans": [
    {
      "PlayFabId": "ABCD1234",
      "reason": "Cheating",
      "durationInHours": 168
    }
  ]
}'
```

### Get user account info

```bash
playfab-cli tool call admin_get_user_account_info --json '{"PlayFabId": "ABCD1234"}'
```

### Update title data

```bash
playfab-cli tool call admin_set_title_data --json '{
  "key": "GameVersion",
  "value": "1.2.0"
}'
```

### Get player data

```bash
playfab-cli tool call admin_get_user_data --json '{
  "PlayFabId": "ABCD1234",
  "keys": ["PlayerLevel", "XP"]
}'
```

### Create a scheduled task

```bash
playfab-cli tool call admin_create_actions_on_players_in_segment_task --json '{
  "name": "DailyRewards",
  "description": "Grant daily rewards to active players",
  "isActive": true,
  "schedule": "0 0 * * *",
  "segmentId": "active-players-segment",
  "actions": {"grantVirtualCurrency": {"currencyCode": "GD", "amount": 100}}
}'
```

### Upload CloudScript

```bash
playfab-cli tool call admin_update_cloud_script --json '{
  "files": [
    {
      "filename": "main.js",
      "fileContents": "handlers.helloWorld = function(args, context) { return { message: \"Hello!\" }; }"
    }
  ],
  "publish": false
}'
```

### Send push notification

```bash
playfab-cli tool call admin_send_push_notification --json '{
  "recipientPlayFabId": "ABCD1234",
  "message": "Welcome back! Check out new items in the shop.",
  "subject": "Welcome Back"
}'
```

### Manage player statistics

```bash
playfab-cli tool call admin_create_player_statistic_definition --json '{
  "statisticName": "HighScore",
  "aggregationMethod": "Max"
}'
```

## Common Workflows

### 1. Player moderation

```bash
# Check player's ban history
playfab-cli tool call admin_get_user_bans --json '{"PlayFabId": "ABCD1234"}'
# Ban the player
playfab-cli tool call admin_ban_users --json '{"bans": [{"PlayFabId": "ABCD1234", "reason": "Exploiting", "durationInHours": 720}]}'
# Verify ban
playfab-cli tool call admin_get_user_bans --json '{"PlayFabId": "ABCD1234"}'
```

### 2. Title configuration update

```bash
# Get current title data
playfab-cli tool call admin_get_title_data --json '{"keys": ["GameVersion", "MaintenanceMode"]}'
# Update title data
playfab-cli tool call admin_set_title_data --json '{"key": "GameVersion", "value": "2.0.0"}'
playfab-cli tool call admin_set_title_data --json '{"key": "MaintenanceMode", "value": "false"}'
```

### 3. CloudScript deployment

```bash
# Upload new revision
playfab-cli tool call admin_update_cloud_script --json '{"files": [...], "publish": false}'
# Review revisions
playfab-cli tool call admin_get_cloud_script_versions --json '{}'
# Publish the new revision
playfab-cli tool call admin_set_published_revision --json '{"version": 5, "revision": 1}'
```

## Notes

- All Admin API operations require the `PLAYFAB_DEV_SECRET_KEY` environment variable.
- Admin API should only be called from trusted server environments, never from game clients.
- Legacy v1 economy tools are marked with "(legacy v1)" — prefer Economy v2 APIs for new development.
- `admin_delete_master_player_account` is irreversible — use with extreme caution.
- Player data operations (get/update) support title-specific, publisher-specific, internal, and read-only variants.
- Push notification settings must be configured per platform before sending notifications.
