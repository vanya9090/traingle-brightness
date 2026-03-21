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
    
    // 1. Перевод локальных координат (x, y) в глобальные PT
    let e1 = triangle.p1 - triangle.p0;
    let e2 = triangle.p2 - triangle.p0;
    
    let p_t = triangle.p0 + 
              e1.normalize() * local_x + 
              e2.normalize() * local_y;

    // 2. Подготовка нормали (N)
    let mut n = triangle.normal();
    if n.dot(observer_dir) < 0.0 {
        n = n * -1.0;
    }

    let mut total_l = Color::zero();
    let mut e_values = Vec::new();

    // 3. Расчет вклада от каждого источника света
    for light in lights {
        // Вектор от точки поверхности к источнику (s)
        let s_vec = light.pos - p_t;
        let r_sq = s_vec.length_squared(); // R^2
        let s_dir = s_vec.normalize();     // Единичный вектор s

        // Сила излучения: I = I0 * cos(theta)
        // theta - угол между направлением на точку и осью источника O
        let cos_theta = s_dir.dot(light.dir.normalize()).abs();
        let i_rgb = light.intensity * cos_theta;

        // Освещенность: E = (I * cos(alpha)) / R^2
        // alpha - угол между направлением света s и нормалью N
        let cos_alpha = s_dir.dot(n).max(0.0); // Отсекаем свет "сзади"
        let e_rgb = i_rgb * (cos_alpha / r_sq);
        e_values.push(e_rgb);

        // 4. BRDF (ДФОС): f = K * kd + ks * (h * N)^ke
        // h - средний вектор (half-vector) между s и v
        let h = (s_dir + observer_dir).normalize();
        let cos_spec = h.dot(n).max(0.0);
        
        // Зеркальная составляющая (модель Блинна-Фонга)
        let specular = material.ks * cos_spec.powf(material.ke);
        
        // Суммируем диффузную (K * kd) и зеркальную компоненты
        let brdf = (material.color * material.kd) + Vec3::new(specular, specular, specular);

        // Итоговая яркость от источника: L = (1/pi) * E * brdf
        total_l = total_l + (e_rgb * brdf) * (1.0 / PI);
    }

    // Для удобства вывода в отчет возвращаем освещенности отдельно
    (e_values[0], e_values[1], total_l)
}
