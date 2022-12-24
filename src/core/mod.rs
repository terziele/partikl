use crate::core::Type::Toggle;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::empty;

#[derive(Clone, Debug)]
pub enum PropertyValue {
    String(String),
    Integer(i32),
    Double(f32),
    Toggle(bool),
}

#[derive(Clone, Debug)]
pub struct EntityProperty {
    name: String,
    value: PropertyValue,
}

impl EntityProperty {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &PropertyValue {
        &self.value
    }
}

pub struct Entity {
    id: String,
    props: Vec<EntityProperty>,
}

#[derive(Clone, Debug)]
pub enum Type {
    String,
    Integer,
    Double,
    Toggle,
}

#[derive(Clone, Debug)]
pub struct SchemaProperty {
    name: String,
    prop_type: Type,
}

impl Into<SchemaProperty> for (&str, Type) {
    fn into(self) -> SchemaProperty {
        SchemaProperty {
            name: self.0.to_string(),
            prop_type: self.1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Schema {
    name: String,
    properties: Vec<SchemaProperty>,
}

impl Schema {
    pub fn create_entity(&self, entity_props: Vec<EntityProperty>) -> Result<Entity, Vec<String>> {
        let schema_props = self.props_map();
        let diff = self.find_missing_keys(&entity_props);
        println!("Entity missing some schema specified keys: {:?}", diff);

        Ok(Entity {
            id: "".to_string(),
            props: vec![],
        })
    }

    fn props_map(&self) -> HashMap<String, SchemaProperty> {
        self.clone()
            .properties
            .iter()
            .cloned()
            .map(|p| (p.name.clone(), p))
            .collect::<HashMap<String, SchemaProperty>>()
    }

    fn find_missing_keys(&self, entity_props: &[EntityProperty]) -> Vec<String> {
        let schema_props = self.props_map();
        let schema_keys = schema_props.keys().collect::<HashSet<&String>>();
        let entity_keys = entity_props
            .iter()
            .map(|p| p.name())
            .collect::<HashSet<&String>>();

        (&schema_keys - &entity_keys).into_iter().cloned().collect()
    }
}

pub struct SchemaBuilder {
    name: String,
    props: Vec<SchemaProperty>,
}

impl SchemaBuilder {
    pub fn named(name: &str) -> SchemaBuilder {
        SchemaBuilder {
            name: name.to_string(),
            props: vec![],
        }
    }

    pub fn with_property(mut self, prop_name: &str, prop_type: Type) -> SchemaBuilder {
        self.props.push((prop_name, prop_type).into());

        self
    }

    pub fn build(self) -> Schema {
        Schema {
            name: self.name,
            properties: self.props,
        }
    }
}

#[test]
fn schema_create_entity_same_properties_as_expected() {
    let schema = SchemaBuilder::named("test_schema")
        .with_property("prop1", Toggle)
        .build();

    let x = schema.create_entity(vec![]);

    assert!(x.is_ok(), "entity creation failed");
}
