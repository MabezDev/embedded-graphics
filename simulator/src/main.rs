extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Translation, UnitQuaternion, Vector3};

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    let mut cubes: Vec<Vec<SceneNode>> = (0..10)
        .map(|y| {
            (0..10)
                .map(|x| {
                    let mut c = window.add_cube(0.1, 0.1, 0.1);

                    c.set_local_translation(Translation::from_vector(Vector3::new(
                        -x as f32 * 0.2,
                        -y as f32 * 0.2,
                        0.0,
                    )));

                    c
                })
                .collect()
        })
        .collect();

    // let mut c = window.add_cube(1.0, 1.0, 1.0);

    // c.set_color(1.0, 0.0, 0.0);

    window.set_light(Light::StickToCamera);

    let rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);

    while window.render() {
        // c.prepend_to_local_rotation(&rot);
    }
}
