# Inventory Operations

Use playfab-cli to manage Economy v2 Inventory for players.

## Available Tools

| Tool | Description |
|------|-------------|
| `add_inventory_items` | Add items to a player's inventory |
| `subtract_inventory_items` | Subtract items from a player's inventory |
| `delete_inventory_items` | Delete items from a player's inventory |
| `update_inventory_items` | Update inventory item properties |
| `get_inventory_items` | Get a player's inventory items |
| `get_inventory_collection_ids` | Get collection IDs for a player's inventory |
| `execute_inventory_operations` | Execute multiple operations atomically |
| `purchase_inventory_items` | Purchase an item from the catalog |
| `transfer_inventory_items` | Transfer items between entities |
| `execute_transfer_operations` | Execute multiple transfers atomically |
| `grant_items_to_users` | Grant items to multiple users in a batch |
| `get_transaction_history` | Get transaction history for an entity |
| `get_inventory_operation_status` | Get status of an inventory operation |
| `redeem_apple_appstore_inventory_items` | Redeem Apple App Store purchase |
| `redeem_google_play_inventory_items` | Redeem Google Play purchase |
| `redeem_microsoft_store_inventory_items` | Redeem Microsoft Store purchase |
| `redeem_nintendo_eshop_inventory_items` | Redeem Nintendo eShop purchase |
| `redeem_playstation_store_inventory_items` | Redeem PlayStation Store purchase |
| `redeem_steam_inventory_items` | Redeem Steam purchase |
| `get_microsoft_store_access_tokens` | Get Microsoft Store access tokens |

## Examples

### Get player inventory

```bash
playfab-cli tool call get_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"}
}'
```

### Add items to inventory

```bash
playfab-cli tool call add_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "item": {"Id": "sword-001"},
  "amount": 1
}'
```

### Subtract items

```bash
playfab-cli tool call subtract_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "item": {"Id": "gold-coin"},
  "amount": 100
}'
```

### Purchase an item

```bash
playfab-cli tool call purchase_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "item": {"Id": "sword-001"},
  "amount": 1,
  "priceAmounts": [{"ItemId": "gold-coin", "Amount": 500}],
  "storeId": "main-store"
}'
```

### Transfer items between players

```bash
playfab-cli tool call transfer_inventory_items --json '{
  "givingEntity": {"Id": "PLAYER_A", "Type": "title_player_account"},
  "receivingEntity": {"Id": "PLAYER_B", "Type": "title_player_account"},
  "item": {"Id": "sword-001"},
  "amount": 1
}'
```

### Atomic multi-operation

```bash
playfab-cli tool call execute_inventory_operations --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "operations": [
    {"Subtract": {"Item": {"Id": "gold-coin"}, "Amount": 500}},
    {"Add": {"Item": {"Id": "sword-001"}, "Amount": 1}}
  ]
}'
```

### Grant items to multiple users

```bash
playfab-cli tool call grant_items_to_users --json '{
  "items": [
    {
      "PlayFabId": "PLAYER_A",
      "items": [{"Id": "reward-001", "Amount": 1}]
    },
    {
      "PlayFabId": "PLAYER_B",
      "items": [{"Id": "reward-001", "Amount": 1}]
    }
  ]
}'
```

### Get transaction history

```bash
playfab-cli tool call get_transaction_history --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "count": 20
}'
```

## Common Workflows

### 1. Check and modify inventory

```bash
# List inventory
playfab-cli tool call get_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
# Add item
playfab-cli tool call add_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "item": {"Id": "item-id"}, "amount": 1}'
# Verify
playfab-cli tool call get_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
```

### 2. Virtual currency management

Virtual currencies are inventory items in Economy v2:

```bash
# Check balance
playfab-cli tool call get_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "filter": "type eq '\''currency'\''"}'
# Add currency
playfab-cli tool call add_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "item": {"Id": "gold-coin"}, "amount": 1000}'
```
