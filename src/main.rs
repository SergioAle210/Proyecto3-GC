use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

mod camera;
mod color;
mod fragment;
mod framebuffer;
// mod line;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use camera::Camera;
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};
use framebuffer::Framebuffer;
use obj::Obj;
use shaders::{fragment_shader, vertex_shader};
use triangle::triangle;
use vertex::Vertex;

pub struct Uniforms {
    model_matrix: Mat4,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    viewport_matrix: Mat4,
    time: u32,
    noise: FastNoiseLite,
}

fn create_noise() -> FastNoiseLite {
    create_cloud_noise()
    // create_cell_noise()
    // create_ground_noise()
    // create_lava_noise()
}

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise
}

fn create_cell_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_frequency(Some(0.1));
    noise
}

fn create_ground_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);

    // Use FBm fractal type to layer multiple octaves of noise
    noise.set_noise_type(Some(NoiseType::Cellular)); // Cellular noise for cracks
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal Brownian Motion
    noise.set_fractal_octaves(Some(5)); // More octaves = more detail
    noise.set_fractal_lacunarity(Some(2.0)); // Lacunarity controls frequency scaling
    noise.set_fractal_gain(Some(0.5)); // Gain controls amplitude scaling
    noise.set_frequency(Some(0.05)); // Lower frequency for larger features

    noise
}

fn create_lava_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);

    // Use FBm for multi-layered noise, giving a "turbulent" feel
    noise.set_noise_type(Some(NoiseType::Perlin)); // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::FBm)); // FBm for layered detail
    noise.set_fractal_octaves(Some(6)); // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.0)); // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.5)); // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.002)); // Low frequency = large features

    noise
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
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            // Apply fragment shader
            let shaded_color = fragment_shader(&fragment, &uniforms);
            let color = shaded_color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
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

    let mut translation1 = Vec3::new(0.0, 0.0, 0.0); // Para el tiefighter
    let mut rotation1 = Vec3::new(0.0, 0.0, 0.0); // Para el tiefighter
    let mut scale1 = 1.0f32; // Para el tiefighter

    let mut translation2 = Vec3::new(2.0, 0.0, 0.0); // Para el charizard, ajusta la posición
    let mut rotation2 = Vec3::new(0.0, 0.0, 0.0); // Para el charizard
    let mut scale2 = 1.0f32; // Para el charizard

    let start_time = Instant::now(); // Tiempo inicial para controlar la rotación
    let mut last_mouse_pos = (0.0, 0.0);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    //Luego hacer un array de modelos para manejar planetas, estrellas, etc.
    let obj1 = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let vertex_arrays1 = obj1.get_vertex_array();
    let time = 0;

    // Cargar el modelo del Charizard
    let obj2 = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let vertex_arrays2 = obj2.get_vertex_array();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Manejar la entrada de usuario
        handle_input(&window, &mut camera, &mut last_mouse_pos);

        framebuffer.clear();
        let noise1 = create_noise();

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix =
            create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix =
            create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        let elapsed_time = start_time.elapsed().as_secs_f32();
        rotation1.y = elapsed_time; // Gira alrededor del eje Y en función del tiempo

        // Configurar uniforms para el tiefighter
        let model_matrix1 = create_model_matrix(translation1, scale1, rotation1);
        let uniforms1 = Uniforms {
            model_matrix: model_matrix1,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: noise1,
        };

        //framebuffer.set_current_color(0xFFDDDD);
        //render(&mut framebuffer, &uniforms1, &vertex_arrays1);

        framebuffer.set_current_color(0xFFDDDD);
        render(&mut framebuffer, &uniforms1, &vertex_arrays1);

        // Rotar charizard en base al tiempo transcurrido
        let elapsed_time = start_time.elapsed().as_secs_f32();
        rotation2.y = elapsed_time; // Gira alrededor del eje Y en función del tiempo

        // Configurar uniforms para el charizard
        let noise2 = create_noise();
        let model_matrix2 = create_model_matrix(translation2, scale2, rotation2);
        let uniforms2 = Uniforms {
            model_matrix: model_matrix2,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise: noise2,
        };

        framebuffer.set_current_color(0xFFAA00); // Un color diferente, si lo prefieres
        render(&mut framebuffer, &uniforms2, &vertex_arrays2);

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
