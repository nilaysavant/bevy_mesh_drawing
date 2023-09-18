use bevy::prelude::*;

use crate::components::Cleanup;

pub fn cleanup_all(mut commands: Commands, query: Query<(Entity, &Cleanup)>) {
    let mut cleanup_count = 0;
    for (entity, cleanup) in query.iter() {
        match cleanup {
            Cleanup::SelfOnly => {
                commands.entity(entity).despawn();
                cleanup_count += 1;
            }
            Cleanup::Descendants => {
                commands.entity(entity).despawn_descendants();
                cleanup_count += 1;
            }
            Cleanup::Recursive => {
                commands.entity(entity).despawn_recursive();
                cleanup_count += 1;
            }
        }
    }
    if cleanup_count > 0 {
        info!("cleanup_count: {:?}", cleanup_count);
    }
}
