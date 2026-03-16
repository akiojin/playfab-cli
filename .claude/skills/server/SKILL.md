---
name: server
description: Use playfab-cli for Server API operations including authentication, player data, title data, inventory, friends, characters, virtual currency, events, shared groups, CloudScript execution, content delivery, push notifications, tags, segments, matchmaker, and account linking. Use when the user needs server-side game logic operations. Do not use for admin-level management (use admin skill) or Economy v2 operations (use catalog/inventory).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: server
---

# Server API Operations

Use playfab-cli for server-side game logic operations.

## Use When

- The user needs server-side authentication (session tickets, server custom IDs).
- The user wants to manage player data from the server side.
- The user needs to manage legacy v1 inventory, virtual currency, or characters.
- The user wants to write events from the server side.
- The user needs friend list management or shared group operations.
- The user wants server-side CloudScript execution.
- The task involves game server instance management (register, heartbeat, matchmaker).

## Do Not Use When

- The operation requires admin-level access (use admin skill).
- The operation uses Economy v2 APIs (use catalog, inventory, or economy skill).
- The user needs entity-based group management (use groups skill).
- The user needs multiplayer server hosting (use multiplayer skill).

## Available Tools

### Authentication

| Tool | Description |
|------|-------------|
| `server_authenticate_session_ticket` | Validates a user's session ticket |
| `server_login_with_server_custom_id` | Log in a player using a server-assigned custom identifier |
| `server_login_with_xbox` | Log in a player using an Xbox Live token |
| `server_login_with_xbox_id` | Log in a player using an Xbox ID and Sandbox |
| `server_set_player_secret` | Set the player secret for sign-in |

### Player Account Management

| Tool | Description |
|------|-------------|
| `server_ban_users` | Ban one or more players from the title |
| `server_get_player_profile` | Retrieve a player's profile information |
| `server_get_user_account_info` | Retrieve user's full account information |
| `server_get_user_bans` | Retrieve all bans for a player |
| `server_revoke_bans` | Revoke one or more bans |
| `server_update_bans` | Update existing player bans |
| `server_delete_player` | Permanently delete a player account |
| `server_update_user_title_display_name` | Update display name for a player |

### Identity Lookup

| Tool | Description |
|------|-------------|
| `server_get_play_fab_id_from_facebook_id` | Retrieve PlayFab ID from Facebook ID |
| `server_get_play_fab_id_from_steam_id` | Retrieve PlayFab ID from Steam ID |
| `server_get_play_fab_id_from_xbox_live_id` | Retrieve PlayFab ID from Xbox Live ID |
| `server_get_play_fab_id_from_nintendo_service_account_id` | Retrieve PlayFab ID from Nintendo ID |
| `server_get_play_fab_id_from_psn_account_id` | Retrieve PlayFab ID from PSN ID |
| `server_get_play_fab_id_from_google_id` | Retrieve PlayFab ID from Google ID |

### Player Data

| Tool | Description |
|------|-------------|
| `server_get_user_data` | Retrieve user-specific custom data |
| `server_update_user_data` | Update user-specific custom data |
| `server_get_user_internal_data` | Retrieve internal user data |
| `server_update_user_internal_data` | Update internal user data |
| `server_get_user_read_only_data` | Retrieve read-only user data |
| `server_update_user_read_only_data` | Update read-only user data |
| `server_get_user_publisher_data` | Retrieve publisher-specific data |
| `server_update_user_publisher_data` | Update publisher-specific data |
| `server_get_user_publisher_internal_data` | Retrieve publisher-specific internal data |
| `server_update_user_publisher_internal_data` | Update publisher-specific internal data |
| `server_get_user_publisher_read_only_data` | Retrieve publisher-specific read-only data |
| `server_update_user_publisher_read_only_data` | Update publisher-specific read-only data |
| `server_get_player_combined_info` | Retrieve all requested data in a single call |

### Player Statistics

| Tool | Description |
|------|-------------|
| `server_get_player_statistics` | Retrieve player statistics |
| `server_update_player_statistics` | Update player statistics |
| `server_get_player_statistic_versions` | Retrieve statistic version information |

### Title Data

| Tool | Description |
|------|-------------|
| `server_get_title_data` | Retrieve key-value data for the title |
| `server_set_title_data` | Set key-value data for the title |
| `server_get_title_internal_data` | Retrieve internal title data |
| `server_set_title_internal_data` | Set internal title data |
| `server_get_title_news` | Retrieve title news items |
| `server_get_publisher_data` | Retrieve publisher-specific key-value data |

### Legacy Economy v1

| Tool | Description |
|------|-------------|
| `server_get_catalog_items` | Retrieve title's catalog (legacy v1) |
| `server_get_store_items` | Retrieve store items (legacy v1) |
| `server_get_user_inventory` | Retrieve player's inventory (legacy v1) |
| `server_grant_items_to_user` | Grant items to a player |
| `server_grant_items_to_users` | Grant items to multiple players |
| `server_modify_item_uses` | Modify remaining uses of an item |
| `server_move_item_to_character_from_user` | Move item from player to character |
| `server_move_item_to_user_from_character` | Move item from character to player |
| `server_consume_item` | Consume a use of an item |
| `server_unlock_container_instance` | Unlock a container item instance |
| `server_unlock_container_item` | Open a container by catalog item ID |
| `server_redeem_coupon` | Redeem a coupon code |
| `server_revoke_inventory_item` | Revoke a single item instance |
| `server_revoke_inventory_items` | Revoke multiple item instances |

### Virtual Currency

| Tool | Description |
|------|-------------|
| `server_add_user_virtual_currency` | Add virtual currency to player |
| `server_subtract_user_virtual_currency` | Subtract virtual currency from player |
| `server_get_user_virtual_currency` | Retrieve player's virtual currency balances |
| `server_add_character_virtual_currency` | Add virtual currency to character |
| `server_subtract_character_virtual_currency` | Subtract virtual currency from character |

### Friends

| Tool | Description |
|------|-------------|
| `server_get_friends_list` | Retrieve friends list for a player |
| `server_add_friend` | Add a friend for a player |
| `server_remove_friend` | Remove a friend |
| `server_set_friend_tags` | Set tags on a friend |

### Characters

| Tool | Description |
|------|-------------|
| `server_get_all_users_characters` | Retrieve all characters for a player |
| `server_get_character_data` | Retrieve character-specific custom data |
| `server_update_character_data` | Update character-specific custom data |
| `server_get_character_internal_data` | Retrieve internal character data |
| `server_update_character_internal_data` | Update internal character data |
| `server_get_character_read_only_data` | Retrieve read-only character data |
| `server_update_character_read_only_data` | Update read-only character data |
| `server_get_character_statistics` | Retrieve character statistics |
| `server_update_character_statistics` | Update character statistics |
| `server_grant_character_to_user` | Create a new character for a player |
| `server_delete_character_from_user` | Delete a character |
| `server_get_character_leaderboard` | Retrieve character statistic leaderboard |

### Events

| Tool | Description |
|------|-------------|
| `server_write_player_event` | Write a custom PlayStream event for a player |
| `server_write_title_event` | Write a custom PlayStream event for the title |
| `server_write_character_event` | Write a custom PlayStream event for a character |

### Shared Groups

| Tool | Description |
|------|-------------|
| `server_create_shared_group` | Create a new shared group |
| `server_get_shared_group_data` | Retrieve data from a shared group |
| `server_update_shared_group_data` | Update data in a shared group |
| `server_remove_shared_group_members` | Remove members from a shared group |
| `server_add_shared_group_members` | Add members to a shared group |

### CloudScript

| Tool | Description |
|------|-------------|
| `server_execute_cloud_script` | Execute CloudScript on behalf of a player |
| `server_execute_cloud_script_server` | Execute CloudScript with server-level permissions |

### Content & Utilities

| Tool | Description |
|------|-------------|
| `server_get_content_download_url` | Get URL for downloading content from CDN |
| `server_get_time` | Retrieve current server time |

### Push Notifications

| Tool | Description |
|------|-------------|
| `server_send_push_notification` | Send a push notification to a player |
| `server_send_push_notification_from_template` | Send push notification using template |
| `server_delete_push_notification_template` | Delete a push notification template |

### Tags & Segments

| Tool | Description |
|------|-------------|
| `server_get_player_tags` | Retrieve tags for a player |
| `server_add_player_tag` | Add a tag to a player |
| `server_remove_player_tag` | Remove a tag from a player |
| `server_get_all_segments` | Retrieve all player segments |
| `server_get_players_in_segment` | Retrieve players in a segment |
| `server_get_player_segment_membership` | Retrieve segment memberships for a player |

### Platform Integration

| Tool | Description |
|------|-------------|
| `server_award_steam_achievement` | Award a Steam achievement |

### Random Result Tables

| Tool | Description |
|------|-------------|
| `server_get_random_result_tables` | Retrieve random result tables |
| `server_evaluate_random_result_table` | Evaluate a random result table |

### Account Linking

| Tool | Description |
|------|-------------|
| `server_link_server_custom_id` | Link a server custom ID to player |
| `server_unlink_server_custom_id` | Unlink a server custom ID |
| `server_link_xbox_account` | Link Xbox Live account to player |
| `server_unlink_xbox_account` | Unlink Xbox Live account |

### Legacy Matchmaker

| Tool | Description |
|------|-------------|
| `server_get_matchmaker_game_info` | Get matchmaker game instance info |
| `server_deregister_game` | Deregister a game server instance |
| `server_register_game` | Register a new game server instance |
| `server_refresh_game_server_instance_heartbeat` | Refresh game server heartbeat |
| `server_redeem_matchmaker_ticket` | Validate matchmaker ticket |
| `server_notify_matchmaker_player_left` | Notify player left the game |
| `server_set_game_server_instance_data` | Set custom data for game server |
| `server_set_game_server_instance_state` | Set game server state |
| `server_set_game_server_instance_tags` | Set tags for game server |

## Examples

### Authenticate a session ticket

```bash
playfab-cli tool call server_authenticate_session_ticket --json '{
  "sessionTicket": "TICKET-ABCD1234..."
}'
```

### Get player data

```bash
playfab-cli tool call server_get_user_data --json '{
  "PlayFabId": "ABCD1234",
  "keys": ["Level", "XP", "Inventory"]
}'
```

### Grant items to a player

```bash
playfab-cli tool call server_grant_items_to_user --json '{
  "PlayFabId": "ABCD1234",
  "catalogVersion": "main",
  "itemIds": ["sword-001", "shield-001"]
}'
```

### Update player statistics

```bash
playfab-cli tool call server_update_player_statistics --json '{
  "PlayFabId": "ABCD1234",
  "statistics": [
    {"statisticName": "HighScore", "value": 9999},
    {"statisticName": "GamesPlayed", "value": 1}
  ]
}'
```

### Write a player event

```bash
playfab-cli tool call server_write_player_event --json '{
  "PlayFabId": "ABCD1234",
  "eventName": "player_completed_quest",
  "body": {"questId": "quest-001", "reward": "gold-100"}
}'
```

### Execute CloudScript

```bash
playfab-cli tool call server_execute_cloud_script --json '{
  "PlayFabId": "ABCD1234",
  "functionName": "GrantDailyReward",
  "functionParameter": {"day": 7},
  "revisionSelection": "Live",
  "generatePlayStreamEvent": true
}'
```

### Manage friends

```bash
playfab-cli tool call server_add_friend --json '{
  "PlayFabId": "PLAYER_A",
  "friendPlayFabId": "PLAYER_B"
}'
```

### Register a game server

```bash
playfab-cli tool call server_register_game --json '{
  "lobbyId": "lobby-123",
  "serverIPV4Address": "10.0.0.1",
  "serverPort": "7777",
  "build": "build-001",
  "region": "USEast",
  "gameMode": "Deathmatch"
}'
```

## Common Workflows

### 1. Player session management

```bash
# Validate session
playfab-cli tool call server_authenticate_session_ticket --json '{"sessionTicket": "..."}'
# Get combined player info
playfab-cli tool call server_get_player_combined_info --json '{"PlayFabId": "...", "infoRequestParameters": {"GetUserData": true, "GetPlayerStatistics": true}}'
```

### 2. Character management

```bash
# Create character
playfab-cli tool call server_grant_character_to_user --json '{"PlayFabId": "...", "characterName": "Warrior", "characterType": "Fighter"}'
# Set character data
playfab-cli tool call server_update_character_data --json '{"PlayFabId": "...", "characterId": "...", "data": {"Level": "1", "HP": "100"}}'
# Get character stats
playfab-cli tool call server_get_character_statistics --json '{"PlayFabId": "...", "characterId": "..."}'
```

### 3. Shared group collaboration

```bash
# Create shared group
playfab-cli tool call server_create_shared_group --json '{"sharedGroupId": "party-alpha"}'
# Add members
playfab-cli tool call server_add_shared_group_members --json '{"sharedGroupId": "party-alpha", "PlayFabIds": ["PLAYER_A", "PLAYER_B"]}'
# Update shared data
playfab-cli tool call server_update_shared_group_data --json '{"sharedGroupId": "party-alpha", "data": {"partyStatus": "active", "currentQuest": "dungeon-01"}}'
```

## Notes

- All Server API operations require the `PLAYFAB_DEV_SECRET_KEY` environment variable.
- Server API is designed for game server backends — never expose the secret key to clients.
- Many Server API tools mirror Admin API tools but with different authentication contexts.
- Legacy v1 inventory operations are included for backward compatibility — prefer Economy v2 for new development.
- `server_authenticate_session_ticket` rate limit: 10,000 requests per 10 seconds.
- Character operations require the character to be created first via `server_grant_character_to_user`.
- Shared groups differ from entity groups — shared groups use PlayFab IDs, entity groups use entity keys.
