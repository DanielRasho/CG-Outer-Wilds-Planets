use tobj;
use nalgebra_glm::{Vec2, Vec3};
use super::entity::{color::Color, vertex::Vertex};

pub struct Obj {
    meshes: Vec<Mesh>,
}

struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
    material_color: Color, // Store the diffuse color of the material
}

impl Obj {

    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, materials) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        // Parse the materials into a map of material names to diffuse colors
        let mut material_colors = std::collections::HashMap::new();
        
        // Match on the materials Result
        match &materials {  // Borrow materials here instead of moving it
            Ok(material_list) => {
                for material in material_list {
                    let diffuse = material.diffuse.unwrap_or([1.0, 1.0, 1.0]);
                    let color = Color::new(
                        (diffuse[0] * 255.0) as u8,
                        (diffuse[1] * 255.0) as u8,
                        (diffuse[2] * 255.0) as u8,
                    );
                    println!("{}, {}", material.name, color.to_string());
                    material_colors.insert(material.name.clone(), color);
                }
            }
            Err(_) => {
                // If there's an error loading materials, log or handle the error
                // Optionally, you can default to white for all meshes if materials can't be loaded
                eprintln!("Warning: Could not load materials. Defaulting to white color.");
            }
        }

        // Borrow `materials` here and pass it to the closure
        let meshes = models.into_iter().map(|model| {
            let mesh = model.mesh;

            // Get the material index from the mesh and use it to find the material name
            let material_index = mesh.material_id.unwrap_or(0); // Default to 0 if no material is specified
            let material_name = materials.as_ref()  // Borrow materials
                .ok()
                .and_then(|materials_list| materials_list.get(material_index))
                .map(|material| material.name.clone())
                .unwrap_or_else(|| "default".to_string()); // Default to "default" if no material is found

            println!("{}", material_name);
            // Get the material color from the material_colors map
            let material_color = material_colors.get(&material_name)
                .cloned()
                .unwrap_or(Color::new(255, 255, 255)); // Default to white if no material is found

            Mesh {
                vertices: mesh.positions.chunks(3)
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect(),
                normals: mesh.normals.chunks(3)
                    .map(|n| Vec3::new(n[0], n[1], n[2]))
                    .collect(),
                texcoords: mesh.texcoords.chunks(2)
                    .map(|t| Vec2::new(t[0], 1.0 - t[1])) // Flip Y for OpenGL-style texcoords
                    .collect(),
                indices: mesh.indices,
                material_color, // Store the diffuse color
            }
        }).collect();

        Ok(Obj { meshes })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for mesh in &self.meshes {
            for &index in &mesh.indices {
                let position = mesh.vertices[index as usize];
                let normal = mesh.normals.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                let tex_coords = mesh.texcoords.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));
                let color = mesh.material_color.clone(); // Use the mesh's material color

                vertices.push(Vertex::new_with_color(position, normal, tex_coords, color));
            }
        }

        vertices
    }
}