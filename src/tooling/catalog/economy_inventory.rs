// Economy v2 Inventory tools: AddInventoryItems, GetInventoryItems, etc.
use crate::tooling::schema_builder::*;

/// Helper: entity key schema (Id + Type required)
fn entity_key_schema(description: &str) -> serde_json::Value {
    object_schema(
        description,
        vec![
            ("Id", string_prop("Entity ID")),
            ("Type", string_prop("Entity type (e.g. title_player_account)")),
        ],
        vec!["Id", "Type"],
    )
}

/// Helper: inventory item reference schema
fn item_ref_schema(description: &str) -> serde_json::Value {
    object_schema(
        description,
        vec![
            ("Id", string_prop("Catalog item ID")),
            ("StackId", optional(string_prop("Stack ID"))),
            ("AlternateId", optional(object_schema(
                "Alternate ID for the item",
                vec![
                    ("Type", string_prop("Type of alternate ID (e.g. FriendlyId, MarketplaceId)")),
                    ("Value", string_prop("Alternate ID value")),
                ],
                vec!["Type", "Value"],
            ))),
        ],
        vec!["Id"],
    )
}

/// Helper: price amount schema for purchases
fn price_amount_schema() -> serde_json::Value {
    object_schema(
        "Price amount",
        vec![
            ("ItemId", string_prop("ID of the currency or item used as payment")),
            ("Amount", integer_range_prop("Amount to pay", Some(1), None)),
            ("StackId", optional(string_prop("Stack ID of the currency item"))),
        ],
        vec!["ItemId", "Amount"],
    )
}

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. add_inventory_items
        super::ToolSpec {
            name: "add_inventory_items".to_string(),
            description: "Add items to a player's inventory (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "AddInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to add items to")),
                    ("item", item_ref_schema("Item to add")),
                    ("amount", integer_range_prop("Amount to add", Some(1), None)),
                    ("collectionId", optional(string_prop("Collection ID (default collection if omitted)"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("durationInSeconds", optional(integer_prop("Duration in seconds for expiring items"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                    ("newStackValues", optional(any_object_prop("Initial values for a new stack if created"))),
                ],
                vec!["entity", "item", "amount"],
            ),
        },
        // 2. subtract_inventory_items
        super::ToolSpec {
            name: "subtract_inventory_items".to_string(),
            description: "Subtract items from a player's inventory (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "SubtractInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SubtractInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to subtract items from")),
                    ("item", item_ref_schema("Item to subtract")),
                    ("amount", integer_range_prop("Amount to subtract", Some(1), None)),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("deleteEmptyStacks", optional(boolean_prop("Delete stack when quantity reaches zero"))),
                    ("durationInSeconds", optional(integer_prop("Duration in seconds"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["entity", "item", "amount"],
            ),
        },
        // 3. delete_inventory_items
        super::ToolSpec {
            name: "delete_inventory_items".to_string(),
            description: "Delete items from a player's inventory (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "DeleteInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to delete items from")),
                    ("item", item_ref_schema("Item to delete")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["entity", "item"],
            ),
        },
        // 4. update_inventory_items
        super::ToolSpec {
            name: "update_inventory_items".to_string(),
            description: "Update inventory item properties (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "UpdateInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity whose item to update")),
                    ("item", object_schema(
                        "Item with updated values",
                        vec![
                            ("Id", string_prop("Catalog item ID")),
                            ("StackId", optional(string_prop("Stack ID"))),
                            ("Amount", optional(integer_prop("New amount"))),
                            ("DisplayProperties", optional(any_object_prop("Display properties to update"))),
                        ],
                        vec!["Id"],
                    )),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["entity", "item"],
            ),
        },
        // 5. get_inventory_items
        super::ToolSpec {
            name: "get_inventory_items".to_string(),
            description: "Get a player's inventory items (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GetInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity whose inventory to retrieve")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("count", optional(integer_range_prop("Number of items to return", Some(1), Some(50)))),
                    ("continuationToken", optional(string_prop("Continuation token for pagination"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("filter", optional(string_prop("OData filter string"))),
                ],
                vec!["entity"],
            ),
        },
        // 6. get_inventory_collection_ids
        super::ToolSpec {
            name: "get_inventory_collection_ids".to_string(),
            description: "Get collection IDs for a player's inventory (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GetInventoryCollectionIds".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetInventoryCollectionIds parameters",
                vec![
                    ("entity", entity_key_schema("Entity whose collection IDs to retrieve")),
                    ("count", optional(integer_range_prop("Number of IDs to return", Some(1), Some(50)))),
                    ("continuationToken", optional(string_prop("Continuation token for pagination"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity"],
            ),
        },
        // 7. execute_inventory_operations
        super::ToolSpec {
            name: "execute_inventory_operations".to_string(),
            description: "Execute multiple inventory operations atomically (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "ExecuteInventoryOperations".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteInventoryOperations parameters",
                vec![
                    ("entity", entity_key_schema("Entity to perform operations on")),
                    ("operations", object_array_prop(
                        "List of inventory operations to execute",
                        object_schema(
                            "An inventory operation",
                            vec![
                                ("Add", optional(object_schema(
                                    "Add operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to add")),
                                        ("Amount", integer_range_prop("Amount", Some(1), None)),
                                        ("DurationInSeconds", optional(integer_prop("Duration in seconds"))),
                                        ("NewStackValues", optional(any_object_prop("New stack values"))),
                                    ],
                                    vec!["Item", "Amount"],
                                ))),
                                ("Subtract", optional(object_schema(
                                    "Subtract operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to subtract")),
                                        ("Amount", integer_range_prop("Amount", Some(1), None)),
                                        ("DeleteEmptyStacks", optional(boolean_prop("Delete empty stacks"))),
                                    ],
                                    vec!["Item", "Amount"],
                                ))),
                                ("Delete", optional(object_schema(
                                    "Delete operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to delete")),
                                    ],
                                    vec!["Item"],
                                ))),
                                ("Update", optional(object_schema(
                                    "Update operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to update")),
                                        ("Amount", optional(integer_prop("New amount"))),
                                    ],
                                    vec!["Item"],
                                ))),
                                ("Purchase", optional(object_schema(
                                    "Purchase operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to purchase")),
                                        ("Amount", integer_range_prop("Amount", Some(1), None)),
                                        ("PriceAmounts", object_array_prop("Prices", price_amount_schema())),
                                    ],
                                    vec!["Item", "Amount", "PriceAmounts"],
                                ))),
                                ("Transfer", optional(object_schema(
                                    "Transfer operation",
                                    vec![
                                        ("Item", item_ref_schema("Item to transfer")),
                                        ("Amount", integer_range_prop("Amount", Some(1), None)),
                                        ("GivingEntity", entity_key_schema("Source entity")),
                                        ("ReceivingEntity", entity_key_schema("Destination entity")),
                                    ],
                                    vec!["Item", "Amount", "GivingEntity", "ReceivingEntity"],
                                ))),
                            ],
                            vec![],
                        ),
                    )),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["entity", "operations"],
            ),
        },
        // 8. purchase_inventory_items
        super::ToolSpec {
            name: "purchase_inventory_items".to_string(),
            description: "Purchase an item from the catalog (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "PurchaseInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PurchaseInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity making the purchase")),
                    ("item", item_ref_schema("Item to purchase")),
                    ("amount", integer_range_prop("Amount to purchase", Some(1), None)),
                    ("priceAmounts", object_array_prop("Price amounts to pay", price_amount_schema())),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("deleteEmptyStacks", optional(boolean_prop("Delete empty stacks after purchase"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                    ("newStackValues", optional(any_object_prop("Initial values for new stack"))),
                    ("storeId", optional(string_prop("Store ID for store-specific pricing"))),
                ],
                vec!["entity", "item", "amount", "priceAmounts"],
            ),
        },
        // 9. redeem_apple_appstore_inventory_items
        super::ToolSpec {
            name: "redeem_apple_appstore_inventory_items".to_string(),
            description: "Redeem items from Apple App Store receipt (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemAppleAppstoreInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemAppleAppStoreInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("receipt", string_prop("Apple App Store receipt data")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity", "receipt"],
            ),
        },
        // 10. redeem_google_play_inventory_items
        super::ToolSpec {
            name: "redeem_google_play_inventory_items".to_string(),
            description: "Redeem items from Google Play purchase (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemGooglePlayInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemGooglePlayInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("purchaseToken", string_prop("Google Play purchase token")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity", "purchaseToken"],
            ),
        },
        // 11. redeem_microsoft_store_inventory_items
        super::ToolSpec {
            name: "redeem_microsoft_store_inventory_items".to_string(),
            description: "Redeem items from Microsoft Store purchase (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemMicrosoftStoreInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemMicrosoftStoreInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("xboxToken", string_prop("Xbox token for authentication")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity", "xboxToken"],
            ),
        },
        // 12. redeem_nintendo_eshop_inventory_items
        super::ToolSpec {
            name: "redeem_nintendo_eshop_inventory_items".to_string(),
            description: "Redeem items from Nintendo eShop purchase (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemNintendoEshopInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemNintendoEShopInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("nintendoServiceAccountIdToken", string_prop("Nintendo service account ID token")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity", "nintendoServiceAccountIdToken"],
            ),
        },
        // 13. redeem_playstation_store_inventory_items
        super::ToolSpec {
            name: "redeem_playstation_store_inventory_items".to_string(),
            description: "Redeem items from PlayStation Store purchase (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemPlaystationStoreInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemPlayStationStoreInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("authorizationCode", string_prop("PlayStation authorization code")),
                    ("serviceLabel", optional(string_prop("PlayStation service label"))),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity", "authorizationCode"],
            ),
        },
        // 14. redeem_steam_inventory_items
        super::ToolSpec {
            name: "redeem_steam_inventory_items".to_string(),
            description: "Redeem items from Steam purchase (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "RedeemSteamInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemSteamInventoryItems parameters",
                vec![
                    ("entity", entity_key_schema("Entity to redeem items for")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity"],
            ),
        },
        // 15. transfer_inventory_items
        super::ToolSpec {
            name: "transfer_inventory_items".to_string(),
            description: "Transfer inventory items between entities (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "TransferInventoryItems".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "TransferInventoryItems parameters",
                vec![
                    ("givingEntity", entity_key_schema("Entity giving the items")),
                    ("receivingEntity", entity_key_schema("Entity receiving the items")),
                    ("item", item_ref_schema("Item to transfer")),
                    ("amount", integer_range_prop("Amount to transfer", Some(1), None)),
                    ("givingCollectionId", optional(string_prop("Source collection ID"))),
                    ("receivingCollectionId", optional(string_prop("Destination collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("deleteEmptyStacks", optional(boolean_prop("Delete empty stacks after transfer"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                    ("newStackValues", optional(any_object_prop("Initial values for new stack"))),
                ],
                vec!["givingEntity", "receivingEntity", "item", "amount"],
            ),
        },
        // 16. get_transaction_history
        super::ToolSpec {
            name: "get_transaction_history".to_string(),
            description: "Get transaction history for an entity (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GetTransactionHistory".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTransactionHistory parameters",
                vec![
                    ("entity", entity_key_schema("Entity whose transaction history to retrieve")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("count", optional(integer_range_prop("Number of transactions to return", Some(1), Some(200)))),
                    ("continuationToken", optional(string_prop("Continuation token for pagination"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("filter", optional(string_prop("OData filter string"))),
                    ("orderBy", optional(string_prop("Order by clause (e.g. 'timestamp desc')"))),
                ],
                vec!["entity"],
            ),
        },
        // 17. get_inventory_operation_status
        super::ToolSpec {
            name: "get_inventory_operation_status".to_string(),
            description: "Get the status of an inventory operation (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GetInventoryOperationStatus".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetInventoryOperationStatus parameters",
                vec![
                    ("entity", entity_key_schema("Entity that owns the operation")),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity"],
            ),
        },
        // 18. execute_transfer_operations
        super::ToolSpec {
            name: "execute_transfer_operations".to_string(),
            description: "Execute multiple transfer operations atomically (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "ExecuteTransferOperations".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteTransferOperations parameters",
                vec![
                    ("givingEntity", entity_key_schema("Entity giving items")),
                    ("receivingEntity", entity_key_schema("Entity receiving items")),
                    ("operations", object_array_prop(
                        "List of transfer operations",
                        object_schema(
                            "A transfer operation",
                            vec![
                                ("Item", item_ref_schema("Item to transfer")),
                                ("Amount", integer_range_prop("Amount to transfer", Some(1), None)),
                                ("DeleteEmptyStacks", optional(boolean_prop("Delete empty stacks"))),
                                ("NewStackValues", optional(any_object_prop("New stack values"))),
                            ],
                            vec!["Item", "Amount"],
                        ),
                    )),
                    ("givingCollectionId", optional(string_prop("Source collection ID"))),
                    ("receivingCollectionId", optional(string_prop("Destination collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["givingEntity", "receivingEntity", "operations"],
            ),
        },
        // 19. grant_items_to_users
        super::ToolSpec {
            name: "grant_items_to_users".to_string(),
            description: "Grant items to multiple users in a batch (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GrantItemsToUsers".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GrantItemsToUsers parameters",
                vec![
                    ("items", object_array_prop(
                        "Items to grant to users",
                        object_schema(
                            "Grant item entry",
                            vec![
                                ("PlayFabId", string_prop("PlayFab ID of the user to grant items to")),
                                ("items", object_array_prop(
                                    "Items to grant to this user",
                                    object_schema(
                                        "Item grant entry",
                                        vec![
                                            ("Id", string_prop("Catalog item ID")),
                                            ("Amount", optional(integer_range_prop("Amount to grant", Some(1), None))),
                                            ("StackId", optional(string_prop("Stack ID"))),
                                        ],
                                        vec!["Id"],
                                    ),
                                )),
                            ],
                            vec!["PlayFabId", "items"],
                        ),
                    )),
                    ("collectionId", optional(string_prop("Collection ID"))),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                    ("idempotencyId", optional(string_prop("Idempotency ID for the request"))),
                ],
                vec!["items"],
            ),
        },
        // 20. get_microsoft_store_access_tokens
        super::ToolSpec {
            name: "get_microsoft_store_access_tokens".to_string(),
            description: "Get Microsoft Store access tokens for inventory operations (Economy v2)".to_string(),
            api_group: "Inventory".to_string(),
            api_method: "GetMicrosoftStoreAccessTokens".to_string(),
            category: super::ToolCategory::EconomyInventory,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMicrosoftStoreAccessTokens parameters",
                vec![
                    ("entity", entity_key_schema("Entity to get tokens for")),
                    ("customTags", optional(any_object_prop("Custom tags for the request"))),
                ],
                vec!["entity"],
            ),
        },
    ]
}
