# Multiplayer Operations

Use playfab-cli to manage matchmaking, lobbies, and multiplayer server hosting.

> Note: Multiplayer tools are being implemented. This skill documents the expected tool interface.

## Available Tools (Planned)

### Matchmaking

| Tool | Description |
|------|-------------|
| `create_matchmaking_ticket` | Create a matchmaking ticket |
| `get_matchmaking_ticket` | Get status of a matchmaking ticket |
| `cancel_matchmaking_ticket` | Cancel a matchmaking ticket |
| `cancel_all_matchmaking_tickets` | Cancel all tickets for an entity |
| `list_matchmaking_tickets` | List matchmaking tickets for a queue |
| `get_match` | Get match details |
| `get_queue_statistics` | Get queue statistics |
| `list_queue_statistics` | List statistics for all queues |
| `create_server_backfill_ticket` | Create a server backfill ticket |
| `cancel_server_backfill_ticket` | Cancel a server backfill ticket |

### Lobbies

| Tool | Description |
|------|-------------|
| `create_lobby` | Create a new lobby |
| `get_lobby` | Get lobby details |
| `update_lobby` | Update lobby properties |
| `delete_lobby` | Delete a lobby |
| `join_lobby` | Join an existing lobby |
| `leave_lobby` | Leave a lobby |
| `find_lobbies` | Search for available lobbies |
| `invite_to_lobby` | Invite a player to a lobby |
| `subscribe_to_lobby_resource` | Subscribe to lobby notifications |
| `unsubscribe_from_lobby_resource` | Unsubscribe from lobby notifications |

### Server Hosting

| Tool | Description |
|------|-------------|
| `list_build_summaries` | List all build summaries |
| `get_build` | Get build details |
| `list_multiplayer_servers` | List active game servers |
| `request_multiplayer_server` | Request a new game server |
| `shutdown_multiplayer_server` | Shutdown a game server |

## Examples

### Create a matchmaking ticket

```bash
playfab-cli tool call create_matchmaking_ticket --json '{
  "creator": {
    "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
    "attributes": {"DataObject": {"skill": 1500}}
  },
  "queueName": "RankedMatch",
  "giveUpAfterSeconds": 120
}'
```

### Check matchmaking status

```bash
playfab-cli tool call get_matchmaking_ticket --json '{
  "ticketId": "ticket-123",
  "queueName": "RankedMatch"
}'
```

### Create a lobby

```bash
playfab-cli tool call create_lobby --json '{
  "owner": {"Id": "ABCD1234", "Type": "title_player_account"},
  "maxPlayers": 8,
  "lobbyData": {"map": "forest", "mode": "deathmatch"},
  "accessPolicy": "Public"
}'
```

### Find available lobbies

```bash
playfab-cli tool call find_lobbies --json '{
  "filter": "lobby/lobbyData/map eq '\''forest'\''",
  "orderBy": "lobby/memberCount desc",
  "pagination": {"pageSize": 20}
}'
```

### Request a game server

```bash
playfab-cli tool call request_multiplayer_server --json '{
  "buildId": "build-abc-123",
  "preferredRegions": ["EastUs", "WestUs"],
  "sessionId": "session-xyz-456"
}'
```

## Common Workflows

### 1. Matchmaking flow

```bash
# Create ticket
playfab-cli tool call create_matchmaking_ticket --json '{"creator": {...}, "queueName": "Ranked", "giveUpAfterSeconds": 120}'
# Poll for match
playfab-cli tool call get_matchmaking_ticket --json '{"ticketId": "...", "queueName": "Ranked"}'
# Get match details
playfab-cli tool call get_match --json '{"matchId": "...", "queueName": "Ranked"}'
```

### 2. Lobby-based game

```bash
# Create lobby
playfab-cli tool call create_lobby --json '{"owner": {...}, "maxPlayers": 4}'
# Invite friends
playfab-cli tool call invite_to_lobby --json '{"lobbyId": "...", "inviteeEntity": {...}}'
# Start game when ready
playfab-cli tool call request_multiplayer_server --json '{"buildId": "...", "sessionId": "..."}'
```
