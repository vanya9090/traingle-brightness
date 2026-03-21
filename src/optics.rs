use crate::math::{Vec3, Color};
use crate::scene::{Triangle, Light, Material};
use std::f64::consts::PI;

pub fn calculate_point_brightness(
    lights: &[Light],
    triangle: &Triangle,
    material: &Material,
    observer_dir: Vec3,
    local_x: f64,
    local_y: f64,
) -> (Color, Color, Color) {
    
    // get basis
    let e1 = triangle.p1 - triangle.p0;
    let e2 = triangle.p2 - triangle.p0;
    
    let p_t = triangle.p0 + 
              e1.normalize() * local_x + 
              e2.normalize() * local_y;

    // normal
    let mut n = triangle.normal();
    if n.dot(observer_dir) < 0.0 {
        n = n * -1.0;
    }

    let mut total_l = Color::zero();
    let mut e_values = Vec::new();

    for light in lights {
        // from point on triangle to light
        let s_vec = light.pos - p_t;
        let r_sq = s_vec.length_squared(); // R^2
        let s_dir = s_vec.normalize();

        // I = I0 * cos(theta)
        // theta - angle between point-light and light axis
        let cos_theta = s_dir.dot(light.dir.normalize()).abs();
        let i_rgb = light.intensity * cos_theta;

        // E = (I * cos(alpha)) / R^2
        // alpha - angle between point-light and normal
        let cos_alpha = s_dir.dot(n).max(0.0);
        let e_rgb = i_rgb * (cos_alpha / r_sq);
        e_values.push(e_rgb);

        // 4. BRDF: f = K * kd + ks * (h * N)^ke
        let h = (s_dir + observer_dir).normalize();
        let cos_spec = h.dot(n).max(0.0);
        
        let specular = material.ks * cos_spec.powf(material.ke);
        
        let brdf = (material.color * material.kd) + Vec3::new(specular, specular, specular);

        // L = E * brdf / pi
        total_l = total_l + (e_rgb * brdf / PI);
    }

    (e_values[0], e_values[1], total_l)
}
