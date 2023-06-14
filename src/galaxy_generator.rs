use std::fmt::format;
use crate::game_manager::{Scene, SceneName};
use crate::star_generator;

pub struct Galaxy {
    stars: Vec<star_generator::Star>,
}

impl Galaxy {
    pub fn new(number_of_stars: u64) -> Galaxy {
        let mut stars = Vec::new();

        for i in 0..number_of_stars {
            stars.push(star_generator::Star::new(i as u64));
        }

        Galaxy { stars }
    }
}

impl Scene for Galaxy {
    fn get_scene_name(&self) -> SceneName {
        SceneName::Galaxy
    }
    
    fn get_system_info(&self) -> String {
        let mut s = String::new();
        for i in 0..self.stars.len() {
            s.push_str(&self.stars[i].get_class().to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("{}", s)
    }

    fn get_unit_info(&self, index: usize) -> String {
        if index < self.stars.len() {
            self.stars[index].get_info()
        }
        else {
            format!("Invalid coordinates.")
        }
    }
}
