use std::{error::Error, fs, path::Path};

pub struct Model {
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[usize; 3]>,
}

impl Model {
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        Self::parse(&fs::read_to_string(path)?)
    }

    fn parse(text: &str) -> Result<Self, Box<dyn Error>> {
        let mut model = Self {
            vertices: Vec::new(),
            faces: Vec::new(),
        };
        for (line_number, line) in text.lines().enumerate() {
            let mut fields = line.split('#').next().unwrap_or("").split_whitespace();
            match fields.next() {
                Some("v") => {
                    let mut vertex = [0.0; 3];
                    for value in &mut vertex {
                        *value = fields.next().ok_or("Vertice incompleto")?.parse::<f32>()?;
                        if !value.is_finite() {
                            return Err("Vertice no finito".into());
                        }
                    }
                    model.vertices.push(vertex);
                }
                Some("f") => {
                    let values: Vec<_> = fields.collect();
                    if values.len() != 3 {
                        return Err(format!(
                            "Linea {}: exporta el OBJ triangulado",
                            line_number + 1
                        )
                        .into());
                    }
                    let mut face = [0; 3];
                    for (i, value) in values.iter().enumerate() {
                        // En v/vt/vn solo necesitamos el primer indice: el vertice.
                        let index = value.split('/').next().unwrap().parse::<isize>()?;
                        let resolved = if index > 0 {
                            index - 1
                        } else if index < 0 {
                            model.vertices.len() as isize + index
                        } else {
                            return Err("OBJ usa indices desde 1, no desde 0".into());
                        };
                        if resolved < 0 || resolved as usize >= model.vertices.len() {
                            return Err("Indice de vertice fuera de rango".into());
                        }
                        face[i] = resolved as usize;
                    }
                    model.faces.push(face);
                }
                _ => {} // No se necesitan UV, normales ni materiales para este lab.
            }
        }
        if model.faces.is_empty() {
            return Err("El OBJ no contiene triangulos".into());
        }
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_obj_vertex_indices() {
        let text = "v 0 0 0\nv 1 0 0\nv 0 1 0\nf 1/9/2 2/8/2 3/7/2\nf -3 -2 -1";
        let model = Model::parse(text).unwrap();
        assert_eq!(model.faces, vec![[0, 1, 2], [0, 1, 2]]);
        assert_eq!(model.vertices[model.faces[0][1]], [1.0, 0.0, 0.0]);
    }

    #[test]
    fn rejects_invalid_faces() {
        let vertices = "v 0 0 0\nv 1 0 0\nv 0 1 0\n";
        for face in ["f 0 2 3", "f 1 2 4", "f 1 2 3 1"] {
            assert!(Model::parse(&format!("{vertices}{face}")).is_err());
        }
    }
}
