use csv::Reader;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct House {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: i64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: i64,
    tax: i64,
    ptratio: f64,
    b: f64,
    lstat: f64,
    medv: f64,
}

fn main() {
    let mut reader = Reader::from_path("examples/boston/boston.csv").unwrap();
    let houses: Vec<House> = reader.deserialize().collect::<Result<_, _>>().unwrap();
}
