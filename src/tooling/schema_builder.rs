use serde_json::{json, Value};

/// Build a JSON Schema object with properties
pub fn object_schema(
    description: &str,
    properties: Vec<(&str, Value)>,
    required: Vec<&str>,
) -> Value {
    let props: serde_json::Map<String, Value> = properties
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();

    let required: Vec<Value> = required.into_iter().map(|s| json!(s)).collect();

    let mut schema = json!({
        "type": "object",
        "description": description,
        "properties": props,
    });

    if !required.is_empty() {
        schema["required"] = json!(required);
    }

    schema
}

/// String property
pub fn string_prop(description: &str) -> Value {
    json!({
        "type": "string",
        "description": description
    })
}

/// String property with enum constraint
pub fn string_enum_prop(description: &str, values: &[&str]) -> Value {
    json!({
        "type": "string",
        "description": description,
        "enum": values
    })
}

/// Integer property
pub fn integer_prop(description: &str) -> Value {
    json!({
        "type": "integer",
        "description": description
    })
}

/// Integer property with bounds
pub fn integer_range_prop(description: &str, min: Option<i64>, max: Option<i64>) -> Value {
    let mut prop = json!({
        "type": "integer",
        "description": description
    });
    if let Some(min) = min {
        prop["minimum"] = json!(min);
    }
    if let Some(max) = max {
        prop["maximum"] = json!(max);
    }
    prop
}

/// Boolean property
pub fn boolean_prop(description: &str) -> Value {
    json!({
        "type": "boolean",
        "description": description
    })
}

/// Array of strings property
pub fn string_array_prop(description: &str) -> Value {
    json!({
        "type": "array",
        "description": description,
        "items": { "type": "string" }
    })
}

/// Array of objects property
pub fn object_array_prop(description: &str, item_schema: Value) -> Value {
    json!({
        "type": "array",
        "description": description,
        "items": item_schema
    })
}

/// Free-form object property (for custom data)
pub fn any_object_prop(description: &str) -> Value {
    json!({
        "type": "object",
        "description": description,
        "additionalProperties": true
    })
}

/// Optional wrapper (just adds a note to description)
pub fn optional(mut prop: Value) -> Value {
    if let Some(desc) = prop.get("description").and_then(|d| d.as_str()) {
        prop["description"] = json!(format!("{} (optional)", desc));
    }
    prop
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_schema_produces_valid_structure_with_required() {
        let schema = object_schema(
            "Test schema",
            vec![
                ("name", string_prop("Player name")),
                ("age", integer_prop("Player age")),
            ],
            vec!["name"],
        );

        assert_eq!(schema["type"], "object");
        assert_eq!(schema["description"], "Test schema");
        assert!(schema["properties"]["name"].is_object());
        assert!(schema["properties"]["age"].is_object());

        let required = schema["required"].as_array().unwrap();
        assert_eq!(required.len(), 1);
        assert_eq!(required[0], "name");
    }

    #[test]
    fn object_schema_omits_required_when_empty() {
        let schema = object_schema(
            "No required fields",
            vec![("foo", string_prop("A field"))],
            vec![],
        );

        assert!(schema.get("required").is_none());
    }

    #[test]
    fn string_prop_produces_correct_schema() {
        let prop = string_prop("A string field");
        assert_eq!(prop["type"], "string");
        assert_eq!(prop["description"], "A string field");
    }

    #[test]
    fn string_enum_prop_includes_enum_values() {
        let prop = string_enum_prop("Direction", &["north", "south", "east", "west"]);
        assert_eq!(prop["type"], "string");
        let vals = prop["enum"].as_array().unwrap();
        assert_eq!(vals.len(), 4);
        assert_eq!(vals[0], "north");
    }

    #[test]
    fn integer_prop_produces_correct_schema() {
        let prop = integer_prop("Count");
        assert_eq!(prop["type"], "integer");
        assert_eq!(prop["description"], "Count");
    }

    #[test]
    fn integer_range_prop_includes_min_max_when_specified() {
        let prop = integer_range_prop("Amount", Some(1), Some(100));
        assert_eq!(prop["type"], "integer");
        assert_eq!(prop["minimum"], 1);
        assert_eq!(prop["maximum"], 100);
    }

    #[test]
    fn integer_range_prop_only_min() {
        let prop = integer_range_prop("Amount", Some(0), None);
        assert_eq!(prop["minimum"], 0);
        assert!(prop.get("maximum").is_none() || prop["maximum"].is_null());
    }

    #[test]
    fn integer_range_prop_only_max() {
        let prop = integer_range_prop("Amount", None, Some(50));
        assert!(prop.get("minimum").is_none() || prop["minimum"].is_null());
        assert_eq!(prop["maximum"], 50);
    }

    #[test]
    fn integer_range_prop_no_bounds() {
        let prop = integer_range_prop("Amount", None, None);
        assert_eq!(prop["type"], "integer");
        assert!(prop.get("minimum").is_none() || prop["minimum"].is_null());
        assert!(prop.get("maximum").is_none() || prop["maximum"].is_null());
    }

    #[test]
    fn boolean_prop_produces_correct_schema() {
        let prop = boolean_prop("Is active");
        assert_eq!(prop["type"], "boolean");
        assert_eq!(prop["description"], "Is active");
    }

    #[test]
    fn string_array_prop_produces_correct_schema() {
        let prop = string_array_prop("Tags");
        assert_eq!(prop["type"], "array");
        assert_eq!(prop["items"]["type"], "string");
    }

    #[test]
    fn object_array_prop_uses_item_schema() {
        let item = json!({"type": "object", "properties": {"id": {"type": "string"}}});
        let prop = object_array_prop("Items", item.clone());
        assert_eq!(prop["type"], "array");
        assert_eq!(prop["items"], item);
    }

    #[test]
    fn any_object_prop_allows_additional_properties() {
        let prop = any_object_prop("Custom data");
        assert_eq!(prop["type"], "object");
        assert_eq!(prop["additionalProperties"], true);
    }

    #[test]
    fn optional_appends_to_description() {
        let prop = optional(string_prop("Player name"));
        assert_eq!(prop["description"], "Player name (optional)");
        assert_eq!(prop["type"], "string");
    }
}
