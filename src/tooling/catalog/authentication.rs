// Authentication tools: GetEntityToken, ValidateEntityToken
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. GetEntityToken
        super::ToolSpec {
            name: "get_entity_token".to_string(),
            description: "Get an entity token for authentication. Can optionally specify an entity key to get a token for a specific entity.".to_string(),
            parameters: object_schema(
                "GetEntityToken parameters",
                vec![
                    (
                        "entity",
                        optional(object_schema(
                            "The entity to get a token for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity (e.g. title, master_player_account, title_player_account)")),
                            ],
                            vec!["Id", "Type"],
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 2. ValidateEntityToken
        super::ToolSpec {
            name: "validate_entity_token".to_string(),
            description: "Validate an entity token and return information about the entity it represents".to_string(),
            parameters: object_schema(
                "ValidateEntityToken parameters",
                vec![
                    (
                        "entityToken",
                        string_prop("The entity token to validate"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entityToken"],
            ),
        },
    ]
}
