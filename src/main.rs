use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Mat4, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

mod color;
mod fragment;
mod framebuffer;
mod line;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use framebuffer::Framebuffer;
use obj::Obj;
use shaders::vertex_shader;
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    model_matrix: Mat4,
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
    // Vertex Shader Stage: Transformación de los vértices usando los uniforms (matriz de modelo)
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms); // Aplicar transformaciones
        transformed_vertices.push(transformed);
    }

    // Ensamblar triángulos de los vértices transformados y rasterizarlos
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            // Llamada a la función triangle para calcular los fragmentos y aplicar el Z-buffer
            triangle(v1, v2, v3, framebuffer);
        }
    }
}

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

    let mut translation = Vec3::new(300.0, 200.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 20.0f32;

    let obj = Obj::load("assets/models/dragon_suavized.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Manejar la entrada de usuario
        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { model_matrix };

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

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
    // Movimiento de traslación: WASD
    if window.is_key_down(Key::D) {
        translation.x += 10.0; // Mover a la derecha
    }
    if window.is_key_down(Key::A) {
        translation.x -= 10.0; // Mover a la izquierda
    }
    if window.is_key_down(Key::W) {
        translation.y -= 10.0; // Mover hacia arriba
    }
    if window.is_key_down(Key::S) {
        translation.y += 10.0; // Mover hacia abajo
    }

    // Escalar: R y F
    if window.is_key_down(Key::R) {
        *scale += 2.0; // Aumentar el tamaño
    }
    if window.is_key_down(Key::F) {
        *scale -= 2.0; // Reducir el tamaño
    }

    // Rotación: Q y E para rotación en el eje X, Z y C para el eje Y, y U e I para el eje Z
    if window.is_key_down(Key::Q) {
        rotation.x -= std::f32::consts::PI / 10.0; // Rotar hacia la izquierda en el eje X
    }
    if window.is_key_down(Key::E) {
        rotation.x += std::f32::consts::PI / 10.0; // Rotar hacia la derecha en el eje X
    }
    if window.is_key_down(Key::Z) {
        rotation.y -= std::f32::consts::PI / 10.0; // Rotar hacia la izquierda en el eje Y
    }
    if window.is_key_down(Key::C) {
        rotation.y += std::f32::consts::PI / 10.0; // Rotar hacia la derecha en el eje Y
    }
    if window.is_key_down(Key::U) {
        rotation.z -= std::f32::consts::PI / 10.0; // Rotar hacia la izquierda en el eje Z
    }
    if window.is_key_down(Key::I) {
        rotation.z += std::f32::consts::PI / 10.0; // Rotar hacia la derecha en el eje Z
    }
}
