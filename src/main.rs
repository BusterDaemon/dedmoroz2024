mod haversine;
mod gifts;

fn main() {
    println!(
        "{}",
        haversine::geo_distance(110.114, 44.427, -71.3593, 53.568)
    );
    let a = gifts::read_gifts("./gifts.csv");
    println!("{:?}", a);
}
