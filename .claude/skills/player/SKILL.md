---
name: player
description: Use playfab-cli to manage player profiles, entity data, file uploads, and authentication tokens. Use when the user needs to get/set player profiles, manage entity objects/files, or work with entity tokens. Do not use for inventory operations, admin-level player management, or server-side player operations.
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: entity
---

# Player Management Operations

Use playfab-cli to manage player profiles, data, and authentication.

## Use When

- The user needs to get or update player profiles (display name, avatar, language).
- The user wants to manage entity objects (key-value data on entities).
- The user needs to upload, download, or delete files attached to entities.
- The user wants to get or validate entity tokens for authentication.
- The task involves profile policies or global entity policies.

## Do Not Use When

- The operation manages player inventory (use inventory skill).
- The operation requires admin-level access like banning or deleting players (use admin skill).
- The operation uses server-side player management APIs (use server skill).

## Available Tools

### Profile Tools

| Tool | Description |
|------|-------------|
| `get_profile` | Get the profile of a specific entity |
| `get_profiles` | Get profiles for multiple entities in a single request |
| `set_profile_language` | Set the preferred language for an entity profile |
| `set_profile_policy` | Set the policy statements for an entity profile |
| `get_title_players_account` | Get the title player account entity for a given title player ID |
| `set_display_name` | Set the display name for an entity profile |
| `set_global_policy` | Set the global policy for all entities in the title |
| `get_global_policy` | Get the global policy for all entities in the title |
| `set_avatar_url` | Set the avatar URL for an entity profile |
| `get_entity_profile_policy` | Get the profile policy statements for a specific entity |

### Data Tools

| Tool | Description |
|------|-------------|
| `set_objects` | Set objects on an entity profile (key-value data) |
| `get_objects` | Get objects from an entity profile |
| `initiate_file_uploads` | Initiate file uploads for an entity |
| `abort_file_uploads` | Abort pending file uploads for an entity |
| `finalize_file_uploads` | Finalize file uploads for an entity |
| `get_files` | Get metadata and download URLs for files attached to an entity |
| `delete_files` | Delete files attached to an entity |

### Authentication Tools

| Tool | Description |
|------|-------------|
| `get_entity_token` | Get an entity token for authentication |
| `validate_entity_token` | Validate an entity token |

## Examples

### Get a player profile

```bash
playfab-cli tool call get_profile --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

### Get multiple profiles

```bash
playfab-cli tool call get_profiles --json '{
  "entities": [
    {"Id": "PLAYER_A", "Type": "title_player_account"},
    {"Id": "PLAYER_B", "Type": "title_player_account"}
  ]
}'
```

### Set display name

```bash
playfab-cli tool call set_display_name --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "displayName": "CoolPlayer42"
}'
```

### Set player data (objects)

```bash
playfab-cli tool call set_objects --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "objects": [
    {"objectName": "PlayerStats", "dataObject": {"level": 10, "xp": 5000}},
    {"objectName": "Settings", "dataObject": {"theme": "dark", "language": "en"}}
  ]
}'
```

### Get player data

```bash
playfab-cli tool call get_objects --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

### Get entity token

```bash
playfab-cli tool call get_entity_token --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

### Upload a file to an entity

```bash
# 1. Initiate upload
playfab-cli tool call initiate_file_uploads --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "fileNames": ["avatar.png"]
}'
# 2. Upload to the returned URL (use curl or similar)
# 3. Finalize upload
playfab-cli tool call finalize_file_uploads --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "fileNames": ["avatar.png"]
}'
```

## Common Workflows

### 1. Player onboarding

```bash
# Get entity token
playfab-cli tool call get_entity_token --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
# Set display name
playfab-cli tool call set_display_name --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "displayName": "NewPlayer"}'
# Initialize player data
playfab-cli tool call set_objects --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "objects": [{"objectName": "Progress", "dataObject": {"level": 1, "xp": 0}}]}'
```

### 2. Profile inspection

```bash
playfab-cli tool call get_profile --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "dataAsObject": true}'
playfab-cli tool call get_objects --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
playfab-cli tool call get_files --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
```

## Notes

- Entity types include: `title_player_account`, `title`, `group`, `master_player_account`.
- Profile operations work on any entity type, not just players.
- File uploads are a two-step process: initiate -> upload to URL -> finalize.
- Entity tokens expire after 24 hours by default.
