use simple_math::Vector3f;

use crate::world::world::{World, WorldObject};

#[derive(Debug)]
pub struct CollisionManager {}

impl Default for CollisionManager {
    fn default() -> Self {
        CollisionManager {}
    }
}

impl CollisionManager {
    pub fn new() -> Self {
        CollisionManager::default()
    }

    pub fn intersect<'a>(
        &self,
        origin: &Vector3f,
        ray: &Vector3f,
        world: &'a World,
    ) -> Option<(&'a WorldObject, f32, Vector3f)> {
        let mut result = None;
        for object in &world.objects {
            let object_intersect_result = object.mesh.intersect(origin, ray);
            if object_intersect_result.is_none() {
                continue;
            }

            let (t_result, normal) = object_intersect_result.unwrap();
            if let Some((_, t, _)) = result {
                if t > t_result {
                    result = Some((object, t_result, normal));
                }
            } else {
                result = Some((object, t_result, normal));
            }
        }
        result
    }
}
