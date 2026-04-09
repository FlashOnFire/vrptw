pub struct vehicule {
    capacity: u8,
    charge: u8, // charge of the vehicule at the moment, charge never exceed capacity
    route: Vec<u8>, // client ids
}
