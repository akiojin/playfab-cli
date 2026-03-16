use crate::tooling::schema_builder::*;

#[allow(clippy::vec_init_then_push)]
pub fn tools() -> Vec<super::ToolSpec> {
    let mut t = Vec::new();

    // =========================================================================
    // Account Management
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_ban_users".into(),
        description: "Bans users by PlayFab ID with optional IP/MAC address bans".into(),
        api_group: "Admin".into(),
        api_method: "BanUsers".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Ban users parameters",
            vec![(
                "Bans",
                object_array_prop(
                    "List of ban requests",
                    object_schema(
                        "Ban request",
                        vec![
                            ("PlayFabId", string_prop("PlayFab ID of the user to ban")),
                            ("IPAddress", optional(string_prop("IP address to ban"))),
                            ("MACAddress", optional(string_prop("MAC address to ban"))),
                            ("Reason", optional(string_prop("Reason for the ban"))),
                            (
                                "DurationInHours",
                                optional(integer_prop(
                                    "Duration of the ban in hours; permanent if not set",
                                )),
                            ),
                        ],
                        vec!["PlayFabId"],
                    ),
                ),
            )],
            vec!["Bans"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_revoke_bans".into(),
        description: "Revokes one or more bans by ban ID".into(),
        api_group: "Admin".into(),
        api_method: "RevokeBans".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Revoke bans parameters",
            vec![("BanIds", string_array_prop("List of ban IDs to revoke"))],
            vec!["BanIds"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_bans".into(),
        description: "Gets all bans for a user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserBans".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user bans parameters",
            vec![("PlayFabId", string_prop("PlayFab ID of the user"))],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_bans".into(),
        description: "Updates information of a list of existing bans".into(),
        api_group: "Admin".into(),
        api_method: "UpdateBans".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update bans parameters",
            vec![(
                "Bans",
                object_array_prop(
                    "List of ban updates",
                    object_schema(
                        "Ban update",
                        vec![
                            ("BanId", string_prop("Ban ID to update")),
                            ("Reason", optional(string_prop("Updated reason"))),
                            (
                                "Expires",
                                optional(string_prop("Expiration date (ISO 8601)")),
                            ),
                            (
                                "Permanent",
                                optional(boolean_prop("Whether the ban is permanent")),
                            ),
                            (
                                "Active",
                                optional(boolean_prop("Whether the ban is currently active")),
                            ),
                        ],
                        vec!["BanId"],
                    ),
                ),
            )],
            vec!["Bans"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_account_info".into(),
        description: "Retrieves the user's PlayFab account details".into(),
        api_group: "Admin".into(),
        api_method: "GetUserAccountInfo".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user account info parameters",
            vec![
                ("PlayFabId", optional(string_prop("PlayFab ID of the user"))),
                ("Email", optional(string_prop("User email address"))),
                ("Username", optional(string_prop("PlayFab username"))),
                (
                    "TitleDisplayName",
                    optional(string_prop("Title-specific display name")),
                ),
            ],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_delete_player".into(),
        description: "Removes a user's player account from the title, deleting all associated data"
            .into(),
        api_group: "Admin".into(),
        api_method: "DeletePlayer".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Delete player parameters",
            vec![("PlayFabId", string_prop("PlayFab ID of the user to delete"))],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_player_profile".into(),
        description: "Retrieves the player's profile".into(),
        api_group: "Admin".into(),
        api_method: "GetPlayerProfile".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get player profile parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "ProfileConstraints",
                    optional(any_object_prop(
                        "Profile constraints to limit returned data",
                    )),
                ),
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_lookup_user_account_info".into(),
        description: "Retrieves the relevant details for a specified user, using a lookup by various identifiers".into(),
        api_group: "Admin".into(),
        api_method: "LookupUserAccountInfo".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Lookup user account info parameters",
            vec![
                ("PlayFabId", optional(string_prop("PlayFab ID"))),
                ("Email", optional(string_prop("User email"))),
                ("Username", optional(string_prop("PlayFab username"))),
                ("TitleDisplayName", optional(string_prop("Title display name"))),
            ],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_send_account_recovery_email".into(),
        description: "Forces an email to be sent to the registered email address for the user's account, with a link allowing the user to change the password".into(),
        api_group: "Admin".into(),
        api_method: "SendAccountRecoveryEmail".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Send account recovery email parameters",
            vec![
                ("Email", string_prop("User email address")),
                ("EmailTemplateId", optional(string_prop("Email template ID to use"))),
            ],
            vec!["Email"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_reset_password".into(),
        description: "Reset a player's password for a given title".into(),
        api_group: "Admin".into(),
        api_method: "ResetPassword".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Reset password parameters",
            vec![
                (
                    "Token",
                    string_prop(
                        "Reset token previously received from the send account recovery email API",
                    ),
                ),
                ("Password", string_prop("New password for the user")),
            ],
            vec!["Token", "Password"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_title_display_name".into(),
        description: "Updates the title-specific display name for a user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserTitleDisplayName".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update display name parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                ("DisplayName", string_prop("New display name")),
            ],
            vec!["PlayFabId", "DisplayName"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_players_in_segment".into(),
        description: "Retrieves the list of players in a given segment".into(),
        api_group: "Admin".into(),
        api_method: "GetPlayersInSegment".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get players in segment parameters",
            vec![
                ("SegmentId", string_prop("Segment ID")),
                (
                    "SecondsToLive",
                    optional(integer_prop(
                        "Number of seconds to keep the continuation token alive",
                    )),
                ),
                (
                    "MaxBatchSize",
                    optional(integer_prop("Maximum number of players to return")),
                ),
                (
                    "ContinuationToken",
                    optional(string_prop("Continuation token for paging")),
                ),
            ],
            vec!["SegmentId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_all_segments".into(),
        description: "Retrieves an array of player segment definitions".into(),
        api_group: "Admin".into(),
        api_method: "GetAllSegments".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema("Get all segments parameters", vec![], vec![]),
    });

    t.push(super::ToolSpec {
        name: "admin_delete_master_player_account".into(),
        description: "Removes a master player account entirely from all titles and deletes all associated data".into(),
        api_group: "Admin".into(),
        api_method: "DeleteMasterPlayerAccount".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Delete master player account parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the master player account")),
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_export_master_player_data".into(),
        description: "Exports all associated data of a master player account".into(),
        api_group: "Admin".into(),
        api_method: "ExportMasterPlayerData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Export master player data parameters",
            vec![(
                "PlayFabId",
                string_prop("PlayFab ID of the master player account"),
            )],
            vec!["PlayFabId"],
        ),
    });

    // =========================================================================
    // Player Data
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_get_user_data".into(),
        description: "Retrieves the title-specific custom data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_data".into(),
        description: "Updates the title-specific custom data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
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
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_internal_data".into(),
        description: "Retrieves the title-specific custom internal data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user internal data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_internal_data".into(),
        description: "Updates the title-specific custom internal data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user internal data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
                (
                    "KeysToRemove",
                    optional(string_array_prop("Keys to remove")),
                ),
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_read_only_data".into(),
        description: "Retrieves the title-specific custom read-only data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserReadOnlyData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user read only data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_read_only_data".into(),
        description: "Updates the title-specific custom read-only data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserReadOnlyData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user read only data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
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
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_publisher_data".into(),
        description: "Retrieves the publisher-specific custom data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserPublisherData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user publisher data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_publisher_data".into(),
        description: "Updates the publisher-specific custom data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserPublisherData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user publisher data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
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
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_publisher_internal_data".into(),
        description: "Retrieves the publisher-specific custom internal data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserPublisherInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user publisher internal data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_publisher_internal_data".into(),
        description: "Updates the publisher-specific custom internal data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserPublisherInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user publisher internal data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
                (
                    "KeysToRemove",
                    optional(string_array_prop("Keys to remove")),
                ),
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_publisher_read_only_data".into(),
        description: "Retrieves the publisher-specific custom read-only data for the user".into(),
        api_group: "Admin".into(),
        api_method: "GetUserPublisherReadOnlyData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user publisher read only data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
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
    });

    t.push(super::ToolSpec {
        name: "admin_update_user_publisher_read_only_data".into(),
        description: "Updates the publisher-specific custom read-only data for the user".into(),
        api_group: "Admin".into(),
        api_method: "UpdateUserPublisherReadOnlyData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update user publisher read only data parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "Data",
                    optional(any_object_prop("Key-value pairs to create/update")),
                ),
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
            ],
            vec!["PlayFabId"],
        ),
    });

    // =========================================================================
    // Title Configuration
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_get_title_data".into(),
        description: "Retrieves the key-value store of custom title settings".into(),
        api_group: "Admin".into(),
        api_method: "GetTitleData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get title data parameters",
            vec![
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
                ),
                (
                    "OverrideLabel",
                    optional(string_prop("Optional override label for title data")),
                ),
            ],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_title_data".into(),
        description: "Updates the key-value store of custom title settings".into(),
        api_group: "Admin".into(),
        api_method: "SetTitleData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set title data parameters",
            vec![
                ("Key", string_prop("Key of the data to set")),
                (
                    "Value",
                    optional(string_prop("Value to set; null removes the key")),
                ),
                (
                    "OverrideLabel",
                    optional(string_prop("Optional override label for title data")),
                ),
            ],
            vec!["Key"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_title_internal_data".into(),
        description: "Retrieves the key-value store of custom internal title settings".into(),
        api_group: "Admin".into(),
        api_method: "GetTitleInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get title internal data parameters",
            vec![
                (
                    "Keys",
                    optional(string_array_prop("Specific keys to retrieve")),
                ),
                (
                    "OverrideLabel",
                    optional(string_prop("Optional override label")),
                ),
            ],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_title_internal_data".into(),
        description: "Updates the key-value store of custom internal title settings".into(),
        api_group: "Admin".into(),
        api_method: "SetTitleInternalData".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set title internal data parameters",
            vec![
                ("Key", string_prop("Key of the data to set")),
                (
                    "Value",
                    optional(string_prop("Value to set; null removes the key")),
                ),
                (
                    "OverrideLabel",
                    optional(string_prop("Optional override label")),
                ),
            ],
            vec!["Key"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_policy".into(),
        description: "Gets the requested policy".into(),
        api_group: "Admin".into(),
        api_method: "GetPolicy".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get policy parameters",
            vec![(
                "PolicyName",
                optional(string_prop(
                    "The name of the policy to read (only 'ApiPolicy' is supported)",
                )),
            )],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_policy".into(),
        description: "Changes a policy for a title".into(),
        api_group: "Admin".into(),
        api_method: "UpdatePolicy".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update policy parameters",
            vec![
                (
                    "PolicyName",
                    string_prop(
                        "The name of the policy being updated (only 'ApiPolicy' is supported)",
                    ),
                ),
                (
                    "PolicyVersion",
                    integer_prop("The current policy version for optimistic concurrency"),
                ),
                (
                    "OverwritePolicy",
                    boolean_prop("Whether to overwrite or append to the existing policy"),
                ),
                (
                    "Statements",
                    object_array_prop(
                        "Policy statements",
                        object_schema(
                            "Policy statement",
                            vec![
                                ("Resource", string_prop("API resource")),
                                ("Action", string_prop("Action (Allow/Deny)")),
                                ("Effect", string_prop("Effect of the statement")),
                                ("Principal", string_prop("Principal")),
                                ("Comment", optional(string_prop("Comment"))),
                                (
                                    "Condition",
                                    optional(any_object_prop("Conditions for the statement")),
                                ),
                            ],
                            vec!["Resource", "Action", "Effect", "Principal"],
                        ),
                    ),
                ),
            ],
            vec![
                "PolicyName",
                "PolicyVersion",
                "OverwritePolicy",
                "Statements",
            ],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_catalog_items".into(),
        description:
            "Retrieves the specified version of the title's catalog of virtual goods (legacy v1)"
                .into(),
        api_group: "Admin".into(),
        api_method: "GetCatalogItems".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get catalog items parameters",
            vec![(
                "CatalogVersion",
                optional(string_prop(
                    "Catalog version; uses default if not specified",
                )),
            )],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_catalog_items".into(),
        description: "Creates or updates the catalog configuration for virtual goods in the specified catalog version (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "SetCatalogItems".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set catalog items parameters",
            vec![
                ("CatalogVersion", optional(string_prop("Catalog version"))),
                ("Catalog", object_array_prop(
                    "Array of catalog items",
                    any_object_prop("Catalog item definition"),
                )),
                ("SetAsDefaultCatalog", optional(boolean_prop("Whether to set this as the default catalog"))),
            ],
            vec!["Catalog"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_store_items".into(),
        description: "Retrieves the set of items defined for the specified store (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "GetStoreItems".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get store items parameters",
            vec![
                ("StoreId", string_prop("Store ID to retrieve")),
                (
                    "CatalogVersion",
                    optional(string_prop("Catalog version for the store")),
                ),
            ],
            vec!["StoreId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_store_items".into(),
        description: "Sets all items in one virtual store (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "SetStoreItems".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set store items parameters",
            vec![
                ("StoreId", string_prop("Store ID")),
                ("CatalogVersion", optional(string_prop("Catalog version"))),
                (
                    "Store",
                    object_array_prop(
                        "Array of store items",
                        any_object_prop("Store item definition"),
                    ),
                ),
                (
                    "MarketingData",
                    optional(any_object_prop("Store marketing data")),
                ),
            ],
            vec!["StoreId", "Store"],
        ),
    });

    // =========================================================================
    // Virtual Currency
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_add_virtual_currency_types".into(),
        description:
            "Adds one or more virtual currencies to the set defined for the title (legacy v1)"
                .into(),
        api_group: "Admin".into(),
        api_method: "AddVirtualCurrencyTypes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Add virtual currency types parameters",
            vec![(
                "VirtualCurrencies",
                object_array_prop(
                    "List of virtual currencies to add",
                    object_schema(
                        "Virtual currency",
                        vec![
                            ("CurrencyCode", string_prop("Currency code (2-character)")),
                            ("DisplayName", optional(string_prop("Display name"))),
                            (
                                "InitialDeposit",
                                optional(integer_prop("Initial amount granted to new users")),
                            ),
                            (
                                "RechargeRate",
                                optional(integer_prop("Rate at which currency recharges per day")),
                            ),
                            (
                                "RechargeMax",
                                optional(integer_prop(
                                    "Maximum amount of currency that can accumulate via recharge",
                                )),
                            ),
                        ],
                        vec!["CurrencyCode"],
                    ),
                ),
            )],
            vec!["VirtualCurrencies"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_list_virtual_currency_types".into(),
        description: "Retrieves the list of virtual currencies defined for the title (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "ListVirtualCurrencyTypes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema("List virtual currency types parameters", vec![], vec![]),
    });

    t.push(super::ToolSpec {
        name: "admin_remove_virtual_currency_types".into(),
        description:
            "Removes one or more virtual currencies from the set defined for the title (legacy v1)"
                .into(),
        api_group: "Admin".into(),
        api_method: "RemoveVirtualCurrencyTypes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Remove virtual currency types parameters",
            vec![(
                "VirtualCurrencies",
                object_array_prop(
                    "List of virtual currencies to remove",
                    object_schema(
                        "Virtual currency",
                        vec![("CurrencyCode", string_prop("Currency code to remove"))],
                        vec!["CurrencyCode"],
                    ),
                ),
            )],
            vec!["VirtualCurrencies"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_add_user_virtual_currency".into(),
        description: "Increments the specified virtual currency by the stated amount (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "AddUserVirtualCurrency".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Add user virtual currency parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "VirtualCurrency",
                    string_prop("Name of the virtual currency (2-character code)"),
                ),
                ("Amount", integer_prop("Amount to add")),
            ],
            vec!["PlayFabId", "VirtualCurrency", "Amount"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_subtract_user_virtual_currency".into(),
        description: "Decrements the specified virtual currency by the stated amount (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "SubtractUserVirtualCurrency".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Subtract user virtual currency parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                (
                    "VirtualCurrency",
                    string_prop("Name of the virtual currency (2-character code)"),
                ),
                ("Amount", integer_prop("Amount to subtract")),
            ],
            vec!["PlayFabId", "VirtualCurrency", "Amount"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_user_inventory".into(),
        description:
            "Retrieves the specified user's current inventory of virtual goods (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "GetUserInventory".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get user inventory parameters",
            vec![("PlayFabId", string_prop("PlayFab ID of the user"))],
            vec!["PlayFabId"],
        ),
    });

    // =========================================================================
    // CloudScript
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_update_cloud_script".into(),
        description: "Creates a new CloudScript revision and uploads source code".into(),
        api_group: "Admin".into(),
        api_method: "UpdateCloudScript".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update cloud script parameters",
            vec![
                (
                    "Files",
                    object_array_prop(
                        "List of CloudScript files",
                        object_schema(
                            "CloudScript file",
                            vec![
                                ("Filename", string_prop("Name of the file")),
                                ("FileContents", string_prop("Contents of the file")),
                            ],
                            vec!["Filename", "FileContents"],
                        ),
                    ),
                ),
                (
                    "Publish",
                    optional(boolean_prop("Whether to publish the revision immediately")),
                ),
                (
                    "DeveloperPlayFabId",
                    optional(string_prop(
                        "PlayFab ID of the developer initiating the request",
                    )),
                ),
            ],
            vec!["Files"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_cloud_script_versions".into(),
        description: "Retrieves a list of all CloudScript versions".into(),
        api_group: "Admin".into(),
        api_method: "GetCloudScriptVersions".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema("Get cloud script versions parameters", vec![], vec![]),
    });

    t.push(super::ToolSpec {
        name: "admin_get_cloud_script_revision".into(),
        description: "Gets a specific CloudScript revision".into(),
        api_group: "Admin".into(),
        api_method: "GetCloudScriptRevision".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get cloud script revision parameters",
            vec![
                ("Version", optional(integer_prop("Version number"))),
                ("Revision", optional(integer_prop("Revision number"))),
            ],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_published_revision".into(),
        description: "Sets the currently published revision of a CloudScript".into(),
        api_group: "Admin".into(),
        api_method: "SetPublishedRevision".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set published revision parameters",
            vec![
                ("Version", integer_prop("Version number")),
                ("Revision", integer_prop("Revision number")),
            ],
            vec!["Version", "Revision"],
        ),
    });

    // =========================================================================
    // Content
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_get_content_list".into(),
        description: "Retrieves the pre-authorized URL for uploading or downloading content".into(),
        api_group: "Admin".into(),
        api_method: "GetContentList".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get content list parameters",
            vec![(
                "Prefix",
                optional(string_prop("Prefix filter for content keys")),
            )],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_content_upload_url".into(),
        description: "Retrieves a pre-authorized URL for uploading content".into(),
        api_group: "Admin".into(),
        api_method: "GetContentUploadUrl".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get content upload URL parameters",
            vec![
                ("Key", string_prop("Content key")),
                (
                    "ContentType",
                    optional(string_prop("MIME type of the content")),
                ),
            ],
            vec!["Key"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_delete_content".into(),
        description: "Deletes a content file from the title".into(),
        api_group: "Admin".into(),
        api_method: "DeleteContent".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Delete content parameters",
            vec![("Key", string_prop("Content key to delete"))],
            vec!["Key"],
        ),
    });

    // =========================================================================
    // Scheduled Tasks
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_create_actions_on_players_in_segment_task".into(),
        description: "Creates a task that performs actions on players within a segment".into(),
        api_group: "Admin".into(),
        api_method: "CreateActionsOnPlayersInSegmentTask".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Create actions on players in segment task parameters",
            vec![
                ("Name", string_prop("Name of the task")),
                (
                    "Description",
                    optional(string_prop("Description of the task")),
                ),
                (
                    "IsActive",
                    optional(boolean_prop("Whether the task is active")),
                ),
                (
                    "Schedule",
                    optional(string_prop("Cron expression for the schedule")),
                ),
                (
                    "Parameter",
                    object_schema(
                        "Task action parameter",
                        vec![
                            ("SegmentId", string_prop("Segment ID to target")),
                            ("ActionId", string_prop("ID of the action to perform")),
                        ],
                        vec!["SegmentId", "ActionId"],
                    ),
                ),
            ],
            vec!["Name", "Parameter"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_actions_on_players_in_segment_task_instance".into(),
        description: "Retrieves information about a specific task instance".into(),
        api_group: "Admin".into(),
        api_method: "GetActionsOnPlayersInSegmentTaskInstance".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get task instance parameters",
            vec![("TaskInstanceId", string_prop("ID of the task instance"))],
            vec!["TaskInstanceId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_tasks".into(),
        description: "Gets the list of all configured tasks, or a specific task by identifier"
            .into(),
        api_group: "Admin".into(),
        api_method: "GetTasks".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get tasks parameters",
            vec![(
                "Identifier",
                optional(object_schema(
                    "Task name identifier",
                    vec![
                        ("Name", optional(string_prop("Task name"))),
                        ("Id", optional(string_prop("Task ID"))),
                    ],
                    vec![],
                )),
            )],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_run_task".into(),
        description: "Runs a task immediately".into(),
        api_group: "Admin".into(),
        api_method: "RunTask".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Run task parameters",
            vec![(
                "Identifier",
                object_schema(
                    "Task identifier",
                    vec![
                        ("Name", optional(string_prop("Task name"))),
                        ("Id", optional(string_prop("Task ID"))),
                    ],
                    vec![],
                ),
            )],
            vec!["Identifier"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_abort_task_instance".into(),
        description: "Aborts a running task instance".into(),
        api_group: "Admin".into(),
        api_method: "AbortTaskInstance".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Abort task instance parameters",
            vec![(
                "TaskInstanceId",
                string_prop("ID of the task instance to abort"),
            )],
            vec!["TaskInstanceId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_task".into(),
        description: "Updates an existing task".into(),
        api_group: "Admin".into(),
        api_method: "UpdateTask".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update task parameters",
            vec![
                (
                    "Identifier",
                    optional(object_schema(
                        "Task identifier",
                        vec![
                            ("Name", optional(string_prop("Task name"))),
                            ("Id", optional(string_prop("Task ID"))),
                        ],
                        vec![],
                    )),
                ),
                ("Name", string_prop("Name of the task")),
                (
                    "Description",
                    optional(string_prop("Description of the task")),
                ),
                (
                    "IsActive",
                    optional(boolean_prop("Whether the task is active")),
                ),
                (
                    "Schedule",
                    optional(string_prop("Cron expression for the schedule")),
                ),
                (
                    "Type",
                    optional(string_enum_prop(
                        "Task type",
                        &[
                            "ActionsOnPlayerSegment",
                            "CloudScript",
                            "InsightsScheduledScaling",
                        ],
                    )),
                ),
                (
                    "Parameter",
                    optional(any_object_prop("Task-specific parameter object")),
                ),
            ],
            vec!["Name"],
        ),
    });

    // =========================================================================
    // Title-Wide Data
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_get_random_result_tables".into(),
        description: "Retrieves the random drop table configuration for the title".into(),
        api_group: "Admin".into(),
        api_method: "GetRandomResultTables".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get random result tables parameters",
            vec![("CatalogVersion", optional(string_prop("Catalog version")))],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_random_result_tables".into(),
        description: "Updates the random drop table configuration for the title".into(),
        api_group: "Admin".into(),
        api_method: "UpdateRandomResultTables".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update random result tables parameters",
            vec![
                ("CatalogVersion", optional(string_prop("Catalog version"))),
                (
                    "Tables",
                    object_array_prop(
                        "Array of random result tables to update",
                        object_schema(
                            "Random result table",
                            vec![
                                ("TableId", string_prop("Table ID")),
                                (
                                    "Nodes",
                                    object_array_prop(
                                        "Table entries",
                                        object_schema(
                                            "Result table node",
                                            vec![
                                                (
                                                    "ResultItemType",
                                                    string_enum_prop(
                                                        "Type of the result",
                                                        &["ItemId", "TableId"],
                                                    ),
                                                ),
                                                (
                                                    "ResultItem",
                                                    string_prop("ID of the item or table"),
                                                ),
                                                ("Weight", integer_prop("Weight of the entry")),
                                            ],
                                            vec!["ResultItemType", "ResultItem", "Weight"],
                                        ),
                                    ),
                                ),
                            ],
                            vec!["TableId", "Nodes"],
                        ),
                    ),
                ),
            ],
            vec!["Tables"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_data_report".into(),
        description: "Retrieves a download URL for the requested report".into(),
        api_group: "Admin".into(),
        api_method: "GetDataReport".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get data report parameters",
            vec![
                ("ReportName", string_prop("Name of the report")),
                ("Year", integer_prop("Year of the report")),
                ("Month", integer_prop("Month of the report")),
                ("Day", integer_prop("Day of the report")),
            ],
            vec!["ReportName", "Year", "Month", "Day"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_player_statistics_definitions".into(),
        description:
            "Retrieves the configuration information for all player statistics defined in the title"
                .into(),
        api_group: "Admin".into(),
        api_method: "GetPlayerStatisticsDefinitions".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get player statistics definitions parameters",
            vec![],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_create_player_statistic_definition".into(),
        description: "Adds a new player statistic configuration to the title".into(),
        api_group: "Admin".into(),
        api_method: "CreatePlayerStatisticDefinition".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Create player statistic definition parameters",
            vec![
                ("StatisticName", string_prop("Name of the statistic")),
                (
                    "VersionChangeInterval",
                    optional(string_enum_prop(
                        "Interval for version changes",
                        &["Never", "Hour", "Day", "Week", "Month"],
                    )),
                ),
                (
                    "AggregationMethod",
                    optional(string_enum_prop(
                        "Aggregation method",
                        &["Last", "Min", "Max", "Sum"],
                    )),
                ),
            ],
            vec!["StatisticName"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_update_player_statistic_definition".into(),
        description: "Updates an existing player statistic definition".into(),
        api_group: "Admin".into(),
        api_method: "UpdatePlayerStatisticDefinition".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Update player statistic definition parameters",
            vec![
                ("StatisticName", string_prop("Name of the statistic")),
                (
                    "VersionChangeInterval",
                    optional(string_enum_prop(
                        "Interval for version changes",
                        &["Never", "Hour", "Day", "Week", "Month"],
                    )),
                ),
                (
                    "AggregationMethod",
                    optional(string_enum_prop(
                        "Aggregation method",
                        &["Last", "Min", "Max", "Sum"],
                    )),
                ),
            ],
            vec!["StatisticName"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_increment_player_statistic_version".into(),
        description: "Resets the indicated statistic, removing all player entries for it and backing up the old values".into(),
        api_group: "Admin".into(),
        api_method: "IncrementPlayerStatisticVersion".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Increment player statistic version parameters",
            vec![
                ("StatisticName", string_prop("Name of the statistic")),
            ],
            vec!["StatisticName"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_player_statistic_versions".into(),
        description:
            "Retrieves the information on the available versions of the specified statistic".into(),
        api_group: "Admin".into(),
        api_method: "GetPlayerStatisticVersions".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get player statistic versions parameters",
            vec![("StatisticName", string_prop("Name of the statistic"))],
            vec!["StatisticName"],
        ),
    });

    // =========================================================================
    // Push Notifications
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_send_push_notification".into(),
        description: "Sends a push notification to a specific user".into(),
        api_group: "Admin".into(),
        api_method: "SendPushNotification".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Send push notification parameters",
            vec![
                ("Recipient", string_prop("PlayFab ID of the recipient")),
                (
                    "Message",
                    optional(string_prop("Text of the notification message")),
                ),
                (
                    "Subject",
                    optional(string_prop("Subject of the notification")),
                ),
                (
                    "Package",
                    optional(any_object_prop(
                        "Push notification package (platform-specific)",
                    )),
                ),
                (
                    "TargetPlatforms",
                    optional(string_array_prop("Target platforms for the notification")),
                ),
            ],
            vec!["Recipient"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_send_push_notification_from_template".into(),
        description: "Sends a push notification to a specific user using a push notification template".into(),
        api_group: "Admin".into(),
        api_method: "SendPushNotificationFromTemplate".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Send push notification from template parameters",
            vec![
                ("Recipient", string_prop("PlayFab ID of the recipient")),
                ("PushNotificationTemplateId", string_prop("ID of the push notification template")),
            ],
            vec!["Recipient", "PushNotificationTemplateId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_push_notification".into(),
        description: "Sets the title push notification settings (for a title)".into(),
        api_group: "Admin".into(),
        api_method: "SetPushNotification".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set push notification parameters",
            vec![
                ("Name", string_prop("Name of the notification")),
                (
                    "Platform",
                    string_prop("Platform for the push notification"),
                ),
                (
                    "Key",
                    optional(string_prop("Credential key for the notification platform")),
                ),
                (
                    "OverwriteOldARN",
                    optional(boolean_prop("Whether to overwrite old ARN")),
                ),
                (
                    "Credential",
                    optional(string_prop("Credential for the notification platform")),
                ),
            ],
            vec!["Name", "Platform"],
        ),
    });

    // =========================================================================
    // Player Item Management
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_grant_items_to_users".into(),
        description: "Adds the specified items to the specified user inventories (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "GrantItemsToUsers".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Grant items to users parameters",
            vec![
                (
                    "CatalogVersion",
                    optional(string_prop(
                        "Catalog version from which items are to be granted",
                    )),
                ),
                (
                    "ItemGrants",
                    object_array_prop(
                        "Array of items to grant and the users to whom they are to be granted",
                        object_schema(
                            "Item grant",
                            vec![
                                ("PlayFabId", string_prop("PlayFab ID of the user")),
                                ("ItemId", string_prop("Catalog item ID to grant")),
                                (
                                    "Annotation",
                                    optional(string_prop("String for the annotation of the grant")),
                                ),
                                (
                                    "CharacterId",
                                    optional(string_prop("Character ID to grant to")),
                                ),
                                (
                                    "Data",
                                    optional(any_object_prop("Key-value pairs for custom data")),
                                ),
                                (
                                    "KeysToRemove",
                                    optional(string_array_prop("Keys to remove from custom data")),
                                ),
                            ],
                            vec!["PlayFabId", "ItemId"],
                        ),
                    ),
                ),
            ],
            vec!["ItemGrants"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_revoke_inventory_item".into(),
        description: "Revokes access to an item in a user's inventory (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "RevokeInventoryItem".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Revoke inventory item parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                ("ItemInstanceId", string_prop("Item instance ID to revoke")),
                (
                    "CharacterId",
                    optional(string_prop("Character ID that owns the item")),
                ),
            ],
            vec!["PlayFabId", "ItemInstanceId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_revoke_inventory_items".into(),
        description: "Revokes access to a list of item instances across multiple users (legacy v1)"
            .into(),
        api_group: "Admin".into(),
        api_method: "RevokeInventoryItems".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Revoke inventory items parameters",
            vec![(
                "Items",
                object_array_prop(
                    "Array of items to revoke",
                    object_schema(
                        "Revoke inventory item",
                        vec![
                            ("PlayFabId", string_prop("PlayFab ID of the user")),
                            ("ItemInstanceId", string_prop("Item instance ID")),
                            ("CharacterId", optional(string_prop("Character ID"))),
                        ],
                        vec!["PlayFabId", "ItemInstanceId"],
                    ),
                ),
            )],
            vec!["Items"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_modify_item_uses".into(),
        description:
            "Modifies the number of remaining uses of a player's inventory item (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "ModifyItemUses".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Modify item uses parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the user")),
                ("ItemInstanceId", string_prop("Item instance ID")),
                (
                    "UsesToAdd",
                    integer_prop("Number of uses to add (can be negative to remove)"),
                ),
            ],
            vec!["PlayFabId", "ItemInstanceId", "UsesToAdd"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_check_limited_edition_item_availability".into(),
        description: "Checks the global count for the limited edition item (legacy v1)".into(),
        api_group: "Admin".into(),
        api_method: "CheckLimitedEditionItemAvailability".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Check limited edition item availability parameters",
            vec![
                ("CatalogVersion", optional(string_prop("Catalog version"))),
                ("ItemId", string_prop("Item ID to check")),
            ],
            vec!["ItemId"],
        ),
    });

    // =========================================================================
    // Authentication
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_create_player_shared_secret".into(),
        description: "Creates a new player shared secret key for the title".into(),
        api_group: "Admin".into(),
        api_method: "CreatePlayerSharedSecret".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Create player shared secret parameters",
            vec![(
                "FriendlyName",
                optional(string_prop("Friendly name for the shared secret")),
            )],
            vec![],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_delete_player_shared_secret".into(),
        description: "Deletes an existing player shared secret key".into(),
        api_group: "Admin".into(),
        api_method: "DeletePlayerSharedSecret".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Delete player shared secret parameters",
            vec![("SecretKey", string_prop("The shared secret key to delete"))],
            vec!["SecretKey"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_player_shared_secrets".into(),
        description: "Gets all player shared secret keys including disabled".into(),
        api_group: "Admin".into(),
        api_method: "GetPlayerSharedSecrets".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema("Get player shared secrets parameters", vec![], vec![]),
    });

    t.push(super::ToolSpec {
        name: "admin_set_player_secret".into(),
        description:
            "Sets or resets the player's secret. Player secrets are used to sign API requests"
                .into(),
        api_group: "Admin".into(),
        api_method: "SetPlayerSecret".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set player secret parameters",
            vec![
                ("PlayFabId", string_prop("PlayFab ID of the player")),
                (
                    "PlayerSecret",
                    optional(string_prop(
                        "Player secret to set; null generates a new one",
                    )),
                ),
            ],
            vec!["PlayFabId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_play_fab_id_from_facebook_instant_games_id".into(),
        description: "Retrieves the unique PlayFab identifiers for the given set of Facebook Instant Games identifiers".into(),
        api_group: "Admin".into(),
        api_method: "GetPlayFabIDFromFacebookInstantGamesId".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get PlayFab ID from Facebook Instant Games ID parameters",
            vec![
                ("FacebookInstantGamesIds", string_array_prop("Array of Facebook Instant Games IDs")),
            ],
            vec!["FacebookInstantGamesIds"],
        ),
    });

    // =========================================================================
    // Matchmaking
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_get_matchmaker_game_info".into(),
        description: "Retrieves the details for a specific completed session (legacy matchmaking)"
            .into(),
        api_group: "Admin".into(),
        api_method: "GetMatchmakerGameInfo".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get matchmaker game info parameters",
            vec![("LobbyId", string_prop("Lobby/game ID"))],
            vec!["LobbyId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_get_matchmaker_game_modes".into(),
        description: "Retrieves the details of defined game modes for the specified game server executable (legacy matchmaking)".into(),
        api_group: "Admin".into(),
        api_method: "GetMatchmakerGameModes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Get matchmaker game modes parameters",
            vec![
                ("BuildVersion", string_prop("Build version to query game modes for")),
            ],
            vec!["BuildVersion"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_modify_matchmaker_game_modes".into(),
        description: "Updates the game server mode details for the specified game server executable (legacy matchmaking)".into(),
        api_group: "Admin".into(),
        api_method: "ModifyMatchmakerGameModes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Modify matchmaker game modes parameters",
            vec![
                ("BuildVersion", string_prop("Build version to update")),
                ("GameModes", object_array_prop(
                    "Array of game modes",
                    object_schema(
                        "Game mode",
                        vec![
                            ("Gamemode", string_prop("Name of the game mode")),
                            ("MinPlayerCount", integer_prop("Minimum player count")),
                            ("MaxPlayerCount", integer_prop("Maximum player count")),
                            ("StartOpen", optional(boolean_prop("Whether game starts open to new players"))),
                        ],
                        vec!["Gamemode", "MinPlayerCount", "MaxPlayerCount"],
                    ),
                )),
            ],
            vec!["BuildVersion", "GameModes"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_delete_matchmaker_game_modes".into(),
        description:
            "Removes the game server executable specified from the title (legacy matchmaking)"
                .into(),
        api_group: "Admin".into(),
        api_method: "DeleteMatchmakerGameModes".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Delete matchmaker game modes parameters",
            vec![("BuildVersion", string_prop("Build version to delete"))],
            vec!["BuildVersion"],
        ),
    });

    // =========================================================================
    // Server-Side CloudScript
    // =========================================================================

    t.push(super::ToolSpec {
        name: "admin_register_game".into(),
        description: "Registers a new game server with PlayFab".into(),
        api_group: "Admin".into(),
        api_method: "RegisterGame".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Register game parameters",
            vec![
                ("Build", string_prop("Unique identifier of the build")),
                (
                    "GameMode",
                    string_prop("Game mode for the registered server"),
                ),
                ("LobbyId", optional(string_prop("Lobby ID"))),
                (
                    "Region",
                    string_enum_prop(
                        "Region where the server is located",
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
                (
                    "ServerIPV4Address",
                    optional(string_prop("IPv4 address of the server")),
                ),
                (
                    "ServerIPV6Address",
                    optional(string_prop("IPv6 address of the server")),
                ),
                ("ServerPort", optional(string_prop("Port of the server"))),
                (
                    "ServerPublicDNSName",
                    optional(string_prop("Public DNS name of the server")),
                ),
                (
                    "Tags",
                    optional(any_object_prop("Tags for the game server")),
                ),
            ],
            vec!["Build", "GameMode", "Region"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_deregister_game".into(),
        description: "Deregisters a game server that was previously registered".into(),
        api_group: "Admin".into(),
        api_method: "DeregisterGame".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Deregister game parameters",
            vec![("LobbyId", string_prop("Lobby ID of the game to deregister"))],
            vec!["LobbyId"],
        ),
    });

    t.push(super::ToolSpec {
        name: "admin_set_game_server_mode".into(),
        description:
            "Sets the mode for a game server, controlling whether it is active or inactive".into(),
        api_group: "Admin".into(),
        api_method: "SetGameServerMode".into(),
        category: super::ToolCategory::Admin,
        auth_mode: super::AuthMode::SecretKey,
        retry_mode: super::RetryMode::Standard,
        input_schema: object_schema(
            "Set game server mode parameters",
            vec![
                ("LobbyId", string_prop("Lobby ID")),
                ("GameMode", string_prop("Game mode for the server")),
            ],
            vec!["LobbyId", "GameMode"],
        ),
    });

    t
}
