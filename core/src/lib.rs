use legion::{serialize::Canon, World};
use serde::de::DeserializeSeed;
use serde_json::Value;
use stage::create_registry;

pub mod math;
pub mod primitive;
pub mod stage;

pub fn load(scene_file: &str) -> World {
    let json: Value = serde_json::from_str(
        &std::fs::read_to_string(scene_file)
            .expect("Failed to read scene file"),
    )
    .expect("Failed to parse scene file");

    let registry = create_registry();
    let entity_serializer = Canon::default();
    let world = registry
        .as_deserialize(&entity_serializer)
        .deserialize(json)
        .expect("Failed to deserialize world!");

    world
}

pub fn save(world: World, scene_file: &str) {
    let registry = create_registry();
    let entity_serializer = Canon::default();

    let serializable_world =
        world.as_serializable(legion::any(), &registry, &entity_serializer);

    //let mut serializer =
    //serde_json::serializer::new(std::fs::File::open(scene_file).unwrap());
    //serializable_world
    //.serialize(&mut serializer)
    //.expect("Failed to serialize world");
    std::fs::write(
        scene_file,
        serde_json::to_vec(&serializable_world).unwrap().as_slice(),
    )
    .unwrap();
}

#[cfg(test)]
mod test {
    use legion::{IntoQuery, World};

    use crate::{load, math::Transform, primitive::Model, save};

    #[test]
    fn test_serialization() {
        let mut demo_world = World::default();
        demo_world.push((Transform::default(), Model::quad(None)));
        save(demo_world, "test.json");

        let mut loaded_world = load("test.json");

        let mut entities = <&Transform>::query();
        let loaded_transform = entities.iter(&mut loaded_world).next().unwrap();

        assert_eq!(&Transform::default(), loaded_transform);
    }
}
