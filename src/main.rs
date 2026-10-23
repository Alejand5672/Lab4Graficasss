mod framebuffer;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use obj::Model;
use raylib::prelude::*;
use std::{error::Error, path::Path};

const WIDTH: usize = 900;
const HEIGHT: usize = 700;

fn subtract(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn face_color(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [u8; 3] {
    let u = subtract(b, a);
    let v = subtract(c, a);
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let length = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-6);
    // Un tono por cara para distinguir las superficies del modelo.
    let intensity =
        0.30 + 0.70 * ((n[0] * 0.3 + n[1] * 0.8 + n[2] * 0.5) / length / 0.9899495).max(0.0);
    [190.0, 210.0, 235.0].map(|channel| (channel * intensity) as u8)
}

fn project(vertices: &[[f32; 3]]) -> Vec<[f32; 3]> {
    let (sy, cy) = 35.0_f32.to_radians().sin_cos();
    let (sp, cp) = 30.0_f32.to_radians().sin_cos();
    let mut screen: Vec<_> = vertices
        .iter()
        .map(|&[x, y, z]| {
            let depth = sy * x + cy * z;
            [cy * x - sy * z, cp * y - sp * depth, -(cp * depth + sp * y)]
        })
        .collect();
    let mut min = [f32::INFINITY; 2];
    let mut max = [f32::NEG_INFINITY; 2];
    for p in &screen {
        for i in 0..2 {
            min[i] = min[i].min(p[i]);
            max[i] = max[i].max(p[i]);
        }
    }
    // Encajar al 80% de la pantalla y centrar por los limites proyectados.
    let scale = (WIDTH as f32 * 0.8 / (max[0] - min[0]).max(1e-6))
        .min(HEIGHT as f32 * 0.8 / (max[1] - min[1]).max(1e-6));
    for p in &mut screen {
        p[0] = (p[0] - (min[0] + max[0]) * 0.5) * scale + WIDTH as f32 * 0.5;
        p[1] = HEIGHT as f32 * 0.5 - (p[1] - (min[1] + max[1]) * 0.5) * scale;
    }
    screen
}

fn render(model: &Model, fb: &mut Framebuffer) {
    fb.clear([20, 25, 35]);
    let screen = project(&model.vertices);
    // Recorrido manual de TODAS las caras: indices -> vertices -> triangulo.
    for face in &model.faces {
        let i0 = face[0];
        let i1 = face[1];
        let i2 = face[2];
        let a = model.vertices[i0];
        let b = model.vertices[i1];
        let c = model.vertices[i2];
        triangle::triangle(fb, screen[i0], screen[i1], screen[i2], face_color(a, b, c));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let model = Model::load(&root.join("nave.obj"))?;
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    render(&model, &mut framebuffer);
    let output = root.join("render.png");
    framebuffer.save(&output)?;
    println!(
        "{} vertices, {} triangulos. Imagen: {}",
        model.vertices.len(),
        model.faces.len(),
        output.display()
    );

    // Permite generar la entrega sin abrir una ventana.
    if std::env::args().any(|arg| arg == "--export") {
        return Ok(());
    }
    let (mut window, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Lab 4 - Nave OBJ")
        .build();
    window.set_target_fps(60);
    let image = Image::load_image(output.to_str().ok_or("Ruta no valida")?)?;
    let texture = window.load_texture_from_image(&thread, &image)?;
    while !window.window_should_close() {
        let mut drawing = window.begin_drawing(&thread);
        drawing.clear_background(Color::BLACK);
        // Raylib solo muestra el framebuffer; no dibuja la geometria 3D.
        drawing.draw_texture(&texture, 0, 0, Color::WHITE);
    }
    Ok(())
}
