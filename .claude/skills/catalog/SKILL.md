---
name: catalog
description: Use playfab-cli to manage Economy v2 Catalog items including search, CRUD, bundles, stores, subscriptions, and moderation. Use when the user needs to create, search, update, publish, or delete catalog items. Do not use for inventory operations on player accounts or for legacy v1 economy APIs.
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: economy
---

# Catalog Operations

Use playfab-cli to manage Economy v2 Catalog items.

### Use When

- The user needs to search, create, update, publish, or delete catalog items.
- The user wants to manage bundles, stores, subscriptions, or crafting recipes.
- The user needs to moderate catalog items or manage content uploads.
- The user wants to inspect catalog configuration or item publish status.
- The task involves Economy v2 catalog management.

### Do Not Use When

- The operation targets player inventory (use inventory skill).
- The operation uses legacy v1 economy APIs (use admin skill).
- The user only needs title-level configuration without item management.

## Available Tools

| Tool | Description |
|------|-------------|
| `search_items` | Search catalog items with optional filter, search, and orderBy |
| `get_item` | Retrieve a specific catalog item by ID |
| `get_items` | Retrieve multiple published catalog items by IDs |
| `create_draft_item` | Create a new draft catalog item |
| `update_draft_item` | Update an existing draft catalog item |
| `publish_draft_item` | Publish a draft item to make it live |
| `delete_item` | Delete a catalog item by ID |
| `get_draft_items` | Retrieve multiple draft items by IDs |
| `get_entity_draft_items` | Retrieve all draft items owned by an entity |
| `batch_create_draft_items` | Create multiple draft items in a batch |
| `get_catalog_config` | Retrieve the catalog configuration |
| `update_catalog_config` | Update the catalog configuration |
| `create_upload_urls` | Create URLs for uploading content files |
| `get_item_moderation_state` | Get moderation state of an item |
| `set_item_moderation_state` | Set moderation state of an item |
| `get_item_publish_status` | Get publish status of an item |
| `get_item_reviews` | Get reviews for an item |
| `submit_item_review_vote` | Vote on an item review |
| `takedown_item_reviews` | Remove reviews (moderation) |
| `report_item` | Report an item for policy violations |
| `get_item_containers` | Get containers referencing an item |
| `create_bundle` | Create a draft bundle item |
| `create_store` | Create a draft store item |
| `create_subscription` | Create a draft subscription item |
| `get_store_by_friendly_id` | Retrieve a store by friendly ID |
| `create_recipe` | Create a crafting recipe |

## Examples

### Search items

```bash
playfab-cli tool call search_items --json '{"search": "sword", "count": 10}'
```

### Search with OData filter

```bash
playfab-cli tool call search_items --json '{"filter": "type eq '\''catalogItem'\''", "orderBy": "title/en-US asc", "count": 25}'
```

### Get specific item

```bash
playfab-cli tool call get_item --json '{"id": "item-123"}'
```

### Get multiple items

```bash
playfab-cli tool call get_items --json '{"ids": ["item-001", "item-002", "item-003"]}'
```

### Create a draft item

```bash
playfab-cli tool call create_draft_item --json '{
  "item": {
    "Type": "catalogItem",
    "Title": {"NEUTRAL": "Iron Sword"},
    "Description": {"NEUTRAL": "A sturdy iron sword"},
    "StartDate": "2024-01-01T00:00:00Z"
  },
  "publish": false
}'
```

### Publish a draft item

```bash
playfab-cli tool call publish_draft_item --json '{"id": "item-123"}'
```

### Delete an item

```bash
playfab-cli tool call delete_item --json '{"id": "item-123"}'
```

### Create a store

```bash
playfab-cli tool call create_store --json '{
  "item": {
    "Title": {"NEUTRAL": "Main Shop"},
    "Description": {"NEUTRAL": "The main in-game store"}
  },
  "publish": true
}'
```

### Create a bundle

```bash
playfab-cli tool call create_bundle --json '{
  "item": {
    "Title": {"NEUTRAL": "Starter Pack"},
    "ItemReferences": [
      {"Id": "item-001", "Amount": 1},
      {"Id": "item-002", "Amount": 5}
    ]
  },
  "publish": false
}'
```

## Common Workflows

### 1. Full item lifecycle

```bash
# Create draft
playfab-cli tool call create_draft_item --json '{"item": {...}}'
# Review draft
playfab-cli tool call get_entity_draft_items --json '{"entity": {"Id": "...", "Type": "title"}}'
# Publish
playfab-cli tool call publish_draft_item --json '{"id": "item-123"}'
# Verify
playfab-cli tool call get_item --json '{"id": "item-123"}'
```

### 2. Content moderation

```bash
# Check moderation state
playfab-cli tool call get_item_moderation_state --json '{"id": "item-123"}'
# Set moderation state
playfab-cli tool call set_item_moderation_state --json '{"id": "item-123", "status": "Approved", "reason": "Content reviewed"}'
```

## Notes

- All catalog operations use Economy v2 API.
- Draft items must be published via `publish_draft_item` before they appear in searches.
- Use `batch_create_draft_items` for bulk item creation to reduce API calls.
- Moderation tools (`get_item_moderation_state`, `set_item_moderation_state`) are admin-level operations.
