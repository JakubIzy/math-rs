pub struct Triangle {
    angle1: f64,
    angle2: f64,
    angle3: f64,
    side1: f64,
    side2: f64,
    side3: f64
}

impl Triangle {
    pub fn new(angle1: f64, angle2: f64, angle3: f64, side1: f64, side2: f64, side3: f64) -> Triangle {
        Triangle {
            angle1: angle1,
            angle2: angle2,
            angle3: angle3,
            side1: side1,
            side2: side2,
            side3: side3
        }
    }
    
    pub fn check(&self) -> bool {
        if  self.side1 + self.side2 >= self.side3 ||
            self.side2 + self.side3 >= self.side1 ||
            self.side3 + self.side1 >= self.side2 ||
            self.angle1 + self.angle2 + self.angle3 != 180 as f64
        {
            false
        } else {
            true
        }
    }

    pub fn solve(&mut self) -> bool {
        // Compute angles first
        if
            self.angle1 == 0 as f64 &&
            self.angle2 != 0 as f64 &&
            self.angle3 != 0 as f64
        {
            self.angle1 = (180 as f64)-self.angle2-self.angle3;

        } else if
            self.angle1 != 0 as f64 &&
            self.angle2 == 0 as f64 &&
            self.angle3 != 0 as f64
        {
            self.angle2 = (180 as f64)-self.angle1-self.angle3;

        } else if
            self.angle1 != 0 as f64 &&
            self.angle2 != 0 as f64 &&
            self.angle3 == 0 as f64
        {
            self.angle3 = (180 as f64)-self.angle1-self.angle2;

        }
        // Main code
        if
            self.angle1 != 0 as f64 &&
            self.angle2 != 0 as f64 &&
            self.angle3 != 0 as f64
        {
            if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                true

            } else if
                self.side1 == 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.side1 = (self.side2/self.angle3.sin())*self.angle2.sin();
                true
            } else if
                self.side1 != 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.side2 = (self.side1/self.angle2.sin())*self.angle3.sin();
                true
            } else if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 == 0 as f64
            {
                self.side3 = (self.side1/self.angle2.sin())*self.angle1.sin();
                true
            } else if
                self.side1 == 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.side1 = (self.side3/self.angle1.sin())*self.angle2.sin();
                self.side2 = (self.side3/self.angle1.sin())*self.angle3.sin();
                true
            } else if
                self.side1 != 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 == 0 as f64
            {
                self.side2 = (self.side1/self.angle2.sin())*self.angle3.sin();
                self.side3 = (self.side1/self.angle2.sin())*self.angle1.sin();
                true
            } else if
                self.side1 == 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 == 0 as f64
            {
                self.side1 = (self.side2/self.angle3.sin())*self.angle2.sin();
                self.side3 = (self.side2/self.angle3.sin())*self.angle1.sin();
                true
            } else {false}
        } else if
            self.angle1 == 0 as f64 &&
            self.angle2 == 0 as f64 &&
            self.angle3 != 0 as f64
        {
            if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.angle1 = ((self.side3/self.side2)*self.angle3.sin()).asin();
                self.angle2 = ((self.side1/self.side2)*self.angle3.sin()).asin();
                true
            } else if
                self.side1 == 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 == 0 as f64
            {
                todo!()
            } else {false}
        } else if
            self.angle1 != 0 as f64 &&
            self.angle2 == 0 as f64 &&
            self.angle3 == 0 as f64
        {
            if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.angle2 = ((self.side1/self.side3)*self.angle1.sin()).asin();
                self.angle3 = ((self.side2/self.side3)*self.angle1.sin()).asin();
                true
            } else if
                self.side1 == 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 == 0 as f64
            {
                todo!()
            } else {false}
        } else if
            self.angle1 == 0 as f64 &&
            self.angle2 != 0 as f64 &&
            self.angle3 == 0 as f64
        {
            if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.angle1 = ((self.side3/self.side1)*self.angle2.sin()).asin();
                self.angle3 = ((self.side2/self.side1)*self.angle2.sin()).asin();
                true
            } else if
                self.side1 == 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 == 0 as f64 &&
                self.side3 != 0 as f64
            {
                todo!()
            } else if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 == 0 as f64
            {
                todo!()
            } else {false}
        } else if
            self.angle1 == 0 as f64 &&
            self.angle2 == 0 as f64 &&
            self.angle3 == 0 as f64
        {
            if
                self.side1 != 0 as f64 &&
                self.side2 != 0 as f64 &&
                self.side3 != 0 as f64
            {
                self.angle1 = ((self.side1.powi(2)+self.side2.powi(2)-self.side3.powi(2))/(2.0*self.side1*self.side2)).acos();
                self.angle2 = ((self.side2.powi(2)+self.side3.powi(2)-self.side1.powi(2))/(2.0*self.side2*self.side3)).acos();
                self.angle3 = ((self.side1.powi(2)+self.side3.powi(2)-self.side2.powi(2))/(2.0*self.side1*self.side3)).acos();
                true
            } else {false}
        } else {false}
    }
}