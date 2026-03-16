// Economy v2 Catalog tools: SearchItems, GetItem, CreateDraftItem, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. SearchItems
        super::ToolSpec {
            name: "search_items".to_string(),
            description: "Search for catalog items using optional filter, search, and orderBy"
                .to_string(),
            api_group: "Catalog".to_string(),
            api_method: "SearchItems".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SearchItems parameters",
                vec![
                    ("search", optional(string_prop("Search text to filter results"))),
                    ("filter", optional(string_prop("OData filter expression"))),
                    ("orderBy", optional(string_prop("OData orderBy expression"))),
                    (
                        "count",
                        optional(integer_range_prop("Number of results", Some(1), Some(50))),
                    ),
                    (
                        "continuationToken",
                        optional(string_prop("Token for pagination")),
                    ),
                    (
                        "select",
                        optional(string_prop("OData select expression to limit returned fields")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 2. GetItem
        super::ToolSpec {
            name: "get_item".to_string(),
            description: "Retrieve a specific catalog item by its ID".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItem parameters",
                vec![
                    ("id", string_prop("The unique ID of the catalog item")),
                    (
                        "alternateId",
                        optional(any_object_prop(
                            "Alternate ID (Type and Value) to identify the item",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity to perform this action on")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 3. CreateDraftItem
        super::ToolSpec {
            name: "create_draft_item".to_string(),
            description: "Create a new draft catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateDraftItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateDraftItem parameters",
                vec![
                    (
                        "item",
                        any_object_prop("The catalog item to create as draft"),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish the item immediately"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["item"],
            ),
        },
        // 4. UpdateDraftItem
        super::ToolSpec {
            name: "update_draft_item".to_string(),
            description: "Update an existing draft catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "UpdateDraftItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateDraftItem parameters",
                vec![
                    (
                        "item",
                        any_object_prop("The updated catalog item definition"),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish the item after update"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["item"],
            ),
        },
        // 5. PublishDraftItem
        super::ToolSpec {
            name: "publish_draft_item".to_string(),
            description: "Publish a draft catalog item to make it live".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "PublishDraftItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PublishDraftItem parameters",
                vec![
                    ("id", string_prop("The unique ID of the draft item to publish")),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "eTag",
                        optional(string_prop("ETag for optimistic concurrency")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 6. DeleteItem
        super::ToolSpec {
            name: "delete_item".to_string(),
            description: "Delete a catalog item by its ID".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "DeleteItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteItem parameters",
                vec![
                    ("id", string_prop("The unique ID of the item to delete")),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 7. GetCatalogConfig
        super::ToolSpec {
            name: "get_catalog_config".to_string(),
            description: "Retrieve the current catalog configuration for the title".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetCatalogConfig".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCatalogConfig parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 8. UpdateCatalogConfig
        super::ToolSpec {
            name: "update_catalog_config".to_string(),
            description: "Update the catalog configuration for the title".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "UpdateCatalogConfig".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateCatalogConfig parameters",
                vec![
                    (
                        "config",
                        any_object_prop("The catalog configuration to apply"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["config"],
            ),
        },
        // 9. CreateDraftItems (batch)
        super::ToolSpec {
            name: "batch_create_draft_items".to_string(),
            description: "Create multiple draft catalog items in a single batch request"
                .to_string(),
            api_group: "Catalog".to_string(),
            api_method: "BatchCreateDraftItems".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateDraftItems parameters",
                vec![
                    (
                        "items",
                        object_array_prop(
                            "Array of catalog items to create as drafts",
                            any_object_prop("A catalog item definition"),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["items"],
            ),
        },
        // 10. GetEntityDraftItems
        super::ToolSpec {
            name: "get_entity_draft_items".to_string(),
            description: "Retrieve all draft items owned by a specific entity".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetEntityDraftItems".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetEntityDraftItems parameters",
                vec![
                    (
                        "entity",
                        any_object_prop("The entity whose draft items to retrieve (Id and Type)"),
                    ),
                    (
                        "count",
                        optional(integer_range_prop("Number of results", Some(1), Some(50))),
                    ),
                    (
                        "continuationToken",
                        optional(string_prop("Token for pagination")),
                    ),
                    (
                        "filter",
                        optional(string_prop("OData filter expression")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
        // 11. CreateUploadUrls
        super::ToolSpec {
            name: "create_upload_urls".to_string(),
            description: "Create URLs for uploading content files to the catalog".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateUploadUrls".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateUploadUrls parameters",
                vec![
                    (
                        "files",
                        object_array_prop(
                            "Array of file descriptions to create upload URLs for",
                            object_schema(
                                "Upload file info",
                                vec![("fileName", string_prop("Name of the file to upload"))],
                                vec!["fileName"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["files"],
            ),
        },
        // 12. GetItemModerationState
        super::ToolSpec {
            name: "get_item_moderation_state".to_string(),
            description: "Get the moderation state of a catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItemModerationState".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItemModerationState parameters",
                vec![
                    ("id", string_prop("The unique ID of the item")),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 13. SetItemModerationState
        super::ToolSpec {
            name: "set_item_moderation_state".to_string(),
            description: "Set the moderation state of a catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "SetItemModerationState".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetItemModerationState parameters",
                vec![
                    ("id", string_prop("The unique ID of the item")),
                    ("reason", string_prop("The reason for the moderation state change")),
                    (
                        "status",
                        string_enum_prop(
                            "The moderation status to set",
                            &["Approved", "AwaitingModeration", "Rejected"],
                        ),
                    ),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id", "reason", "status"],
            ),
        },
        // 14. GetDraftItems
        super::ToolSpec {
            name: "get_draft_items".to_string(),
            description: "Retrieve multiple draft catalog items by their IDs".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetDraftItems".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetDraftItems parameters",
                vec![
                    (
                        "ids",
                        string_array_prop("Array of item IDs to retrieve"),
                    ),
                    (
                        "alternateIds",
                        optional(object_array_prop(
                            "Array of alternate IDs to identify items",
                            any_object_prop("An alternate ID with Type and Value"),
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ids"],
            ),
        },
        // 15. GetItems
        super::ToolSpec {
            name: "get_items".to_string(),
            description: "Retrieve multiple published catalog items by their IDs".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItems".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItems parameters",
                vec![
                    (
                        "ids",
                        string_array_prop("Array of item IDs to retrieve"),
                    ),
                    (
                        "alternateIds",
                        optional(object_array_prop(
                            "Array of alternate IDs to identify items",
                            any_object_prop("An alternate ID with Type and Value"),
                        )),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity to perform this action on")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ids"],
            ),
        },
        // 16. GetItemPublishStatus
        super::ToolSpec {
            name: "get_item_publish_status".to_string(),
            description: "Get the publish status of a catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItemPublishStatus".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItemPublishStatus parameters",
                vec![
                    ("id", string_prop("The unique ID of the item")),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 17. GetItemReviews
        super::ToolSpec {
            name: "get_item_reviews".to_string(),
            description: "Get reviews for a specific catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItemReviews".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItemReviews parameters",
                vec![
                    ("id", string_prop("The unique ID of the item")),
                    (
                        "count",
                        optional(integer_range_prop("Number of reviews to return", Some(1), Some(200))),
                    ),
                    (
                        "continuationToken",
                        optional(string_prop("Token for pagination")),
                    ),
                    (
                        "orderBy",
                        optional(string_prop("OData orderBy expression")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 18. SubmitItemReviewVote
        super::ToolSpec {
            name: "submit_item_review_vote".to_string(),
            description: "Submit a vote on an item review (helpful or unhelpful)".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "SubmitItemReviewVote".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SubmitItemReviewVote parameters",
                vec![
                    ("reviewId", string_prop("The ID of the review to vote on")),
                    (
                        "vote",
                        string_enum_prop("The vote value", &["Helpful", "Unhelpful", "None"]),
                    ),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity to perform this action on")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["reviewId", "vote"],
            ),
        },
        // 19. TakedownItemReviews
        super::ToolSpec {
            name: "takedown_item_reviews".to_string(),
            description: "Remove reviews from a catalog item (moderation action)".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "TakedownItemReviews".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "TakedownItemReviews parameters",
                vec![
                    (
                        "reviews",
                        object_array_prop(
                            "Array of reviews to take down",
                            object_schema(
                                "Review takedown request",
                                vec![
                                    ("itemId", string_prop("The item ID the review belongs to")),
                                    ("reviewId", string_prop("The review ID to take down")),
                                ],
                                vec!["itemId", "reviewId"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["reviews"],
            ),
        },
        // 20. ReportItem
        super::ToolSpec {
            name: "report_item".to_string(),
            description: "Report a catalog item for policy violations".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "ReportItem".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ReportItem parameters",
                vec![
                    ("id", string_prop("The unique ID of the item to report")),
                    (
                        "reason",
                        string_enum_prop(
                            "The reason for reporting",
                            &[
                                "ChildExploitation",
                                "Discrimination",
                                "Exploits",
                                "Fraud",
                                "OffensiveContent",
                                "Other",
                                "Profanity",
                                "Spam",
                            ],
                        ),
                    ),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "concern",
                        optional(string_prop("Additional details about the concern")),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity submitting the report")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id", "reason"],
            ),
        },
        // 21. GetItemContainers
        super::ToolSpec {
            name: "get_item_containers".to_string(),
            description: "Get containers that reference a specific catalog item".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetItemContainers".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetItemContainers parameters",
                vec![
                    ("id", string_prop("The unique ID of the item")),
                    (
                        "alternateId",
                        optional(any_object_prop("Alternate ID to identify the item")),
                    ),
                    (
                        "count",
                        optional(integer_range_prop("Number of results", Some(1), Some(50))),
                    ),
                    (
                        "continuationToken",
                        optional(string_prop("Token for pagination")),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity to perform this action on")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["id"],
            ),
        },
        // 22. CreateBundle (CreateDraftItem with type=bundle)
        super::ToolSpec {
            name: "create_bundle".to_string(),
            description: "Create a draft bundle item (a collection of items sold together)"
                .to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateBundle".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateBundle parameters",
                vec![
                    (
                        "item",
                        any_object_prop(
                            "The bundle item definition. Type will be set to 'bundle' automatically",
                        ),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish immediately"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["item"],
            ),
        },
        // 23. CreateStore (CreateDraftItem with type=store)
        super::ToolSpec {
            name: "create_store".to_string(),
            description: "Create a draft store item (a storefront for selling items)".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateStore".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateStore parameters",
                vec![
                    (
                        "item",
                        any_object_prop(
                            "The store item definition. Type will be set to 'store' automatically",
                        ),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish immediately"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["item"],
            ),
        },
        // 24. CreateSubscription (CreateDraftItem with type=subscription)
        super::ToolSpec {
            name: "create_subscription".to_string(),
            description: "Create a draft subscription item (recurring purchase)".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateSubscription".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateSubscription parameters",
                vec![
                    (
                        "item",
                        any_object_prop(
                            "The subscription item definition. Type will be set to 'subscription' automatically",
                        ),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish immediately"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["item"],
            ),
        },
        // 25. GetStoreByFriendlyId (GetItem with AlternateId type=FriendlyId)
        super::ToolSpec {
            name: "get_store_by_friendly_id".to_string(),
            description: "Retrieve a store item by its friendly ID".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "GetStoreByFriendlyId".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetStoreByFriendlyId parameters",
                vec![
                    (
                        "friendlyId",
                        string_prop("The friendly ID of the store item"),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("Entity to perform this action on")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["friendlyId"],
            ),
        },
        // 26. CreateRecipe
        super::ToolSpec {
            name: "create_recipe".to_string(),
            description: "Create a crafting recipe that defines item conversion rules".to_string(),
            api_group: "Catalog".to_string(),
            api_method: "CreateRecipe".to_string(),
            category: super::ToolCategory::EconomyCatalog,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateRecipe parameters",
                vec![
                    (
                        "recipe",
                        any_object_prop("The recipe definition with inputs and outputs"),
                    ),
                    ("publish", optional(boolean_prop("Whether to publish immediately"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["recipe"],
            ),
        },
    ]
}
