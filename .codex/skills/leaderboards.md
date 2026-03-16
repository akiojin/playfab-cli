# Leaderboard and Statistics Operations

Use playfab-cli to manage leaderboards and player statistics via the Progression API.

> Note: Progression tools are being implemented. This skill documents the expected tool interface.

## Available Tools (Planned)

| Tool | Description |
|------|-------------|
| `create_leaderboard_definition` | Create a new leaderboard definition |
| `delete_leaderboard_definition` | Delete a leaderboard definition |
| `get_leaderboard_definition` | Get a leaderboard definition |
| `list_leaderboard_definitions` | List all leaderboard definitions |
| `update_leaderboard_entries` | Update entries in a leaderboard |
| `delete_leaderboard_entries` | Delete entries from a leaderboard |
| `get_leaderboard` | Get leaderboard rankings |
| `get_leaderboard_around_entity` | Get rankings around a specific entity |
| `get_leaderboard_for_entities` | Get rankings for specific entities |
| `increment_leaderboard_version` | Increment (reset) leaderboard version |
| `create_statistic_definition` | Create a new statistic definition |
| `delete_statistic_definition` | Delete a statistic definition |
| `get_statistic_definition` | Get a statistic definition |
| `list_statistic_definitions` | List all statistic definitions |
| `update_statistics` | Update player statistics |
| `get_statistics` | Get player statistics |
| `get_statistics_for_entities` | Get statistics for multiple entities |
| `delete_statistics` | Delete player statistics |

## Examples

### Create a leaderboard

```bash
playfab-cli tool call create_leaderboard_definition --json '{
  "name": "HighScores",
  "sizeLimit": 1000,
  "entityType": "title_player_account",
  "versionConfiguration": {"maxQueryableVersions": 5, "resetInterval": "Month"},
  "columns": [{"name": "Score", "sortDirection": "Descending"}]
}'
```

### Get leaderboard rankings

```bash
playfab-cli tool call get_leaderboard --json '{
  "name": "HighScores",
  "startingPosition": 0,
  "count": 10
}'
```

### Update leaderboard entries

```bash
playfab-cli tool call update_leaderboard_entries --json '{
  "name": "HighScores",
  "entries": [
    {"entityId": "PLAYER_A", "scores": ["1500"]},
    {"entityId": "PLAYER_B", "scores": ["1200"]}
  ]
}'
```

### Get rankings around a player

```bash
playfab-cli tool call get_leaderboard_around_entity --json '{
  "name": "HighScores",
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "maxSurroundingEntries": 5
}'
```

### Create a statistic

```bash
playfab-cli tool call create_statistic_definition --json '{
  "name": "TotalKills",
  "entityType": "title_player_account",
  "aggregationMethod": "Sum"
}'
```

### Update player statistics

```bash
playfab-cli tool call update_statistics --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "statistics": [
    {"name": "TotalKills", "value": 42},
    {"name": "GamesPlayed", "value": 1}
  ]
}'
```

### Get player statistics

```bash
playfab-cli tool call get_statistics --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

## Common Workflows

### 1. Set up a leaderboard

```bash
# Create statistic definition
playfab-cli tool call create_statistic_definition --json '{"name": "Score", "entityType": "title_player_account", "aggregationMethod": "Max"}'
# Create leaderboard definition
playfab-cli tool call create_leaderboard_definition --json '{"name": "WeeklyHighScores", "columns": [{"name": "Score", "sortDirection": "Descending"}], "entityType": "title_player_account", "sizeLimit": 500}'
```

### 2. Weekly reset cycle

```bash
# View current leaderboard
playfab-cli tool call get_leaderboard --json '{"name": "WeeklyHighScores", "count": 50}'
# Reset (increment version)
playfab-cli tool call increment_leaderboard_version --json '{"name": "WeeklyHighScores"}'
```
