# Entity Group Management

Use playfab-cli to manage entity groups (guilds, clans, teams, etc.).

## Available Tools

### Group CRUD

| Tool | Description |
|------|-------------|
| `create_group` | Create a new group |
| `delete_group` | Delete a group |
| `get_group` | Get group information |
| `update_group` | Update group properties |

### Membership

| Tool | Description |
|------|-------------|
| `list_group_members` | List all members of a group |
| `remove_members` | Remove members from a group |
| `is_member` | Check if an entity is a member |
| `list_membership` | List all groups an entity belongs to |
| `change_member_role` | Change a member's role |

### Applications and Invitations

| Tool | Description |
|------|-------------|
| `apply_to_group` | Apply to join a group |
| `accept_group_application` | Accept a join application |
| `remove_group_application` | Remove a join application |
| `list_group_applications` | List pending applications |
| `invite_to_group` | Invite an entity to join |
| `accept_group_invitation` | Accept an invitation |
| `remove_group_invitation` | Remove an invitation |
| `list_group_invitations` | List pending invitations |
| `list_membership_opportunities` | List invitations and applications for an entity |

### Blocking

| Tool | Description |
|------|-------------|
| `block_entity` | Block an entity from joining |
| `unblock_entity` | Unblock a blocked entity |
| `list_group_blocks` | List all blocked entities |

### Roles

| Tool | Description |
|------|-------------|
| `create_role` | Create a new role in a group |
| `delete_role` | Delete a role from a group |
| `update_role` | Update role properties |
| `list_group_roles` | List all roles in a group |

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
