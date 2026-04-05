use macroquad::prelude::*;
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub lifetime: f32,     // seconds remaining
    pub max_lifetime: f32, // total lifetime
    pub size: f32,
    pub color: Color,
    pub fade: bool, // whether to fade out over time
}

impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, lifetime: f32, size: f32, color: Color) -> Self {
        Self {
            x,
            y,
            vx,
            vy,
            lifetime,
            max_lifetime: lifetime,
            size,
            color,
            fade: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.lifetime -= dt;
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Apply gravity
        self.vy += 100.0 * dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }

    pub fn get_color(&self) -> Color {
        if self.fade {
            let alpha = self.lifetime / self.max_lifetime;
            Color {
                r: self.color.r,
                g: self.color.g,
                b: self.color.b,
                a: self.color.a * alpha,
            }
        } else {
            self.color
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParticleEmitter {
    pub x: f32,
    pub y: f32,
    pub particles: Vec<Particle>,
    pub emission_rate: f32, // particles per second
    pub accumulated_time: f32,
}

impl ParticleEmitter {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            particles: Vec::new(),
            emission_rate: 20.0,
            accumulated_time: 0.0,
        }
    }

    pub fn emit_burst(
        &mut self,
        count: usize,
        speed_range: (f32, f32),
        lifetime: f32,
        size: f32,
        color: Color,
    ) {
        for i in 0..count {
            let angle = (i as f32) * (2.0 * PI) / (count as f32);
            let speed = speed_range.0 + (speed_range.1 - speed_range.0) * (i as f32 / count as f32);

            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;

            let particle = Particle::new(self.x, self.y, vx, vy, lifetime, size, color);
            self.particles.push(particle);
        }
    }

    pub fn emit_random(
        &mut self,
        count: usize,
        speed: f32,
        lifetime: f32,
        size: f32,
        color: Color,
    ) {
        use macroquad::rand::gen_range;

        for _ in 0..count {
            let angle = gen_range(0.0, 2.0 * PI);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed;

            let particle = Particle::new(self.x, self.y, vx, vy, lifetime, size, color);
            self.particles.push(particle);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.update(dt);
        }
        self.particles.retain(|p| p.is_alive());
    }

    pub fn render(&self) {
        for particle in &self.particles {
            let color = particle.get_color();
            draw_circle(particle.x, particle.y, particle.size, color);
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParticleSystem {
    pub emitters: Vec<ParticleEmitter>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            emitters: Vec::new(),
        }
    }

    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.push(emitter);
    }

    pub fn update(&mut self, dt: f32) {
        for emitter in &mut self.emitters {
            emitter.update(dt);
        }
        self.emitters.retain(|e| !e.particles.is_empty());
    }

    pub fn render(&self) {
        for emitter in &self.emitters {
            emitter.render();
        }
    }

    // Convenience methods for common effect patterns

    pub fn brick_destruction(&mut self, x: f32, y: f32, color: Color) {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.emit_burst(12, (100.0, 200.0), 0.6, 3.0, color);
        self.add_emitter(emitter);
    }

    pub fn ball_collision(&mut self, x: f32, y: f32, color: Color) {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.emit_burst(8, (80.0, 150.0), 0.4, 2.0, color);
        self.add_emitter(emitter);
    }

    pub fn power_up_spawn(&mut self, x: f32, y: f32, color: Color) {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.emit_burst(16, (50.0, 100.0), 0.5, 2.0, color);
        self.add_emitter(emitter);
    }

    pub fn paddle_hit(&mut self, x: f32, y: f32, color: Color) {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.emit_burst(10, (60.0, 120.0), 0.35, 2.0, color);
        self.add_emitter(emitter);
    }

    pub fn power_up_pickup(&mut self, x: f32, y: f32, color: Color) {
        let mut emitter = ParticleEmitter::new(x, y);
        emitter.emit_burst(20, (100.0, 180.0), 0.5, 2.5, color);
        self.add_emitter(emitter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let particle = Particle::new(100.0, 100.0, 10.0, 20.0, 1.0, 5.0, WHITE);
        assert_eq!(particle.x, 100.0);
        assert_eq!(particle.y, 100.0);
        assert!(particle.is_alive());
    }

    #[test]
    fn test_particle_lifetime() {
        let mut particle = Particle::new(0.0, 0.0, 0.0, 0.0, 1.0, 5.0, WHITE);
        assert!(particle.is_alive());
        particle.lifetime = -0.1;
        assert!(!particle.is_alive());
    }

    #[test]
    fn test_particle_emitter() {
        let emitter = ParticleEmitter::new(100.0, 100.0);
        assert_eq!(emitter.particles.len(), 0);
    }

    #[test]
    fn test_particle_system() {
        let mut system = ParticleSystem::new();
        assert_eq!(system.emitters.len(), 0);

        let emitter = ParticleEmitter::new(100.0, 100.0);
        system.add_emitter(emitter);
        assert_eq!(system.emitters.len(), 1);
    }
}
