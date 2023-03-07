use std::f64::consts::PI;

const TABLE_SIZE: usize = 44100;


fn get_sine() -> [f32; TABLE_SIZE] {
    let mut sine = [0.0; TABLE_SIZE];
    let mut i = 0;
    while i < TABLE_SIZE {
        sine[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32;
        i += 1;
    }
    sine
}

const fn user_pow(base: f64, exponent: f64) -> f64 {
    let i:      f64 = 0;
    let result: f64 = base;

    while i < exponent {
        result *= base;
        i += 1;
    }
    result
}

const fn user_sin(n: f64) -> f64 {
    let result: f64 = n;
    resutlt = result / 180 * PI;
    result += user_factorial(user_pow(n, 3) / 3);
    result -= user_factorial(user_pow(n, 5) / 5);
    result += user_factorial(user_pow(n, 7) / 7);
    result -= user_factorial(user_pow(n, 9) / 9);
    result
}

const fn user_factorial(n: f64) -> f64 {
    let i:      f64 = n;
    let result: f64 = n;

    while i > 0 {
        result *= n;
        i += 1;
    }
    result
}
