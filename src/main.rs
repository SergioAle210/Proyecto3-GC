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
    // Crear la matriz de transformación de escala
    let scale_matrix = nalgebra_glm::scaling(&Vec3::new(scale, scale, scale));

    // Crear la matriz de rotación (aplicando rotaciones alrededor de los 3 ejes)
    let rotation_matrix = nalgebra_glm::rotation(rotation.x, &Vec3::x_axis())
        * nalgebra_glm::rotation(rotation.y, &Vec3::y_axis())
        * nalgebra_glm::rotation(rotation.z, &Vec3::z_axis());

    // Crear la matriz de traslación
    let translation_matrix = nalgebra_glm::translation(&translation);

    // Combinarlas todas en una sola matriz de modelo
    translation_matrix * rotation_matrix * scale_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        // Transformación de vértices
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Ensamblar triángulos de vértices transformados
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            // Obtener los vértices de cada triángulo
            let v1 = &transformed_vertices[i];
            let v2 = &transformed_vertices[i + 1];
            let v3 = &transformed_vertices[i + 2];

            // Rasterizar el triángulo directamente
            let fragments = triangle(v1, v2, v3);

            // Procesar los fragmentos y renderizarlos
            for fragment in fragments {
                let x = fragment.position.x as usize;
                let y = fragment.position.y as usize;
                if x < framebuffer.width && y < framebuffer.height {
                    framebuffer.point_with_color(x, y, fragment.color);
                }
            }
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 600;
    let framebuffer_width = 1300;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

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
    let mut scale = 100.0f32;

    let obj = Obj::load("assets/models/dragon.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Donde esta mi modelo, solo me deja mover un modelo
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
