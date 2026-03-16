// Experimentation tools: CreateExperiment, GetExperiments, ExclusionGroups, etc.
use crate::tooling::schema_builder::*;

pub fn tools() -> Vec<super::ToolSpec> {
    vec![
        // 1. create_experiment
        super::ToolSpec {
            name: "create_experiment".to_string(),
            description: "Create a new experiment with variants and audience targeting".to_string(),
            parameters: object_schema(
                "CreateExperiment parameters",
                vec![
                    ("name", string_prop("The name of the experiment")),
                    (
                        "variants",
                        object_array_prop(
                            "The variants for the experiment",
                            object_schema(
                                "Experiment variant",
                                vec![
                                    ("name", string_prop("Name of the variant")),
                                    (
                                        "trafficPercentage",
                                        integer_range_prop(
                                            "Percentage of traffic for this variant",
                                            Some(0),
                                            Some(100),
                                        ),
                                    ),
                                    (
                                        "variables",
                                        optional(object_array_prop(
                                            "Variables overridden in this variant",
                                            any_object_prop("A variable override"),
                                        )),
                                    ),
                                    (
                                        "isControl",
                                        optional(boolean_prop("Whether this is the control variant")),
                                    ),
                                ],
                                vec!["name", "trafficPercentage"],
                            ),
                        ),
                    ),
                    (
                        "audienceTargeting",
                        any_object_prop("Audience targeting configuration for the experiment"),
                    ),
                    (
                        "description",
                        optional(string_prop("Description of the experiment")),
                    ),
                    (
                        "startDate",
                        optional(string_prop("Start date for the experiment (ISO 8601)")),
                    ),
                    (
                        "endDate",
                        optional(string_prop("End date for the experiment (ISO 8601)")),
                    ),
                    (
                        "duration",
                        optional(string_prop("Duration of the experiment")),
                    ),
                    (
                        "segmentId",
                        optional(string_prop("Segment ID for targeting")),
                    ),
                    (
                        "exclusionGroupId",
                        optional(string_prop("Exclusion group ID for the experiment")),
                    ),
                    (
                        "exclusionGroupTrafficAllocation",
                        optional(integer_range_prop(
                            "Traffic allocation percentage within exclusion group",
                            Some(0),
                            Some(100),
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name", "variants", "audienceTargeting"],
            ),
        },
        // 2. delete_experiment
        super::ToolSpec {
            name: "delete_experiment".to_string(),
            description: "Delete an experiment by its ID".to_string(),
            parameters: object_schema(
                "DeleteExperiment parameters",
                vec![
                    (
                        "experimentId",
                        string_prop("The unique ID of the experiment to delete"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["experimentId"],
            ),
        },
        // 3. get_experiments
        super::ToolSpec {
            name: "get_experiments".to_string(),
            description: "Get all experiments for the title".to_string(),
            parameters: object_schema(
                "GetExperiments parameters",
                vec![
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 4. get_latest_scorecard
        super::ToolSpec {
            name: "get_latest_scorecard".to_string(),
            description: "Get the latest scorecard results for an experiment".to_string(),
            parameters: object_schema(
                "GetLatestScorecard parameters",
                vec![
                    (
                        "experimentId",
                        string_prop("The unique ID of the experiment"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["experimentId"],
            ),
        },
        // 5. get_treatment_assignment
        super::ToolSpec {
            name: "get_treatment_assignment".to_string(),
            description: "Get the treatment assignment for an entity in active experiments"
                .to_string(),
            parameters: object_schema(
                "GetTreatmentAssignment parameters",
                vec![
                    (
                        "entity",
                        optional(any_object_prop(
                            "The entity to get treatment assignment for (Id and Type)",
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
        // 6. start_experiment
        super::ToolSpec {
            name: "start_experiment".to_string(),
            description: "Start a previously created experiment".to_string(),
            parameters: object_schema(
                "StartExperiment parameters",
                vec![
                    (
                        "experimentId",
                        string_prop("The unique ID of the experiment to start"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["experimentId"],
            ),
        },
        // 7. stop_experiment
        super::ToolSpec {
            name: "stop_experiment".to_string(),
            description: "Stop a running experiment".to_string(),
            parameters: object_schema(
                "StopExperiment parameters",
                vec![
                    (
                        "experimentId",
                        string_prop("The unique ID of the experiment to stop"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["experimentId"],
            ),
        },
        // 8. update_experiment
        super::ToolSpec {
            name: "update_experiment".to_string(),
            description: "Update an existing experiment configuration".to_string(),
            parameters: object_schema(
                "UpdateExperiment parameters",
                vec![
                    (
                        "experimentId",
                        string_prop("The unique ID of the experiment to update"),
                    ),
                    ("name", string_prop("The updated name of the experiment")),
                    (
                        "description",
                        optional(string_prop("Updated description of the experiment")),
                    ),
                    (
                        "variants",
                        optional(object_array_prop(
                            "Updated variants for the experiment",
                            object_schema(
                                "Experiment variant",
                                vec![
                                    ("name", string_prop("Name of the variant")),
                                    (
                                        "trafficPercentage",
                                        integer_range_prop(
                                            "Percentage of traffic for this variant",
                                            Some(0),
                                            Some(100),
                                        ),
                                    ),
                                    (
                                        "variables",
                                        optional(object_array_prop(
                                            "Variables overridden in this variant",
                                            any_object_prop("A variable override"),
                                        )),
                                    ),
                                    (
                                        "isControl",
                                        optional(boolean_prop("Whether this is the control variant")),
                                    ),
                                ],
                                vec!["name", "trafficPercentage"],
                            ),
                        )),
                    ),
                    (
                        "audienceTargeting",
                        optional(any_object_prop("Updated audience targeting configuration")),
                    ),
                    (
                        "startDate",
                        optional(string_prop("Updated start date (ISO 8601)")),
                    ),
                    (
                        "endDate",
                        optional(string_prop("Updated end date (ISO 8601)")),
                    ),
                    (
                        "duration",
                        optional(string_prop("Updated duration of the experiment")),
                    ),
                    (
                        "segmentId",
                        optional(string_prop("Updated segment ID for targeting")),
                    ),
                    (
                        "exclusionGroupId",
                        optional(string_prop("Exclusion group ID for the experiment")),
                    ),
                    (
                        "exclusionGroupTrafficAllocation",
                        optional(integer_range_prop(
                            "Traffic allocation percentage within exclusion group",
                            Some(0),
                            Some(100),
                        )),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["experimentId", "name"],
            ),
        },
        // 9. create_exclusion_group
        super::ToolSpec {
            name: "create_exclusion_group".to_string(),
            description: "Create an exclusion group to prevent experiments from overlapping"
                .to_string(),
            parameters: object_schema(
                "CreateExclusionGroup parameters",
                vec![
                    (
                        "name",
                        string_prop("The name of the exclusion group"),
                    ),
                    (
                        "description",
                        optional(string_prop("Description of the exclusion group")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["name"],
            ),
        },
        // 10. delete_exclusion_group
        super::ToolSpec {
            name: "delete_exclusion_group".to_string(),
            description: "Delete an exclusion group by its ID".to_string(),
            parameters: object_schema(
                "DeleteExclusionGroup parameters",
                vec![
                    (
                        "exclusionGroupId",
                        string_prop("The unique ID of the exclusion group to delete"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["exclusionGroupId"],
            ),
        },
        // 11. get_exclusion_groups
        super::ToolSpec {
            name: "get_exclusion_groups".to_string(),
            description: "Get all exclusion groups for the title".to_string(),
            parameters: object_schema(
                "GetExclusionGroups parameters",
                vec![
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec![],
            ),
        },
        // 12. update_exclusion_group
        super::ToolSpec {
            name: "update_exclusion_group".to_string(),
            description: "Update an existing exclusion group".to_string(),
            parameters: object_schema(
                "UpdateExclusionGroup parameters",
                vec![
                    (
                        "exclusionGroupId",
                        string_prop("The unique ID of the exclusion group to update"),
                    ),
                    (
                        "name",
                        string_prop("The updated name of the exclusion group"),
                    ),
                    (
                        "description",
                        optional(string_prop("Updated description of the exclusion group")),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["exclusionGroupId", "name"],
            ),
        },
        // 13. get_exclusion_group_traffic
        super::ToolSpec {
            name: "get_exclusion_group_traffic".to_string(),
            description: "Get traffic allocation details for an exclusion group".to_string(),
            parameters: object_schema(
                "GetExclusionGroupTraffic parameters",
                vec![
                    (
                        "exclusionGroupId",
                        string_prop("The unique ID of the exclusion group"),
                    ),
                    (
                        "customTags",
                        optional(any_object_prop("Custom tags for the request")),
                    ),
                ],
                vec!["exclusionGroupId"],
            ),
        },
    ]
}
