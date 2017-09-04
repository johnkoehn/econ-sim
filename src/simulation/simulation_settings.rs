use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct SimulationSettings {
    pub filename: String,
    pub food_per_worker: f64,
    pub wood_per_house: f64,
    pub workers_per_house: u32,
    pub stone_per_house: f64,
    pub wood_to_build_house: f64,
    pub stone_to_build_house: f64,
    pub avg_worker_age: u32,
    pub percent_offspring_chance: f64,
}

impl SimulationSettings {
    pub fn new(filename: &str) -> SimulationSettings {
        let mut simulation_settings = SimulationSettings {
            filename: filename.to_string(),
            food_per_worker: 1.0,
            wood_per_house: 1.0,
            workers_per_house: 2,
            stone_per_house: 1.0,
            wood_to_build_house: 1.0,
            stone_to_build_house: 1.0,
            avg_worker_age: 10,
            percent_offspring_chance: 10.0,
        };
        let mut file = File::open(filename).expect("Settings file not found!");
        let mut reader = BufReader::new(file);

        // the format of the settings file follows variable_name:variable_value
        for line in reader.lines() {
            let clean_line = line.unwrap().replace(" ", "");
            // if the string is empty ignore it
            if !clean_line.is_empty() {
                let split_line: Vec<&str> = clean_line.split(":").collect();
                let mut variable_name: &str = "";
                let mut variable_value: &str = "";

                match split_line.get(0) {
                    None => println!("Invalid data in Settings file!"),
                    Some(x) => variable_name = x,
                }
                match split_line.get(1) {
                    None => println!("Invalid data in Setting file"),
                    Some(x) => variable_value = x,
                }

                // check for bad data
                if !variable_name.is_empty() && !variable_value.is_empty() {
                    if variable_name == "food_per_worker" {
                        simulation_settings.food_per_worker = SimulationSettings::parse_float(variable_value);
                    } else if variable_name == "wood_per_house" {
                        simulation_settings.wood_per_house = SimulationSettings::parse_float(variable_value);
                    } else if variable_name == "workers_per_house" {
                        simulation_settings.workers_per_house = SimulationSettings::parse_int(variable_value);
                    } else if variable_name == "stone_per_house" {
                        simulation_settings.stone_per_house = SimulationSettings::parse_float(variable_value);
                    } else if variable_name == "wood_to_build_house" {
                        simulation_settings.wood_to_build_house = SimulationSettings::parse_float(variable_value);
                    } else if variable_name == "stone_to_build_house" {
                        simulation_settings.stone_to_build_house = SimulationSettings::parse_float(variable_value);
                    } else if variable_name == "avg_worker_age" {
                        simulation_settings.avg_worker_age = SimulationSettings::parse_int(variable_value);
                    } else if variable_name == "percent_offspring_chance" {
                        simulation_settings.percent_offspring_chance = SimulationSettings::parse_float(variable_value);
                    } else {
                        println!("Invalid Data in setting file!!! {}", variable_name);
                    }
                }
            }
        }
        simulation_settings
    }

    fn parse_float(variable_value: &str) -> f64 {
        let mut value = 0.0;
        match variable_value.parse::<f64>() {
            Err(E) => println!("Invalid variable value in settings file!! {}", variable_value),
            Ok(x) => value = x,
        }
        value
    }

    fn parse_int(variable_value: &str) -> u32 {
        let mut value = 0;
        match variable_value.parse::<u32>() {
            Err(E) => println!("Invalid variable value in settings file!! {}", variable_value),
            Ok(x) => value = x,
        }
        value
    }

}

#[cfg(test)]
mod tests {
    use simulation::simulation_settings::*;

    #[test]
    fn read_in_settings() {
        let settings = SimulationSettings::new(&"test_files/settings.txt");
        assert_eq!(settings.food_per_worker, 0.25);
        assert_eq!(settings.wood_per_house, 0.25);
        assert_eq!(settings.workers_per_house, 4);
        assert_eq!(settings.stone_per_house, 0.25);
        assert_eq!(settings.wood_to_build_house, 10.0);
        assert_eq!(settings.stone_to_build_house, 10.0);
        assert_eq!(settings.avg_worker_age, 50);
        assert_eq!(settings.percent_offspring_chance, 5.0);
    }
}