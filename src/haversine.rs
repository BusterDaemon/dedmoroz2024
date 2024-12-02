fn rads(x: f64) -> f64 {
    let pi = 3.14159265358979323846;
    let angle = 180.0;
    x * (pi / angle)
}

pub fn geo_distance(lon1: f64, mut lat1: f64, lon2: f64, mut lat2: f64) -> f64 {
    let earth_radius = 6371.0;
    let dlon = rads(lon2) - rads(lon1);
    let dlat = rads(lat2) - rads(lat1);
    lat1 = rads(lat1);
    lat2 = rads(lat2);
    let a = (dlat / 2.0).sin().powf(2.0) + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powf(2.0);
    let c = 2.0 * a.sqrt().asin();

    c * earth_radius
}
