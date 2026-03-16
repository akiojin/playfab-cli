// Data tools: SetObjects, GetObjects, file upload/download management
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. SetObjects
        super::ToolSpec {
            name: "set_objects".to_string(),
            description: "Set objects on an entity profile. Objects are key-value data attached to entities.".to_string(),
            api_group: "Data".to_string(),
            api_method: "SetObjects".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "SetObjects parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to set objects on",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "objects",
                        object_array_prop(
                            "Array of objects to set on the entity",
                            object_schema(
                                "Object to set",
                                vec![
                                    ("objectName", string_prop("The name of the object")),
                                    ("dataObject", any_object_prop("The data to store in the object")),
                                    ("deleteObject", optional(boolean_prop("If true, delete this object instead of setting it"))),
                                    ("escapedDataObject", optional(string_prop("JSON-escaped string of the data object"))),
                                ],
                                vec!["objectName"],
                            ),
                        ),
                    ),
                    (
                        "expectedProfileVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "objects"],
            ),
        },
        // 2. GetObjects
        super::ToolSpec {
            name: "get_objects".to_string(),
            description: "Get objects from an entity profile".to_string(),
            api_group: "Data".to_string(),
            api_method: "GetObjects".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetObjects parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to get objects from",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "escapedDataObject",
                        optional(boolean_prop("Whether to return data objects as escaped JSON strings")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity"],
            ),
        },
        // 3. InitiateFileUploads
        super::ToolSpec {
            name: "initiate_file_uploads".to_string(),
            description: "Initiate file uploads for an entity. Returns upload URLs for the specified files.".to_string(),
            api_group: "Data".to_string(),
            api_method: "InitiateFileUploads".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "InitiateFileUploads parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to upload files for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "fileNames",
                        string_array_prop("Array of file names to initiate uploads for"),
                    ),
                    (
                        "profileVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "fileNames"],
            ),
        },
        // 4. AbortFileUploads
        super::ToolSpec {
            name: "abort_file_uploads".to_string(),
            description: "Abort pending file uploads for an entity".to_string(),
            api_group: "Data".to_string(),
            api_method: "AbortFileUploads".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "AbortFileUploads parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to abort file uploads for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "fileNames",
                        string_array_prop("Array of file names to abort uploads for"),
                    ),
                    (
                        "profileVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "fileNames"],
            ),
        },
        // 5. FinalizeFileUploads
        super::ToolSpec {
            name: "finalize_file_uploads".to_string(),
            description: "Finalize file uploads for an entity after uploading content to the provided URLs".to_string(),
            api_group: "Data".to_string(),
            api_method: "FinalizeFileUploads".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "FinalizeFileUploads parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to finalize file uploads for",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "fileNames",
                        string_array_prop("Array of file names to finalize uploads for"),
                    ),
                    (
                        "profileVersion",
                        optional(integer_prop("Profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "fileNames"],
            ),
        },
        // 6. GetFiles
        super::ToolSpec {
            name: "get_files".to_string(),
            description: "Get metadata and download URLs for files attached to an entity".to_string(),
            api_group: "Data".to_string(),
            api_method: "GetFiles".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "GetFiles parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to get files for",
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
        // 7. DeleteFiles
        super::ToolSpec {
            name: "delete_files".to_string(),
            description: "Delete files attached to an entity".to_string(),
            api_group: "Data".to_string(),
            api_method: "DeleteFiles".to_string(),
            category: super::ToolCategory::Data,
            auth_mode: super::AuthMode::EntityToken,
            retry_mode: super::RetryMode::Standard,
            input_schema: object_schema(
                "DeleteFiles parameters",
                vec![
                    (
                        "entity",
                        object_schema(
                            "The entity to delete files from",
                            vec![
                                ("Id", string_prop("The unique ID of the entity")),
                                ("Type", string_prop("The type of the entity")),
                            ],
                            vec!["Id", "Type"],
                        ),
                    ),
                    (
                        "fileNames",
                        string_array_prop("Array of file names to delete"),
                    ),
                    (
                        "profileVersion",
                        optional(integer_prop("Expected profile version for concurrency control")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["entity", "fileNames"],
            ),
        },
    ]
}
