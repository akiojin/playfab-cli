// Multiplayer tools: Matchmaking, Server Hosting, Lobby, Party, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // =====================================================================
        // Matchmaking (~10 tools)
        // =====================================================================
        // 1. mp_create_matchmaking_ticket
        super::ToolSpec {
            name: "mp_create_matchmaking_ticket".to_string(),
            description: "Create a matchmaking ticket for a player to find a match".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateMatchmakingTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateMatchmakingTicket parameters",
                vec![
                    (
                        "creator",
                        any_object_prop("The creator entity and attributes for the ticket"),
                    ),
                    (
                        "giveUpAfterSeconds",
                        integer_range_prop(
                            "How long to attempt matching in seconds",
                            Some(1),
                            Some(600),
                        ),
                    ),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue to join"),
                    ),
                    (
                        "membersToMatchWith",
                        optional(object_array_prop(
                            "Other members to match with",
                            any_object_prop("Entity key and attributes"),
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["creator", "giveUpAfterSeconds", "queueName"],
            ),
        },
        // 2. mp_get_matchmaking_ticket
        super::ToolSpec {
            name: "mp_get_matchmaking_ticket".to_string(),
            description: "Get the status and details of a matchmaking ticket".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetMatchmakingTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMatchmakingTicket parameters",
                vec![
                    ("ticketId", string_prop("The ID of the matchmaking ticket")),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "escapeObject",
                        optional(boolean_prop("Escape the JSON object in the response")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ticketId", "queueName"],
            ),
        },
        // 3. mp_cancel_matchmaking_ticket
        super::ToolSpec {
            name: "mp_cancel_matchmaking_ticket".to_string(),
            description: "Cancel a specific matchmaking ticket".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CancelMatchmakingTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CancelMatchmakingTicket parameters",
                vec![
                    (
                        "ticketId",
                        string_prop("The ID of the matchmaking ticket to cancel"),
                    ),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ticketId", "queueName"],
            ),
        },
        // 4. mp_cancel_all_matchmaking_tickets_for_player
        super::ToolSpec {
            name: "mp_cancel_all_matchmaking_tickets_for_player".to_string(),
            description: "Cancel all matchmaking tickets for a specific player".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CancelAllMatchmakingTicketsForPlayer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CancelAllMatchmakingTicketsForPlayer parameters",
                vec![
                    (
                        "queueName",
                        optional(string_prop(
                            "The name of the queue to cancel tickets in (all queues if omitted)",
                        )),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("The entity key of the player")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 5. mp_list_matchmaking_tickets_for_player
        super::ToolSpec {
            name: "mp_list_matchmaking_tickets_for_player".to_string(),
            description: "List all matchmaking tickets for a specific player".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListMatchmakingTicketsForPlayer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListMatchmakingTicketsForPlayer parameters",
                vec![
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "entity",
                        optional(any_object_prop("The entity key of the player")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["queueName"],
            ),
        },
        // 6. mp_get_match
        super::ToolSpec {
            name: "mp_get_match".to_string(),
            description: "Get details of a completed match including matched players".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetMatch".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMatch parameters",
                vec![
                    ("matchId", string_prop("The ID of the match")),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "escapeObject",
                        optional(boolean_prop("Escape the JSON object in the response")),
                    ),
                    (
                        "returnMemberAttributes",
                        optional(boolean_prop("Whether to return member attributes")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["matchId", "queueName"],
            ),
        },
        // 7. mp_list_matchmaking_queues_for_title
        super::ToolSpec {
            name: "mp_list_matchmaking_queues_for_title".to_string(),
            description: "List all matchmaking queue configurations for the title".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListMatchmakingQueuesForTitle".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListMatchmakingQueuesForTitle parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 8. mp_set_matchmaking_queue
        super::ToolSpec {
            name: "mp_set_matchmaking_queue".to_string(),
            description: "Create or update a matchmaking queue configuration".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "SetMatchmakingQueue".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetMatchmakingQueue parameters",
                vec![
                    (
                        "matchmakingQueue",
                        any_object_prop("The matchmaking queue configuration to set"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["matchmakingQueue"],
            ),
        },
        // 9. mp_remove_matchmaking_queue
        super::ToolSpec {
            name: "mp_remove_matchmaking_queue".to_string(),
            description: "Remove a matchmaking queue configuration".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "RemoveMatchmakingQueue".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveMatchmakingQueue parameters",
                vec![
                    ("queueName", string_prop("The name of the queue to remove")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["queueName"],
            ),
        },
        // 10. mp_get_queue_statistics
        super::ToolSpec {
            name: "mp_get_queue_statistics".to_string(),
            description: "Get statistics for a matchmaking queue".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetQueueStatistics".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetQueueStatistics parameters",
                vec![
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["queueName"],
            ),
        },
        // =====================================================================
        // Server Hosting - Build Management (~15 tools)
        // =====================================================================
        // 11. mp_list_build_summaries
        super::ToolSpec {
            name: "mp_list_build_summaries".to_string(),
            description: "List summaries of all multiplayer server builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListBuildSummaries".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListBuildSummariesV2 parameters",
                vec![
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 12. mp_get_build
        super::ToolSpec {
            name: "mp_get_build".to_string(),
            description: "Get details of a specific multiplayer server build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetBuild".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetBuild parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId"],
            ),
        },
        // 13. mp_create_build_with_managed_container
        super::ToolSpec {
            name: "mp_create_build_with_managed_container".to_string(),
            description: "Create a multiplayer server build with a managed container image"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateBuildWithManagedContainer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateBuildWithManagedContainer parameters",
                vec![
                    ("buildName", string_prop("The name of the build")),
                    (
                        "containerFlavor",
                        string_enum_prop(
                            "The container flavor",
                            &[
                                "ManagedWindowsServerCore",
                                "ManagedWindowsServerCorePreview",
                                "CustomLinux",
                                "Invalid",
                            ],
                        ),
                    ),
                    (
                        "gameAssetReferences",
                        object_array_prop(
                            "The game assets for the build",
                            any_object_prop("Game asset reference with FileName and MountPath"),
                        ),
                    ),
                    (
                        "gameCertificateReferences",
                        optional(object_array_prop(
                            "Game certificate references",
                            any_object_prop("Certificate reference"),
                        )),
                    ),
                    (
                        "gameWorkingDirectory",
                        optional(string_prop("The working directory for the game process")),
                    ),
                    (
                        "instrumentationConfiguration",
                        optional(any_object_prop("Instrumentation configuration")),
                    ),
                    (
                        "metadata",
                        optional(any_object_prop("Build metadata key-value pairs")),
                    ),
                    (
                        "multiplayerServerCountPerVm",
                        integer_prop("Number of multiplayer servers per VM"),
                    ),
                    (
                        "osPlatform",
                        optional(string_prop("The OS platform (Linux or Windows)")),
                    ),
                    (
                        "ports",
                        object_array_prop(
                            "The ports the build is mapped on",
                            any_object_prop("Port with Name, Num, and Protocol"),
                        ),
                    ),
                    (
                        "regionConfigurations",
                        object_array_prop(
                            "Region configurations for the build",
                            any_object_prop(
                                "Region config with Region, MaxServers, StandbyServers",
                            ),
                        ),
                    ),
                    (
                        "startMultiplayerServerCommand",
                        string_prop("The command to start the multiplayer server"),
                    ),
                    (
                        "vmSize",
                        optional(string_enum_prop(
                            "The VM size",
                            &[
                                "Standard_A1",
                                "Standard_A2",
                                "Standard_A3",
                                "Standard_A4",
                                "Standard_A1_v2",
                                "Standard_A2_v2",
                                "Standard_A4_v2",
                                "Standard_A8_v2",
                                "Standard_D1_v2",
                                "Standard_D2_v2",
                                "Standard_D3_v2",
                                "Standard_D4_v2",
                                "Standard_D5_v2",
                                "Standard_F1",
                                "Standard_F2",
                                "Standard_F4",
                                "Standard_F8",
                                "Standard_F16",
                                "Standard_F2s_v2",
                                "Standard_F4s_v2",
                                "Standard_F8s_v2",
                                "Standard_F16s_v2",
                                "Standard_D2as_v4",
                                "Standard_D4as_v4",
                                "Standard_D8as_v4",
                                "Standard_D16as_v4",
                                "Standard_A8",
                                "Standard_HB120_16rs_v3",
                                "Standard_HB120_32rs_v3",
                                "Standard_HB120_64rs_v3",
                                "Standard_HB120_96rs_v3",
                                "Standard_HB120rs_v3",
                            ],
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "buildName",
                    "containerFlavor",
                    "gameAssetReferences",
                    "multiplayerServerCountPerVm",
                    "ports",
                    "regionConfigurations",
                    "startMultiplayerServerCommand",
                ],
            ),
        },
        // 14. mp_create_build_with_custom_container
        super::ToolSpec {
            name: "mp_create_build_with_custom_container".to_string(),
            description: "Create a multiplayer server build with a custom container image"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateBuildWithCustomContainer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateBuildWithCustomContainer parameters",
                vec![
                    ("buildName", string_prop("The name of the build")),
                    (
                        "containerImageReference",
                        optional(any_object_prop(
                            "Container image reference with ImageName and Tag",
                        )),
                    ),
                    (
                        "containerRunCommand",
                        optional(string_prop("The command to run when the container starts")),
                    ),
                    (
                        "gameAssetReferences",
                        optional(object_array_prop(
                            "Game asset references",
                            any_object_prop("Game asset reference"),
                        )),
                    ),
                    (
                        "gameCertificateReferences",
                        optional(object_array_prop(
                            "Game certificate references",
                            any_object_prop("Certificate reference"),
                        )),
                    ),
                    (
                        "linuxInstrumentationConfiguration",
                        optional(any_object_prop("Linux instrumentation configuration")),
                    ),
                    (
                        "metadata",
                        optional(any_object_prop("Build metadata key-value pairs")),
                    ),
                    (
                        "multiplayerServerCountPerVm",
                        integer_prop("Number of multiplayer servers per VM"),
                    ),
                    (
                        "ports",
                        object_array_prop(
                            "The ports the build is mapped on",
                            any_object_prop("Port with Name, Num, and Protocol"),
                        ),
                    ),
                    (
                        "regionConfigurations",
                        object_array_prop(
                            "Region configurations for the build",
                            any_object_prop(
                                "Region config with Region, MaxServers, StandbyServers",
                            ),
                        ),
                    ),
                    ("vmSize", optional(string_prop("The VM size for the build"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "buildName",
                    "multiplayerServerCountPerVm",
                    "ports",
                    "regionConfigurations",
                ],
            ),
        },
        // 15. mp_create_build_with_process_based_server
        super::ToolSpec {
            name: "mp_create_build_with_process_based_server".to_string(),
            description: "Create a multiplayer server build with a process-based server"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateBuildWithProcessBasedServer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateBuildWithProcessBasedServer parameters",
                vec![
                    ("buildName", string_prop("The name of the build")),
                    (
                        "gameAssetReferences",
                        object_array_prop(
                            "Game asset references",
                            any_object_prop("Game asset reference with FileName and MountPath"),
                        ),
                    ),
                    (
                        "gameCertificateReferences",
                        optional(object_array_prop(
                            "Game certificate references",
                            any_object_prop("Certificate reference"),
                        )),
                    ),
                    (
                        "gameWorkingDirectory",
                        optional(string_prop("The working directory for the game process")),
                    ),
                    (
                        "instrumentationConfiguration",
                        optional(any_object_prop("Instrumentation configuration")),
                    ),
                    (
                        "metadata",
                        optional(any_object_prop("Build metadata key-value pairs")),
                    ),
                    (
                        "multiplayerServerCountPerVm",
                        integer_prop("Number of multiplayer servers per VM"),
                    ),
                    (
                        "osPlatform",
                        optional(string_prop("The OS platform (Linux or Windows)")),
                    ),
                    (
                        "ports",
                        object_array_prop(
                            "The ports the build is mapped on",
                            any_object_prop("Port with Name, Num, and Protocol"),
                        ),
                    ),
                    (
                        "regionConfigurations",
                        object_array_prop(
                            "Region configurations for the build",
                            any_object_prop(
                                "Region config with Region, MaxServers, StandbyServers",
                            ),
                        ),
                    ),
                    (
                        "startMultiplayerServerCommand",
                        string_prop("The command to start the multiplayer server"),
                    ),
                    ("vmSize", optional(string_prop("The VM size for the build"))),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "buildName",
                    "gameAssetReferences",
                    "multiplayerServerCountPerVm",
                    "ports",
                    "regionConfigurations",
                    "startMultiplayerServerCommand",
                ],
            ),
        },
        // 16. mp_delete_build
        super::ToolSpec {
            name: "mp_delete_build".to_string(),
            description: "Delete a multiplayer server build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteBuild".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteBuild parameters",
                vec![
                    ("buildId", string_prop("The ID of the build to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId"],
            ),
        },
        // 17. mp_update_build_name
        super::ToolSpec {
            name: "mp_update_build_name".to_string(),
            description: "Update the name of a multiplayer server build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UpdateBuildName".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateBuildName parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("buildName", string_prop("The new name for the build")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "buildName"],
            ),
        },
        // 18. mp_update_build_region
        super::ToolSpec {
            name: "mp_update_build_region".to_string(),
            description: "Update a single region configuration for a build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UpdateBuildRegion".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateBuildRegion parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    (
                        "buildRegion",
                        any_object_prop("The updated region configuration"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "buildRegion"],
            ),
        },
        // 19. mp_update_build_regions
        super::ToolSpec {
            name: "mp_update_build_regions".to_string(),
            description: "Update multiple region configurations for a build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UpdateBuildRegions".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateBuildRegions parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    (
                        "buildRegions",
                        object_array_prop(
                            "The updated region configurations",
                            any_object_prop("Region configuration"),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "buildRegions"],
            ),
        },
        // 20. mp_list_build_aliases
        super::ToolSpec {
            name: "mp_list_build_aliases".to_string(),
            description: "List all build aliases for the title".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListBuildAliases".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListBuildAliases parameters",
                vec![
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 21. mp_create_build_alias
        super::ToolSpec {
            name: "mp_create_build_alias".to_string(),
            description: "Create a build alias to route traffic between builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateBuildAlias".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateBuildAlias parameters",
                vec![
                    ("aliasName", string_prop("The name of the alias")),
                    (
                        "buildSelectionCriteria",
                        object_array_prop(
                            "Build selection criteria with weights",
                            any_object_prop("Build selection criterion with BuildId and Weight"),
                        ),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["aliasName", "buildSelectionCriteria"],
            ),
        },
        // 22. mp_update_build_alias
        super::ToolSpec {
            name: "mp_update_build_alias".to_string(),
            description: "Update an existing build alias".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UpdateBuildAlias".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateBuildAlias parameters",
                vec![
                    ("aliasId", string_prop("The ID of the alias to update")),
                    ("aliasName", optional(string_prop("The updated alias name"))),
                    (
                        "buildSelectionCriteria",
                        optional(object_array_prop(
                            "Updated build selection criteria",
                            any_object_prop("Build selection criterion"),
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["aliasId"],
            ),
        },
        // 23. mp_delete_build_alias
        super::ToolSpec {
            name: "mp_delete_build_alias".to_string(),
            description: "Delete a build alias".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteBuildAlias".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteBuildAlias parameters",
                vec![
                    ("aliasId", string_prop("The ID of the alias to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["aliasId"],
            ),
        },
        // 24. mp_get_build_alias
        super::ToolSpec {
            name: "mp_get_build_alias".to_string(),
            description: "Get details of a specific build alias".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetBuildAlias".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetBuildAlias parameters",
                vec![
                    ("aliasId", string_prop("The ID of the alias")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["aliasId"],
            ),
        },
        // 25. mp_rollover_container_registry_credentials
        super::ToolSpec {
            name: "mp_rollover_container_registry_credentials".to_string(),
            description: "Roll over container registry credentials for the title".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "RolloverContainerRegistryCredentials".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RolloverContainerRegistryCredentials parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // =====================================================================
        // Server Hosting - Container Management (~5 tools)
        // =====================================================================
        // 26. mp_list_container_images
        super::ToolSpec {
            name: "mp_list_container_images".to_string(),
            description: "List container images available for multiplayer server builds"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListContainerImages".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListContainerImages parameters",
                vec![
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 27. mp_list_container_image_tags
        super::ToolSpec {
            name: "mp_list_container_image_tags".to_string(),
            description: "List tags for a specific container image".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListContainerImageTags".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListContainerImageTags parameters",
                vec![
                    ("imageName", string_prop("The name of the container image")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["imageName"],
            ),
        },
        // 28. mp_get_container_registry_credentials
        super::ToolSpec {
            name: "mp_get_container_registry_credentials".to_string(),
            description: "Get the credentials for the container registry".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetContainerRegistryCredentials".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetContainerRegistryCredentials parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 29. mp_upload_certificate
        super::ToolSpec {
            name: "mp_upload_certificate".to_string(),
            description: "Upload a certificate for use with multiplayer server builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UploadCertificate".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UploadCertificate parameters",
                vec![
                    (
                        "gameCertificate",
                        any_object_prop(
                            "The game certificate to upload (Name, Base64EncodedValue, Password)",
                        ),
                    ),
                    (
                        "forceUpdate",
                        optional(boolean_prop(
                            "Whether to force update an existing certificate",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["gameCertificate"],
            ),
        },
        // 30. mp_delete_certificate
        super::ToolSpec {
            name: "mp_delete_certificate".to_string(),
            description: "Delete a certificate used by multiplayer server builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteCertificate".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteCertificate parameters",
                vec![
                    ("name", string_prop("The name of the certificate to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // =====================================================================
        // Server Hosting - Server Management (~10 tools)
        // =====================================================================
        // 31. mp_list_multiplayer_servers
        super::ToolSpec {
            name: "mp_list_multiplayer_servers".to_string(),
            description: "List multiplayer servers for a build in a region".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListMultiplayerServers".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListMultiplayerServers parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region to list servers for")),
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region"],
            ),
        },
        // 32. mp_request_multiplayer_server
        super::ToolSpec {
            name: "mp_request_multiplayer_server".to_string(),
            description: "Request a multiplayer server session from a build".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "RequestMultiplayerServer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RequestMultiplayerServer parameters",
                vec![
                    (
                        "buildId",
                        string_prop("The ID of the build to request a server from"),
                    ),
                    (
                        "preferredRegions",
                        string_array_prop("Preferred regions in order of priority"),
                    ),
                    ("sessionId", string_prop("A unique ID for the game session")),
                    (
                        "buildAliasParams",
                        optional(any_object_prop(
                            "Build alias parameters to use instead of buildId",
                        )),
                    ),
                    (
                        "initialPlayers",
                        optional(string_array_prop(
                            "List of player IDs to initialize the server with",
                        )),
                    ),
                    (
                        "sessionCookie",
                        optional(string_prop("An opaque string passed to the game server")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["preferredRegions", "sessionId"],
            ),
        },
        // 33. mp_get_multiplayer_server_details
        super::ToolSpec {
            name: "mp_get_multiplayer_server_details".to_string(),
            description: "Get the details of a specific multiplayer server session".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetMultiplayerServerDetails".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMultiplayerServerDetails parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region the server is in")),
                    ("sessionId", string_prop("The session ID of the server")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region", "sessionId"],
            ),
        },
        // 34. mp_shutdown_multiplayer_server
        super::ToolSpec {
            name: "mp_shutdown_multiplayer_server".to_string(),
            description: "Shut down a multiplayer server session".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ShutdownMultiplayerServer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ShutdownMultiplayerServer parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region the server is in")),
                    (
                        "sessionId",
                        string_prop("The session ID of the server to shut down"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region", "sessionId"],
            ),
        },
        // 35. mp_list_virtual_machine_summaries
        super::ToolSpec {
            name: "mp_list_virtual_machine_summaries".to_string(),
            description: "List virtual machine summaries for a build in a region".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListVirtualMachineSummaries".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListVirtualMachineSummaries parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region to list VMs for")),
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region"],
            ),
        },
        // 36. mp_get_remote_login_endpoint
        super::ToolSpec {
            name: "mp_get_remote_login_endpoint".to_string(),
            description: "Get the remote login endpoint for a VM in a multiplayer server build"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetRemoteLoginEndpoint".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetRemoteLoginEndpoint parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region of the VM")),
                    ("vmId", string_prop("The ID of the virtual machine")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region", "vmId"],
            ),
        },
        // 37. mp_create_remote_user
        super::ToolSpec {
            name: "mp_create_remote_user".to_string(),
            description: "Create a remote user on a VM for debugging".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateRemoteUser".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateRemoteUser parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region of the VM")),
                    ("username", string_prop("The username for the remote user")),
                    ("vmId", string_prop("The ID of the virtual machine")),
                    (
                        "expirationTime",
                        optional(string_prop("Expiration time for the user (ISO 8601)")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region", "username", "vmId"],
            ),
        },
        // 38. mp_delete_remote_user
        super::ToolSpec {
            name: "mp_delete_remote_user".to_string(),
            description: "Delete a remote user from a VM".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteRemoteUser".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteRemoteUser parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    ("region", string_prop("The region of the VM")),
                    ("username", string_prop("The username to delete")),
                    ("vmId", string_prop("The ID of the virtual machine")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region", "username", "vmId"],
            ),
        },
        // 39. mp_list_party_qos_servers
        super::ToolSpec {
            name: "mp_list_party_qos_servers".to_string(),
            description: "List quality-of-service servers for Party".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListPartyQosServers".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListPartyQosServers parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 40. mp_request_party_service
        super::ToolSpec {
            name: "mp_request_party_service".to_string(),
            description: "Request a Party service endpoint for network communication".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "RequestPartyService".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RequestPartyService parameters",
                vec![
                    (
                        "networkConfiguration",
                        any_object_prop("The network configuration for the party"),
                    ),
                    (
                        "preferredRegions",
                        optional(string_array_prop("Preferred regions in order of priority")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["networkConfiguration"],
            ),
        },
        // =====================================================================
        // Server Hosting - Configuration (~5 tools)
        // =====================================================================
        // 41. mp_get_title_enabled_for_multiplayer_servers_status
        super::ToolSpec {
            name: "mp_get_title_enabled_for_multiplayer_servers_status".to_string(),
            description: "Check if multiplayer servers are enabled for the title".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetTitleEnabledForMultiplayerServersStatus".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTitleEnabledForMultiplayerServersStatus parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 42. mp_enable_multiplayer_servers_for_title
        super::ToolSpec {
            name: "mp_enable_multiplayer_servers_for_title".to_string(),
            description: "Enable multiplayer servers for the title".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "EnableMultiplayerServersForTitle".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "EnableMultiplayerServersForTitle parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
        // 43. mp_list_assets_summaries
        super::ToolSpec {
            name: "mp_list_assets_summaries".to_string(),
            description: "List asset summaries for multiplayer server builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListAssetsSummaries".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListAssetsSummaries parameters",
                vec![
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 44. mp_get_asset_upload_url
        super::ToolSpec {
            name: "mp_get_asset_upload_url".to_string(),
            description: "Get a URL for uploading an asset to the multiplayer server".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetAssetUploadUrl".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetAssetUploadUrl parameters",
                vec![
                    ("fileName", string_prop("The name of the asset file")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["fileName"],
            ),
        },
        // 45. mp_delete_asset
        super::ToolSpec {
            name: "mp_delete_asset".to_string(),
            description: "Delete an asset used by multiplayer server builds".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteAsset".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteAsset parameters",
                vec![
                    (
                        "fileName",
                        string_prop("The name of the asset file to delete"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["fileName"],
            ),
        },
        // =====================================================================
        // Lobby (~10 tools)
        // =====================================================================
        // 46. mp_create_lobby
        super::ToolSpec {
            name: "mp_create_lobby".to_string(),
            description: "Create a new lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateLobby parameters",
                vec![
                    (
                        "owner",
                        any_object_prop("The entity key of the lobby owner"),
                    ),
                    (
                        "maxPlayers",
                        integer_range_prop(
                            "Maximum number of players in the lobby",
                            Some(1),
                            Some(128),
                        ),
                    ),
                    (
                        "accessPolicy",
                        optional(string_enum_prop(
                            "The access policy for the lobby",
                            &["Public", "Friends", "Private"],
                        )),
                    ),
                    (
                        "lobbyData",
                        optional(any_object_prop("Key-value pairs of lobby data")),
                    ),
                    (
                        "memberData",
                        optional(any_object_prop(
                            "Key-value pairs of member data for the creator",
                        )),
                    ),
                    (
                        "members",
                        optional(object_array_prop(
                            "Initial members to add to the lobby",
                            any_object_prop("Member entity and data"),
                        )),
                    ),
                    (
                        "ownerMigrationPolicy",
                        optional(string_enum_prop(
                            "Policy for owner migration",
                            &["None", "Automatic", "Manual", "Server"],
                        )),
                    ),
                    (
                        "searchData",
                        optional(any_object_prop("Search data for lobby discovery")),
                    ),
                    (
                        "useConnections",
                        optional(boolean_prop(
                            "Whether to use real-time connections for the lobby",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["owner", "maxPlayers"],
            ),
        },
        // 47. mp_delete_lobby
        super::ToolSpec {
            name: "mp_delete_lobby".to_string(),
            description: "Delete a lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "DeleteLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId"],
            ),
        },
        // 48. mp_find_lobbies
        super::ToolSpec {
            name: "mp_find_lobbies".to_string(),
            description: "Find lobbies matching specified criteria".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "FindLobbies".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "FindLobbies parameters",
                vec![
                    (
                        "filter",
                        optional(string_prop("OData filter expression for lobby search")),
                    ),
                    ("orderBy", optional(string_prop("OData orderBy expression"))),
                    (
                        "pagination",
                        optional(any_object_prop(
                            "Pagination options (PageSizeRequested, ContinuationToken)",
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
        // 49. mp_get_lobby
        super::ToolSpec {
            name: "mp_get_lobby".to_string(),
            description: "Get details of a specific lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId"],
            ),
        },
        // 50. mp_invite_to_lobby
        super::ToolSpec {
            name: "mp_invite_to_lobby".to_string(),
            description: "Invite a player to join a lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "InviteToLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "InviteToLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby")),
                    (
                        "inviteeEntity",
                        any_object_prop("The entity key of the player to invite"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId", "inviteeEntity"],
            ),
        },
        // 51. mp_join_lobby
        super::ToolSpec {
            name: "mp_join_lobby".to_string(),
            description: "Join an existing lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "JoinLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "JoinLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby to join")),
                    (
                        "memberEntity",
                        any_object_prop("The entity key of the player joining"),
                    ),
                    (
                        "connectionString",
                        optional(string_prop("A connection string for the lobby")),
                    ),
                    (
                        "memberData",
                        optional(any_object_prop("Key-value pairs of member data")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId", "memberEntity"],
            ),
        },
        // 52. mp_leave_lobby
        super::ToolSpec {
            name: "mp_leave_lobby".to_string(),
            description: "Leave a lobby".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "LeaveLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LeaveLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby to leave")),
                    (
                        "memberEntity",
                        any_object_prop("The entity key of the player leaving"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId", "memberEntity"],
            ),
        },
        // 53. mp_update_lobby
        super::ToolSpec {
            name: "mp_update_lobby".to_string(),
            description: "Update lobby properties and data".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UpdateLobby".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateLobby parameters",
                vec![
                    ("lobbyId", string_prop("The ID of the lobby to update")),
                    (
                        "accessPolicy",
                        optional(string_enum_prop(
                            "Updated access policy",
                            &["Public", "Friends", "Private"],
                        )),
                    ),
                    (
                        "lobbyData",
                        optional(any_object_prop("Updated lobby data key-value pairs")),
                    ),
                    (
                        "lobbyDataToDelete",
                        optional(string_array_prop("Keys to delete from lobby data")),
                    ),
                    (
                        "maxPlayers",
                        optional(integer_range_prop(
                            "Updated max player count",
                            Some(1),
                            Some(128),
                        )),
                    ),
                    (
                        "memberData",
                        optional(any_object_prop("Updated member data for the caller")),
                    ),
                    (
                        "memberDataToDelete",
                        optional(string_array_prop("Keys to delete from member data")),
                    ),
                    (
                        "memberEntity",
                        optional(any_object_prop(
                            "The entity key of the member making the update",
                        )),
                    ),
                    (
                        "owner",
                        optional(any_object_prop("New owner entity key (for owner transfer)")),
                    ),
                    (
                        "searchData",
                        optional(any_object_prop("Updated search data")),
                    ),
                    (
                        "searchDataToDelete",
                        optional(string_array_prop("Keys to delete from search data")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["lobbyId"],
            ),
        },
        // 54. mp_subscribe_to_lobby_resource
        super::ToolSpec {
            name: "mp_subscribe_to_lobby_resource".to_string(),
            description: "Subscribe to lobby resource notifications".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "SubscribeToLobbyResource".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SubscribeToLobbyResource parameters",
                vec![
                    (
                        "entityKey",
                        any_object_prop("The entity key subscribing to the resource"),
                    ),
                    (
                        "pubSubConnectionHandle",
                        string_prop("The PubSub connection handle"),
                    ),
                    ("resourceId", string_prop("The resource ID to subscribe to")),
                    (
                        "subscriptionVersion",
                        integer_prop("The subscription version number"),
                    ),
                    ("type", string_prop("The type of resource subscription")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "entityKey",
                    "pubSubConnectionHandle",
                    "resourceId",
                    "subscriptionVersion",
                    "type",
                ],
            ),
        },
        // 55. mp_unsubscribe_from_lobby_resource
        super::ToolSpec {
            name: "mp_unsubscribe_from_lobby_resource".to_string(),
            description: "Unsubscribe from lobby resource notifications".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "UnsubscribeFromLobbyResource".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnsubscribeFromLobbyResource parameters",
                vec![
                    ("entityKey", any_object_prop("The entity key unsubscribing")),
                    (
                        "pubSubConnectionHandle",
                        string_prop("The PubSub connection handle"),
                    ),
                    (
                        "resourceId",
                        string_prop("The resource ID to unsubscribe from"),
                    ),
                    (
                        "subscriptionVersion",
                        integer_prop("The subscription version number"),
                    ),
                    ("type", string_prop("The type of resource subscription")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "entityKey",
                    "pubSubConnectionHandle",
                    "resourceId",
                    "subscriptionVersion",
                    "type",
                ],
            ),
        },
        // =====================================================================
        // Party (~5 tools, mp_list_party_qos_servers already defined above)
        // =====================================================================
        // 56. mp_create_party
        super::ToolSpec {
            name: "mp_create_party".to_string(),
            description: "Create a new Party network for voice and data communication".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateParty".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateParty parameters",
                vec![
                    (
                        "networkConfiguration",
                        any_object_prop("The network configuration for the party"),
                    ),
                    (
                        "partyId",
                        optional(string_prop("Custom Party ID (auto-generated if omitted)")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["networkConfiguration"],
            ),
        },
        // 57. mp_get_party
        super::ToolSpec {
            name: "mp_get_party".to_string(),
            description: "Get details of a specific Party network".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetParty".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetParty parameters",
                vec![
                    ("partyId", string_prop("The ID of the party")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["partyId"],
            ),
        },
        // 58. mp_join_party
        super::ToolSpec {
            name: "mp_join_party".to_string(),
            description: "Join an existing Party network".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "JoinParty".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "JoinParty parameters",
                vec![
                    ("partyId", string_prop("The ID of the party to join")),
                    (
                        "networkConfiguration",
                        optional(any_object_prop(
                            "Network configuration for the joining member",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["partyId"],
            ),
        },
        // 59. mp_leave_party
        super::ToolSpec {
            name: "mp_leave_party".to_string(),
            description: "Leave a Party network".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "LeaveParty".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LeaveParty parameters",
                vec![
                    ("partyId", string_prop("The ID of the party to leave")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["partyId"],
            ),
        },
        // =====================================================================
        // Additional (~9 tools)
        // =====================================================================
        // 60. mp_list_server_backfill_tickets_for_player
        super::ToolSpec {
            name: "mp_list_server_backfill_tickets_for_player".to_string(),
            description: "List server backfill tickets for a specific player".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListServerBackfillTicketsForPlayer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListServerBackfillTicketsForPlayer parameters",
                vec![
                    ("entity", any_object_prop("The entity key of the player")),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "queueName"],
            ),
        },
        // 61. mp_create_server_backfill_ticket
        super::ToolSpec {
            name: "mp_create_server_backfill_ticket".to_string(),
            description: "Create a server backfill ticket to find additional players for a match"
                .to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CreateServerBackfillTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateServerBackfillTicket parameters",
                vec![
                    (
                        "giveUpAfterSeconds",
                        integer_range_prop(
                            "How long to attempt backfill in seconds",
                            Some(1),
                            Some(600),
                        ),
                    ),
                    (
                        "members",
                        object_array_prop(
                            "Current members in the match",
                            any_object_prop("Member with Entity and Team"),
                        ),
                    ),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "serverDetails",
                        optional(any_object_prop("Server details including IP and ports")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["giveUpAfterSeconds", "members", "queueName"],
            ),
        },
        // 62. mp_get_server_backfill_ticket
        super::ToolSpec {
            name: "mp_get_server_backfill_ticket".to_string(),
            description: "Get the status and details of a server backfill ticket".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetServerBackfillTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetServerBackfillTicket parameters",
                vec![
                    ("ticketId", string_prop("The ID of the backfill ticket")),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "escapeObject",
                        optional(boolean_prop("Escape the JSON object in the response")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ticketId", "queueName"],
            ),
        },
        // 63. mp_cancel_server_backfill_ticket
        super::ToolSpec {
            name: "mp_cancel_server_backfill_ticket".to_string(),
            description: "Cancel a specific server backfill ticket".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CancelServerBackfillTicket".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CancelServerBackfillTicket parameters",
                vec![
                    (
                        "ticketId",
                        string_prop("The ID of the backfill ticket to cancel"),
                    ),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ticketId", "queueName"],
            ),
        },
        // 64. mp_cancel_all_server_backfill_tickets_for_player
        super::ToolSpec {
            name: "mp_cancel_all_server_backfill_tickets_for_player".to_string(),
            description: "Cancel all server backfill tickets for a specific player".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "CancelAllServerBackfillTicketsForPlayer".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CancelAllServerBackfillTicketsForPlayer parameters",
                vec![
                    ("entity", any_object_prop("The entity key of the player")),
                    (
                        "queueName",
                        string_prop("The name of the matchmaking queue"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "queueName"],
            ),
        },
        // 65. mp_list_archived_multiplayer_servers
        super::ToolSpec {
            name: "mp_list_archived_multiplayer_servers".to_string(),
            description: "List archived multiplayer servers for a build in a region".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListArchivedMultiplayerServers".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListArchivedMultiplayerServers parameters",
                vec![
                    ("buildId", string_prop("The ID of the build")),
                    (
                        "region",
                        string_prop("The region to list archived servers for"),
                    ),
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["buildId", "region"],
            ),
        },
        // 66. mp_get_multiplayer_server_logs
        super::ToolSpec {
            name: "mp_get_multiplayer_server_logs".to_string(),
            description: "Get logs for a multiplayer server session".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetMultiplayerServerLogs".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMultiplayerServerLogs parameters",
                vec![
                    ("serverId", string_prop("The ID of the multiplayer server")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["serverId"],
            ),
        },
        // 67. mp_list_build_summaries_v2
        super::ToolSpec {
            name: "mp_list_build_summaries_v2".to_string(),
            description: "List build summaries with enhanced v2 response format".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "ListBuildSummariesV2".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListBuildSummariesV2 parameters",
                vec![
                    (
                        "pageSize",
                        optional(integer_range_prop(
                            "Number of results per page",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "skipToken",
                        optional(string_prop("Skip token for pagination")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 68. mp_get_title_multiplayer_servers_quotas
        super::ToolSpec {
            name: "mp_get_title_multiplayer_servers_quotas".to_string(),
            description: "Get the quota limits and usage for multiplayer servers".to_string(),
            api_group: "Multiplayer".to_string(),
            api_method: "GetTitleMultiplayerServersQuotas".to_string(),
            category: super::ToolCategory::Multiplayer,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTitleMultiplayerServersQuotas parameters",
                vec![(
                    "customTags",
                    optional(any_object_prop("Custom tags for the request")),
                )],
                vec![],
            ),
        },
    ]
}
