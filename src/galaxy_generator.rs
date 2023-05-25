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

    pub fn get_galaxy_info(&self) -> String{
        let mut s = String::new();
        for i in 0..self.stars.len() {
            s.push_str(&self.stars[i].get_class().to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("{}", s)
    }

    pub fn get_star_info(&self, index: usize) -> String {
        if index < self.stars.len() {
            self.stars[index].get_info()
        }
        else {
            format!("Invalid coordinates.")
        }
    }

    pub fn get_star(&mut self, index: usize) -> star_generator::Star {
        if index < self.stars.len() {
            self.stars.remove(index)
        }
        else {
            self.stars.remove(0)
        }
    }
}
