use super::vec3::Vec3;

pub struct ONB {
    pub axis: [Vec3; 3],
}

impl ONB {
    pub fn u(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    pub fn build_from_w(n: &Vec3) -> Self {
        let mut new_onb = Self {
            axis: Default::default(),
        };
        new_onb.axis[2] = n.to_unit();
        let a = if new_onb.axis[2][0].abs() > 0.9 {
            Vec3::new(0., 1., 0.)
        } else {
            Vec3::new(1., 0., 0.)
        };

        new_onb.axis[1] = Vec3::cross(&a, &new_onb.axis[2]).to_unit();
        new_onb.axis[0] = Vec3::cross(&new_onb.axis[1], &new_onb.axis[2]);

        new_onb
    }

    pub fn local(&self, a: &Vec3) -> Vec3 {
        self.axis[0] * a[0] + self.axis[1] * a[1] + self.axis[2] * a[2]
    }
}
