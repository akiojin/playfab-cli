---
name: groups
description: Use playfab-cli to manage entity groups (guilds, clans, teams). Use when the user needs to create, manage, or query groups, handle membership, applications, invitations, blocking, or roles. Do not use for individual player management or shared groups (legacy API).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: entity
---

# Entity Group Management

Use playfab-cli to manage entity groups (guilds, clans, teams, etc.).

## Use When

- The user needs to create, update, delete, or query groups.
- The user wants to manage group membership (add, remove, change roles).
- The user needs to handle join applications or invitations.
- The user wants to block/unblock entities from groups.
- The task involves role management within groups.

## Do Not Use When

- The user needs shared group operations (use server skill for shared groups).
- The operation targets individual player profiles (use player skill).

## Available Tools

| Tool | Description |
|------|-------------|
| `create_group` | Create a new group |
| `delete_group` | Delete a group and all roles, invitations, join requests, and blocks |
| `get_group` | Get information about a group by group key or group name |
| `update_group` | Update the group name and other properties |
| `list_group_members` | List all members of a group across all roles |
| `list_group_applications` | List all outstanding requests to join the group |
| `list_group_invitations` | List all outstanding invitations for the group |
| `list_group_blocks` | List all entities blocked from joining the group |
| `list_membership_opportunities` | List all groups and roles that an entity has been invited to or applied to |
| `apply_to_group` | Apply to join a group (creates a pending application) |
| `accept_group_application` | Accept an outstanding application to join the group |
| `accept_group_invitation` | Accept an invitation to join a group |
| `invite_to_group` | Invite an entity to join the group |
| `remove_group_application` | Remove an outstanding application to join the group |
| `remove_group_invitation` | Remove an outstanding invitation to join the group |
| `remove_members` | Remove members from a group |
| `block_entity` | Block an entity from joining a group |
| `unblock_entity` | Unblock an entity from joining a group |
| `change_member_role` | Change the role of members within a group |
| `is_member` | Check whether an entity is a member of a group |
| `list_membership` | List all groups and roles that an entity is a member of |
| `create_role` | Create a new role within a group |
| `delete_role` | Delete a role from a group |
| `update_role` | Update properties of a role within a group |
| `list_group_roles` | List all roles within a group |

## Examples

### Create a group (guild)

```bash
playfab-cli tool call create_group --json '{"groupName": "Dragon Slayers"}'
```

### Get group info

```bash
playfab-cli tool call get_group --json '{"groupName": "Dragon Slayers"}'
```

### Get group by ID

```bash
playfab-cli tool call get_group --json '{"group": {"Id": "group-123", "Type": "group"}}'
```

### List group members

```bash
playfab-cli tool call list_group_members --json '{"group": {"Id": "group-123", "Type": "group"}}'
```

### Invite a player

```bash
playfab-cli tool call invite_to_group --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "entity": {"Id": "PLAYER_A", "Type": "title_player_account"},
  "roleId": "member"
}'
```

### Accept an application

```bash
playfab-cli tool call accept_group_application --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "entity": {"Id": "PLAYER_B", "Type": "title_player_account"}
}'
```

### Create a role

```bash
playfab-cli tool call create_role --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "roleId": "officer",
  "roleName": "Officer"
}'
```

### Promote a member

```bash
playfab-cli tool call change_member_role --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "members": [{"Id": "PLAYER_A", "Type": "title_player_account"}],
  "originRoleId": "member",
  "destinationRoleId": "officer"
}'
```

### Check membership

```bash
playfab-cli tool call is_member --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "entity": {"Id": "PLAYER_A", "Type": "title_player_account"}
}'
```

### Remove a member

```bash
playfab-cli tool call remove_members --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "members": [{"Id": "PLAYER_A", "Type": "title_player_account"}]
}'
```

### Block an entity

```bash
playfab-cli tool call block_entity --json '{
  "group": {"Id": "group-123", "Type": "group"},
  "entity": {"Id": "BAD_PLAYER", "Type": "title_player_account"}
}'
```

### List player's groups

```bash
playfab-cli tool call list_membership --json '{
  "entity": {"Id": "PLAYER_A", "Type": "title_player_account"}
}'
```

## Common Workflows

### 1. Create a guild with roles

```bash
# Create the group
playfab-cli tool call create_group --json '{"groupName": "Dragon Slayers"}'
# Create roles
playfab-cli tool call create_role --json '{"group": {"Id": "...", "Type": "group"}, "roleId": "officer", "roleName": "Officer"}'
playfab-cli tool call create_role --json '{"group": {"Id": "...", "Type": "group"}, "roleId": "recruit", "roleName": "Recruit"}'
# List roles
playfab-cli tool call list_group_roles --json '{"group": {"Id": "...", "Type": "group"}}'
```

### 2. Process join requests

```bash
# List pending applications
playfab-cli tool call list_group_applications --json '{"group": {"Id": "...", "Type": "group"}}'
# Accept a player
playfab-cli tool call accept_group_application --json '{"group": {"Id": "...", "Type": "group"}, "entity": {"Id": "...", "Type": "title_player_account"}}'
# Reject another
playfab-cli tool call remove_group_application --json '{"group": {"Id": "...", "Type": "group"}, "entity": {"Id": "...", "Type": "title_player_account"}}'
```

### 3. Group administration

```bash
# View members
playfab-cli tool call list_group_members --json '{"group": {"Id": "...", "Type": "group"}}'
# View blocked entities
playfab-cli tool call list_group_blocks --json '{"group": {"Id": "...", "Type": "group"}}'
# Update group name
playfab-cli tool call update_group --json '{"group": {"Id": "...", "Type": "group"}, "groupName": "Elite Dragon Slayers"}'
```

## Notes

- Groups support entity types: `title_player_account`, `master_player_account`.
- Default roles are created automatically: `members` and `admins`.
- Group operations require appropriate permissions -- admin role or group owner.
- Maximum members per group depends on title configuration.
