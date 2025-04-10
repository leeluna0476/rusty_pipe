// (°F − 32) × 5/9 = °C
// °F = °C * 9/5 + 32

fn main() {
    let mut c = -100.0;
    while c <= 100.0 {
        println!("{c}°C\t{}°F", celsius_to_fahrenheit(c));
        c += 1.0;
    }

    let mut f = -100.0;
    while f <= 100.0 {
        println!("{f}°F\t{}°C", fahrenheit_to_celsius(f));
        f += 1.0;
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
