use super::particle::Particle;

pub trait ParticleVec {
    fn add_vec(&mut self, particles: &Vec<Particle>);
}

pub trait Solver {
    fn add_vec(&mut self, particles: &Vec<Particle>);
}

pub struct ParticleSystem<PV, S> 
where
    PV: ParticleVec,
{
    particle_vec: PV,
    solver: S,
}

impl<PV, S> ParticleSystem<PV, S>
where
    PV: ParticleVec,
{

    fn add_particles(&mut self, particles: &Vec<Particle>) {
        self.particle_vec.add_vec(particles);
    }
}