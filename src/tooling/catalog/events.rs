// Events tools: WriteEvents, WriteTelemetryEvents, TelemetryKey management, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. write_events
        super::ToolSpec {
            name: "write_events".to_string(),
            description: "Write one or more PlayStream events".to_string(),
            api_group: "Events".to_string(),
            api_method: "WriteEvents".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "WriteEvents parameters",
                vec![
                    (
                        "events",
                        object_array_prop(
                            "Array of events to write",
                            object_schema(
                                "Event entry",
                                vec![
                                    ("name", string_prop("Name of the event")),
                                    (
                                        "eventNamespace",
                                        string_prop("Namespace of the event (e.g. custom.*)"),
                                    ),
                                    ("payload", any_object_prop("Event-specific payload data")),
                                    (
                                        "entity",
                                        optional(any_object_prop(
                                            "Entity associated with the event (Id and Type)",
                                        )),
                                    ),
                                    (
                                        "originalId",
                                        optional(string_prop(
                                            "Original unique ID for deduplication",
                                        )),
                                    ),
                                    (
                                        "originalTimestamp",
                                        optional(string_prop(
                                            "Original event timestamp (ISO 8601)",
                                        )),
                                    ),
                                ],
                                vec!["name", "eventNamespace", "payload"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["events"],
            ),
        },
        // 2. write_telemetry_events
        super::ToolSpec {
            name: "write_telemetry_events".to_string(),
            description: "Write one or more telemetry events to the telemetry pipeline".to_string(),
            api_group: "Events".to_string(),
            api_method: "WriteTelemetryEvents".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "WriteTelemetryEvents parameters",
                vec![
                    (
                        "events",
                        object_array_prop(
                            "Array of telemetry events to write",
                            object_schema(
                                "Telemetry event entry",
                                vec![
                                    ("name", string_prop("Name of the telemetry event")),
                                    ("eventNamespace", string_prop("Namespace of the event")),
                                    ("payload", any_object_prop("Event-specific payload data")),
                                    (
                                        "entity",
                                        optional(any_object_prop(
                                            "Entity associated with the event (Id and Type)",
                                        )),
                                    ),
                                    (
                                        "originalId",
                                        optional(string_prop(
                                            "Original unique ID for deduplication",
                                        )),
                                    ),
                                    (
                                        "originalTimestamp",
                                        optional(string_prop(
                                            "Original event timestamp (ISO 8601)",
                                        )),
                                    ),
                                ],
                                vec!["name", "eventNamespace", "payload"],
                            ),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["events"],
            ),
        },
        // 3. create_telemetry_key
        super::ToolSpec {
            name: "create_telemetry_key".to_string(),
            description: "Create a new telemetry key for the title".to_string(),
            api_group: "Events".to_string(),
            api_method: "CreateTelemetryKey".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateTelemetryKey parameters",
                vec![
                    (
                        "keyName",
                        string_prop("Name of the telemetry key to create"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["keyName"],
            ),
        },
        // 4. delete_telemetry_key
        super::ToolSpec {
            name: "delete_telemetry_key".to_string(),
            description: "Delete an existing telemetry key".to_string(),
            api_group: "Events".to_string(),
            api_method: "DeleteTelemetryKey".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteTelemetryKey parameters",
                vec![
                    (
                        "keyName",
                        string_prop("Name of the telemetry key to delete"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["keyName"],
            ),
        },
        // 5. get_telemetry_key
        super::ToolSpec {
            name: "get_telemetry_key".to_string(),
            description: "Get details of a specific telemetry key".to_string(),
            api_group: "Events".to_string(),
            api_method: "GetTelemetryKey".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTelemetryKey parameters",
                vec![
                    (
                        "keyName",
                        string_prop("Name of the telemetry key to retrieve"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["keyName"],
            ),
        },
        // 6. list_telemetry_keys
        super::ToolSpec {
            name: "list_telemetry_keys".to_string(),
            description: "List all telemetry keys for the title".to_string(),
            api_group: "Events".to_string(),
            api_method: "ListTelemetryKeys".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListTelemetryKeys parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 7. set_telemetry_key_active
        super::ToolSpec {
            name: "set_telemetry_key_active".to_string(),
            description: "Activate or deactivate a telemetry key".to_string(),
            api_group: "Events".to_string(),
            api_method: "SetTelemetryKeyActive".to_string(),
            category: super::ToolCategory::Events,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetTelemetryKeyActive parameters",
                vec![
                    ("keyName", string_prop("Name of the telemetry key")),
                    (
                        "active",
                        boolean_prop("Whether the telemetry key should be active"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["keyName", "active"],
            ),
        },
    ]
}
