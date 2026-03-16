// Profiles tools: GetProfile, SetProfilePolicy, SetDisplayName, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. GetProfile
        super::ToolSpec {
            name: "get_profile".to_string(),
            description: "Get the profile of a specific entity".to_string(),
            parameters: object_schema(
                "GetProfile parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to get the profile for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "dataAsObject",
                        optional(boolean_prop("Whether to return profile data as a JSON object")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
        // 2. GetProfiles
        super::ToolSpec {
            name: "get_profiles".to_string(),
            description: "Get profiles for multiple entities in a single request".to_string(),
            parameters: object_schema(
                "GetProfiles parameters",
                vec![
                    (
                        "entities",
                        object_array_prop(
                            "Array of entity keys to retrieve profiles for",
                            object_schema(
                                "Entity key",
                                vec![
                                    ("Id", string_prop("The unique ID of the entity")),
                                    ("Type", string_prop("The type of the entity")),
                                ],
                                vec!["Id", "Type"],
                            ),
                        ),
                    ),
                    (
                        "dataAsObject",
                        optional(boolean_prop("Whether to return profile data as a JSON object")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entities"],
            ),
        },
        // 3. SetProfileLanguage
        super::ToolSpec {
            name: "set_profile_language".to_string(),
            description: "Set the preferred language for an entity profile".to_string(),
            parameters: object_schema(
                "SetProfileLanguage parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to set the language for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "language",
                        string_prop("The language to set for the profile (e.g. en, ja, fr)"),
                    ),
                    (
                        "expectedVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "language"],
            ),
        },
        // 4. SetProfilePolicy
        super::ToolSpec {
            name: "set_profile_policy".to_string(),
            description: "Set the policy statements for an entity profile".to_string(),
            parameters: object_schema(
                "SetProfilePolicy parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to set the policy for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "statements",
                        object_array_prop(
                            "Array of policy statements to set",
                            object_schema(
                                "Policy statement",
                                vec![
                                    ("action", string_prop("The action this statement applies to")),
                                    ("effect", string_enum_prop("The effect of this statement", &["Allow", "Deny"])),
                                    ("resource", string_prop("The resource this statement applies to")),
                                    ("principal", string_prop("The principal this statement applies to")),
                                    ("comment", optional(string_prop("A comment about this statement"))),
                                    ("condition", optional(any_object_prop("Conditions for this statement"))),
                                ],
                                vec!["action", "effect", "resource", "principal"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "statements"],
            ),
        },
        // 5. GetTitlePlayersAccount
        super::ToolSpec {
            name: "get_title_players_account".to_string(),
            description: "Get the title player account entity for a given title player ID".to_string(),
            parameters: object_schema(
                "GetTitlePlayersAccount parameters",
                vec![
                    (
                        "titlePlayerId",
                        string_prop("The title player ID to look up"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["titlePlayerId"],
            ),
        },
        // 6. SetDisplayName
        super::ToolSpec {
            name: "set_display_name".to_string(),
            description: "Set the display name for an entity profile".to_string(),
            parameters: object_schema(
                "SetDisplayName parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to set the display name for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "displayName",
                        string_prop("The display name to set"),
                    ),
                    (
                        "expectedVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "displayName"],
            ),
        },
        // 7. SetGlobalPolicy
        super::ToolSpec {
            name: "set_global_policy".to_string(),
            description: "Set the global policy for all entities in the title".to_string(),
            parameters: object_schema(
                "SetGlobalPolicy parameters",
                vec![
                    (
                        "permissions",
                        object_array_prop(
                            "Array of permission statements for the global policy",
                            object_schema(
                                "Permission statement",
                                vec![
                                    ("action", string_prop("The action this permission applies to")),
                                    ("effect", string_enum_prop("The effect of this permission", &["Allow", "Deny"])),
                                    ("resource", string_prop("The resource this permission applies to")),
                                    ("principal", string_prop("The principal this permission applies to")),
                                    ("comment", optional(string_prop("A comment about this permission"))),
                                    ("condition", optional(any_object_prop("Conditions for this permission"))),
                                ],
                                vec!["action", "effect", "resource", "principal"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["permissions"],
            ),
        },
        // 8. GetGlobalPolicy
        super::ToolSpec {
            name: "get_global_policy".to_string(),
            description: "Get the global policy for all entities in the title".to_string(),
            parameters: object_schema(
                "GetGlobalPolicy parameters",
                vec![
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 9. SetAvatarUrl
        super::ToolSpec {
            name: "set_avatar_url".to_string(),
            description: "Set the avatar URL for an entity profile".to_string(),
            parameters: object_schema(
                "SetAvatarUrl parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to set the avatar URL for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "imageUrl",
                        string_prop("The URL of the avatar image"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "imageUrl"],
            ),
        },
        // 10. GetEntityProfilePolicy
        super::ToolSpec {
            name: "get_entity_profile_policy".to_string(),
            description: "Get the profile policy statements for a specific entity".to_string(),
            parameters: object_schema(
                "GetEntityProfilePolicy parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to get the profile policy for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
    ]
}
