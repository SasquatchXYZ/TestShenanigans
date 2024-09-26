pub fn test_arrays_have_iterators() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];

    for city in my_cities {
        println!("{city}");
    }

    for city in &my_cities {
        println!("{city}")
    }

    for city in my_cities.iter() {
        println!("{city}")
    }
}