// Server API tools: Authentication, Player Management, Statistics, Title Data, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // =====================================================================
        // Authentication (~5 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_authenticate_session_ticket".to_string(),
            description: "Validates a user's session ticket and returns the user's PlayFab ID"
                .to_string(),
            api_group: "Server".to_string(),
            api_method: "AuthenticateSessionTicket".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AuthenticateSessionTicket parameters",
                vec![(
                    "SessionTicket",
                    string_prop("The client session ticket to authenticate"),
                )],
                vec!["SessionTicket"],
            ),
        },
        super::ToolSpec {
            name: "server_login_with_server_custom_id".to_string(),
            description: "Log in a player using a server-assigned custom identifier".to_string(),
            api_group: "Server".to_string(),
            api_method: "LoginWithServerCustomId".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LoginWithServerCustomId parameters",
                vec![
                    (
                        "ServerCustomId",
                        string_prop("Server-assigned custom identifier for the player"),
                    ),
                    (
                        "CreateAccount",
                        optional(boolean_prop(
                            "Whether to create an account if one does not exist",
                        )),
                    ),
                    (
                        "InfoRequestParameters",
                        optional(any_object_prop("Player info request parameters")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ServerCustomId"],
            ),
        },
        super::ToolSpec {
            name: "server_login_with_xbox".to_string(),
            description: "Log in a player using an Xbox Live token".to_string(),
            api_group: "Server".to_string(),
            api_method: "LoginWithXbox".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LoginWithXbox parameters",
                vec![
                    (
                        "XboxToken",
                        string_prop("Xbox Live token for authentication"),
                    ),
                    (
                        "CreateAccount",
                        optional(boolean_prop(
                            "Whether to create an account if one does not exist",
                        )),
                    ),
                    (
                        "InfoRequestParameters",
                        optional(any_object_prop("Player info request parameters")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["XboxToken"],
            ),
        },
        super::ToolSpec {
            name: "server_login_with_xbox_id".to_string(),
            description: "Log in a player using an Xbox ID and Sandbox".to_string(),
            api_group: "Server".to_string(),
            api_method: "LoginWithXboxId".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LoginWithXboxId parameters",
                vec![
                    ("XboxId", string_prop("Xbox user ID")),
                    ("Sandbox", string_prop("Xbox Live sandbox name")),
                    (
                        "CreateAccount",
                        optional(boolean_prop(
                            "Whether to create an account if one does not exist",
                        )),
                    ),
                    (
                        "InfoRequestParameters",
                        optional(any_object_prop("Player info request parameters")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["XboxId", "Sandbox"],
            ),
        },
        super::ToolSpec {
            name: "server_set_player_secret".to_string(),
            description: "Set the player secret used for sign-in with server custom ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetPlayerSecret".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetPlayerSecret parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "PlayerSecret",
                        optional(string_prop(
                            "Player secret to set (leave empty to generate)",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        // =====================================================================
        // Account Management (~10 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_ban_users".to_string(),
            description: "Ban one or more players from the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "BanUsers".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "BanUsers parameters",
                vec![(
                    "Bans",
                    object_array_prop(
                        "Array of ban requests",
                        object_schema(
                            "Ban request",
                            vec![
                                ("PlayFabId", string_prop("PlayFab ID of the player to ban")),
                                ("Reason", optional(string_prop("Reason for the ban"))),
                                (
                                    "DurationInHours",
                                    optional(integer_prop(
                                        "Duration of the ban in hours (omit for permanent)",
                                    )),
                                ),
                                ("IPAddress", optional(string_prop("IP address to ban"))),
                            ],
                            vec!["PlayFabId"],
                        ),
                    ),
                )],
                vec!["Bans"],
            ),
        },
        super::ToolSpec {
            name: "server_get_player_profile".to_string(),
            description: "Retrieve a player's profile information".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerProfile".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerProfile parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ProfileConstraints",
                        optional(any_object_prop(
                            "Profile constraints to limit the fields returned",
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_account_info".to_string(),
            description: "Retrieve the user's full account information by PlayFab ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserAccountInfo".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserAccountInfo parameters",
                vec![(
                    "PlayFabId",
                    string_prop("PlayFab unique identifier of the player"),
                )],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_facebook_id".to_string(),
            description: "Retrieve PlayFab ID from a Facebook ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromFacebookIDs".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromFacebookIDs parameters",
                vec![(
                    "FacebookIDs",
                    string_array_prop("Array of Facebook IDs to look up"),
                )],
                vec!["FacebookIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_steam_id".to_string(),
            description: "Retrieve PlayFab ID from a Steam ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromSteamIDs".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromSteamIDs parameters",
                vec![(
                    "SteamStringIDs",
                    string_array_prop("Array of Steam IDs to look up"),
                )],
                vec!["SteamStringIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_xbox_live_id".to_string(),
            description: "Retrieve PlayFab ID from an Xbox Live ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromXboxLiveIDs".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromXboxLiveIDs parameters",
                vec![
                    (
                        "XboxLiveAccountIDs",
                        string_array_prop("Array of Xbox Live account IDs to look up"),
                    ),
                    ("Sandbox", optional(string_prop("Xbox Live sandbox name"))),
                ],
                vec!["XboxLiveAccountIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_nintendo_service_account_id".to_string(),
            description: "Retrieve PlayFab ID from a Nintendo Service Account ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromNintendoServiceAccountIds".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromNintendoServiceAccountIds parameters",
                vec![(
                    "NintendoAccountIds",
                    string_array_prop("Array of Nintendo Service Account IDs to look up"),
                )],
                vec!["NintendoAccountIds"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_psn_account_id".to_string(),
            description: "Retrieve PlayFab ID from a PSN Account ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromPSNAccountIDs".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromPSNAccountIDs parameters",
                vec![
                    (
                        "PSNAccountIDs",
                        string_array_prop("Array of PSN account IDs to look up"),
                    ),
                    ("IssuerId", optional(integer_prop("Issuer ID for PSN"))),
                ],
                vec!["PSNAccountIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_get_play_fab_id_from_google_id".to_string(),
            description: "Retrieve PlayFab ID from a Google account ID".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayFabIDsFromGoogleIDs".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayFabIDsFromGoogleIDs parameters",
                vec![(
                    "GoogleIDs",
                    string_array_prop("Array of Google account IDs to look up"),
                )],
                vec!["GoogleIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_send_push_notification".to_string(),
            description: "Send a push notification to a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "SendPushNotification".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SendPushNotification parameters",
                vec![
                    ("Recipient", string_prop("PlayFab ID of the recipient")),
                    (
                        "Message",
                        optional(string_prop("Text of the notification message")),
                    ),
                    (
                        "Subject",
                        optional(string_prop("Subject of the push notification")),
                    ),
                    (
                        "Package",
                        optional(any_object_prop(
                            "Push notification package with platform-specific payloads",
                        )),
                    ),
                    (
                        "AdvancedPlatformDelivery",
                        optional(object_array_prop(
                            "Advanced platform delivery settings",
                            any_object_prop("Platform-specific delivery options"),
                        )),
                    ),
                    (
                        "TargetPlatforms",
                        optional(string_array_prop("Target platforms for delivery")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Recipient"],
            ),
        },
        // =====================================================================
        // Player Data Management (~16 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_user_data".to_string(),
            description: "Retrieve user-specific custom data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_data".to_string(),
            description: "Update user-specific custom data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Data",
                        optional(any_object_prop("Key-value pairs to set as user data")),
                    ),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove from user data")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_internal_data".to_string(),
            description: "Retrieve internal user data (not visible to client)".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_internal_data".to_string(),
            description: "Update internal user data (not visible to client)".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Data",
                        optional(any_object_prop("Key-value pairs to set as internal data")),
                    ),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove from internal data")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_read_only_data".to_string(),
            description: "Retrieve read-only user data visible to the client".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_read_only_data".to_string(),
            description: "Update read-only user data (visible to client but not writable)"
                .to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Data",
                        optional(any_object_prop("Key-value pairs to set as read-only data")),
                    ),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove from read-only data")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_publisher_data".to_string(),
            description: "Retrieve publisher-specific custom data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserPublisherData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserPublisherData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_publisher_data".to_string(),
            description: "Update publisher-specific custom data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserPublisherData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserPublisherData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Data",
                        optional(any_object_prop("Key-value pairs to set as publisher data")),
                    ),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove from publisher data")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_publisher_internal_data".to_string(),
            description: "Retrieve publisher-specific internal data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserPublisherInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserPublisherInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_publisher_internal_data".to_string(),
            description: "Update publisher-specific internal data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserPublisherInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserPublisherInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_publisher_read_only_data".to_string(),
            description: "Retrieve publisher-specific read-only data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserPublisherReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserPublisherReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_publisher_read_only_data".to_string(),
            description: "Update publisher-specific read-only data for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserPublisherReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserPublisherReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_player_combined_info".to_string(),
            description: "Retrieve all requested data for a player in a single call".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerCombinedInfo".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerCombinedInfo parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "InfoRequestParameters",
                        any_object_prop("Flags indicating which data to return"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "InfoRequestParameters"],
            ),
        },
        super::ToolSpec {
            name: "server_get_player_statistics".to_string(),
            description: "Retrieve player statistics for a specific player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerStatistics".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerStatistics parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "StatisticNames",
                        optional(string_array_prop("Statistic names to retrieve")),
                    ),
                    (
                        "StatisticNameVersions",
                        optional(object_array_prop(
                            "Statistics with specific versions",
                            object_schema(
                                "StatisticNameVersion",
                                vec![
                                    ("StatisticName", string_prop("Name of the statistic")),
                                    ("Version", integer_prop("Version of the statistic")),
                                ],
                                vec!["StatisticName", "Version"],
                            ),
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_player_statistics".to_string(),
            description: "Update player statistics for a specific player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdatePlayerStatistics".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdatePlayerStatistics parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "Statistics",
                        object_array_prop(
                            "Array of statistics to update",
                            object_schema(
                                "StatisticUpdate",
                                vec![
                                    ("StatisticName", string_prop("Name of the statistic")),
                                    ("Value", integer_prop("New value for the statistic")),
                                    (
                                        "Version",
                                        optional(integer_prop(
                                            "Version of the statistic for concurrency",
                                        )),
                                    ),
                                ],
                                vec!["StatisticName", "Value"],
                            ),
                        ),
                    ),
                    (
                        "ForceUpdate",
                        optional(boolean_prop("Force update even if version doesn't match")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "Statistics"],
            ),
        },
        super::ToolSpec {
            name: "server_get_player_statistic_versions".to_string(),
            description: "Retrieve version information for all player statistics".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerStatisticVersions".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerStatisticVersions parameters",
                vec![
                    (
                        "StatisticName",
                        optional(string_prop("Filter to a specific statistic name")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // =====================================================================
        // Title-Wide Data (~8 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_title_data".to_string(),
            description: "Retrieve key-value data for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetTitleData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTitleData parameters",
                vec![
                    (
                        "Keys",
                        optional(string_array_prop("Specific keys to retrieve")),
                    ),
                    (
                        "OverrideLabel",
                        optional(string_prop("Override label for title data")),
                    ),
                ],
                vec![],
            ),
        },
        super::ToolSpec {
            name: "server_set_title_data".to_string(),
            description: "Set key-value data for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetTitleData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetTitleData parameters",
                vec![
                    ("Key", string_prop("Key of the title data to set")),
                    (
                        "Value",
                        optional(string_prop("Value to set (null to delete)")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Key"],
            ),
        },
        super::ToolSpec {
            name: "server_get_title_internal_data".to_string(),
            description: "Retrieve internal title data (not visible to clients)".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetTitleInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTitleInternalData parameters",
                vec![
                    (
                        "Keys",
                        optional(string_array_prop("Specific keys to retrieve")),
                    ),
                    (
                        "OverrideLabel",
                        optional(string_prop("Override label for title data")),
                    ),
                ],
                vec![],
            ),
        },
        super::ToolSpec {
            name: "server_set_title_internal_data".to_string(),
            description: "Set internal title data (not visible to clients)".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetTitleInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetTitleInternalData parameters",
                vec![
                    ("Key", string_prop("Key of the internal data to set")),
                    (
                        "Value",
                        optional(string_prop("Value to set (null to delete)")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Key"],
            ),
        },
        super::ToolSpec {
            name: "server_get_title_news".to_string(),
            description: "Retrieve title news items for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetTitleNews".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetTitleNews parameters",
                vec![(
                    "Count",
                    optional(integer_range_prop(
                        "Number of news items to return",
                        Some(1),
                        Some(100),
                    )),
                )],
                vec![],
            ),
        },
        super::ToolSpec {
            name: "server_get_catalog_items".to_string(),
            description: "Retrieve the specified version of the title's catalog (legacy v1)"
                .to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCatalogItems".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCatalogItems parameters",
                vec![(
                    "CatalogVersion",
                    optional(string_prop("Catalog version to retrieve")),
                )],
                vec![],
            ),
        },
        super::ToolSpec {
            name: "server_get_store_items".to_string(),
            description: "Retrieve store items from a specified store (legacy v1)".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetStoreItems".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetStoreItems parameters",
                vec![
                    ("StoreId", string_prop("ID of the store to retrieve")),
                    (
                        "CatalogVersion",
                        optional(string_prop("Catalog version for the store")),
                    ),
                    (
                        "PlayFabId",
                        optional(string_prop("PlayFab ID for player-specific pricing")),
                    ),
                ],
                vec!["StoreId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_publisher_data".to_string(),
            description: "Retrieve publisher-specific key-value data".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPublisherData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPublisherData parameters",
                vec![("Keys", string_array_prop("Specific keys to retrieve"))],
                vec!["Keys"],
            ),
        },
        // =====================================================================
        // Player Item Management (~12 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_user_inventory".to_string(),
            description: "Retrieve a player's inventory (legacy v1)".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserInventory".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserInventory parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CatalogVersion",
                        optional(string_prop("Catalog version for the inventory items")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_grant_items_to_user".to_string(),
            description: "Grant items to a player's inventory".to_string(),
            api_group: "Server".to_string(),
            api_method: "GrantItemsToUser".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GrantItemsToUser parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ItemIds",
                        string_array_prop("Array of catalog item IDs to grant"),
                    ),
                    (
                        "CatalogVersion",
                        optional(string_prop("Catalog version for the items")),
                    ),
                    (
                        "Annotation",
                        optional(string_prop("Annotation for the grant")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ItemIds"],
            ),
        },
        super::ToolSpec {
            name: "server_grant_items_to_users".to_string(),
            description: "Grant items to multiple players simultaneously".to_string(),
            api_group: "Server".to_string(),
            api_method: "GrantItemsToUsers".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GrantItemsToUsers parameters",
                vec![
                    (
                        "ItemGrants",
                        object_array_prop(
                            "Array of item grants",
                            object_schema(
                                "ItemGrant",
                                vec![
                                    ("PlayFabId", string_prop("PlayFab ID of the player")),
                                    ("ItemId", string_prop("Catalog item ID to grant")),
                                    ("CatalogVersion", optional(string_prop("Catalog version"))),
                                    (
                                        "Annotation",
                                        optional(string_prop("Annotation for the grant")),
                                    ),
                                    (
                                        "Data",
                                        optional(any_object_prop(
                                            "Custom data for the item instance",
                                        )),
                                    ),
                                ],
                                vec!["PlayFabId", "ItemId"],
                            ),
                        ),
                    ),
                    (
                        "CatalogVersion",
                        optional(string_prop("Default catalog version")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["ItemGrants"],
            ),
        },
        super::ToolSpec {
            name: "server_modify_item_uses".to_string(),
            description: "Modify the number of remaining uses for a player's inventory item"
                .to_string(),
            api_group: "Server".to_string(),
            api_method: "ModifyItemUses".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ModifyItemUses parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ItemInstanceId",
                        string_prop("Instance ID of the item to modify"),
                    ),
                    (
                        "UsesToAdd",
                        integer_prop("Number of uses to add (can be negative)"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ItemInstanceId", "UsesToAdd"],
            ),
        },
        super::ToolSpec {
            name: "server_move_item_to_character_from_user".to_string(),
            description: "Move an item from a player's inventory to a character".to_string(),
            api_group: "Server".to_string(),
            api_method: "MoveItemToCharacterFromUser".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "MoveItemToCharacterFromUser parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID to move item to")),
                    (
                        "ItemInstanceId",
                        string_prop("Instance ID of the item to move"),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "ItemInstanceId"],
            ),
        },
        super::ToolSpec {
            name: "server_move_item_to_user_from_character".to_string(),
            description: "Move an item from a character's inventory to the player".to_string(),
            api_group: "Server".to_string(),
            api_method: "MoveItemToUserFromCharacter".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "MoveItemToUserFromCharacter parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID to move item from")),
                    (
                        "ItemInstanceId",
                        string_prop("Instance ID of the item to move"),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "ItemInstanceId"],
            ),
        },
        super::ToolSpec {
            name: "server_consume_item".to_string(),
            description: "Consume a use of an item in a player's inventory".to_string(),
            api_group: "Server".to_string(),
            api_method: "ConsumeItem".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ConsumeItem parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ItemInstanceId",
                        string_prop("Instance ID of the item to consume"),
                    ),
                    (
                        "ConsumeCount",
                        integer_range_prop("Number of uses to consume", Some(1), None),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ItemInstanceId", "ConsumeCount"],
            ),
        },
        super::ToolSpec {
            name: "server_unlock_container_instance".to_string(),
            description: "Unlock a container item instance in a player's inventory".to_string(),
            api_group: "Server".to_string(),
            api_method: "UnlockContainerInstance".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnlockContainerInstance parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ContainerItemInstanceId",
                        string_prop("Instance ID of the container to unlock"),
                    ),
                    (
                        "KeyItemInstanceId",
                        optional(string_prop("Instance ID of the key item (if required)")),
                    ),
                    ("CatalogVersion", optional(string_prop("Catalog version"))),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ContainerItemInstanceId"],
            ),
        },
        super::ToolSpec {
            name: "server_unlock_container_item".to_string(),
            description: "Open a container item by catalog item ID for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UnlockContainerItem".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnlockContainerItem parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ContainerItemId",
                        string_prop("Catalog item ID of the container"),
                    ),
                    ("CatalogVersion", optional(string_prop("Catalog version"))),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ContainerItemId"],
            ),
        },
        super::ToolSpec {
            name: "server_redeem_coupon".to_string(),
            description: "Redeem a coupon code for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "RedeemCoupon".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemCoupon parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CouponCode", string_prop("The coupon code to redeem")),
                    ("CatalogVersion", optional(string_prop("Catalog version"))),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CouponCode"],
            ),
        },
        super::ToolSpec {
            name: "server_revoke_inventory_item".to_string(),
            description: "Revoke a single item instance from a player's inventory".to_string(),
            api_group: "Server".to_string(),
            api_method: "RevokeInventoryItem".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RevokeInventoryItem parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "ItemInstanceId",
                        string_prop("Instance ID of the item to revoke"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ItemInstanceId"],
            ),
        },
        super::ToolSpec {
            name: "server_revoke_inventory_items".to_string(),
            description: "Revoke multiple inventory item instances across players".to_string(),
            api_group: "Server".to_string(),
            api_method: "RevokeInventoryItems".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RevokeInventoryItems parameters",
                vec![
                    (
                        "Items",
                        object_array_prop(
                            "Array of items to revoke",
                            object_schema(
                                "RevokeInventoryItem",
                                vec![
                                    ("PlayFabId", string_prop("PlayFab ID of the player")),
                                    ("ItemInstanceId", string_prop("Instance ID to revoke")),
                                ],
                                vec!["PlayFabId", "ItemInstanceId"],
                            ),
                        ),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Items"],
            ),
        },
        // =====================================================================
        // Friends (~5 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_friends_list".to_string(),
            description: "Retrieve the friends list for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetFriendsList".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetFriendsList parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "IncludeFacebookFriends",
                        optional(boolean_prop("Include Facebook friends")),
                    ),
                    (
                        "IncludeSteamFriends",
                        optional(boolean_prop("Include Steam friends")),
                    ),
                    (
                        "ProfileConstraints",
                        optional(any_object_prop(
                            "Profile constraints to limit fields returned",
                        )),
                    ),
                    (
                        "XboxToken",
                        optional(string_prop("Xbox token for cross-platform friends")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_add_friend".to_string(),
            description: "Add a friend for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "AddFriend".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddFriend parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab ID of the player to add friend for"),
                    ),
                    (
                        "FriendPlayFabId",
                        optional(string_prop("PlayFab ID of the friend to add")),
                    ),
                    (
                        "FriendUsername",
                        optional(string_prop("Username of the friend to add")),
                    ),
                    (
                        "FriendEmail",
                        optional(string_prop("Email of the friend to add")),
                    ),
                    (
                        "FriendTitleDisplayName",
                        optional(string_prop("Title display name of the friend")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_remove_friend".to_string(),
            description: "Remove a friend from a player's friend list".to_string(),
            api_group: "Server".to_string(),
            api_method: "RemoveFriend".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveFriend parameters",
                vec![
                    ("PlayFabId", string_prop("PlayFab ID of the player")),
                    (
                        "FriendPlayFabId",
                        string_prop("PlayFab ID of the friend to remove"),
                    ),
                ],
                vec!["PlayFabId", "FriendPlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_set_friend_tags".to_string(),
            description: "Set tags on a friend in the player's friend list".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetFriendTags".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetFriendTags parameters",
                vec![
                    ("PlayFabId", string_prop("PlayFab ID of the player")),
                    ("FriendPlayFabId", string_prop("PlayFab ID of the friend")),
                    ("Tags", string_array_prop("Tags to set on the friend")),
                ],
                vec!["PlayFabId", "FriendPlayFabId", "Tags"],
            ),
        },
        super::ToolSpec {
            name: "server_add_shared_group_members".to_string(),
            description: "Add members to a shared group".to_string(),
            api_group: "Server".to_string(),
            api_method: "AddSharedGroupMembers".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddSharedGroupMembers parameters",
                vec![
                    ("SharedGroupId", string_prop("ID of the shared group")),
                    (
                        "PlayFabIds",
                        string_array_prop("PlayFab IDs of players to add"),
                    ),
                ],
                vec!["SharedGroupId", "PlayFabIds"],
            ),
        },
        // =====================================================================
        // Characters (~12 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_all_users_characters".to_string(),
            description: "Retrieve all characters for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetAllUsersCharacters".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetAllUsersCharacters parameters (ListUsersCharacters)",
                vec![(
                    "PlayFabId",
                    string_prop("PlayFab unique identifier of the player"),
                )],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_character_data".to_string(),
            description: "Retrieve character-specific custom data".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCharacterData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCharacterData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CharacterId",
                        string_prop("Character ID to retrieve data for"),
                    ),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_character_data".to_string(),
            description: "Update character-specific custom data".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateCharacterData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateCharacterData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CharacterId",
                        string_prop("Character ID to update data for"),
                    ),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_character_internal_data".to_string(),
            description: "Retrieve internal character data (not visible to client)".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCharacterInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCharacterInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_character_internal_data".to_string(),
            description: "Update internal character data (not visible to client)".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateCharacterInternalData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateCharacterInternalData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_character_read_only_data".to_string(),
            description: "Retrieve read-only character data visible to the client".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCharacterReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCharacterReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    (
                        "Keys",
                        optional(string_array_prop("Specific data keys to retrieve")),
                    ),
                    (
                        "IfChangedFromDataVersion",
                        optional(integer_prop(
                            "Return data only if changed from this version",
                        )),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_character_read_only_data".to_string(),
            description: "Update read-only character data".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateCharacterReadOnlyData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateCharacterReadOnlyData parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_character_statistics".to_string(),
            description: "Retrieve statistics for a character".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCharacterStatistics".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCharacterStatistics parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_character_statistics".to_string(),
            description: "Update statistics for a character".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateCharacterStatistics".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateCharacterStatistics parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    (
                        "CharacterStatistics",
                        any_object_prop("Key-value pairs of statistic names and values"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "CharacterStatistics"],
            ),
        },
        super::ToolSpec {
            name: "server_grant_character_to_user".to_string(),
            description: "Create a new character for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GrantCharacterToUser".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GrantCharacterToUser parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CharacterName",
                        string_prop("Display name of the new character"),
                    ),
                    ("CharacterType", string_prop("Type of the character")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterName", "CharacterType"],
            ),
        },
        super::ToolSpec {
            name: "server_delete_character_from_user".to_string(),
            description: "Delete a character from a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "DeleteCharacterFromUser".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteCharacterFromUser parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID to delete")),
                    (
                        "SaveCharacterInventory",
                        optional(boolean_prop(
                            "Whether to save character inventory to the player",
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_character_leaderboard".to_string(),
            description: "Retrieve a leaderboard for a character statistic".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetCharacterLeaderboard".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetCharacterLeaderboard parameters",
                vec![
                    (
                        "StatisticName",
                        string_prop("Name of the character statistic"),
                    ),
                    (
                        "StartPosition",
                        integer_range_prop("Starting position in the leaderboard", Some(0), None),
                    ),
                    (
                        "MaxResultsCount",
                        optional(integer_range_prop(
                            "Maximum number of results",
                            Some(1),
                            Some(100),
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["StatisticName", "StartPosition"],
            ),
        },
        // =====================================================================
        // Virtual Currency (~5 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_add_user_virtual_currency".to_string(),
            description: "Add virtual currency to a player's balance".to_string(),
            api_group: "Server".to_string(),
            api_method: "AddUserVirtualCurrency".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddUserVirtualCurrency parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "VirtualCurrency",
                        string_prop("Virtual currency code (e.g., 'GC')"),
                    ),
                    ("Amount", integer_range_prop("Amount to add", Some(0), None)),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "VirtualCurrency", "Amount"],
            ),
        },
        super::ToolSpec {
            name: "server_subtract_user_virtual_currency".to_string(),
            description: "Subtract virtual currency from a player's balance".to_string(),
            api_group: "Server".to_string(),
            api_method: "SubtractUserVirtualCurrency".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SubtractUserVirtualCurrency parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "VirtualCurrency",
                        string_prop("Virtual currency code (e.g., 'GC')"),
                    ),
                    (
                        "Amount",
                        integer_range_prop("Amount to subtract", Some(0), None),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "VirtualCurrency", "Amount"],
            ),
        },
        super::ToolSpec {
            name: "server_get_user_virtual_currency".to_string(),
            description: "Retrieve a player's virtual currency balances (via GetUserInventory)"
                .to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserVirtualCurrency".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserInventory parameters (for virtual currency)",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_add_character_virtual_currency".to_string(),
            description: "Add virtual currency to a character's balance".to_string(),
            api_group: "Server".to_string(),
            api_method: "AddCharacterVirtualCurrency".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddCharacterVirtualCurrency parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    ("VirtualCurrency", string_prop("Virtual currency code")),
                    ("Amount", integer_range_prop("Amount to add", Some(0), None)),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "VirtualCurrency", "Amount"],
            ),
        },
        super::ToolSpec {
            name: "server_subtract_character_virtual_currency".to_string(),
            description: "Subtract virtual currency from a character's balance".to_string(),
            api_group: "Server".to_string(),
            api_method: "SubtractCharacterVirtualCurrency".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SubtractCharacterVirtualCurrency parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    ("VirtualCurrency", string_prop("Virtual currency code")),
                    (
                        "Amount",
                        integer_range_prop("Amount to subtract", Some(0), None),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "VirtualCurrency", "Amount"],
            ),
        },
        // =====================================================================
        // PlayStream (~3 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_write_player_event".to_string(),
            description: "Write a custom PlayStream event for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "WritePlayerEvent".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "WritePlayerEvent parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("EventName", string_prop("Name of the custom event")),
                    (
                        "Body",
                        optional(any_object_prop("Event body with custom properties")),
                    ),
                    (
                        "Timestamp",
                        optional(string_prop("Timestamp of the event (ISO 8601)")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "EventName"],
            ),
        },
        super::ToolSpec {
            name: "server_write_title_event".to_string(),
            description: "Write a custom PlayStream event for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "WriteTitleEvent".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "WriteTitleEvent parameters",
                vec![
                    ("EventName", string_prop("Name of the custom event")),
                    (
                        "Body",
                        optional(any_object_prop("Event body with custom properties")),
                    ),
                    (
                        "Timestamp",
                        optional(string_prop("Timestamp of the event (ISO 8601)")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["EventName"],
            ),
        },
        super::ToolSpec {
            name: "server_write_character_event".to_string(),
            description: "Write a custom PlayStream event for a character".to_string(),
            api_group: "Server".to_string(),
            api_method: "WriteCharacterEvent".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "WriteCharacterEvent parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("CharacterId", string_prop("Character ID")),
                    ("EventName", string_prop("Name of the custom event")),
                    (
                        "Body",
                        optional(any_object_prop("Event body with custom properties")),
                    ),
                    (
                        "Timestamp",
                        optional(string_prop("Timestamp of the event (ISO 8601)")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "CharacterId", "EventName"],
            ),
        },
        // =====================================================================
        // Shared Group Data (~5 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_create_shared_group".to_string(),
            description: "Create a new shared group".to_string(),
            api_group: "Server".to_string(),
            api_method: "CreateSharedGroup".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateSharedGroup parameters",
                vec![(
                    "SharedGroupId",
                    optional(string_prop(
                        "ID for the new shared group (auto-generated if omitted)",
                    )),
                )],
                vec![],
            ),
        },
        super::ToolSpec {
            name: "server_get_shared_group_data".to_string(),
            description: "Retrieve data from a shared group".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetSharedGroupData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetSharedGroupData parameters",
                vec![
                    ("SharedGroupId", string_prop("ID of the shared group")),
                    (
                        "Keys",
                        optional(string_array_prop("Specific keys to retrieve")),
                    ),
                    (
                        "GetMembers",
                        optional(boolean_prop("Whether to include member list")),
                    ),
                ],
                vec!["SharedGroupId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_shared_group_data".to_string(),
            description: "Update data in a shared group".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateSharedGroupData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateSharedGroupData parameters",
                vec![
                    ("SharedGroupId", string_prop("ID of the shared group")),
                    ("Data", optional(any_object_prop("Key-value pairs to set"))),
                    (
                        "KeysToRemove",
                        optional(string_array_prop("Keys to remove")),
                    ),
                    (
                        "Permission",
                        optional(string_enum_prop(
                            "Permission for the data",
                            &["Private", "Public"],
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["SharedGroupId"],
            ),
        },
        super::ToolSpec {
            name: "server_remove_shared_group_members".to_string(),
            description: "Remove members from a shared group".to_string(),
            api_group: "Server".to_string(),
            api_method: "RemoveSharedGroupMembers".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveSharedGroupMembers parameters",
                vec![
                    ("SharedGroupId", string_prop("ID of the shared group")),
                    (
                        "PlayFabIds",
                        string_array_prop("PlayFab IDs of players to remove"),
                    ),
                ],
                vec!["SharedGroupId", "PlayFabIds"],
            ),
        },
        // =====================================================================
        // CloudScript (~2 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_execute_cloud_script".to_string(),
            description: "Execute a CloudScript function on behalf of a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "ExecuteCloudScript".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteCloudScript parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "FunctionName",
                        string_prop("Name of the CloudScript function"),
                    ),
                    (
                        "FunctionParameter",
                        optional(any_object_prop("Parameters to pass to the function")),
                    ),
                    (
                        "RevisionSelection",
                        optional(string_enum_prop(
                            "CloudScript revision selection",
                            &["Live", "Latest", "Specific"],
                        )),
                    ),
                    (
                        "SpecificRevision",
                        optional(integer_prop("Specific revision number")),
                    ),
                    (
                        "GeneratePlayStreamEvent",
                        optional(boolean_prop("Whether to generate a PlayStream event")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "FunctionName"],
            ),
        },
        super::ToolSpec {
            name: "server_execute_cloud_script_server".to_string(),
            description: "Execute a CloudScript function with server-level permissions".to_string(),
            api_group: "Server".to_string(),
            api_method: "ExecuteCloudScriptServer".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ExecuteCloudScript (server) parameters",
                vec![
                    (
                        "FunctionName",
                        string_prop("Name of the CloudScript function"),
                    ),
                    (
                        "PlayFabId",
                        optional(string_prop("PlayFab ID of the player context")),
                    ),
                    (
                        "FunctionParameter",
                        optional(any_object_prop("Parameters to pass to the function")),
                    ),
                    (
                        "RevisionSelection",
                        optional(string_enum_prop(
                            "CloudScript revision selection",
                            &["Live", "Latest", "Specific"],
                        )),
                    ),
                    (
                        "SpecificRevision",
                        optional(integer_prop("Specific revision number")),
                    ),
                    (
                        "GeneratePlayStreamEvent",
                        optional(boolean_prop("Whether to generate a PlayStream event")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["FunctionName"],
            ),
        },
        // =====================================================================
        // Content (~2 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_content_download_url".to_string(),
            description: "Get a URL for downloading content from the CDN".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetContentDownloadUrl".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetContentDownloadUrl parameters",
                vec![
                    ("Key", string_prop("Content key / path to retrieve")),
                    (
                        "HttpMethod",
                        optional(string_prop("HTTP method for the download URL")),
                    ),
                    ("ThruCDN", optional(boolean_prop("Whether to use the CDN"))),
                ],
                vec!["Key"],
            ),
        },
        super::ToolSpec {
            name: "server_get_time".to_string(),
            description: "Retrieve the current server time".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetTime".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema("GetTime parameters", vec![], vec![]),
        },
        // =====================================================================
        // Push Notifications (~2 additional tools beyond server_send_push_notification)
        // =====================================================================
        super::ToolSpec {
            name: "server_send_push_notification_from_template".to_string(),
            description: "Send a push notification using a predefined template".to_string(),
            api_group: "Server".to_string(),
            api_method: "SendPushNotificationFromTemplate".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SendPushNotificationFromTemplate parameters",
                vec![
                    ("Recipient", string_prop("PlayFab ID of the recipient")),
                    (
                        "PushNotificationTemplateId",
                        string_prop("ID of the push notification template"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Recipient", "PushNotificationTemplateId"],
            ),
        },
        super::ToolSpec {
            name: "server_delete_push_notification_template".to_string(),
            description: "Delete a push notification template".to_string(),
            api_group: "Server".to_string(),
            api_method: "DeletePushNotificationTemplate".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeletePushNotificationTemplate parameters",
                vec![(
                    "PushNotificationTemplateId",
                    string_prop("ID of the push notification template to delete"),
                )],
                vec!["PushNotificationTemplateId"],
            ),
        },
        // =====================================================================
        // Analytics (~5 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_player_tags".to_string(),
            description: "Retrieve tags for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerTags".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerTags parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("Namespace", optional(string_prop("Tag namespace filter"))),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_add_player_tag".to_string(),
            description: "Add a tag to a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "AddPlayerTag".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AddPlayerTag parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("TagName", string_prop("Tag to add")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "TagName"],
            ),
        },
        super::ToolSpec {
            name: "server_remove_player_tag".to_string(),
            description: "Remove a tag from a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "RemovePlayerTag".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemovePlayerTag parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("TagName", string_prop("Tag to remove")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "TagName"],
            ),
        },
        super::ToolSpec {
            name: "server_get_all_segments".to_string(),
            description: "Retrieve all player segments defined for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetAllSegments".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema("GetAllSegments parameters", vec![], vec![]),
        },
        super::ToolSpec {
            name: "server_get_players_in_segment".to_string(),
            description: "Retrieve players in a specific segment".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayersInSegment".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayersInSegment parameters",
                vec![
                    ("SegmentId", string_prop("ID of the segment")),
                    (
                        "MaxBatchSize",
                        optional(integer_range_prop(
                            "Max players per batch",
                            Some(1),
                            Some(10000),
                        )),
                    ),
                    (
                        "SecondsToLive",
                        optional(integer_prop("Seconds to keep the continuation token alive")),
                    ),
                    (
                        "ContinuationToken",
                        optional(string_prop("Continuation token for pagination")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["SegmentId"],
            ),
        },
        // =====================================================================
        // Platform-specific (~8 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_award_steam_achievement".to_string(),
            description: "Award a Steam achievement to a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "AwardSteamAchievement".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AwardSteamAchievement parameters",
                vec![(
                    "Achievements",
                    object_array_prop(
                        "Array of achievements to award",
                        object_schema(
                            "AwardSteamAchievementItem",
                            vec![
                                ("PlayFabId", string_prop("PlayFab ID of the player")),
                                (
                                    "AchievementName",
                                    string_prop("Name of the Steam achievement"),
                                ),
                                (
                                    "Result",
                                    boolean_prop(
                                        "Whether the achievement was successfully awarded",
                                    ),
                                ),
                            ],
                            vec!["PlayFabId", "AchievementName", "Result"],
                        ),
                    ),
                )],
                vec!["Achievements"],
            ),
        },
        super::ToolSpec {
            name: "server_get_random_result_tables".to_string(),
            description: "Retrieve random result tables for the title".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetRandomResultTables".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetRandomResultTables parameters",
                vec![
                    (
                        "TableIDs",
                        string_array_prop("Array of table IDs to retrieve"),
                    ),
                    (
                        "CatalogVersion",
                        optional(string_prop("Catalog version for the tables")),
                    ),
                ],
                vec!["TableIDs"],
            ),
        },
        super::ToolSpec {
            name: "server_evaluate_random_result_table".to_string(),
            description: "Evaluate a random result table and return one result".to_string(),
            api_group: "Server".to_string(),
            api_method: "EvaluateRandomResultTable".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "EvaluateRandomResultTable parameters",
                vec![
                    ("TableId", string_prop("ID of the random result table")),
                    ("CatalogVersion", optional(string_prop("Catalog version"))),
                ],
                vec!["TableId"],
            ),
        },
        super::ToolSpec {
            name: "server_link_server_custom_id".to_string(),
            description: "Link a server custom ID to a player's account".to_string(),
            api_group: "Server".to_string(),
            api_method: "LinkServerCustomId".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LinkServerCustomId parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("ServerCustomId", string_prop("Server custom ID to link")),
                    (
                        "ForceLink",
                        optional(boolean_prop(
                            "Force link even if already linked to another player",
                        )),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ServerCustomId"],
            ),
        },
        super::ToolSpec {
            name: "server_unlink_server_custom_id".to_string(),
            description: "Unlink a server custom ID from a player's account".to_string(),
            api_group: "Server".to_string(),
            api_method: "UnlinkServerCustomId".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnlinkServerCustomId parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("ServerCustomId", string_prop("Server custom ID to unlink")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "ServerCustomId"],
            ),
        },
        super::ToolSpec {
            name: "server_link_xbox_account".to_string(),
            description: "Link an Xbox Live account to a player's PlayFab account".to_string(),
            api_group: "Server".to_string(),
            api_method: "LinkXboxAccount".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "LinkXboxAccount parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    ("XboxToken", string_prop("Xbox Live token")),
                    (
                        "ForceLink",
                        optional(boolean_prop("Force link even if already linked")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "XboxToken"],
            ),
        },
        super::ToolSpec {
            name: "server_unlink_xbox_account".to_string(),
            description: "Unlink an Xbox Live account from a player's PlayFab account".to_string(),
            api_group: "Server".to_string(),
            api_method: "UnlinkXboxAccount".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnlinkXboxAccount parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        // =====================================================================
        // Additional Server APIs (~15 tools)
        // =====================================================================
        super::ToolSpec {
            name: "server_get_user_bans".to_string(),
            description: "Retrieve all bans for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetUserBans".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetUserBans parameters",
                vec![(
                    "PlayFabId",
                    string_prop("PlayFab unique identifier of the player"),
                )],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_revoke_bans".to_string(),
            description: "Revoke one or more bans for players".to_string(),
            api_group: "Server".to_string(),
            api_method: "RevokeBans".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RevokeBans parameters",
                vec![("BanIds", string_array_prop("Array of ban IDs to revoke"))],
                vec!["BanIds"],
            ),
        },
        super::ToolSpec {
            name: "server_update_bans".to_string(),
            description: "Update one or more existing player bans".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateBans".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateBans parameters",
                vec![(
                    "Bans",
                    object_array_prop(
                        "Array of ban updates",
                        object_schema(
                            "UpdateBanRequest",
                            vec![
                                ("BanId", string_prop("ID of the ban to update")),
                                ("Reason", optional(string_prop("Updated reason"))),
                                (
                                    "Expires",
                                    optional(string_prop("New expiration date (ISO 8601)")),
                                ),
                                (
                                    "Permanent",
                                    optional(boolean_prop("Whether the ban is permanent")),
                                ),
                                (
                                    "Active",
                                    optional(boolean_prop("Whether the ban is active")),
                                ),
                            ],
                            vec!["BanId"],
                        ),
                    ),
                )],
                vec!["Bans"],
            ),
        },
        super::ToolSpec {
            name: "server_delete_player".to_string(),
            description: "Permanently delete a player account and all associated data".to_string(),
            api_group: "Server".to_string(),
            api_method: "DeletePlayer".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeletePlayer parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player to delete"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_update_user_title_display_name".to_string(),
            description: "Update the display name for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "UpdateUserTitleDisplayName".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateUserTitleDisplayName parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "DisplayName",
                        string_prop("New display name for the player"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId", "DisplayName"],
            ),
        },
        super::ToolSpec {
            name: "server_get_player_segment_membership".to_string(),
            description: "Retrieve segment memberships for a player".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetPlayerSegmentMembership".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetPlayerSegments parameters",
                vec![
                    (
                        "PlayFabId",
                        string_prop("PlayFab unique identifier of the player"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_get_matchmaker_game_info".to_string(),
            description: "Retrieve information about a matchmaker game instance".to_string(),
            api_group: "Server".to_string(),
            api_method: "GetMatchmakerGameInfo".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetMatchmakerGameInfo parameters",
                vec![("LobbyId", string_prop("Lobby ID of the game instance"))],
                vec!["LobbyId"],
            ),
        },
        super::ToolSpec {
            name: "server_deregister_game".to_string(),
            description: "Deregister a game server instance from the matchmaker".to_string(),
            api_group: "Server".to_string(),
            api_method: "DeregisterGame".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeregisterGame parameters",
                vec![
                    ("LobbyId", string_prop("Lobby ID of the game to deregister")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["LobbyId"],
            ),
        },
        super::ToolSpec {
            name: "server_register_game".to_string(),
            description: "Register a new game server instance with the matchmaker".to_string(),
            api_group: "Server".to_string(),
            api_method: "RegisterGame".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RegisterGame parameters",
                vec![
                    (
                        "ServerIPV4Address",
                        string_prop("IP address of the game server"),
                    ),
                    ("ServerPort", string_prop("Port of the game server")),
                    ("Build", string_prop("Build version of the game server")),
                    (
                        "Region",
                        string_enum_prop(
                            "Server region",
                            &[
                                "USCentral",
                                "USEast",
                                "EUWest",
                                "Singapore",
                                "Japan",
                                "Brazil",
                                "Australia",
                            ],
                        ),
                    ),
                    ("GameMode", string_prop("Game mode identifier")),
                    (
                        "ServerHost",
                        optional(string_prop("Hostname of the game server")),
                    ),
                    (
                        "Tags",
                        optional(any_object_prop("Custom tags for the game server instance")),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![
                    "ServerIPV4Address",
                    "ServerPort",
                    "Build",
                    "Region",
                    "GameMode",
                ],
            ),
        },
        super::ToolSpec {
            name: "server_refresh_game_server_instance_heartbeat".to_string(),
            description: "Refresh the heartbeat for a game server instance".to_string(),
            api_group: "Server".to_string(),
            api_method: "RefreshGameServerInstanceHeartbeat".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RefreshGameServerInstanceHeartbeat parameters",
                vec![("LobbyId", string_prop("Lobby ID of the game server"))],
                vec!["LobbyId"],
            ),
        },
        super::ToolSpec {
            name: "server_redeem_matchmaker_ticket".to_string(),
            description: "Validate a matchmaker ticket and return player info".to_string(),
            api_group: "Server".to_string(),
            api_method: "RedeemMatchmakerTicket".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RedeemMatchmakerTicket parameters",
                vec![
                    ("Ticket", string_prop("Matchmaker ticket to redeem")),
                    ("LobbyId", string_prop("Lobby ID of the game")),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["Ticket", "LobbyId"],
            ),
        },
        super::ToolSpec {
            name: "server_notify_matchmaker_player_left".to_string(),
            description: "Notify the matchmaker that a player has left the game".to_string(),
            api_group: "Server".to_string(),
            api_method: "NotifyMatchmakerPlayerLeft".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "NotifyMatchmakerPlayerLeft parameters",
                vec![
                    ("LobbyId", string_prop("Lobby ID of the game")),
                    (
                        "PlayFabId",
                        string_prop("PlayFab ID of the player who left"),
                    ),
                    (
                        "CustomTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["LobbyId", "PlayFabId"],
            ),
        },
        super::ToolSpec {
            name: "server_set_game_server_instance_data".to_string(),
            description: "Set custom data for a game server instance".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetGameServerInstanceData".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetGameServerInstanceData parameters",
                vec![
                    ("LobbyId", string_prop("Lobby ID of the game server")),
                    (
                        "GameServerData",
                        string_prop("Custom data string for the instance"),
                    ),
                ],
                vec!["LobbyId", "GameServerData"],
            ),
        },
        super::ToolSpec {
            name: "server_set_game_server_instance_state".to_string(),
            description: "Set the state of a game server instance".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetGameServerInstanceState".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetGameServerInstanceState parameters",
                vec![
                    ("LobbyId", string_prop("Lobby ID of the game server")),
                    (
                        "State",
                        string_enum_prop("Server instance state", &["Open", "Closed"]),
                    ),
                ],
                vec!["LobbyId", "State"],
            ),
        },
        super::ToolSpec {
            name: "server_set_game_server_instance_tags".to_string(),
            description: "Set tags for a game server instance".to_string(),
            api_group: "Server".to_string(),
            api_method: "SetGameServerInstanceTags".to_string(),
            category: super::ToolCategory::Server,
            auth_mode: super::AuthMode::SecretKey,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetGameServerInstanceTags parameters",
                vec![
                    ("LobbyId", string_prop("Lobby ID of the game server")),
                    ("Tags", any_object_prop("Key-value pairs of tags to set")),
                ],
                vec!["LobbyId", "Tags"],
            ),
        },
    ]
}
