enum FuelType {
    Petrol,
    Diesel,
    Electric,
}

struct Vehicle {
    brand: String,
    model: String,
    fuel_type: FuelType,
}

fn filter_electric(vehicles: Vec<Vehicle>) {
    for v in vehicles {
        if let FuelType::Electric = v.fuel_type {
            println!("Electric Vehicle: {} {}", v.brand, v.model);
        }
    }
}

fn main() {
    let vehicles = vec![
        Vehicle { brand: "Tesla".to_string(), model: "Model 3".to_string(), fuel_type: FuelType::Electric },
        Vehicle { brand: "Ford".to_string(), model: "F-150".to_string(), fuel_type: FuelType::Petrol },
    ];

    filter_electric(vehicles);
}
