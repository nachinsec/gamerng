#[derive(Debug)]

pub enum VitalResource {
    Health(u32),
    Battery(u32),
}
impl VitalResource {
    pub fn value(&self) -> u32 {
        match self {
            VitalResource::Health(v) => *v,
            VitalResource::Battery(v) => *v,
        }
    }

    pub fn reduce(&mut self, amount: u32) {
        match self {
            VitalResource::Health(v) => *v = v.saturating_sub(amount),
            VitalResource::Battery(v) => *v = v.saturating_sub(amount),
        }
    }
}
#[derive(Debug)]
pub struct Stats {
    vital: VitalResource,
    energy: u32,
}

impl Stats {
    pub fn new(vital: VitalResource, energy: u32) -> Stats {
        Stats { vital, energy }
    }
    pub fn vital(&self) -> &VitalResource {
        &self.vital
    }

    pub fn vital_value(&self) -> u32 {
        self.vital.value()
    }
    pub fn energy(&self) -> u32 {
        self.energy
    }

    pub fn reduce_vital(&mut self, amount: u32) {
        self.vital.reduce(amount)
    }

    pub fn reduce_energy(&mut self, energy: u32) {
        self.energy = self.energy.saturating_sub(energy)
    }
}
