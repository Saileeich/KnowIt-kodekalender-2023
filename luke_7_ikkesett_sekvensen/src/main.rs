use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("Det er {:?} ikkesette tall mellom 0 og 100.000", regn_antall_ikkesett_tall(0, 100_000));
    let duration = start.elapsed();
    println!("Tid brukt: {:?}", duration);
}

fn regn_antall_ikkesett_tall(start: i32, slutt: i32) -> i32 {
    let mut tallsekvens: String = String::new();
    let mut antall: i32 = 0;
    for i in start..slutt {
        if !tallsekvens.contains(&i.to_string()) {
            antall += 1;
            tallsekvens += &i.to_string();
        }
    }
    return antall;
}
