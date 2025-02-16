pub struct Bdimentions {
    pub unit_size: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

impl Bdimentions {
    pub fn new() -> Bdimentions {
        let width = 1280;
        let height = 720;

        Bdimentions {
            unit_size: height / 22,
            left: (width / 2) - (5 * (height / 22)),
            right: (width / 2) + (5 * (height / 22)),
            bottom: height - (height / 22),
            top: height - (21 * (height / 22)),
            width,
            height,
        }
    }
}

impl Default for Bdimentions {
    fn default() -> Self {
        Self::new()
    }
}
