use crate::{rand_f64, Color, Hit, Ray, SolidColor, Texture, Vec3};

pub trait Material {
    /// Returns the resulting ray and attenuation color.
    // Accept a `Hit` to avoid passing a lot of parameters.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)>;

    /// Emitted light.
    // This could be part of the hit record instead.
    fn emit(&self, _u: f64, _v: f64, _point: Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

pub struct Metal {
    pub albedo: Color,
    /// Should be in the range [0.0, 1.0].
    pub fuzz: f64,
}

pub struct Dialectric {
    /// Refraction index.
    pub ri: f64,
}

pub struct DiffuseLight {
    pub emit: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Box::new(SolidColor { color }),
        }
    }

    pub fn textured(albedo: Box<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self {
            emit: Box::new(SolidColor { color }),
        }
    }
}

impl Material for Lambertian {
    // Alternatively, we could scatter only with probability p and have attenuation be albedo / p.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let scatter_dir = hit.normal + Vec3::new_random_unit();
        let scattered = Ray::new(hit.point, scatter_dir, ray.time);
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some((scattered, attenuation))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        let reflected = ray.direction.unit().reflect(hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzz * Vec3::new_random_in_sphere(),
            ray.time,
        );
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, Color)> {
        // Approximation to account for the reflectivity varying on the angle
        fn schlick(cosine: f64, ri: f64) -> f64 {
            let r0 = (1.0 - ri) / (1.0 + ri);
            let r0 = r0.powi(2);
            r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
        }

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let etai_over_etat = if hit.front_face {
            1.0 / self.ri
        } else {
            self.ri
        };

        let unit_dir = ray.direction.unit();

        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        if etai_over_etat * sin_theta > 1.0 {
            // No solution for the formula, can't refract
            let reflected = ray.direction.unit().reflect(hit.normal);
            let scattered = Ray::new(hit.point, reflected, ray.time);
            return Some((scattered, attenuation));
        }

        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if rand_f64() < reflect_prob {
            let reflected = ray.direction.unit().reflect(hit.normal);
            let scattered = Ray::new(hit.point, reflected, ray.time);
            return Some((scattered, attenuation));
        }

        let refracted = unit_dir.refract(hit.normal, etai_over_etat);
        let scattered = Ray::new(hit.point, refracted, ray.time);
        Some((scattered, attenuation))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<(Ray, Color)> {
        None
    }

    fn emit(&self, u: f64, v: f64, point: Vec3) -> Color {
        self.emit.value(u, v, point)
    }
}
