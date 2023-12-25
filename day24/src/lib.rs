/// Inspired by https://tamaskis.github.io/files/Intersection_of_Two_Lines.pdf
#[derive(Debug, Default, Clone)]
pub struct Line {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub m_y: f64, // v_y / v_x
    pub m_z: f64, // v_z / v_x
    pub dir: (f64, f64, f64),
}

pub fn intersect(line1: &Line, line2: &Line) -> Option<(f64, f64, f64)> {
    // Parallel or invalid
    if line1.m_y == line2.m_y
        || line1.m_y.is_nan()
        || line1.m_y.is_infinite()
        || line2.m_y.is_nan()
        || line2.m_y.is_infinite()
    {
        return None;
    }

    // Compute intersection point
    // (line1.v_y/line1.v_x * line1.x - line2.v_y/line2.v_x * line2.x - (line1.y - line2.y)) / (line1.v_y / line1.v_x - line2.v_y / line2.v_x)
    let x_int =
        (line1.m_y * line1.x - line2.m_y * line2.x - (line1.y - line2.y)) / (line1.m_y - line2.m_y);

    let y_int = line1.y + line1.m_y * (x_int - line1.x);

    let z_int = line1.z + line1.m_z * (x_int - line1.x);

    if x_int.is_nan() {
        dbg!(x_int, line1, line2);
    }

    Some((x_int, y_int, z_int))
}

pub fn parse_input(input: &str) -> Vec<Line> {
    let mut lines = Vec::new();
    for line in input.lines().map(str::trim) {
        let (p, v) = line.split_once('@').unwrap();
        let mut ps = p.split(',').map(|v| v.trim().parse::<f64>().unwrap());
        let mut vs = v.split(',').map(|v| v.trim().parse::<f64>().unwrap());

        let p_x = ps.next().unwrap();
        let p_y = ps.next().unwrap();
        let p_z = ps.next().unwrap();
        let v_x = vs.next().unwrap();
        let v_y = vs.next().unwrap();
        let v_z = vs.next().unwrap();

        lines.push(Line {
            x: p_x,
            y: p_y,
            z: p_z,
            m_y: round(v_y / v_x),
            m_z: round(v_z / v_x),
            dir: (v_x, v_y, v_z),
        })
    }

    lines
}

/// Normalize to 2 decimal places
#[inline]
fn round(v: f64) -> f64 {
    f64::trunc(v * 1000.0) / 1000.0
}
