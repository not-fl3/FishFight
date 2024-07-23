use macroquad::{
    experimental::scene::{CapabilityTrait, HandleUntyped, NodeWith},
    math::Rect,
};

/// Anything that can be thrown, pushed, dragged etc
#[derive(Copy, Clone)]
pub struct PhysicsObject {
    /// Indicates if the object wants to interact
    /// For example, picked up weapons do not really want to interact with
    /// sproingers, but they still have a collider
    pub active: fn(node: HandleUntyped) -> bool,
    /// Get an object rectangle
    pub collider: fn(node: HandleUntyped) -> Rect,
    pub set_speed_x: fn(node: HandleUntyped, speed: f32),
    pub set_speed_y: fn(node: HandleUntyped, speed: f32),
}

pub trait PhysicsObjectTrait {
    fn active(&self) -> bool;
    fn collider(&self) -> Rect;
    fn set_speed_x(&self, speed: f32) -> ();
    fn set_speed_y(&self, speed: f32) -> ();
}
impl PhysicsObjectTrait for NodeWith<PhysicsObject> {
    fn active(&self) -> bool {
        (self.capability.active)(self.node)
    }
    fn collider(&self) -> Rect {
        (self.capability.collider)(self.node)
    }
    fn set_speed_x(&self, speed: f32) -> () {
        (self.capability.set_speed_x)(self.node, speed)
    }
    fn set_speed_y(&self, speed: f32) -> () {
        (self.capability.set_speed_y)(self.node, speed)
    }
}
