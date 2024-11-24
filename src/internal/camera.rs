use nalgebra_glm::Vec3;
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,          // Camera position
    pub center: Vec3,       // Subject origin position
    pub up: Vec3,           // The upwards direction
    pub has_changed: bool,  // Tracks if the camera state has changed

    pub min_radius: f32,    // Minimum allowed radius
    pub max_radius: f32,    // Maximum allowed radius
    pub current_radius: f32, // Current radius
}

impl Camera {
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, min_radius: f32, max_radius: f32) -> Self {
        let current_radius = (eye - center).magnitude().clamp(min_radius, max_radius);
        let direction = (eye - center).normalize();
        let adjusted_eye = center + direction * current_radius;

        Camera {
            eye: adjusted_eye,
            center,
            up,
            has_changed: true,
            min_radius,
            max_radius,
            current_radius,
        }
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        let radius_vector = self.eye - self.center;

        let current_yaw = radius_vector.z.atan2(radius_vector.x);
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Calculate new yaw (horizontal) and pitch (vertical)
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Update the `eye` position using the current radius
        self.eye = self.center + Vec3::new(
            self.current_radius * new_yaw.cos() * new_pitch.cos(),
            -self.current_radius * new_pitch.sin(),
            self.current_radius * new_yaw.sin() * new_pitch.cos(),
        );

        self.has_changed = true;
    }

    pub fn change_center(&mut self, new_center: Vec3) {
        let direction = (self.eye - self.center).normalize(); // Current direction to the subject
        self.center = new_center;

        // Update `eye` to maintain the fixed radius and direction
        self.eye = self.center + direction * self.current_radius;

        self.has_changed = true;
    }

    pub fn zoom(&mut self, delta: f32) {
        // Adjust the radius within allowed bounds
        self.current_radius = (self.current_radius - delta)
            .clamp(self.min_radius, self.max_radius);

        // Update `eye` to maintain the direction and new radius
        let direction = (self.eye - self.center).normalize();
        self.eye = self.center + direction * self.current_radius;

        self.has_changed = true;
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