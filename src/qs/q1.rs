pub struct Spaceship {
    modules: Vec<Module>
}

impl Spaceship {

    pub fn naive_fuel_cost(&self) -> u64 {
        self.modules.iter().map(|m | m.naive_fuel_cost()).sum()
    }

    pub fn fuel_cost(&self) -> u64 {
        self.modules.iter().map(|m | m.fuel_cost()).sum()
    }
}

pub struct Module {
    mass: u64
}

impl Module {
    
    pub fn naive_fuel_cost(&self) -> u64 {
        (self.mass - 6)/3
    }

    pub fn fuel_cost(&self) -> u64 {
        let mut cost: u64 = 0;
        let mut m = self.mass;
        while m > 8 {
            m = (m-6)/3;
            cost += m;
        }
        cost
    }
}

pub fn get_modules_from_masses(masses: Vec<u64>) -> Vec<Module> {
    masses.iter().map(|m| Module{ mass: *m}).collect()
}

pub fn get_masses_from_string(int_string: String) -> Vec<u64> {
    let mut masses = Vec::new();
    for i in int_string.split_whitespace(){
        let j: Option<u64> = i.parse().map_or_else(|_| None, |ok| Some(ok));
        if let true = j.is_some() {
            masses.push(j.unwrap());
        }
    }
    masses
}

    pub fn q1a(masses: String) {

        let spaceship = Spaceship{
            modules: get_modules_from_masses(get_masses_from_string(masses))
        };
        println!("Total fuel cost: {}", spaceship.naive_fuel_cost());
    }

    pub fn q1b(masses: String) {

        let spaceship = Spaceship{
            modules: get_modules_from_masses(get_masses_from_string(masses))
        };
        println!("Total fuel cost: {}", spaceship.fuel_cost());
    }
