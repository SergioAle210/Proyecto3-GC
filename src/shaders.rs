use crate::color::Color;
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::Uniforms;
use nalgebra_glm::{dot, mat4_to_mat3, Mat3, Vec3, Vec4};
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

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Apply the fragment intensity to the mixed color
    combined_shader(fragment, uniforms)
    // combined_blend_shader(fragment, "normal")
    // combined_blend_shader(fragment, "multiply")
    // combined_blend_shader(fragment, "add")
    // combined_blend_shader(fragment, "subtract")
    // neon_light_shader(fragment)
}

fn static_pattern_shader(fragment: &Fragment) -> Color {
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;

    let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

    let r = (pattern * 255.0) as u8;
    let g = ((1.0 - pattern) * 255.0) as u8;
    let b = 128;

    Color::new(r, g, b)
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
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
    let base_color = static_pattern_shader(fragment);
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
pub fn neon_light_shader(fragment: &Fragment) -> Color {
    let background = background_shader(fragment);
    let glow = glow_shader(fragment);
    let core = core_shader(fragment);

    // Blend the glow with the background using "screen" blend mode
    let blended_glow = background.blend_screen(&glow);

    // Add the core on top using "add" blend mode
    blended_glow.blend_add(&core) * fragment.intensity
}
