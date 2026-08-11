const BRAILLE_BASE: u32 = 0x2800;

const DOT_BITS: [[u8; 4]; 2] = [[0x01, 0x02, 0x04, 0x40], [0x08, 0x10, 0x20, 0x80]];

const SPIN_PER_FRAME: f32 = 0.06;
const TILT: f32 = 0.45;
const PERSPECTIVE: f32 = 0.22;

pub struct Cloud {
    points: Vec<[f32; 3]>,
}

impl Cloud {
    pub fn cube(per_edge: usize) -> Self {
        let corner = |i: usize| {
            [
                if i & 1 == 0 { -1.0 } else { 1.0 },
                if i & 2 == 0 { -1.0 } else { 1.0 },
                if i & 4 == 0 { -1.0 } else { 1.0 },
            ]
        };

        let mut points = Vec::new();
        for a in 0..8usize {
            for b in (a + 1)..8usize {
                if (a ^ b).count_ones() != 1 {
                    continue;
                }
                let (p, q): ([f32; 3], [f32; 3]) = (corner(a), corner(b));
                for s in 0..per_edge {
                    let t = s as f32 / per_edge as f32;
                    points.push([
                        p[0] + (q[0] - p[0]) * t,
                        p[1] + (q[1] - p[1]) * t,
                        p[2] + (q[2] - p[2]) * t,
                    ]);
                }
            }
        }

        Cloud { points }
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    pub fn render(&self, frame: u64, width: usize, height: usize) -> String {
        let (dots_w, dots_h) = (width * 2, height * 4);

        let mut near = vec![0u8; width * height];
        let mut far = vec![0u8; width * height];

        let (sin_y, cos_y) = (frame as f32 * SPIN_PER_FRAME).sin_cos();
        let (sin_x, cos_x) = TILT.sin_cos();

        let scale = dots_h as f32 * 0.30;
        let (cx, cy) = (dots_w as f32 / 2.0, dots_h as f32 / 2.0);

        for p in &self.points {
            let x = p[0] * cos_y + p[2] * sin_y;
            let z = p[2] * cos_y - p[0] * sin_y;
            let y = p[1] * cos_x - z * sin_x;
            let z = p[1] * sin_x + z * cos_x;

            let k = 1.0 / (1.0 + z * PERSPECTIVE);
            let px = (cx + x * scale * k).round();
            let py = (cy + y * scale * k).round();

            if px < 0.0 || py < 0.0 || px >= dots_w as f32 || py >= dots_h as f32 {
                continue;
            }
            let (px, py) = (px as usize, py as usize);

            let layer = if z <= 0.0 { &mut near } else { &mut far };
            layer[(py / 4) * width + (px / 2)] |= DOT_BITS[px % 2][py % 4];
        }

        let mut out = String::new();
        for row in 0..height {
            for col in 0..width {
                let i = row * width + col;
                match (near[i], far[i]) {
                    (0, 0) => out.push(' '),
                    (0, f) => out.push_str(&format!("\x1B[2m{}\x1B[0m", braille(f))),
                    (n, f) => out.push(braille(n | f)),
                }
            }
            out.push('\n');
        }
        out
    }
}

fn braille(mask: u8) -> char {
    char::from_u32(BRAILLE_BASE + mask as u32).unwrap()
}
