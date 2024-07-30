pub struct Field {
    scale: f32,
    xpos: f32,
    ypos: f32,
    scale_down: f32,
    scale_up: f32,
    homex     : f32,
    homey     : f32,
}

impl Field {
    pub fn new(
        scale_down: f32,
        scale_up  : f32,
        homex     : f32,
        homey     : f32,
    ) -> Self {
        Self {
            scale: 1.0,
            xpos: homex,
            ypos: homey,
            scale_down,
            scale_up,
            homex,
            homey,
        }
    }

    pub fn home(&mut self) {
        *self = Self::new(
            self.scale_down,
            self.scale_up,
            self.homex,
            self.homey,
        );
    }

    pub fn shift(&mut self, xrel: f32, yrel: f32) {
        self.xpos += xrel;
        self.ypos += yrel;
    }

    fn clip(&self, val: f32) -> f32 {
        // if self.scale_down < val { return self.scale_down; }
        // if val < self.scale_up { return self.scale_up; }
        val
    }

    pub fn scale(&mut self, x: f32, y: f32, scale_inc: f32) {
        let mut new_scale = self.scale + self.scale*scale_inc;
        // if self.scale_up < new_scale { new_scale = self.scale_up; }
        new_scale = self.clip(new_scale);

        self.xpos = (self.xpos-x)*(new_scale/self.scale) + x;
        self.ypos = (self.ypos-y)*(new_scale/self.scale) + y;
        self.scale = new_scale;
    }

    pub fn get_xpos(&self) -> f32 { self.xpos }
    pub fn get_ypos(&self) -> f32 { self.ypos }
    pub fn get_scale(&self) -> f32 { self.scale }
}
