fn print_country(country: &str) {
    println!("We are in the country of {country}");
}
pub mod province {
    fn print_province(province: &str) {
        println!("in the province of {province}");
    }
    pub mod city {
        pub fn print_city(country: &str, province: &str, city: &str) {
            crate::country::print_country(country);
            super::super::print_country(country);

            crate::country::province::print_province(province);
            super::print_province(province);
            println!("in the city of {city}");
        }
    }
}
