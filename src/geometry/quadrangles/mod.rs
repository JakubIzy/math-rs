pub struct Quadrangle {
    angle1: f64,
    angle2: f64,
    angle3: f64,
    angle4: f64,
    side1: f64,
    side2: f64,
    side3: f64,
    side4: f64
}

impl Quadrangle {
    pub fn new(angle1: f64, angle2: f64, angle3: f64, angle4: f64, side1: f64, side2: f64, side3: f64, side4: f64) -> Quadrangle {
        Quadrangle {
            angle1: angle1,
            angle2: angle2,
            angle3: angle3,
            angle4: angle4,
            side1: side1,
            side2: side2,
            side3: side3,
            side4: side4
        }
    }

    pub fn check(&self) -> bool {
        if self.angle1 + self.angle2 + self.angle3 + self.angle4 != 360 as f64 ||
            self.side1 >= self.side2 + self.side3 + self.side4 ||
            self.side2 >= self.side1 + self.side3 + self.side4 ||
            self.side3 >= self.side1 + self.side2 + self.side4 ||
            self.side4 >= self.side1 + self.side2 + self.side3 {
                false
        } else {true}
    }

    pub fn area() -> f64 {
        todo!()
    }

    pub fn perimeter(&self) -> f64 {
        self.side1 + self.side2 + self.side3 + self.side4
    }
}