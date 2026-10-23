use crate::framebuffer::Framebuffer;

fn edge(a: [f32; 3], b: [f32; 3], x: f32, y: f32) -> f32 {
    (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0])
}

// Rasterizacion propia: recorre pixeles y usa coordenadas baricentricas.
pub fn triangle(fb: &mut Framebuffer, a: [f32; 3], b: [f32; 3], c: [f32; 3], color: [u8; 3]) {
    let area = edge(a, b, c[0], c[1]);
    if area.abs() < 1e-6 {
        return;
    }
    let min_x = a[0].min(b[0]).min(c[0]).floor().max(0.0) as usize;
    let max_x = a[0].max(b[0]).max(c[0]).ceil().min(fb.width as f32 - 1.0) as usize;
    let min_y = a[1].min(b[1]).min(c[1]).floor().max(0.0) as usize;
    let max_y = a[1].max(b[1]).max(c[1]).ceil().min(fb.height as f32 - 1.0) as usize;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let u = edge(b, c, px, py) / area;
            let v = edge(c, a, px, py) / area;
            let w = edge(a, b, px, py) / area;
            if u >= -1e-6 && v >= -1e-6 && w >= -1e-6 {
                fb.point(x, y, u * a[2] + v * b[2] + w * c[2], color);
            }
        }
    }
}
