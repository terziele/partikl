use crate::core::Type::Toggle;
use std::error::Error;

pub enum PropertyValue {
    String(String),
    Integer(i32),
    Double(f32),
    Toggle(bool),
}

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

pub enum Type {
    String,
    Integer,
    Double,
    Toggle,
}

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

pub struct Schema {
    name: String,
    properties: Vec<SchemaProperty>,
}

impl Schema {
    pub fn create_entity(&self, entity_props: Vec<EntityProperty>) -> Result<Entity, Vec<String>> {
        Ok(Entity {
            id: "".to_string(),
            props: vec![],
        })
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
