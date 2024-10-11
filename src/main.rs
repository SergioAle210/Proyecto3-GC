use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

mod camera;
mod color;
mod fragment;
mod framebuffer;
mod line;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use camera::Camera;
use framebuffer::Framebuffer;
use obj::Obj;
use shaders::vertex_shader;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let scale_matrix = nalgebra_glm::scaling(&Vec3::new(scale, scale, scale));
    let rotation_matrix = nalgebra_glm::rotation(rotation.x, &Vec3::x_axis())
        * nalgebra_glm::rotation(rotation.y, &Vec3::y_axis())
        * nalgebra_glm::rotation(rotation.z, &Vec3::z_axis());
    let translation_matrix = nalgebra_glm::translation(&translation);
    translation_matrix * rotation_matrix * scale_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            // Llamar a la función triangle, pasando la nueva dirección de la luz
            triangle(v1, v2, v3, framebuffer);
        }
    }
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 1000.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0,
        0.0,
        0.0,
        width / 2.0,
        0.0,
        -height / 2.0,
        0.0,
        height / 2.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    )
}

//view port transformation agregar el 128

fn main() {
    let window_width = 1300;
    let window_height = 600;
    let framebuffer_width = 1300;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    // Crear el framebuffer
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Rust Graphics - Renderer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0f32;

    let mut last_mouse_pos = (0.0, 0.0);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 30.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    //Luego hacer un array de modelos para manejar planetas, estrellas, etc.
    let obj = Obj::load("assets/models/dragon.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Manejar la entrada de usuario
        handle_input(&window, &mut camera, &mut last_mouse_pos);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix =
            create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix =
            create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);
        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };

        framebuffer.set_current_color(0xFFDDDD);
        render(&mut framebuffer, &uniforms, &vertex_arrays);

        window
            .update_with_buffer(
                &framebuffer.to_u32_buffer(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, camera: &mut Camera, last_mouse_pos: &mut (f32, f32)) {
    // Movimiento de la cámara hacia adelante y hacia atrás: W/S
    if window.is_key_down(Key::W) {
        camera.zoom(1.0); // Acercar la cámara
    }
    if window.is_key_down(Key::S) {
        camera.zoom(-1.0); // Alejar la cámara
    }

    // Movimiento lateral de la cámara: A/D (Orbitar alrededor del centro)
    if window.is_key_down(Key::A) {
        camera.orbit(PI / 180.0, 0.0); // Orbitar hacia la izquierda
    }
    if window.is_key_down(Key::D) {
        camera.orbit(-PI / 180.0, 0.0); // Orbitar hacia la derecha
    }

    // Movimiento vertical de la cámara: Q/E
    if window.is_key_down(Key::Q) {
        camera.orbit(0.0, PI / 180.0); // Elevar la cámara
    }
    if window.is_key_down(Key::E) {
        camera.orbit(0.0, -PI / 180.0); // Bajar la cámara
    }

    // Zoom de la cámara: Z/X
    if window.is_key_down(Key::Z) {
        camera.zoom(2.0); // Incrementar el zoom
    }
    if window.is_key_down(Key::X) {
        camera.zoom(-2.0); // Disminuir el zoom
    }

    // Obtener la posición actual del mouse
    if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let sensitivity = 0.005; // Ajusta la sensibilidad del mouse

        // Calcular el delta (diferencia) del movimiento del mouse
        let dx = mouse_x as f32 - last_mouse_pos.0;
        let dy = mouse_y as f32 - last_mouse_pos.1;

        // Solo aplica la rotación si el botón izquierdo del mouse está presionado
        if window.get_mouse_down(minifb::MouseButton::Left) {
            camera.orbit(-dx * sensitivity, -dy * sensitivity); // Controlar yaw y pitch con el mouse
        }

        // Actualizar la última posición del mouse
        *last_mouse_pos = (mouse_x as f32, mouse_y as f32);
    }
}
