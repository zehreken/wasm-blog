use macroquad::rand::rand;

type SimulationResult = (
    (usize, Option<Box<dyn Particle>>),
    (usize, Option<Box<dyn Particle>>),
);

#[derive(Clone)]
pub struct ParticleProperties {
    pub kind: u8,
    color: u32,
    pub density: u8,
    // pub flammable: bool,
}

pub trait Particle: ParticleClone {
    // POST: Explain why you have 'where Self: Sized'
    fn new() -> Self
    where
        Self: Sized;
    fn get_properties(&self) -> &ParticleProperties;
    fn update(
        &self,
        index: usize,
        width: u32,
        height: u32,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult>;
}

// POST: Explain clone implemention for traits
pub trait ParticleClone {
    fn clone_box(&self) -> Box<dyn Particle>;
}

impl<T: 'static + Particle + Clone> ParticleClone for T {
    fn clone_box(&self) -> Box<dyn Particle> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Particle> {
    fn clone(&self) -> Box<dyn Particle> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Sand {
    properties: ParticleProperties,
}

impl Particle for Sand {
    fn new() -> Sand {
        Sand {
            properties: ParticleProperties::sand(),
        }
    }

    fn get_properties(&self) -> &ParticleProperties {
        &self.properties
    }

    fn update(
        &self,
        index: usize,
        width: u32,
        height: u32,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult> {
        let mut result = None;
        if let Some(below) = get_index_down(index, width as usize, height as usize) {
            if let Some(below_p) = &particles[below] {
                if below_p.get_properties().density < self.get_properties().density {
                    let temp = self.clone();
                    let sim_res: SimulationResult = (
                        (index, Some(below_p.clone())),
                        (below, Some(Box::new(temp))),
                    );
                    result = Some(sim_res);
                } else {
                    if let Some(left_to_below) =
                        get_index_left(below, width as usize, height as usize)
                    {
                        if let Some(left_to_below_p) = &particles[left_to_below] {
                            if left_to_below_p.get_properties().density
                                < self.get_properties().density
                            {
                                let temp = self.clone();
                                let sim_res: SimulationResult = (
                                    (index, Some(left_to_below_p.clone())),
                                    (left_to_below, Some(Box::new(temp))),
                                );
                                result = Some(sim_res);
                            }
                        } else {
                            let temp = self.clone();
                            let sim_res: SimulationResult =
                                ((index, None), (left_to_below, Some(Box::new(temp))));
                            result = Some(sim_res);
                        }
                    }
                    if let Some(right_to_below) =
                        get_index_right(below, width as usize, height as usize)
                    {
                        if let Some(right_to_below_p) = &particles[right_to_below] {
                            if right_to_below_p.get_properties().density
                                < self.get_properties().density
                            {
                                let temp = self.clone();
                                let sim_res: SimulationResult = (
                                    (index, Some(right_to_below_p.clone())),
                                    (right_to_below, Some(Box::new(temp))),
                                );
                                result = Some(sim_res);
                            }
                        } else {
                            let temp = self.clone();
                            let sim_res: SimulationResult =
                                ((index, None), (right_to_below, Some(Box::new(temp))));
                            result = Some(sim_res);
                        }
                    }
                }
            } else {
                // If there is nothing below, just fall
                let temp = self.clone();
                let sim_res: SimulationResult = ((index, None), (below, Some(Box::new(temp))));
                result = Some(sim_res);
            }
        }

        result
    }
}

#[derive(Clone)]
pub struct Water {
    properties: ParticleProperties,
}

impl Particle for Water {
    fn new() -> Water {
        Water {
            properties: ParticleProperties::water(),
        }
    }

    fn get_properties(&self) -> &ParticleProperties {
        &self.properties
    }

    fn update(
        &self,
        index: usize,
        width: u32,
        height: u32,
        particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult> {
        let mut result = None;
        if let Some(below) = get_index_down(index, width as usize, height as usize) {
            if let Some(_) = &particles[below] {
                let mut moved = false;
                if let Some(left) = get_index_left(index, width as usize, height as usize) {
                    if let None = &particles[left] {
                        let temp = self.clone();
                        let sim_res: SimulationResult =
                            ((index, None), (left, Some(Box::new(temp))));
                        result = Some(sim_res);
                        moved = true;
                    }
                }
                if !moved {
                    if let Some(right) = get_index_right(index, width as usize, height as usize) {
                        if let None = &particles[right] {
                            let temp = self.clone();
                            let sim_res: SimulationResult =
                                ((index, None), (right, Some(Box::new(temp))));
                            result = Some(sim_res);
                        }
                    }
                }
            } else {
                // If there is nothing below, just fall
                let temp = self.clone();
                let sim_res: SimulationResult = ((index, None), (below, Some(Box::new(temp))));
                result = Some(sim_res);
            }
        }

        result
    }
}

#[derive(Clone)]
pub struct Rock {
    pub properties: ParticleProperties,
}

impl Particle for Rock {
    fn new() -> Rock {
        Rock {
            properties: ParticleProperties::rock(),
        }
    }
    fn get_properties(&self) -> &ParticleProperties {
        &self.properties
    }
    fn update(
        &self,
        _index: usize,
        width: u32,
        height: u32,
        _particles: &Vec<Option<Box<dyn Particle>>>,
    ) -> Option<SimulationResult> {
        None
    }
}

impl ParticleProperties {
    pub fn sand() -> Self {
        ParticleProperties {
            kind: 0,
            color: if rand() % 2 == 0 { 0xFFDEAD } else { 0xCCB28A },
            density: 10,
        }
    }

    pub fn water() -> Self {
        ParticleProperties {
            kind: 1,
            color: if rand() % 2 == 0 { 0x0EBFE9 } else { 0x0B99BA },
            density: 1,
        }
    }

    pub fn rock() -> Self {
        ParticleProperties {
            kind: 2,
            color: if rand() % 2 == 0 { 0x595959 } else { 0x797979 },
            density: 10,
        }
    }

    pub fn color(&self) -> u32 {
        return match self.kind {
            1 => {
                if rand() % 2 == 0 {
                    0x0EBFE9
                } else {
                    0x0B99BA
                }
            }
            _ => self.color,
        };
    }
}

pub struct ParticleModel {
    pub width: u32,
    pub height: u32,
    pub particles: Vec<Option<ParticleProperties>>,
    pub _particles: Vec<Option<Box<dyn Particle>>>,
}

impl ParticleModel {
    pub fn new(width: u32, height: u32) -> Self {
        ParticleModel {
            width,
            height,
            particles: vec![None; width as usize * height as usize],
            _particles: vec![None; width as usize * height as usize],
        }
    }

    pub fn update(&mut self) {
        // Simulate from bottom to top
        for i in (0..self._particles.len()).rev() {
            if let Some(p) = &self._particles[i] {
                let result = p.update(i, self.width, self.height, &self._particles);
                if let Some(r) = result {
                    self._particles[(r.0).0] = (r.0).1;
                    self._particles[(r.1).0] = (r.1).1;
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self._particles = vec![None; self.width as usize * self.height as usize]
    }
}

fn get_index_down(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index + width;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}

fn get_index_left(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index - 1;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}

fn get_index_right(index: usize, width: usize, height: usize) -> Option<usize> {
    let index = index + 1;
    if index < width * height {
        Some(index)
    } else {
        None
    }
}
