use anyhow::{Context, Result};
use hecs::{
    serialize::row::{try_serialize, DeserializeContext, SerializeContext},
    EntityBuilder, EntityRef, World,
};
use math::Transform;
use primitive::Model;
use serde::{Deserialize, Serialize};

pub mod math;
pub mod primitive;
pub mod stage;

#[derive(Serialize, Deserialize)]
enum ComponentId {
    Transform,
    Model,
}

struct WorldContext;

impl DeserializeContext for WorldContext {
    fn deserialize_entity<'de, M>(
        &mut self,
        mut map: M,
        entity: &mut EntityBuilder,
    ) -> Result<(), M::Error>
    where
        M: serde::de::MapAccess<'de>,
    {
        while let Some(key) = map.next_key()? {
            match key {
                ComponentId::Transform => {
                    entity.add::<Transform>(map.next_value()?);
                }
                ComponentId::Model => {
                    entity.add::<Model>(map.next_value()?);
                }
            }
        }
        Ok(())
    }
}

impl SerializeContext for WorldContext {
    fn serialize_entity<S>(
        &mut self,
        entity: EntityRef<'_>,
        map: &mut S,
    ) -> Result<(), S::Error>
    where
        S: serde::ser::SerializeMap,
    {
        try_serialize::<Transform, _, _>(
            &entity,
            &ComponentId::Transform,
            map,
        )?;
        try_serialize::<Model, _, _>(&entity, &ComponentId::Model, map)?;

        Ok(())
    }
}

pub fn load(scene_file: &str) -> Result<World> {
    let mut context = WorldContext {};

    let world = hecs::serialize::row::deserialize(
        &mut context,
        &mut serde_json::Deserializer::from_str(
            &std::fs::read_to_string(scene_file)
                .context("Failed to read scene file")?,
        ),
    )
    .context("Failed to deserialize scene file")?;

    Ok(world)
}

pub fn save(world: World, scene_file: &str) -> Result<()> {
    let mut context = WorldContext {};
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"\t");

    let mut serializer = serde_json::Serializer::with_formatter(
        std::fs::File::create(scene_file)
            .context("Failed to open world file")?,
        formatter,
    );

    hecs::serialize::row::serialize(&world, &mut context, &mut serializer)
        .context("Failed to serialize world")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use hecs::World;

    use crate::{load, math::Transform, primitive::Model, save};

    #[test]
    fn test_serialization() -> Result<()> {
        let mut demo_world = World::default();
        demo_world.spawn((1, Transform::default(), Model::quad(None)));
        save(demo_world, "test.json")?;

        let loaded_world = load("test.json")?;

        let mut entities = loaded_world.query::<&Transform>();
        let loaded_entity = entities.iter().next().unwrap();

        assert_eq!(&Transform::default(), loaded_entity.1);

        Ok(())
    }
}
