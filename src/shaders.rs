use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::{dot, mat4_to_mat3, Mat3, Vec3, Vec4};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Transform position
    let position = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);
    let transformed =
        uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    // Perform perspective division
    let w = transformed.w;
    let ndc_position = Vec4::new(transformed.x / w, transformed.y / w, transformed.z / w, 1.0);

    // apply viewport matrix
    let screen_position = uniforms.viewport_matrix * ndc_position;

    // Transform normal
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3
        .transpose()
        .try_inverse()
        .unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_fn: fn(&Fragment, &Uniforms) -> Color,
) -> Color {
    shader_fn(fragment, uniforms)
}

pub fn static_pattern_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;

    Color::new(r, g, b)
}

pub fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let time = uniforms.time as f32 * 0.05;
    let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
    let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;

    let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
    let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();

    let circle_size = 0.1;
    let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
    let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };

    let circle_intensity = (circle1 + circle2).min(1.0f32);

    Color::new(
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8,
        (circle_intensity * 255.0) as u8,
    )
}

// Combined shader
pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let base_color = static_pattern_shader(fragment, uniforms);
    let circle_color = moving_circles_shader(fragment, uniforms);

    // Combine shaders: use circle color if it's not black, otherwise use base color
    if !circle_color.is_black() {
        circle_color * fragment.intensity
    } else {
        base_color * fragment.intensity
    }
}

// Simple purple shader
fn purple_shader(_fragment: &Fragment) -> Color {
    Color::new(128, 0, 128) // Purple color
}

// Circle shader
fn circle_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let distance = (x * x + y * y).sqrt();

    if distance < 0.25 {
        // Circle radius
        Color::new(255, 255, 0) // Yellow circle
    } else {
        Color::new(0, 0, 0) // Black (transparent) background
    }
}

// Combined shader with blend mode parameter
pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
    let base_color = purple_shader(fragment);
    let circle_color = circle_shader(fragment);

    let combined_color = match blend_mode {
        "normal" => base_color.blend_normal(&circle_color),
        "multiply" => base_color.blend_multiply(&circle_color),
        "add" => base_color.blend_add(&circle_color),
        "subtract" => base_color.blend_subtract(&circle_color),
        _ => base_color, // Default to base color if unknown blend mode
    };

    combined_color * fragment.intensity
}

fn glow_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let glow_size = 0.05;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI / 2.0).sin();

    // Neon blue color for the glow
    Color::new(
        (0.0 * glow_intensity * 255.0) as u8,
        (0.5 * glow_intensity * 255.0) as u8,
        (glow_intensity * 255.0) as u8,
    )
}

fn core_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let core_size = 0.02;

    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let core_intensity = if distance_to_center < core_size {
        1.0
    } else {
        0.0
    };

    Color::new(
        (0.8 * core_intensity * 255.0) as u8,
        (0.9 * core_intensity * 255.0) as u8,
        (core_intensity * 255.0) as u8,
    )
}

fn background_shader(_fragment: &Fragment) -> Color {
    Color::new(10, 10, 20) // Dark blue background
}

// Combined neon light shader
pub fn neon_light_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    let background = background_shader(fragment);
    let glow = glow_shader(fragment);
    let core = core_shader(fragment);

    let blended_glow = background.blend_screen(&glow);
    blended_glow.blend_add(&core) * fragment.intensity
}

fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as u64;

    let mut rng = StdRng::seed_from_u64(seed);

    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);

    let random_color = Color::new(r, g, b);

    random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

    let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

    let random_number = rng.gen_range(0..=100);

    let black_or_white = if random_number < 50 {
        Color::new(0, 0, 0)
    } else {
        Color::new(255, 255, 255)
    };

    black_or_white * fragment.intensity
}

pub fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let noise_value = uniforms
        .noise
        .get_noise_2d((x + ox) * zoom, (y + oy) * zoom);

    let spot_threshold = 0.5;
    let spot_color = Color::new(255, 255, 255); // White
    let base_color = Color::new(0, 0, 0); // Black

    let noise_color = if noise_value < spot_threshold {
        spot_color
    } else {
        base_color
    };

    noise_color * fragment.intensity
}

pub fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 300.0; // to move our values
    let ox = 100.0; // offset x in the noise map
    let oy = 10.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let t = uniforms.time as f32 * 0.5;

    let noise_value = uniforms
        .noise
        .get_noise_2d(x * zoom + ox + t, y * zoom + oy);

    // Define cloud threshold and colors
    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds
    let sky_color = Color::new(30, 97, 145); // Sky blue

    // Determine if the pixel is part of a cloud or sky
    let noise_color = if noise_value > cloud_threshold {
        cloud_color
    } else {
        sky_color
    };

    noise_color * fragment.intensity
}

pub fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 300.0; // Zoom factor to adjust the scale of the cell pattern
    let ox = 50.0; // Offset x in the noise map
    let oy = 50.0; // Offset y in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms
        .noise
        .get_noise_2d(x * zoom + ox, y * zoom + oy)
        .abs();

    // Define different shades of green for the plant cells
    let cell_color_1 = Color::new(85, 107, 47); // Dark olive green
    let cell_color_2 = Color::new(124, 252, 0); // Light green
    let cell_color_3 = Color::new(34, 139, 34); // Forest green
    let cell_color_4 = Color::new(39, 101, 167); // Yellow green

    // Use the noise value to assign a different color to each cell
    let final_color = if cell_noise_value < 0.15 {
        cell_color_1
    } else if cell_noise_value < 0.7 {
        cell_color_2
    } else if cell_noise_value < 0.75 {
        cell_color_3
    } else {
        cell_color_4
    };

    // Adjust intensity to simulate lighting effects (optional)
    final_color * fragment.intensity
}

pub fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Base colors for the lava effect
    let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
    let dark_color = Color::new(130, 20, 0); // Darker red-orange

    // Get fragment position
    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    // Base frequency and amplitude for the pulsating effect
    let base_frequency = 0.2;
    let pulsate_amplitude = 0.5;
    let t = uniforms.time as f32 * 0.01;

    // Pulsate on the z-axis to change spot size
    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    // Apply noise to coordinates with subtle pulsating on z-axis
    let zoom = 1000.0; // Constant zoom factor
    let noise_value1 = uniforms.noise.get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom,
    );
    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x + 1000.0) * zoom,
        (position.y + 1000.0) * zoom,
        (position.z + 1000.0 + pulsate) * zoom,
    );
    let noise_value = (noise_value1 + noise_value2) * 0.5; // Averaging noise for smoother transitions

    // Use lerp for color blending based on noise value
    let color = dark_color.lerp(&bright_color, noise_value);

    color * fragment.intensity
}

pub fn earth(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 30.0; // Zoom factor to adjust the scale of the cell pattern
    let base_offset = 50.0; // Base offset in the noise map
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    // Desplazamiento dinámico en el tiempo para el efecto de movimiento
    let t = uniforms.time as f32 * 0.5; // Velocidad del movimiento
    let dynamic_offset_x = base_offset + t.sin() * 10.0; // Movimiento sinusoidal en x
    let dynamic_offset_y = base_offset + t.cos() * 10.0; // Movimiento sinusoidal en y

    // Use a cellular noise function to create the plant cell pattern
    let cell_noise_value = uniforms
        .noise
        .get_noise_2d(x * zoom + dynamic_offset_x, y * zoom + dynamic_offset_y)
        .abs();

    // Define different shades of green for the plant cells
    let base_color = if cell_noise_value < 0.15 {
        Color::new(85, 107, 47) // Dark olive green
    } else if cell_noise_value < 0.7 {
        Color::new(2, 100, 177) // Celeste
    } else if cell_noise_value < 0.75 {
        Color::new(85, 107, 47) // Forest green
    } else {
        Color::new(133, 98, 57) // Cafe
    };

    // Add cloud effect with blending
    let cloud_zoom = 100.0; // to move our values
    let cloud_ox = 100.0; // offset x in the noise map
    let cloud_oy = 100.0;
    let cloud_noise_value = uniforms
        .noise
        .get_noise_2d(x * cloud_zoom + cloud_ox + t, y * cloud_zoom + cloud_oy);

    let cloud_threshold = 0.5; // Adjust this value to change cloud density
    let cloud_color = Color::new(255, 255, 255); // White for clouds

    let blended_color = if cloud_noise_value > cloud_threshold {
        // Blend the cloud color with the base color using a blending factor to keep the base visible
        base_color.blend_normal(&cloud_color) * 0.5 + base_color * 0.5
    } else {
        base_color
    };

    // Adjust intensity to simulate lighting effects (optional)
    blended_color * fragment.intensity
}

pub fn luna_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let zoom = 100.0;
    let ox = 0.0;
    let oy = 0.0;
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let noise_value = uniforms
        .noise
        .get_noise_2d((x + ox) * zoom, (y + oy) * zoom);

    let spot_threshold = 0.5;
    let spot_color = Color::new(135, 135, 135); // gris oscuro
    let base_color = Color::new(191, 191, 191); // Black

    let noise_color = if noise_value < spot_threshold {
        spot_color
    } else {
        base_color
    };

    noise_color * fragment.intensity
}
