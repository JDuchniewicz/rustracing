use rand::Rng;
use std::fmt;
use std::ops;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

macro_rules! impl_binop {
    (VEC, $op_trait: ident, $fn_name: ident, $op:tt, $target: ident, $rhs: ident) => {
        impl std::ops::$op_trait<$rhs> for $target {
            type Output = $target;


            fn $fn_name(self, rhs: $rhs) -> Self::Output {
                $target {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }
    };

    (SCALAR, $op_trait: ident, $fn_name: ident, $op:tt, $target: ident, $rhs: ident) => {
        impl std::ops::$op_trait<$rhs> for $target {
            type Output = $target;


            fn $fn_name(self, rhs:  $rhs) -> Self::Output {
                $target {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        impl std::ops::$op_trait<$target> for $rhs {
            type Output = $target;


            fn $fn_name(self, rhs:  $target) -> Self::Output {
                $target {
                    x: rhs.x $op self,
                    y: rhs.y $op self,
                    z: rhs.z $op self,
                }
            }
        }
    };
}

impl_binop!(VEC, Add, add, +, Vec3, Vec3);
impl_binop!(VEC, Sub, sub, -, Vec3, Vec3);
impl_binop!(VEC, Mul, mul, *, Vec3, Vec3);

impl_binop!(SCALAR, Mul, mul, *, Vec3, f64);
impl_binop!(SCALAR, Div, div, /, Vec3, f64);

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    pub fn dot(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    pub fn vec3_random(rng: &mut impl rand::Rng) -> Vec3 {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn vec3_random_range(rng: &mut impl rand::Rng, range: std::ops::Range<f64>) -> Vec3 {
        Vec3::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }

    pub fn random_in_unit_sphere(rng: &mut impl rand::Rng) -> Vec3 {
        loop {
            let p = Vec3::vec3_random_range(rng, -1.0..1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(rng: &mut impl rand::Rng, normal: Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere(rng);
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();

        // TODO: this seems suboptimal, is it used often?
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector(rng: &mut impl rand::Rng) -> Vec3 {
        Vec3::random_in_unit_sphere(rng).normalize()
    }

    pub fn near_zero(&self) -> bool {
        // TODO: check if is_normal() is not enough
        const S: f64 = 1.0e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let uv = self;
        let cos_theta = (-uv.dot(n)).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
        r_out_perp + r_out_parallel
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
