use fragment::Fragment;
use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{look_at, perspective, Mat4, Vec3, Vec4};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

mod camera;
mod color;
mod fragment;
mod framebuffer;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use camera::Camera;
use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};
use framebuffer::Framebuffer;
use image::{GenericImageView, RgbaImage};
use obj::Obj;
use shaders::{
    cellular_shader, cloud_shader, combined_shader, comet_shader, dalmata_shader, earth,
    fragment_shader, lava_shader, luna_shader, moving_circles_shader, neon_light_shader,
    neon_normal_map_shader, static_pattern_shader, sun_shader, vertex_shader,
};
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

fn create_noise_for_planet(index: usize) -> FastNoiseLite {
    match index {
        0 => create_lava_noise(),
        1 => create_neon_noise(),
        2 => create_sun_noise(),
        3 => create_dalmata_noise(),
        4 => create_cloud_noise(),
        5 => create_combined_noise(),
        6 => create_cloud_noise(),
        7 => create_cloud_noise(), // noise para la luna y cometa
        _ => create_noise(),       // Por defecto
    }
}

fn create_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1330);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_lacunarity(Some(1.480));
    noise.set_fractal_octaves(Some(6));
    noise.set_frequency(Some(0.005));
    noise
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

fn create_dalmata_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2)); // Cambiar a Cellular
    noise.set_frequency(Some(0.3)); // Ajusta la frecuencia para el detalle deseado
    noise.set_fractal_type(Some(FractalType::FBm)); // Puedes usar FBm para agregar más detalle
    noise
}

fn create_neon_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(8888);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2S)); // Variación más suave
    noise.set_frequency(Some(0.02)); // Características amplias
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_gain(Some(0.45));
    noise
}

fn create_static_pattern_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(9999);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.08)); // Patrones más definidos
    noise.set_fractal_type(Some(FractalType::None)); // Sin fractales
    noise
}

fn create_combined_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1234);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_frequency(Some(0.03));
    noise.set_fractal_octaves(Some(6));
    noise.set_fractal_gain(Some(0.5));
    noise.set_fractal_lacunarity(Some(2.0));
    noise
}

fn create_sun_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(12345); // Semilla para el ruido
    noise.set_noise_type(Some(NoiseType::OpenSimplex2)); // Ruido suave para superficies fluidas
    noise.set_frequency(Some(0.02)); // Frecuencia baja para detalles amplios
    noise.set_fractal_type(Some(FractalType::FBm)); // Fractal para agregar detalles
    noise.set_fractal_octaves(Some(5)); // Más octavas para complejidad
    noise.set_fractal_gain(Some(0.6)); // Escala de amplitud de detalles pequeños
    noise.set_fractal_lacunarity(Some(2.5)); // Relación entre las frecuencias
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

fn render(
    framebuffer: &mut Framebuffer,
    uniforms: &Uniforms,
    vertex_array: &[Vertex],
    shader: fn(&Fragment, &Uniforms) -> color::Color,
) {
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
            let shaded_color = shader(&fragment, uniforms);
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

fn is_visible(position: &Vec3, view_matrix: &Mat4, projection_matrix: &Mat4) -> bool {
    // Transformar la posición del objeto al espacio de clip
    let clip_space_position =
        projection_matrix * view_matrix * Vec4::new(position.x, position.y, position.z, 1.0);
    let w = clip_space_position.w;

    // Normalizar las coordenadas (clip space -> NDC)
    let x_ndc = clip_space_position.x / w;
    let y_ndc = clip_space_position.y / w;
    let z_ndc = clip_space_position.z / w;

    // Verificar si está dentro del rango [-1, 1]
    x_ndc >= -1.0 && x_ndc <= 1.0 && y_ndc >= -1.0 && y_ndc <= 1.0 && z_ndc >= -1.0 && z_ndc <= 1.0
}

fn check_collision(position: &Vec3, planet_position: &Vec3, planet_radius: f32) -> bool {
    let distance = nalgebra_glm::distance(position, planet_position);
    distance < planet_radius
}

fn render_orbit(
    framebuffer: &mut Framebuffer,
    center: Vec3,
    radius: f32,
    segments: usize,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
) {
    let mut points = Vec::new();
    for i in 0..segments {
        let angle = 2.0 * PI * i as f32 / segments as f32;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();
        points.push(Vec3::new(x, y, center.z));
    }

    for i in 0..segments {
        let start = points[i];
        let end = points[(i + 1) % segments];
        let start_ndc = projection_matrix * view_matrix * Vec4::new(start.x, start.y, start.z, 1.0);
        let end_ndc = projection_matrix * view_matrix * Vec4::new(end.x, end.y, end.z, 1.0);

        let start_ndc = start_ndc / start_ndc.w;
        let end_ndc = end_ndc / end_ndc.w;

        framebuffer.draw_line(
            ((start_ndc.x + 1.0) * framebuffer.width as f32 * 0.5) as usize,
            ((1.0 - start_ndc.y) * framebuffer.height as f32 * 0.5) as usize,
            ((end_ndc.x + 1.0) * framebuffer.width as f32 * 0.5) as usize,
            ((1.0 - end_ndc.y) * framebuffer.height as f32 * 0.5) as usize,
            0xFFFFFF, // Color blanco para las órbitas
        );
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
        "Sistema Solar - Proyecto Final",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    // Posiciones iniciales en el plano eclíptico
    let mut planet_orbits = vec![
        4.0,  // Marte
        6.0,  // Neon
        8.0,  // Sol (solo referencia para mantener alineación)
        10.0, // Dalmata
        12.0, // Saturno
        14.0, // Kepler-452b
        16.0, // Tierra
    ];

    let mut translations = vec![
        Vec3::new(2.0, 0.0, 0.0),  // Marte
        Vec3::new(0.0, 0.0, 0.0),  // Neon
        Vec3::new(-2.0, 0.0, 0.0), // Sol
        Vec3::new(0.0, 2.0, 0.0),  // Dalmata
        Vec3::new(0.0, 4.0, 0.0),  // Saturno
        Vec3::new(1.0, 2.0, 0.0),  // Kepler-452b
        Vec3::new(-1.0, 2.0, 0.0), // Tierra
        Vec3::new(0.0, 0.0, 0.0),  // Cometa (posición inicial)
    ];

    let mut rotations = vec![Vec3::new(0.0, 0.0, 0.0); 8];
    let scales = vec![1.0f32; 8];
    let shaders = vec![
        lava_shader,            // Marte
        neon_normal_map_shader, // Neon
        static_pattern_shader,  // Sol
        dalmata_shader,         // Dalmata
        combined_shader,        // Saturno
        cellular_shader,        // Kepler-452b
        earth,                  // Tierra
        comet_shader,           // Cometa
    ];

    // OBJs

    //Luego hacer un array de modelos para manejar planetas, estrellas, etc.
    let obj = Obj::load("assets/models/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    let obj_ring = Obj::load("assets/models/saturn.obj").expect("Failed to load obj_ring");
    let vertex_arrays_ring = obj_ring.get_vertex_array();

    let obj_moon = Obj::load("assets/models/sphere.obj").expect("Failed to load obj_moon");
    let vertex_arrays_moon = obj_moon.get_vertex_array();

    let obj_comet = Obj::load("assets/models/sphere.obj").expect("Failed to load obj_comet");
    let vertex_arrays_comet = obj_comet.get_vertex_array();

    // OBJ de mi nave
    let obj_tie_fighter =
        Obj::load("assets/models/tiefighter.obj").expect("Failed to load tiefigther.obj");
    let vertex_arrays_tie_fighter = obj_tie_fighter.get_vertex_array();

    // Variables para la nave
    let mut tie_fighter_position = Vec3::new(0.0, 0.0, 7.0); // Posición inicial
    let mut tie_fighter_direction = Vec3::new(0.0, 0.0, -1.0); // Dirección inicial
    let mut tie_fighter_up = Vec3::new(0.0, 1.0, 0.0); // Vector "arriba"

    let start_time = Instant::now(); // Tiempo inicial para controlar la rotación
    let mut last_mouse_pos = (0.0, 0.0);

    // Configuración de la cámara
    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0), // Posición inicial de la cámara
        Vec3::new(0.0, 0.0, 0.0), // Centro (Sol)
        Vec3::new(0.0, 1.0, 0.0), // Vector "arriba"
    );

    let mut should_update_camera_target = false;

    let mut current_camera_target = 0; // Índice del planeta seleccionado

    let mut zoom_factor = 5.0; // Zoom inicial

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            let zoom_sensitivity = 0.1; // Ajusta la sensibilidad
            zoom_factor -= scroll_y as f32 * zoom_sensitivity;
            zoom_factor = zoom_factor.clamp(2.0, 50.0); // Limitar el zoom
        }

        // Control de cámara con teclas numéricas
        if window.is_key_down(Key::Key1) {
            current_camera_target = 0; // Marte
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key2) {
            current_camera_target = 1; // Neon
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key3) {
            current_camera_target = 2; // Sol
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key4) {
            current_camera_target = 3; // Dalmata
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key5) {
            current_camera_target = 4; // Saturno
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key6) {
            current_camera_target = 5; // Kepler-452b
            should_update_camera_target = true;
        } else if window.is_key_down(Key::Key7) {
            current_camera_target = 6; // Tierra
            should_update_camera_target = true;
        }

        // Asegúrate de que current_camera_target esté dentro del rango válido
        if current_camera_target >= planet_orbits.len() {
            current_camera_target = 0; // Regresar al valor por defecto (Marte)
            should_update_camera_target = true;
        }

        if should_update_camera_target {
            let planet_position = translations[current_camera_target];
            let planet_radius = scales[current_camera_target] * 1.5;

            // Normalizar la dirección hacia el Sol
            let direction_to_sun =
                nalgebra_glm::normalize(&(Vec3::new(0.0, 0.0, 0.0) - planet_position));

            // Calcular la posición de la cámara
            camera.eye = planet_position - direction_to_sun * (planet_radius * 2.0);

            // Validar la posición de la cámara
            if camera.eye.norm() > 1e6 || camera.eye.norm() < 1e-3 {
                println!(
                    "Advertencia: Posición de la cámara fuera de rango: {:?}",
                    camera.eye
                );
                camera.eye = Vec3::new(0.0, 0.0, 10.0); // Restablecer
            }

            println!("Posición de la cámara: {:?}", camera.eye);
            camera.center = Vec3::new(0.0, 0.0, 0.0); // Mirar al Sol
            should_update_camera_target = false; // Actualización completa
        }

        //handle_input(&window, &mut camera, &mut last_mouse_pos);

        // Verificar colisiones para la nave
        for (i, planet_position) in translations.iter().enumerate() {
            let planet_radius = scales[i] + 0.5; // Aumentar ligeramente el radio para mayor seguridad
            if check_collision(&tie_fighter_position, planet_position, planet_radius) {
                // Ajustar la posición de la nave para evitar la colisión
                let direction = nalgebra_glm::normalize(&(tie_fighter_position - planet_position));
                tie_fighter_position = *planet_position + direction * (planet_radius + 0.05);
                //println!(
                //    "Colisión detectada con el planeta {}. Posición de la nave ajustada.",
                //    i
                //);
            }
        }

        // Actualizar la posición y orientación de la cámara para seguir la nave
        camera.eye =
            tie_fighter_position - tie_fighter_direction * zoom_factor + tie_fighter_up * 2.0;
        camera.center = tie_fighter_position;
        camera.up = tie_fighter_up;

        // Manejar los controles de la nave
        handle_tie_fighter_input(
            &window,
            &mut tie_fighter_position,
            &mut tie_fighter_direction,
            &mut tie_fighter_up,
            &mut camera,
            &mut last_mouse_pos,
        );

        framebuffer.clear();

        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix =
            create_perspective_matrix(window_width as f32, window_height as f32);
        let viewport_matrix =
            create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

        let elapsed_time = start_time.elapsed().as_secs_f32();

        let model_matrix_tie_fighter = nalgebra_glm::translation(&tie_fighter_position)
            * nalgebra_glm::look_at(&Vec3::zeros(), &tie_fighter_direction, &tie_fighter_up)
            * nalgebra_glm::scaling(&Vec3::new(0.1, 0.1, 0.1));

        let uniforms_tie_fighter = Uniforms {
            model_matrix: model_matrix_tie_fighter,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time: elapsed_time as u32,
            noise: create_noise(),
        };

        // Renderizar la nave
        render(
            &mut framebuffer,
            &uniforms_tie_fighter,
            &vertex_arrays_tie_fighter,
            |_, _| color::Color::new(39, 101, 167),
        );

        for i in 0..translations.len() {
            // Movimiento orbital
            if i != 2
                && i < planet_orbits.len()
                && is_visible(&translations[i], &view_matrix, &projection_matrix)
            {
                let orbit_angle = elapsed_time * (0.1 + i as f32 * 0.05);
                translations[i].x = planet_orbits[i] * 1.5 * orbit_angle.cos(); // Factor 1.5 para separarlos más
                translations[i].y = planet_orbits[i] * 1.5 * orbit_angle.sin(); // Factor 1.5 para separarlos más

                render_orbit(
                    &mut framebuffer,
                    Vec3::new(0.0, 0.0, 0.0), // Centro de la órbita (el Sol)
                    planet_orbits[i] * 1.5,   // Radio de la órbita
                    100,                      // Número de segmentos para el círculo
                    &view_matrix,
                    &projection_matrix,
                );
            }

            rotations[i].y = elapsed_time * (0.1 + i as f32 * 0.05);

            if is_visible(&translations[i], &view_matrix, &projection_matrix) {
                let model_matrix = create_model_matrix(translations[i], scales[i], rotations[i]);
                let noise = create_noise_for_planet(i);

                let uniforms = Uniforms {
                    model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time: elapsed_time as u32,
                    noise,
                };

                if i == 1 && is_visible(&translations[i], &view_matrix, &projection_matrix) {
                    // Renderizar Neon con el shader que usa el mapa normal
                    render(
                        &mut framebuffer,
                        &uniforms,
                        &vertex_arrays,
                        neon_normal_map_shader,
                    );
                } else if i == 4 && is_visible(&translations[i], &view_matrix, &projection_matrix) {
                    // Renderizar el anillo adicional para el planeta con ID 4 (Saturno)
                    let ring_model_matrix = create_model_matrix(
                        translations[i], // Posición igual al planeta
                        scales[i] * 0.7, // Escala ajustada (1.5 veces el tamaño del planeta)
                        rotations[i],    // Rotación igual al planeta
                    );

                    let noise_ring = create_noise_for_planet(i);

                    let ring_uniforms = Uniforms {
                        model_matrix: ring_model_matrix, // Matriz específica del anillo
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time: elapsed_time as u32,
                        noise: noise_ring,
                    };

                    render(
                        &mut framebuffer,
                        &ring_uniforms,
                        &vertex_arrays_ring,
                        shaders[i],
                    );
                } else if i == 6 && is_visible(&translations[i], &view_matrix, &projection_matrix) {
                    // Renderizar la Tierra
                    render(&mut framebuffer, &uniforms, &vertex_arrays, earth);

                    // Calcular la órbita de la luna
                    let moon_orbit_radius = 0.7; // Radio de la órbita
                    let moon_speed = 0.5; // Velocidad de la órbita
                    let moon_angle = elapsed_time * moon_speed;

                    let moon_x = translations[i].x + moon_orbit_radius * moon_angle.cos();
                    let moon_y = translations[i].y + moon_orbit_radius * moon_angle.sin();

                    let moon_translation = Vec3::new(moon_x, moon_y, 0.0);
                    let moon_model_matrix =
                        create_model_matrix(moon_translation, scales[i] * 0.3, rotations[i]);

                    let moon_uniforms = Uniforms {
                        model_matrix: moon_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time: elapsed_time as u32,
                        noise: create_noise_for_planet(7),
                    };

                    // Renderizar la Luna
                    render(
                        &mut framebuffer,
                        &moon_uniforms,
                        &vertex_arrays_moon,
                        luna_shader,
                    );
                } else if i == 7 && is_visible(&translations[i], &view_matrix, &projection_matrix) {
                    // Renderizar el cometa
                    let comet_x = elapsed_time.sin() * 4.0; // Movimiento en el eje X
                    let comet_y = elapsed_time.cos() * 2.0; // Movimiento en el eje Y
                    let comet_translation = Vec3::new(comet_x, comet_y, 0.0);

                    let comet_model_matrix =
                        create_model_matrix(comet_translation, 0.2, Vec3::new(0.0, 0.0, 0.0));

                    let comet_uniforms = Uniforms {
                        model_matrix: comet_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time: elapsed_time as u32,
                        noise: create_noise_for_planet(i),
                    };

                    render(
                        &mut framebuffer,
                        &comet_uniforms,
                        &vertex_arrays_comet,
                        comet_shader,
                    );
                } else if i == 2 && is_visible(&translations[i], &view_matrix, &projection_matrix) {
                    // Renderizar el Sol
                    let sun_translation = Vec3::new(0.0, 0.0, 0.0);
                    let sun_model_matrix = create_model_matrix(
                        sun_translation,
                        scales[i] * 1.5,
                        Vec3::new(0.0, 0.0, 0.0),
                    );

                    let sun_uniforms = Uniforms {
                        model_matrix: sun_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time: elapsed_time as u32,
                        noise: create_noise_for_planet(i),
                    };

                    render(&mut framebuffer, &sun_uniforms, &vertex_arrays, sun_shader);
                } else {
                    // Renderizar los demás planetas normalmente
                    render(&mut framebuffer, &uniforms, &vertex_arrays, shaders[i]);
                }
            }
        }

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

    // Controlar el zoom con el scroll del mouse
    if let Some((scroll_x, scroll_y)) = window.get_scroll_wheel() {
        // Scroll hacia arriba (zoom in) o abajo (zoom out)
        let zoom_sensitivity = 0.1;
        camera.eye.z -= scroll_y as f32 * zoom_sensitivity;

        // Limitar la distancia mínima y máxima del zoom
        camera.eye.z = camera.eye.z.clamp(2.0, 100.0); // Valores arbitrarios, ajustables según tu escena
    }
}

fn calculate_sphere_radius(vertices: &[Vertex]) -> f32 {
    // Inicializar el radio máximo en 0
    let mut max_distance = 0.0;

    for vertex in vertices {
        // Calcular la distancia desde el origen (0,0,0)
        let distance = nalgebra_glm::length(&Vec3::new(
            vertex.position.x,
            vertex.position.y,
            vertex.position.z,
        ));

        // Actualizar el radio máximo si encontramos una distancia mayor
        if distance > max_distance {
            max_distance = distance;
        }
    }

    max_distance
}

fn handle_tie_fighter_input(
    window: &Window,
    position: &mut Vec3,
    direction: &mut Vec3,
    up: &mut Vec3,
    camera: &mut Camera,
    last_mouse_pos: &mut (f32, f32),
) {
    let speed = 0.5; // Velocidad de la nave
    let rotation_speed = 0.05; // Velocidad de rotación
    let sensitivity = 0.005; // Sensibilidad del mouse
    let zoom_sensitivity = 0.001; // Sensibilidad del zoom

    // Movimiento adelante/atrás de la nave
    if window.is_key_down(Key::Up) {
        *position += *direction * speed; // Avanzar en la dirección actual
    }
    if window.is_key_down(Key::Down) {
        *position -= *direction * speed; // Retroceder en la dirección actual
    }

    // Rotación con teclas hacia arriba/abajo (pitch)
    if window.is_key_down(Key::T) {
        let right = nalgebra_glm::cross(&direction, &up).normalize();
        let rotation_matrix = nalgebra_glm::rotation(rotation_speed, &right);
        *direction = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(direction)));
        *up = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(up)));
    }
    if window.is_key_down(Key::G) {
        let right = nalgebra_glm::cross(&direction, &up).normalize();
        let rotation_matrix = nalgebra_glm::rotation(-rotation_speed, &right);
        *direction = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(direction)));
        *up = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(up)));
    }

    // Rotación con teclas hacia los lados (yaw)
    if window.is_key_down(Key::F) {
        let rotation_matrix = nalgebra_glm::rotation(rotation_speed, &up.normalize());
        *direction = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(direction)));
    }
    if window.is_key_down(Key::H) {
        let rotation_matrix = nalgebra_glm::rotation(-rotation_speed, &up.normalize());
        *direction = nalgebra_glm::normalize(&(rotation_matrix.transform_vector(direction)));
    }

    // Rotación con clic derecho y movimiento del mouse
    if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let dx = mouse_x as f32 - last_mouse_pos.0;
        let dy = mouse_y as f32 - last_mouse_pos.1;

        if window.get_mouse_down(minifb::MouseButton::Right) {
            let right = nalgebra_glm::cross(&direction, &up).normalize();

            // Aplicar rotación pitch (vertical)
            let pitch_rotation = nalgebra_glm::rotation(-dy * sensitivity, &right);
            *direction = nalgebra_glm::normalize(&(pitch_rotation.transform_vector(direction)));
            *up = nalgebra_glm::normalize(&(pitch_rotation.transform_vector(up)));

            // Aplicar rotación yaw (horizontal)
            let yaw_rotation = nalgebra_glm::rotation(-dx * sensitivity, &up.normalize());
            *direction = nalgebra_glm::normalize(&(yaw_rotation.transform_vector(direction)));
        }

        // Actualizar la última posición del mouse
        *last_mouse_pos = (mouse_x as f32, mouse_y as f32);
    }
}
