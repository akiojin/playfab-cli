---
name: economy
description: Use playfab-cli for Economy v2 virtual currency, store, subscription, and purchase management. Use when the user needs to set up stores, manage virtual currencies, process purchases, handle platform redemptions, or create crafting recipes. Do not use for raw catalog item CRUD (use catalog) or direct inventory manipulation (use inventory).
allowed-tools: Bash, Read, Grep, Glob
metadata:
  author: akiojin
  version: 0.2.0
  category: economy
---

# Virtual Currency and Store Operations

Use playfab-cli to manage Economy v2 virtual currencies, stores, and purchases.

In Economy v2, virtual currencies are managed as inventory items. Stores are special catalog items.

## Use When

- The user needs to create or manage stores and storefronts.
- The user wants to set up virtual currencies as Economy v2 items.
- The user needs to process purchases using virtual currency.
- The user wants to handle platform store redemptions (Apple, Google, Steam, etc.).
- The user needs to create subscriptions or crafting recipes.

## Do Not Use When

- The user needs raw catalog item CRUD without store/currency context (use catalog skill).
- The user needs direct inventory manipulation without purchase context (use inventory skill).
- The operation uses legacy v1 economy APIs (use admin skill).

## Available Tools

### Store Management

| Tool | Description |
|------|-------------|
| `create_store` | Create a draft store item |
| `get_store_by_friendly_id` | Retrieve a store by its friendly ID |
| `search_items` | Search for store items (filter by type) |

### Purchase Operations

| Tool | Description |
|------|-------------|
| `purchase_inventory_items` | Purchase an item using virtual currency |
| `execute_inventory_operations` | Execute purchase as part of atomic operations |

### Currency Management

| Tool | Description |
|------|-------------|
| `add_inventory_items` | Add virtual currency to a player |
| `subtract_inventory_items` | Subtract virtual currency from a player |
| `get_inventory_items` | Check virtual currency balance |

### Subscription Management

| Tool | Description |
|------|-------------|
| `create_subscription` | Create a draft subscription item |

### Platform Redemption

| Tool | Description |
|------|-------------|
| `redeem_apple_appstore_inventory_items` | Redeem Apple App Store purchase |
| `redeem_google_play_inventory_items` | Redeem Google Play purchase |
| `redeem_microsoft_store_inventory_items` | Redeem Microsoft Store purchase |
| `redeem_nintendo_eshop_inventory_items` | Redeem Nintendo eShop purchase |
| `redeem_playstation_store_inventory_items` | Redeem PlayStation Store purchase |
| `redeem_steam_inventory_items` | Redeem Steam purchase |

## Examples

### Create a store

```bash
playfab-cli tool call create_store --json '{
  "item": {
    "Title": {"NEUTRAL": "Main Shop"},
    "Description": {"NEUTRAL": "The primary in-game store"},
    "AlternateIds": [{"Type": "FriendlyId", "Value": "main-shop"}]
  },
  "publish": true
}'
```

### Get a store by friendly ID

```bash
playfab-cli tool call get_store_by_friendly_id --json '{"friendlyId": "main-shop"}'
```

### Add virtual currency

```bash
playfab-cli tool call add_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "item": {"Id": "gold-coin"},
  "amount": 1000
}'
```

### Check currency balance

```bash
playfab-cli tool call get_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "filter": "type eq '\''currency'\''"
}'
```

### Purchase with virtual currency

```bash
playfab-cli tool call purchase_inventory_items --json '{
  "entity": {"Id": "ABCD1234", "Type": "title_player_account"},
  "item": {"Id": "premium-sword"},
  "amount": 1,
  "priceAmounts": [{"ItemId": "gold-coin", "Amount": 500}],
  "storeId": "main-shop"
}'
```

### Create a subscription

```bash
playfab-cli tool call create_subscription --json '{
  "item": {
    "Title": {"NEUTRAL": "VIP Pass"},
    "Description": {"NEUTRAL": "Monthly VIP subscription"}
  },
  "publish": false
}'
```

### Create a crafting recipe

```bash
playfab-cli tool call create_recipe --json '{
  "recipe": {
    "Title": {"NEUTRAL": "Forge Iron Sword"},
    "Inputs": [
      {"Id": "iron-ore", "Amount": 3},
      {"Id": "wood-plank", "Amount": 1}
    ],
    "Outputs": [
      {"Id": "iron-sword", "Amount": 1}
    ]
  },
  "publish": true
}'
```

## Common Workflows

### 1. Set up a store with items

```bash
# Create currency item
playfab-cli tool call create_draft_item --json '{"item": {"Type": "currency", "Title": {"NEUTRAL": "Gold Coin"}, "AlternateIds": [{"Type": "FriendlyId", "Value": "gold-coin"}]}, "publish": true}'
# Create store
playfab-cli tool call create_store --json '{"item": {"Title": {"NEUTRAL": "Shop"}}, "publish": true}'
# Create purchasable items with prices
playfab-cli tool call create_draft_item --json '{"item": {"Type": "catalogItem", "Title": {"NEUTRAL": "Sword"}, "PriceOptions": {"Prices": [{"Amounts": [{"ItemId": "gold-coin", "Amount": 100}]}]}}, "publish": true}'
```

### 2. Process real-money purchase

```bash
# Redeem platform receipt
playfab-cli tool call redeem_google_play_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}, "purchaseToken": "..."}'
# Verify inventory updated
playfab-cli tool call get_inventory_items --json '{"entity": {"Id": "...", "Type": "title_player_account"}}'
```

## Notes

- In Economy v2, virtual currencies are catalog items of type `currency`.
- Stores are catalog items that reference other items with price configurations.
- Platform redemption tools validate receipts from external stores (Apple, Google, Steam, etc.).
- Purchase operations automatically deduct the price from the buyer's inventory.
