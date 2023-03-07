use crate::star_generator;

pub struct Galaxy {
    pub stars: Vec<star_generator::Star>,
}

impl Galaxy {
    pub fn new(number_of_stars: usize) -> Galaxy {
        let mut stars = Vec::new();

        for i in 1..=number_of_stars {
            stars.push(star_generator::Star::new(i as u64));
        }

        Galaxy { stars }
    }

    pub fn get_info(&self) -> String{
        let mut s = String::new();
        for i in 0..self.stars.len() {
            s.push_str(&self.stars[i].class.to_string());
            s.push_str("_");
            s.push_str(&i.to_string());
            s.push_str(" ");
        }
        format!("{}", s)
    }
}
