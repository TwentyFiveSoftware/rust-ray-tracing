use crate::Vector3;

pub fn random_color() -> Vector3 {
    let h: f64 = (fastrand::f64() * 360.0).floor();
    hsv_to_rgb(h, 0.75, 0.45)
}

pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Vector3 {
    let c: f64 = s * v;
    let x: f64 = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m: f64 = v - c;

    let r: f64;
    let g: f64;
    let b: f64;

    if h >= 0.0 && h < 60.0 {
        r = c;
        g = x;
        b = 0.0;
    } else if h >= 60.0 && h < 120.0 {
        r = x;
        g = c;
        b = 0.0;
    } else if h >= 120.0 && h < 180.0 {
        r = 0.0;
        g = c;
        b = x;
    } else if h >= 180.0 && h < 240.0 {
        r = 0.0;
        g = x;
        b = c;
    } else if h >= 240.0 && h < 300.0 {
        r = x;
        g = 0.0;
        b = c;
    } else {
        r = c;
        g = 0.0;
        b = x;
    }

    Vector3 { x: r + m, y: g + m, z: b + m }
}
