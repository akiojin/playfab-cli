// Progression tools: Leaderboards, Statistics, and related operations.
use crate::tooling::schema_builder::*;

/// Helper: entity key schema (Id + Type)
fn entity_key_prop(description: &str) -> serde_json::Value {
    object_schema(
        description,
        vec![
            ("Id", string_prop("Entity ID")),
            (
                "Type",
                string_prop("Entity type (e.g. title_player_account)"),
            ),
        ],
        vec!["Id", "Type"],
    )
}

/// Helper: leaderboard column definition
fn column_def_schema() -> serde_json::Value {
    object_schema(
        "Leaderboard column definition",
        vec![
            ("name", string_prop("Column name")),
            (
                "sortDirection",
                optional(string_enum_prop(
                    "Sort direction for this column",
                    &["Ascending", "Descending"],
                )),
            ),
        ],
        vec!["name"],
    )
}

/// Helper: statistic update entry
fn statistic_update_schema() -> serde_json::Value {
    object_schema(
        "Statistic update entry",
        vec![
            ("name", string_prop("Statistic name")),
            ("value", integer_prop("Statistic value")),
            (
                "version",
                optional(integer_prop("Expected version for concurrency control")),
            ),
        ],
        vec!["name", "value"],
    )
}

/// Helper: statistic delete entry
fn statistic_delete_schema() -> serde_json::Value {
    object_schema(
        "Statistic to delete",
        vec![
            ("name", string_prop("Statistic name")),
            (
                "version",
                optional(integer_prop("Expected version for concurrency control")),
            ),
        ],
        vec!["name"],
    )
}

/// Helper: leaderboard entry update
fn leaderboard_entry_update_schema() -> serde_json::Value {
    object_schema(
        "Leaderboard entry to update",
        vec![
            (
                "entityId",
                string_prop("Entity ID for the leaderboard entry"),
            ),
            (
                "scores",
                string_array_prop("Array of score values as strings"),
            ),
            (
                "metadata",
                optional(string_prop("Optional metadata for the entry")),
            ),
        ],
        vec!["entityId", "scores"],
    )
}

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. create_leaderboard_definition
        super::ToolSpec {
            name: "create_leaderboard_definition".to_string(),
            description: "Create a new leaderboard definition".to_string(),
            api_group: "Progression".to_string(),
            api_method: "CreateLeaderboardDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateLeaderboardDefinition parameters",
                vec![
                    ("name", string_prop("The name of the leaderboard")),
                    (
                        "columns",
                        object_array_prop(
                            "Array of column definitions for the leaderboard",
                            column_def_schema(),
                        ),
                    ),
                    (
                        "sizeLimit",
                        optional(integer_prop("Maximum number of entries in the leaderboard")),
                    ),
                    (
                        "entityType",
                        optional(string_prop("Entity type for the leaderboard entries")),
                    ),
                    (
                        "versionConfiguration",
                        optional(any_object_prop("Version configuration for the leaderboard")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name", "columns"],
            ),
        },
        // 2. delete_leaderboard_definition
        super::ToolSpec {
            name: "delete_leaderboard_definition".to_string(),
            description: "Delete a leaderboard definition".to_string(),
            api_group: "Progression".to_string(),
            api_method: "DeleteLeaderboardDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteLeaderboardDefinition parameters",
                vec![
                    ("name", string_prop("The name of the leaderboard to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 3. get_leaderboard_definition
        super::ToolSpec {
            name: "get_leaderboard_definition".to_string(),
            description: "Get the definition of a leaderboard".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetLeaderboardDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetLeaderboardDefinition parameters",
                vec![
                    ("name", string_prop("The name of the leaderboard")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 4. update_statistics
        super::ToolSpec {
            name: "update_statistics".to_string(),
            description: "Update statistics for an entity".to_string(),
            api_group: "Progression".to_string(),
            api_method: "UpdateStatistics".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateStatistics parameters",
                vec![
                    ("entity", entity_key_prop("The entity to update statistics for")),
                    (
                        "statistics",
                        object_array_prop(
                            "Array of statistic updates to apply",
                            statistic_update_schema(),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "statistics"],
            ),
        },
        // 5. get_statistics
        super::ToolSpec {
            name: "get_statistics".to_string(),
            description: "Get statistics for an entity".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetStatistics".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetStatistics parameters",
                vec![
                    ("entity", entity_key_prop("The entity to get statistics for")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
        // 6. get_statistics_for_entities
        super::ToolSpec {
            name: "get_statistics_for_entities".to_string(),
            description: "Get statistics for multiple entities".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetStatisticsForEntities".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetStatisticsForEntities parameters",
                vec![
                    (
                        "entities",
                        object_array_prop(
                            "Array of entity keys to get statistics for",
                            entity_key_prop("Entity to query"),
                        ),
                    ),
                    (
                        "statisticNames",
                        string_array_prop("Array of statistic names to retrieve"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entities", "statisticNames"],
            ),
        },
        // 7. delete_statistics
        super::ToolSpec {
            name: "delete_statistics".to_string(),
            description: "Delete statistics for an entity".to_string(),
            api_group: "Progression".to_string(),
            api_method: "DeleteStatistics".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteStatistics parameters",
                vec![
                    ("entity", entity_key_prop("The entity to delete statistics for")),
                    (
                        "statistics",
                        object_array_prop(
                            "Array of statistics to delete",
                            statistic_delete_schema(),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "statistics"],
            ),
        },
        // 8. create_statistic_definition
        super::ToolSpec {
            name: "create_statistic_definition".to_string(),
            description: "Create a new statistic definition".to_string(),
            api_group: "Progression".to_string(),
            api_method: "CreateStatisticDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateStatisticDefinition parameters",
                vec![
                    ("name", string_prop("The name of the statistic")),
                    (
                        "aggregationMethod",
                        optional(string_enum_prop(
                            "How values are aggregated",
                            &["Last", "Max", "Min", "Sum"],
                        )),
                    ),
                    (
                        "entityType",
                        optional(string_prop("Entity type for this statistic")),
                    ),
                    (
                        "versionConfiguration",
                        optional(any_object_prop("Version configuration for the statistic")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 9. delete_statistic_definition
        super::ToolSpec {
            name: "delete_statistic_definition".to_string(),
            description: "Delete a statistic definition".to_string(),
            api_group: "Progression".to_string(),
            api_method: "DeleteStatisticDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteStatisticDefinition parameters",
                vec![
                    ("name", string_prop("The name of the statistic to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 10. get_statistic_definition
        super::ToolSpec {
            name: "get_statistic_definition".to_string(),
            description: "Get the definition of a statistic".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetStatisticDefinition".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetStatisticDefinition parameters",
                vec![
                    ("name", string_prop("The name of the statistic")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 11. get_leaderboard
        super::ToolSpec {
            name: "get_leaderboard".to_string(),
            description: "Get a leaderboard with rankings".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetLeaderboard".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetLeaderboard parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "startingPosition",
                        optional(integer_range_prop("Starting position in the leaderboard", Some(0), None)),
                    ),
                    (
                        "pageSize",
                        optional(integer_range_prop("Number of entries to return", Some(1), Some(100))),
                    ),
                    (
                        "version",
                        optional(integer_prop("Leaderboard version to query")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName"],
            ),
        },
        // 12. get_leaderboard_for_entities
        super::ToolSpec {
            name: "get_leaderboard_for_entities".to_string(),
            description: "Get leaderboard entries for specific entities".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetLeaderboardForEntities".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetLeaderboardForEntities parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entities",
                        string_array_prop("Array of entity IDs to get leaderboard entries for"),
                    ),
                    (
                        "version",
                        optional(integer_prop("Leaderboard version to query")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName", "entities"],
            ),
        },
        // 13. get_leaderboard_around_entity
        super::ToolSpec {
            name: "get_leaderboard_around_entity".to_string(),
            description: "Get leaderboard entries around a specific entity".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetLeaderboardAroundEntity".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetLeaderboardAroundEntity parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entity",
                        optional(entity_key_prop("The entity to center the leaderboard around")),
                    ),
                    (
                        "maxSurroundingEntries",
                        optional(integer_range_prop("Number of entries around the entity to return", Some(1), Some(100))),
                    ),
                    (
                        "version",
                        optional(integer_prop("Leaderboard version to query")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName"],
            ),
        },
        // 14. increment_leaderboard_version
        super::ToolSpec {
            name: "increment_leaderboard_version".to_string(),
            description: "Increment the version of a leaderboard, resetting all entries".to_string(),
            api_group: "Progression".to_string(),
            api_method: "IncrementLeaderboardVersion".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "IncrementLeaderboardVersion parameters",
                vec![
                    ("name", string_prop("The name of the leaderboard")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 15. list_statistic_definitions
        super::ToolSpec {
            name: "list_statistic_definitions".to_string(),
            description: "List all statistic definitions for the title".to_string(),
            api_group: "Progression".to_string(),
            api_method: "ListStatisticDefinitions".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListStatisticDefinitions parameters",
                vec![
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 16. list_leaderboard_definitions
        super::ToolSpec {
            name: "list_leaderboard_definitions".to_string(),
            description: "List all leaderboard definitions for the title".to_string(),
            api_group: "Progression".to_string(),
            api_method: "ListLeaderboardDefinitions".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListLeaderboardDefinitions parameters",
                vec![
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 17. update_leaderboard_entries
        super::ToolSpec {
            name: "update_leaderboard_entries".to_string(),
            description: "Update or insert entries in a leaderboard".to_string(),
            api_group: "Progression".to_string(),
            api_method: "UpdateLeaderboardEntries".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateLeaderboardEntries parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entries",
                        object_array_prop(
                            "Array of leaderboard entries to update",
                            leaderboard_entry_update_schema(),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName", "entries"],
            ),
        },
        // 18. delete_leaderboard_entries
        super::ToolSpec {
            name: "delete_leaderboard_entries".to_string(),
            description: "Delete entries from a leaderboard".to_string(),
            api_group: "Progression".to_string(),
            api_method: "DeleteLeaderboardEntries".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteLeaderboardEntries parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entityIds",
                        string_array_prop("Array of entity IDs to remove from the leaderboard"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName", "entityIds"],
            ),
        },
        // 19. unlink_leaderboard_from_statistic
        super::ToolSpec {
            name: "unlink_leaderboard_from_statistic".to_string(),
            description: "Unlink a leaderboard from a statistic".to_string(),
            api_group: "Progression".to_string(),
            api_method: "UnlinkLeaderboardFromStatistic".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnlinkLeaderboardFromStatistic parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "statisticName",
                        string_prop("The name of the statistic to unlink"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName", "statisticName"],
            ),
        },
        // 20. link_leaderboard_to_statistic
        super::ToolSpec {
            name: "link_leaderboard_to_statistic".to_string(),
            description: "Link a leaderboard to a statistic for automatic updates".to_string(),
            api_group: "Progression".to_string(),
            api_method: "LinkLeaderboardToStatistic".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LinkLeaderboardToStatistic parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "statisticName",
                        string_prop("The name of the statistic to link"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName", "statisticName"],
            ),
        },
        // 21. get_friend_leaderboard_for_entity
        super::ToolSpec {
            name: "get_friend_leaderboard_for_entity".to_string(),
            description: "Get a leaderboard filtered to an entity's friends".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetFriendLeaderboardForEntity".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetFriendLeaderboardForEntity parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entity",
                        optional(entity_key_prop("The entity whose friends leaderboard to retrieve")),
                    ),
                    (
                        "version",
                        optional(integer_prop("Leaderboard version to query")),
                    ),
                    (
                        "externalFriendSources",
                        optional(string_prop("External friend sources to include")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName"],
            ),
        },
        // 22. list_leaderboard_around_entity (alias for get_leaderboard_around_entity)
        super::ToolSpec {
            name: "list_leaderboard_around_entity".to_string(),
            description: "List leaderboard entries around a specific entity (alias for get_leaderboard_around_entity)".to_string(),
            api_group: "Progression".to_string(),
            api_method: "ListLeaderboardAroundEntity".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListLeaderboardAroundEntity parameters",
                vec![
                    (
                        "leaderboardName",
                        string_prop("The name of the leaderboard"),
                    ),
                    (
                        "entity",
                        optional(entity_key_prop("The entity to center the leaderboard around")),
                    ),
                    (
                        "maxSurroundingEntries",
                        optional(integer_range_prop("Number of entries around the entity to return", Some(1), Some(100))),
                    ),
                    (
                        "version",
                        optional(integer_prop("Leaderboard version to query")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["leaderboardName"],
            ),
        },
        // 23. get_entity_statistics (alias for get_statistics)
        super::ToolSpec {
            name: "get_entity_statistics".to_string(),
            description: "Get statistics for an entity (alias for get_statistics)".to_string(),
            api_group: "Progression".to_string(),
            api_method: "GetEntityStatistics".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetEntityStatistics parameters",
                vec![
                    ("entity", entity_key_prop("The entity to get statistics for")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
        // 24. update_entity_statistics (alias for update_statistics)
        super::ToolSpec {
            name: "update_entity_statistics".to_string(),
            description: "Update statistics for an entity (alias for update_statistics)".to_string(),
            api_group: "Progression".to_string(),
            api_method: "UpdateEntityStatistics".to_string(),
            category: super::ToolCategory::Progression,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateEntityStatistics parameters",
                vec![
                    ("entity", entity_key_prop("The entity to update statistics for")),
                    (
                        "statistics",
                        object_array_prop(
                            "Array of statistic updates to apply",
                            statistic_update_schema(),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "statistics"],
            ),
        },
    ]
}
