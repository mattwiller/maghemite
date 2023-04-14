use super::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

const RESOURCE_JSON: &[u8] = include_bytes!("specification/R4B/profiles-resources.json");
const DATA_TYPE_JSON: &[u8] = include_bytes!("specification/R4B/profiles-types.json");

lazy_static! {
    pub static ref RESOURCES: HashMap<String, StructureDefinition> =
        StructureDefinition::load_bundle(RESOURCE_JSON)
            .unwrap()
            .iter()
            .map(|sd| (sd.name.clone(), sd.clone()))
            .collect();
}

lazy_static! {
    pub static ref DATA_TYPES: HashMap<String, StructureDefinition> =
        StructureDefinition::load_bundle(DATA_TYPE_JSON)
            .unwrap()
            .iter()
            .map(|sd| (sd.name.clone(), sd.clone()))
            .collect();
}

lazy_static! {
    pub static ref FIELDS: HashMap<String, ElementDefinition> = RESOURCES
        .iter()
        .map(|(_, sd)| sd
            .snapshot
            .as_ref()
            .unwrap()
            .element
            .iter()
            .map(|el| (el.path.clone(), el.clone())))
        .flatten()
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resources_loaded() {
        assert_eq!(RESOURCES.len(), 143);
        assert_eq!(
            RESOURCES["Patient"].url,
            "http://hl7.org/fhir/StructureDefinition/Patient"
        );
        assert_eq!(
            RESOURCES["Patient"]
                .snapshot
                .as_ref()
                .unwrap()
                .element
                .len(),
            45
        );
    }

    #[test]
    fn test_data_types_loaded() {
        assert_eq!(DATA_TYPES.len(), 64);
        assert_eq!(
            DATA_TYPES["Meta"].url,
            "http://hl7.org/fhir/StructureDefinition/Meta"
        );
        assert_eq!(
            DATA_TYPES["Meta"].snapshot.as_ref().unwrap().element.len(),
            9
        );
    }

    #[test]
    fn test_fields_loaded() {
        let birthday = &FIELDS["Patient.birthDate"];
        assert_eq!(birthday.min, Some(0));
        assert_eq!(birthday.max, Some("1".to_string()));
        assert_eq!(birthday.r#type[0].code, "date");
    }
}
