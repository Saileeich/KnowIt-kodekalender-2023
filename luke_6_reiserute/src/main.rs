use std::fs;

fn main() {
    let data: String = fs::read_to_string("rute.txt").unwrap();
    let koordinater: Vec<&str> = data.lines().collect();
    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();
    for koordinat in koordinater.iter() {
        let ind_kor: Vec<&str> = koordinat.split(",").collect();
        x.push(ind_kor[0].parse::<i32>().unwrap());
        y.push(ind_kor[1].parse::<i32>().unwrap());
    }

    let mut tot_avstand: i32 = 0;
    for i in 0..x.len() {
        if i == 0 {
            tot_avstand += regn_avstanden(x[i], y[i], x[0], y[0]);
        } else {
            tot_avstand += regn_avstanden(x[i], y[i], x[i-1], y[i-1]);
        }
    }

    println!(
        "Julenissen reiser totalt {:?}km",
        tot_avstand
    );
    println!(
        "Det vil si at nissen trenger {:?}kg lyng og lav for å mate reinsdyrene sine.",
        ((tot_avstand) * 2 / 1000)
    );
}

fn regn_avstanden(x1: i32, y1: i32, x2: i32, y2: i32) -> i32{
    return (x1 - x2).abs() + (y1 - y2).abs();
}
