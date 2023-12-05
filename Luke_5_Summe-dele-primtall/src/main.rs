fn sjekk_for_primtall(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn summer_siffer(mut n: u64) -> u64 {
    let mut sum: u64 = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    return sum;
}

fn main() {
    let mut kode: i32 = 0;
    for i in 1..=100000000 {
        let siffer_sum: u64 = summer_siffer(i);
        if siffer_sum != 0 && i % siffer_sum == 0 && sjekk_for_primtall(i / siffer_sum) {
            kode += 1;
        }
    }
    println!("Koden er {}", kode)
}
