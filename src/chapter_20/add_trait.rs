use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
            gdp,
        }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            name: format!("{} and {}", self.name, rhs.name),
            population: self.population + rhs.population,
            gdp: self.gdp + rhs.gdp,
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

pub fn test_add_trait() {
    let nauru = Country::new("Nauru", 12_511, 133_200_000);
    let vanuatu = Country::new("Vanuatu", 219_137, 956_300_000);
    let micronesia = Country::new("Micronesia", 113_131, 404_000_000);

    println!("{}", nauru);
    let nauru_and_vanuatu = nauru + vanuatu;
    println!("{}", nauru_and_vanuatu);
    println!("{}", nauru_and_vanuatu + micronesia);
}