use bevy::prelude::*;

/// Marker Component for cleanup delete entity marking.
#[derive(Debug, Component)]
pub enum Cleanup {
    /// Cleanup only the self on which this is assigned.
    SelfOnly,
    /// Cleanup only the descendants on which this is assigned.
    Descendants,
    /// Cleanup self + all descendants recursively.
    Recursive,
}

impl Default for Cleanup {
    fn default() -> Self {
        Self::SelfOnly
    }
}
