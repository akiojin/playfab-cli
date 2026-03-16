---
name: leaderboards
description: Use playfab-cli to manage leaderboards and player statistics via the Progression API. Use when the user needs to create leaderboard/statistic definitions, update scores, query rankings, or manage leaderboard versions. Do not use for legacy v1 player statistics (use admin/server skill).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: progression
---

# Leaderboard and Statistics Operations

Use playfab-cli to manage leaderboards and player statistics via the Progression API.

## Use When

- The user needs to create or manage leaderboard definitions.
- The user wants to update leaderboard entries or player statistics.
- The user needs to query rankings or get leaderboard data.
- The user wants to manage statistic definitions.
- The task involves competitive features, rankings, or progression tracking.

## Do Not Use When

- The operation uses legacy v1 player statistics APIs (use admin or server skill).
- The user needs character-level statistics (use server skill).
- The user needs player-level statistics without leaderboard context.

## Available Tools

| Tool | Description |
|------|-------------|
| `create_leaderboard_definition` | Create a new leaderboard definition |
| `delete_leaderboard_definition` | Delete a leaderboard definition |
| `get_leaderboard_definition` | Get the definition of a leaderboard |
| `update_statistics` | Update statistics for an entity |
| `get_statistics` | Get statistics for an entity |
| `get_statistics_for_entities` | Get statistics for multiple entities |
| `delete_statistics` | Delete statistics for an entity |
| `create_statistic_definition` | Create a new statistic definition |
| `delete_statistic_definition` | Delete a statistic definition |
| `get_statistic_definition` | Get the definition of a statistic |
| `get_leaderboard` | Get a leaderboard with rankings |
| `get_leaderboard_for_entities` | Get leaderboard entries for specific entities |
| `get_leaderboard_around_entity` | Get leaderboard entries around a specific entity |
| `increment_leaderboard_version` | Increment the version of a leaderboard, resetting all entries |
| `list_statistic_definitions` | List all statistic definitions for the title |
| `list_leaderboard_definitions` | List all leaderboard definitions for the title |
| `update_leaderboard_entries` | Update or insert entries in a leaderboard |
| `delete_leaderboard_entries` | Delete entries from a leaderboard |
| `unlink_leaderboard_from_statistic` | Unlink a leaderboard from a statistic |
| `link_leaderboard_to_statistic` | Link a leaderboard to a statistic for automatic updates |
| `get_friend_leaderboard_for_entity` | Get a leaderboard filtered to an entity's friends |
| `list_leaderboard_around_entity` | List leaderboard entries around a specific entity (alias) |
| `get_entity_statistics` | Get statistics for an entity (alias for get_statistics) |
| `update_entity_statistics` | Update statistics for an entity (alias for update_statistics) |

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

## Notes

- Leaderboards use the Progression API (separate from legacy v1 statistics).
- Statistic definitions control aggregation method (Sum, Max, Min, Last).
- Leaderboards can be linked to statistics for automatic score updates.
- `increment_leaderboard_version` resets all entries -- use for seasonal resets.
- Alias tools (`get_entity_statistics`, `update_entity_statistics`, `list_leaderboard_around_entity`) provide backward compatibility.
