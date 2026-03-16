# Player Management Operations

Use playfab-cli to manage player profiles, data, and authentication.

## Available Tools

### Profile Tools

| Tool | Description |
|------|-------------|
| `get_profile` | Get the profile of a specific entity |
| `get_profiles` | Get profiles for multiple entities |
| `set_display_name` | Set display name for an entity |
| `set_avatar_url` | Set avatar URL for an entity |
| `set_profile_language` | Set preferred language for an entity |
| `set_profile_policy` | Set policy statements for an entity |
| `get_entity_profile_policy` | Get profile policy for an entity |
| `get_title_players_account` | Get title player account entity |

### Data Tools

| Tool | Description |
|------|-------------|
| `set_objects` | Set key-value objects on an entity |
| `get_objects` | Get objects from an entity |
| `initiate_file_uploads` | Start file uploads for an entity |
| `finalize_file_uploads` | Finalize file uploads |
| `abort_file_uploads` | Abort pending file uploads |
| `get_files` | Get file metadata and download URLs |
| `delete_files` | Delete files from an entity |

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
