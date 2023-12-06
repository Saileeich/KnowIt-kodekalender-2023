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

    let mut tot_avstand: f32 = 0.0;
    for i in 1..x.len() {
        tot_avstand += regn_avstanden(x[i], y[i], x[i-1], y[i-1]);
    }

    println!(
        "Julenissen reiser totalt {:?}km. Da trenger han minst {:?}kg lyng og lav for å mate reinsdyrene sine. Han vil derfor ta med {:?}kg lyng og lav.",
        tot_avstand,
        ((tot_avstand * 9.0) / 1000.0),
        ((tot_avstand * 9.0) / 1000.0).ceil() as i32
    );
}

fn regn_avstanden(x1: i32, y1: i32, x2: i32, y2: i32) -> f32{
    let dif_x: f32 = (x1 - x2).abs() as f32;
    let dif_y: f32 = (y1 - y2).abs() as f32;
    return ((dif_x.powf(2.0)) + (dif_y.powf(2.0))).sqrt();
}
