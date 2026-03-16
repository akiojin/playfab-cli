// Groups API tools: CreateGroup, ListGroupMembers, Roles, Invitations, etc.
use crate::tooling::schema_builder::*;

/// Helper: entity key schema (Id + Type)
fn entity_key_prop(description: &str) -> serde_json::Value {
    object_schema(
        description,
        vec![
            ("Id", string_prop("Entity ID")),
            (
                "Type",
                string_prop("Entity type (e.g. title_player_account, master_player_account)"),
            ),
        ],
        vec!["Id", "Type"],
    )
}

/// Helper: group entity key (always entity_group type)
fn group_key_prop(description: &str) -> serde_json::Value {
    object_schema(
        description,
        vec![
            ("Id", string_prop("Group entity ID")),
            ("Type", string_prop("Entity type (typically 'group')")),
        ],
        vec!["Id", "Type"],
    )
}

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. create_group
        super::ToolSpec {
            name: "create_group".to_string(),
            description: "Create a new group".to_string(),
            api_group: "Group".to_string(),
            api_method: "CreateGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateGroup parameters",
                vec![
                    ("groupName", string_prop("The name of the group to create")),
                    (
                        "entity",
                        optional(entity_key_prop("The entity to create the group for")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["groupName"],
            ),
        },
        // 2. delete_group
        super::ToolSpec {
            name: "delete_group".to_string(),
            description: "Delete a group and all roles, invitations, join requests, and blocks"
                .to_string(),
            api_group: "Group".to_string(),
            api_method: "DeleteGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteGroup parameters",
                vec![
                    (
                        "group",
                        group_key_prop("The identifier of the group to delete"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 3. get_group
        super::ToolSpec {
            name: "get_group".to_string(),
            description: "Get information about a group by group key or group name".to_string(),
            api_group: "Group".to_string(),
            api_method: "GetGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetGroup parameters",
                vec![
                    (
                        "group",
                        optional(group_key_prop("The identifier of the group")),
                    ),
                    (
                        "groupName",
                        optional(string_prop("The name of the group to find")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 4. update_group
        super::ToolSpec {
            name: "update_group".to_string(),
            description: "Update the group name and other properties".to_string(),
            api_group: "Group".to_string(),
            api_method: "UpdateGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateGroup parameters",
                vec![
                    (
                        "group",
                        group_key_prop("The identifier of the group to update"),
                    ),
                    (
                        "groupName",
                        optional(string_prop("The new name for the group")),
                    ),
                    (
                        "expectedProfileVersion",
                        optional(integer_prop(
                            "Expected profile version for concurrency control",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 5. list_group_members
        super::ToolSpec {
            name: "list_group_members".to_string(),
            description: "List all members of a group across all roles".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListGroupMembers".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListGroupMembers parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 6. list_group_applications
        super::ToolSpec {
            name: "list_group_applications".to_string(),
            description: "List all outstanding requests to join the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListGroupApplications".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListGroupApplications parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 7. list_group_invitations
        super::ToolSpec {
            name: "list_group_invitations".to_string(),
            description: "List all outstanding invitations for the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListGroupInvitations".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListGroupInvitations parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 8. list_group_blocks
        super::ToolSpec {
            name: "list_group_blocks".to_string(),
            description: "List all entities blocked from joining the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListGroupBlocks".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListGroupBlocks parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 9. list_membership_opportunities
        super::ToolSpec {
            name: "list_membership_opportunities".to_string(),
            description:
                "List all groups and roles that an entity has been invited to or applied to"
                    .to_string(),
            api_group: "Group".to_string(),
            api_method: "ListMembershipOpportunities".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListMembershipOpportunities parameters",
                vec![
                    (
                        "entity",
                        optional(entity_key_prop(
                            "The entity to check membership opportunities for",
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
        // 10. apply_to_group
        super::ToolSpec {
            name: "apply_to_group".to_string(),
            description: "Apply to join a group (creates a pending application)".to_string(),
            api_group: "Group".to_string(),
            api_method: "ApplyToGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ApplyToGroup parameters",
                vec![
                    (
                        "group",
                        group_key_prop("The identifier of the group to apply to"),
                    ),
                    (
                        "entity",
                        optional(entity_key_prop("The entity applying to join")),
                    ),
                    (
                        "autoAcceptOutstandingInvitation",
                        optional(boolean_prop(
                            "Automatically accept an outstanding invitation if one exists",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 11. accept_group_application
        super::ToolSpec {
            name: "accept_group_application".to_string(),
            description: "Accept an outstanding application to join the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "AcceptGroupApplication".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AcceptGroupApplication parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        entity_key_prop("The entity whose application to accept"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 12. accept_group_invitation
        super::ToolSpec {
            name: "accept_group_invitation".to_string(),
            description: "Accept an invitation to join a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "AcceptGroupInvitation".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AcceptGroupInvitation parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        optional(entity_key_prop("The entity accepting the invitation")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
        // 13. invite_to_group
        super::ToolSpec {
            name: "invite_to_group".to_string(),
            description: "Invite an entity to join the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "InviteToGroup".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "InviteToGroup parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        entity_key_prop("The entity to invite to the group"),
                    ),
                    (
                        "roleId",
                        optional(string_prop("The role to assign the entity upon joining")),
                    ),
                    (
                        "autoAcceptOutstandingApplication",
                        optional(boolean_prop(
                            "Automatically accept an outstanding application if one exists",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 14. remove_group_application
        super::ToolSpec {
            name: "remove_group_application".to_string(),
            description: "Remove an outstanding application to join the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "RemoveGroupApplication".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveGroupApplication parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        entity_key_prop("The entity whose application to remove"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 15. remove_group_invitation
        super::ToolSpec {
            name: "remove_group_invitation".to_string(),
            description: "Remove an outstanding invitation to join the group".to_string(),
            api_group: "Group".to_string(),
            api_method: "RemoveGroupInvitation".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveGroupInvitation parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        entity_key_prop("The entity whose invitation to remove"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 16. remove_members
        super::ToolSpec {
            name: "remove_members".to_string(),
            description: "Remove members from a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "RemoveMembers".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "RemoveMembers parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "members",
                        object_array_prop(
                            "Array of entity keys to remove from the group",
                            entity_key_prop("Entity to remove"),
                        ),
                    ),
                    (
                        "roleId",
                        optional(string_prop("The role to remove the members from")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "members"],
            ),
        },
        // 17. block_entity
        super::ToolSpec {
            name: "block_entity".to_string(),
            description: "Block an entity from joining a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "BlockEntity".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "BlockEntity parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    ("entity", entity_key_prop("The entity to block")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 18. unblock_entity
        super::ToolSpec {
            name: "unblock_entity".to_string(),
            description: "Unblock an entity from joining a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "UnblockEntity".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UnblockEntity parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    ("entity", entity_key_prop("The entity to unblock")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 19. change_member_role
        super::ToolSpec {
            name: "change_member_role".to_string(),
            description: "Change the role of members within a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "ChangeMemberRole".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ChangeMemberRole parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "members",
                        object_array_prop(
                            "Array of entity keys whose role to change",
                            entity_key_prop("Entity to change role for"),
                        ),
                    ),
                    (
                        "originRoleId",
                        string_prop("The current role ID of the members"),
                    ),
                    (
                        "destinationRoleId",
                        string_prop("The target role ID to move the members to"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "members", "originRoleId", "destinationRoleId"],
            ),
        },
        // 20. is_member
        super::ToolSpec {
            name: "is_member".to_string(),
            description: "Check whether an entity is a member of a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "IsMember".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "IsMember parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "entity",
                        entity_key_prop("The entity to check membership for"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "entity"],
            ),
        },
        // 21. list_membership
        super::ToolSpec {
            name: "list_membership".to_string(),
            description: "List all groups and roles that an entity is a member of".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListMembership".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListMembership parameters",
                vec![
                    (
                        "entity",
                        optional(entity_key_prop("The entity to list memberships for")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 22. create_role
        super::ToolSpec {
            name: "create_role".to_string(),
            description: "Create a new role within a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "CreateRole".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "CreateRole parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    ("roleId", string_prop("The ID of the role to create")),
                    ("roleName", string_prop("The display name of the role")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "roleId", "roleName"],
            ),
        },
        // 23. delete_role
        super::ToolSpec {
            name: "delete_role".to_string(),
            description: "Delete a role from a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "DeleteRole".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteRole parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    ("roleId", string_prop("The ID of the role to delete")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "roleId"],
            ),
        },
        // 24. update_role
        super::ToolSpec {
            name: "update_role".to_string(),
            description: "Update properties of a role within a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "UpdateRole".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "UpdateRole parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    ("roleId", string_prop("The ID of the role to update")),
                    (
                        "roleName",
                        optional(string_prop("The new display name for the role")),
                    ),
                    (
                        "expectedProfileVersion",
                        optional(integer_prop(
                            "Expected profile version for concurrency control",
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group", "roleId"],
            ),
        },
        // 25. list_group_roles
        super::ToolSpec {
            name: "list_group_roles".to_string(),
            description: "List all roles within a group".to_string(),
            api_group: "Group".to_string(),
            api_method: "ListGroupRoles".to_string(),
            category: super::ToolCategory::Groups,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "ListGroupRoles parameters",
                vec![
                    ("group", group_key_prop("The identifier of the group")),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["group"],
            ),
        },
    ]
}
