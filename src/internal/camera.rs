use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3, // Camera position
    pub center: Vec3, // Subject origin position
    pub up: Vec3, // The upwards direction
    pub has_changed: bool
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        Camera {
            eye,
            center,
            up,
            has_changed: true,
        }
    }
    
    pub fn orbit( &mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();
        
        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        
        // Calculate current yaw
        let current_pitch = (- radius_vector.y).atan2(radius_xz);
        
        // Calculate new yaw (horizontal) and pitch (vertical).
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        
        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );
        self.has_changed = true;
        self.eye = new_eye;
    }

    pub fn zoom(&mut self, delta: f32) {
        let direction = self.center - self.eye; // Vector pointing from eye to center
        let magnitude = direction.magnitude();
    
        // Minimum allowable distance between eye and center
        let min_distance = 0.1;
    
        if magnitude > min_distance || delta < 0.0 {
            // Ensure delta doesn't collapse the eye position below min_distance
            let clamped_delta = if delta > 0.0 {
                delta.min(magnitude - min_distance)
            } else {
                delta
            };
    
            // Apply the zoom
            self.eye += direction.normalize() * clamped_delta;
            self.has_changed = true;
        }
    }

    pub fn check_if_changed(&mut self) -> bool {
        if self.has_changed {
            self.has_changed = false;
            true
        } else {
            false
        }
    }
}