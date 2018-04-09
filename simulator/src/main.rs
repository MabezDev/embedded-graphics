extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::light::Light;
use kiss3d::scene::SceneNode;
use kiss3d::window::Window;
use na::{Translation, UnitQuaternion, Vector3};
use rand::random;

const DISP_WIDTH: u8 = 128;
const DISP_HEIGHT: u8 = 64;
const PIXEL_SIZE: f32 = 0.1;

fn main() {
    let mut window = Window::new("Display sim");

    let left = -(PIXEL_SIZE * DISP_WIDTH as f32 / 2.0);
    let top = -(PIXEL_SIZE * DISP_HEIGHT as f32 / 2.0);

    let mut cubes: Vec<Vec<SceneNode>> = (0..DISP_HEIGHT)
        .map(|y| {
            (0..DISP_WIDTH)
                .map(|x| {
                    let mut c = window.add_quad(0.1, 0.1, 1, 1);

                    c.set_local_translation(Translation::from_vector(Vector3::new(
                        left + x as f32 * PIXEL_SIZE,
                        top + y as f32 * PIXEL_SIZE,
                        0.0,
                    )));
                    c.set_color(random(), random(), random());

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
