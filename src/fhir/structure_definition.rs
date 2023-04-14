use super::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StructureDefinition {
    pub id: String,
    pub url: String,
    pub version: Option<String>,
    pub name: String,
    pub status: String,
    pub experimental: Option<bool>,
    pub kind: String,
    pub r#abstract: bool,
    pub r#type: String,
    pub base_definition: Option<String>,
    pub snapshot: Option<Definitions>,
    pub differential: Option<Definitions>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Definitions {
    pub element: Vec<ElementDefinition>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementDefinition {
    pub id: String,
    pub path: String,
    pub slice_name: Option<String>,
    pub slicing: Option<SlicingRules>,
    pub min: Option<u32>,
    pub max: Option<String>,
    pub base: Option<BaseElement>,
    pub content_reference: Option<String>,
    #[serde(default)]
    pub r#type: Vec<ElementType>,
    #[serde(default)]
    pub constraint: Vec<Constraint>,
    pub binding: Option<TerminologyBinding>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlicingRules {
    #[serde(default)]
    pub discriminator: Vec<Discriminator>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Discriminator {
    pub r#type: String,
    pub path: String,
    pub description: Option<String>,
    pub ordered: Option<bool>,
    pub rules: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BaseElement {
    pub path: String,
    pub min: u32,
    pub max: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ElementType {
    pub code: String,
    #[serde(default)]
    pub profile: Vec<String>,
    #[serde(default)]
    pub target_profile: Vec<String>,
    #[serde(default)]
    pub aggregation: Vec<String>,
    pub versioning: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Constraint {
    pub key: String,
    pub severity: String,
    pub human: String,
    pub expression: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TerminologyBinding {
    pub strength: String,
    pub value_set: Option<String>,
}

impl StructureDefinition {
    pub fn load_bundle(data: &[u8]) -> Result<Vec<StructureDefinition>, ParseError> {
        let bundle: serde_json::Value =
            serde_json::from_slice(data).map_err(|e| ParseError::InvalidJSON(e))?;
        let Some(entries) = bundle["entry"].as_array() else {return Err(ParseError::MalformedBundle)};

        let mut struct_defs = Vec::new();
        for entry in entries {
            let Some(resource) = entry["resource"].as_object() else {return Err(ParseError::MalformedBundle)};
            if resource["resourceType"].as_str().unwrap_or("") == "StructureDefinition" {
                let struct_def: StructureDefinition =
                    serde_json::from_value(entry["resource"].clone())
                        .map_err(|e| ParseError::InvalidJSON(e))?;
                struct_defs.push(struct_def);
            }
        }

        Ok(struct_defs)
    }
}
