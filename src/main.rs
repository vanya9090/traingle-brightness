pub mod math;
pub mod scene;
pub mod optics;

use math::Vec3;
use scene::{Triangle, Light, Material};
use optics::calculate_point_brightness;

fn main() {
    let triangle = Triangle::new(
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(0.0, 5.0, 0.0),
    );

    let material = Material {
        color: Vec3::new(0.8, 0.2, 0.2),
        kd: 0.7,
        ks: 0.3,
        ke: 50.0,
    };

    let lights = vec![
        Light {
            pos: Vec3::new(2.5, 2.5, 5.0),
            dir: Vec3::new(0.0, 0.0, -1.0),
            intensity: Vec3::new(100.0, 100.0, 100.0),
        },
        Light {
            pos: Vec3::new(10.0, 10.0, 3.0),
            dir: Vec3::new(-1.0, -1.0, -0.5),
            intensity: Vec3::new(80.0, 90.0, 100.0),
        },
    ];

    let observer_dir = Vec3::new(0.0, 0.0, 1.0).normalize();

    let test_points = [(0.5, 0.5), (1.0, 1.0), (2.0, 1.5), (3.0, 0.5), (0.2, 4.0)];

    println!("Таблица 1. Освещенность E1 (Локальные координаты)");
    for (x, y) in test_points {
        let (e1, _, _) = calculate_point_brightness(&lights, &triangle, &material, observer_dir, x, y);
        println!("[{:.1}], [{:.1}], [{:.4}], [{:.4}], [{:.4}],", x, y, e1.x, e1.y, e1.z);
    }

    println!("\nТаблица 2. Освещенность E2 (Глобальные координаты)");
    let e1_vec = triangle.p1 - triangle.p0;
    let e2_vec = triangle.p2 - triangle.p0;

    for (x, y) in test_points {
        let pt = triangle.p0 + e1_vec.normalize() * x + e2_vec.normalize() * y;
        
        let (_, e2, _) = calculate_point_brightness(&lights, &triangle, &material, observer_dir, x, y);
        
        println!("[{:.1}], [{:.1}], [{:.4}], [{:.4}], [{:.4}], [{:.4}],", pt.x, pt.y, pt.z, e2.x, e2.y, e2.z);
    }

    println!("\nТаблица 3. Яркость L (Условия наблюдения)");
    for (x, y) in test_points {
        let (_, _, l) = calculate_point_brightness(&lights, &triangle, &material, observer_dir, x, y);
        println!("[{:.1}], [{:.1}], [{:.4}], [{:.4}], [{:.4}],", x, y, l.x, l.y, l.z);
    }
}
