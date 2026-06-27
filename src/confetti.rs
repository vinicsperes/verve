use rand::Rng;

pub const DURATION: f32 = 3.5;

const GLYPHS: &[char] = &['*', '+', '\u{2726}', '\u{25cf}', '\u{2727}', '\u{00b7}'];

const COLORS: &[(u8, u8, u8)] = &[
    (215, 119, 87),
    (224, 108, 117),
    (229, 192, 123),
    (152, 195, 121),
    (97, 175, 239),
    (198, 120, 221),
    (86, 182, 194),
];

pub struct Particle {
    pub x: f32,
    pub y0: f32,
    pub vy: f32,
    pub sway_amp: f32,
    pub sway_freq: f32,
    pub color: (u8, u8, u8),
    pub glyph: char,
}

impl Particle {
    pub fn position(&self, t: f32) -> (f32, f32) {
        let y = self.y0 + self.vy * t;
        let x = self.x + self.sway_amp * (t * self.sway_freq).sin();
        (x, y)
    }
}

pub fn burst(count: usize) -> Vec<Particle> {
    let mut rng = rand::thread_rng();
    (0..count)
        .map(|_| Particle {
            x: rng.gen_range(0.0..1.0),
            y0: rng.gen_range(-0.7..0.0),
            vy: rng.gen_range(0.45..1.25),
            sway_amp: rng.gen_range(0.0..0.05),
            sway_freq: rng.gen_range(2.0..6.0),
            color: COLORS[rng.gen_range(0..COLORS.len())],
            glyph: GLYPHS[rng.gen_range(0..GLYPHS.len())],
        })
        .collect()
}
