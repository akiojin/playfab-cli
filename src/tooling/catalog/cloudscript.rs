// CloudScript tools: ExecuteFunction, ListFunctions, RegisterFunction, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. execute_function
        super::ToolSpec {
            name: "execute_function".to_string(),
            description: "Execute a CloudScript function by name (Azure Functions integration)"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "ExecuteFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the CloudScript function to execute"),
                    ),
                    (
                        "functionParameter",
                        optional(any_object_prop(
                            "Object passed as the FunctionArgument to the function",
                        )),
                    ),
                    (
                        "entity",
                        optional(any_object_prop(
                            "Entity profile of the caller (Id and Type)",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                    (
                        "generatePlayStreamEvent",
                        optional(boolean_prop(
                            "Generate a PlayStream event for this function execution",
                        )),
                    ),
                ],
                vec!["functionName"],
            ),
        },
        // 2. list_functions
        super::ToolSpec {
            name: "list_functions".to_string(),
            description: "List all registered CloudScript Azure Functions".to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "ListFunctions".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListFunctions parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 3. register_function
        super::ToolSpec {
            name: "register_function".to_string(),
            description: "Register a CloudScript Azure Function".to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "RegisterFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RegisterFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the function to register"),
                    ),
                    (
                        "functionUrl",
                        string_prop("Full URL of the Azure Function endpoint"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName", "functionUrl"],
            ),
        },
        // 4. list_http_functions
        super::ToolSpec {
            name: "list_http_functions".to_string(),
            description: "List all registered CloudScript HTTP-triggered Azure Functions"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "ListHttpFunctions".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListHttpFunctions parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 5. register_http_function
        super::ToolSpec {
            name: "register_http_function".to_string(),
            description: "Register an HTTP-triggered CloudScript Azure Function".to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "RegisterHttpFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RegisterHttpFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the HTTP function to register"),
                    ),
                    (
                        "functionUrl",
                        string_prop("Full URL of the HTTP Azure Function endpoint"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName", "functionUrl"],
            ),
        },
        // 6. unregister_function
        super::ToolSpec {
            name: "unregister_function".to_string(),
            description: "Unregister a previously registered CloudScript Azure Function"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "UnregisterFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnregisterFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the function to unregister"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName"],
            ),
        },
        // 7. list_queued_functions
        super::ToolSpec {
            name: "list_queued_functions".to_string(),
            description: "List all registered CloudScript queue-triggered Azure Functions"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "ListQueuedFunctions".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListQueuedFunctions parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 8. register_queued_function
        super::ToolSpec {
            name: "register_queued_function".to_string(),
            description: "Register a queue-triggered CloudScript Azure Function".to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "RegisterQueuedFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RegisterQueuedFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the queued function to register"),
                    ),
                    (
                        "queueName",
                        string_prop("Name of the Azure Storage queue to bind"),
                    ),
                    (
                        "connectionString",
                        optional(string_prop("Azure Storage connection string")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName", "queueName"],
            ),
        },
        // 9. unregister_queued_function
        super::ToolSpec {
            name: "unregister_queued_function".to_string(),
            description: "Unregister a previously registered queue-triggered Azure Function"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "UnregisterQueuedFunction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnregisterQueuedFunction parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the queued function to unregister"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName"],
            ),
        },
        // 10. post_function_result_for_entity_triggered_action
        super::ToolSpec {
            name: "post_function_result_for_entity_triggered_action".to_string(),
            description: "Post the result of an entity-triggered CloudScript function execution"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "PostFunctionResultForEntityTriggeredAction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PostFunctionResultForEntityTriggeredAction parameters",
                vec![
                    (
                        "entity",
                        any_object_prop("Entity profile of the caller (Id and Type)"),
                    ),
                    (
                        "functionResult",
                        any_object_prop("Result of the CloudScript function execution"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "functionResult"],
            ),
        },
        // 11. post_function_result_for_function_execution
        super::ToolSpec {
            name: "post_function_result_for_function_execution".to_string(),
            description: "Post the result of a CloudScript function execution".to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "PostFunctionResultForFunctionExecution".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PostFunctionResultForFunctionExecution parameters",
                vec![
                    (
                        "entity",
                        any_object_prop("Entity profile of the caller (Id and Type)"),
                    ),
                    (
                        "functionResult",
                        any_object_prop("Result of the CloudScript function execution"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "functionResult"],
            ),
        },
        // 12. post_function_result_for_player_triggered_action
        super::ToolSpec {
            name: "post_function_result_for_player_triggered_action".to_string(),
            description: "Post the result of a player-triggered CloudScript function execution"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "PostFunctionResultForPlayerTriggeredAction".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PostFunctionResultForPlayerTriggeredAction parameters",
                vec![
                    (
                        "entity",
                        any_object_prop("Entity profile of the caller (Id and Type)"),
                    ),
                    (
                        "functionResult",
                        any_object_prop("Result of the CloudScript function execution"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "functionResult"],
            ),
        },
        // 13. post_function_result_for_scheduled_task
        super::ToolSpec {
            name: "post_function_result_for_scheduled_task".to_string(),
            description: "Post the result of a scheduled-task CloudScript function execution"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "PostFunctionResultForScheduledTask".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "PostFunctionResultForScheduledTask parameters",
                vec![
                    (
                        "entity",
                        any_object_prop("Entity profile of the caller (Id and Type)"),
                    ),
                    (
                        "functionResult",
                        any_object_prop("Result of the CloudScript function execution"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "functionResult"],
            ),
        },
        // 14. execute_entity_cloud_script
        super::ToolSpec {
            name: "execute_entity_cloud_script".to_string(),
            description: "Execute an entity CloudScript function (legacy revision-based)"
                .to_string(),
            api_group: "CloudScript".to_string(),
            api_method: "ExecuteEntityCloudScript".to_string(),
            category: super::ToolCategory::CloudScript,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteEntityCloudScript parameters",
                vec![
                    (
                        "functionName",
                        string_prop("Name of the CloudScript function to execute"),
                    ),
                    (
                        "entity",
                        optional(any_object_prop(
                            "Entity to execute the function on behalf of (Id and Type)",
                        )),
                    ),
                    (
                        "functionParameter",
                        optional(any_object_prop(
                            "Object passed as the FunctionArgument to the function",
                        )),
                    ),
                    (
                        "revisionSelection",
                        optional(string_enum_prop(
                            "CloudScript revision selection",
                            &["Live", "Latest", "Specific"],
                        )),
                    ),
                    (
                        "specificRevision",
                        optional(integer_prop(
                            "Specific revision to execute (when revisionSelection is Specific)",
                        )),
                    ),
                    (
                        "generatePlayStreamEvent",
                        optional(boolean_prop(
                            "Generate a PlayStream event for this execution",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["functionName"],
            ),
        },
    ]
}
