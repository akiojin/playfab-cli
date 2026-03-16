---
name: multiplayer
description: Use playfab-cli to manage matchmaking, lobbies, multiplayer server hosting, Party networking, and build management. Use when the user needs matchmaking, lobby operations, server builds, container management, or Party features. Do not use for legacy matchmaking APIs (use admin/server skill).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: multiplayer
---

# Multiplayer Operations

Use playfab-cli to manage matchmaking, lobbies, multiplayer server hosting, Party networking, and build management.

## Use When

- The user needs matchmaking operations (tickets, queues, matches).
- The user wants to create or manage lobbies.
- The user needs to manage multiplayer server builds and deployments.
- The user wants to manage container images, certificates, or assets.
- The task involves Party networking or server backfill.

## Do Not Use When

- The operation uses legacy matchmaking APIs (use admin or server skill).
- The user needs individual player management (use player skill).
- The user needs server-side game instance management (use server skill for `server_register_game` etc.).

## Available Tools

### Matchmaking

| Tool | Description |
|------|-------------|
| `mp_create_matchmaking_ticket` | Create a matchmaking ticket for a player to find a match |
| `mp_get_matchmaking_ticket` | Get the status and details of a matchmaking ticket |
| `mp_cancel_matchmaking_ticket` | Cancel a specific matchmaking ticket |
| `mp_cancel_all_matchmaking_tickets_for_player` | Cancel all matchmaking tickets for a specific player |
| `mp_list_matchmaking_tickets_for_player` | List all matchmaking tickets for a specific player |
| `mp_get_match` | Get details of a completed match including matched players |
| `mp_list_matchmaking_queues_for_title` | List all matchmaking queue configurations for the title |
| `mp_set_matchmaking_queue` | Create or update a matchmaking queue configuration |
| `mp_remove_matchmaking_queue` | Remove a matchmaking queue configuration |
| `mp_get_queue_statistics` | Get statistics for a matchmaking queue |

### Server Hosting -- Builds

| Tool | Description |
|------|-------------|
| `mp_list_build_summaries` | List summaries of all multiplayer server builds |
| `mp_get_build` | Get details of a specific multiplayer server build |
| `mp_create_build_with_managed_container` | Create a build with a managed container image |
| `mp_create_build_with_custom_container` | Create a build with a custom container image |
| `mp_create_build_with_process_based_server` | Create a build with a process-based server |
| `mp_delete_build` | Delete a multiplayer server build |
| `mp_update_build_name` | Update the name of a build |
| `mp_update_build_region` | Update a single region configuration for a build |
| `mp_update_build_regions` | Update multiple region configurations for a build |
| `mp_list_build_summaries_v2` | List build summaries with enhanced v2 response format |

### Server Hosting -- Aliases

| Tool | Description |
|------|-------------|
| `mp_list_build_aliases` | List all build aliases for the title |
| `mp_create_build_alias` | Create a build alias to route traffic between builds |
| `mp_update_build_alias` | Update an existing build alias |
| `mp_delete_build_alias` | Delete a build alias |
| `mp_get_build_alias` | Get details of a specific build alias |

### Server Hosting -- Containers

| Tool | Description |
|------|-------------|
| `mp_rollover_container_registry_credentials` | Roll over container registry credentials |
| `mp_list_container_images` | List container images available for builds |
| `mp_list_container_image_tags` | List tags for a specific container image |
| `mp_get_container_registry_credentials` | Get the credentials for the container registry |

### Server Hosting -- Certificates and Assets

| Tool | Description |
|------|-------------|
| `mp_upload_certificate` | Upload a certificate for use with builds |
| `mp_delete_certificate` | Delete a certificate |
| `mp_list_assets_summaries` | List asset summaries for builds |
| `mp_get_asset_upload_url` | Get a URL for uploading an asset |
| `mp_delete_asset` | Delete an asset |

### Server Hosting -- Servers

| Tool | Description |
|------|-------------|
| `mp_list_multiplayer_servers` | List multiplayer servers for a build in a region |
| `mp_request_multiplayer_server` | Request a multiplayer server session |
| `mp_get_multiplayer_server_details` | Get details of a server session |
| `mp_shutdown_multiplayer_server` | Shut down a server session |
| `mp_list_virtual_machine_summaries` | List VM summaries for a build in a region |
| `mp_get_remote_login_endpoint` | Get remote login endpoint for a VM |
| `mp_create_remote_user` | Create a remote user on a VM for debugging |
| `mp_delete_remote_user` | Delete a remote user from a VM |
| `mp_list_archived_multiplayer_servers` | List archived servers |
| `mp_get_multiplayer_server_logs` | Get logs for a server session |
| `mp_get_title_enabled_for_multiplayer_servers_status` | Check if multiplayer servers are enabled |
| `mp_enable_multiplayer_servers_for_title` | Enable multiplayer servers |
| `mp_get_title_multiplayer_servers_quotas` | Get quota limits and usage |

### Lobbies

| Tool | Description |
|------|-------------|
| `mp_create_lobby` | Create a new lobby |
| `mp_delete_lobby` | Delete a lobby |
| `mp_find_lobbies` | Find lobbies matching criteria |
| `mp_get_lobby` | Get details of a specific lobby |
| `mp_invite_to_lobby` | Invite a player to join a lobby |
| `mp_join_lobby` | Join an existing lobby |
| `mp_leave_lobby` | Leave a lobby |
| `mp_update_lobby` | Update lobby properties and data |
| `mp_subscribe_to_lobby_resource` | Subscribe to lobby notifications |
| `mp_unsubscribe_from_lobby_resource` | Unsubscribe from lobby notifications |

### Party

| Tool | Description |
|------|-------------|
| `mp_list_party_qos_servers` | List QoS servers for Party |
| `mp_request_party_service` | Request a Party service endpoint |
| `mp_create_party` | Create a new Party network |
| `mp_get_party` | Get details of a Party network |
| `mp_join_party` | Join a Party network |
| `mp_leave_party` | Leave a Party network |

### Server Backfill

| Tool | Description |
|------|-------------|
| `mp_list_server_backfill_tickets_for_player` | List server backfill tickets for a player |
| `mp_create_server_backfill_ticket` | Create a server backfill ticket |
| `mp_get_server_backfill_ticket` | Get status of a server backfill ticket |
| `mp_cancel_server_backfill_ticket` | Cancel a server backfill ticket |
| `mp_cancel_all_server_backfill_tickets_for_player` | Cancel all server backfill tickets for a player |

## Examples

### Create a matchmaking ticket

```bash
playfab-cli tool call mp_create_matchmaking_ticket --json '{
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
playfab-cli tool call mp_get_matchmaking_ticket --json '{
  "ticketId": "ticket-123",
  "queueName": "RankedMatch"
}'
```

### Create a lobby

```bash
playfab-cli tool call mp_create_lobby --json '{
  "owner": {"Id": "ABCD1234", "Type": "title_player_account"},
  "maxPlayers": 8,
  "lobbyData": {"map": "forest", "mode": "deathmatch"},
  "accessPolicy": "Public"
}'
```

### Find available lobbies

```bash
playfab-cli tool call mp_find_lobbies --json '{
  "filter": "lobby/lobbyData/map eq '\''forest'\''",
  "orderBy": "lobby/memberCount desc",
  "pagination": {"pageSize": 20}
}'
```

### Request a game server

```bash
playfab-cli tool call mp_request_multiplayer_server --json '{
  "buildId": "build-abc-123",
  "preferredRegions": ["EastUs", "WestUs"],
  "sessionId": "session-xyz-456"
}'
```

## Common Workflows

### 1. Matchmaking flow

```bash
# Create ticket
playfab-cli tool call mp_create_matchmaking_ticket --json '{"creator": {...}, "queueName": "Ranked", "giveUpAfterSeconds": 120}'
# Poll for match
playfab-cli tool call mp_get_matchmaking_ticket --json '{"ticketId": "...", "queueName": "Ranked"}'
# Get match details
playfab-cli tool call mp_get_match --json '{"matchId": "...", "queueName": "Ranked"}'
```

### 2. Lobby-based game

```bash
# Create lobby
playfab-cli tool call mp_create_lobby --json '{"owner": {...}, "maxPlayers": 4}'
# Invite friends
playfab-cli tool call mp_invite_to_lobby --json '{"lobbyId": "...", "inviteeEntity": {...}}'
# Start game when ready
playfab-cli tool call mp_request_multiplayer_server --json '{"buildId": "...", "sessionId": "..."}'
```

## Notes

- Multiplayer tools use the `mp_` prefix to distinguish from other APIs.
- Matchmaking queues must be configured before creating tickets.
- Server builds require multiplayer servers to be enabled for the title.
- Party networking provides voice chat and data communication between players.
