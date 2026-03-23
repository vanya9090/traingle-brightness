use crate::math::{Vec3, Color};
use crate::scene::{Triangle, Light, Material};
use std::f64::consts::PI;

pub fn calculate_point_brightness(
    lights: &[Light],
    triangle: &Triangle,
    material: &Material,
    observer_pos: Vec3,
    local_x: f64,
    local_y: f64,
) -> (Color, Color, Color) {
    
    // get basis
    let e1 = triangle.p1 - triangle.p0;
    let e2 = triangle.p2 - triangle.p0;
    
    let p_t = triangle.p0 + 
              e1.normalize() * local_x + 
              e2.normalize() * local_y;

    // direction from point to observer
    let observer_dir = (observer_pos - p_t).normalize();

    // normal
    let geom_n = triangle.normal();
    
    // view normal
    let mut view_n = geom_n;
    if view_n.dot(observer_dir) < 0.0 {
        view_n = view_n * -1.0;
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
        let ray_from_light = s_dir * -1.0;
        let cos_theta = ray_from_light.dot(light.dir.normalize()).max(0.0);
        let i_rgb = light.intensity * cos_theta;

        // E = (I * cos(alpha)) / R^2
        // alpha - angle between point-light and normal (absolute irradiance)
        let cos_alpha_geom = s_dir.dot(geom_n).abs();
        let e_rgb = i_rgb * (cos_alpha_geom / r_sq);
        e_values.push(e_rgb);

        let cos_alpha_view = s_dir.dot(view_n).max(0.0);
        
        // only render if light hits the visible side
        if cos_alpha_view > 0.0 {
            let e_visible = i_rgb * (cos_alpha_view / r_sq);

            // 4. BRDF: f = K * kd + ks * (h * N)^ke
            let h = (s_dir + observer_dir).normalize();
            let cos_spec = h.dot(view_n).max(0.0);
            
            let specular = material.ks * cos_spec.powf(material.ke);
            
            let brdf = (material.color * material.kd) + Vec3::new(specular, specular, specular);

            // L = E * brdf / pi
            total_l = total_l + (e_visible * brdf / PI);
        }
    }

    (e_values[0], e_values[1], total_l)
}
