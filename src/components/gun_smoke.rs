use macroquad::{
    experimental::collections::storage,
    math::{vec2, Vec2},
    time::get_frame_time,
};
use macroquad_particles::EmittersCache;

use crate::components::PhysicsBody;

#[derive(Clone)]
pub struct GunSmoke {
    pub smoke_fx_counter: i8,
    pub smoke_fx_timer: f32,
}

impl GunSmoke {
    pub fn new() -> GunSmoke {
        GunSmoke {
            smoke_fx_counter: 0,
            smoke_fx_timer: 0.0,
        }
    }

    /// `fx_active` - gun_fx from a GunlikeAnimation - when a second, fx-only sprite is displayed
    /// `thrown` - from Throwable, to indicate the weapon is attached to a fish or not
    pub fn update(
        &mut self,
        fx: &mut EmittersCache,
        fx_active: bool,
        thrown: bool,
        body: &PhysicsBody,
    ) {
        if thrown == false {
            if fx_active == false {
                if self.smoke_fx_counter < 5 {
                    if self.smoke_fx_timer > 0.15 {
                        self.smoke_fx_timer = 0.1;
                        self.smoke_fx_counter += 1;
                        {
                            fx.spawn(body.pos + vec2(16.0, 15.0) + body.facing_dir() * 32.0);
                        }
                    }
                    self.smoke_fx_timer += get_frame_time()
                }
            } else {
                self.smoke_fx_timer = 0.0;
                self.smoke_fx_counter = 0;
            }
        }
    }
}
