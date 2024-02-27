use stl_io::Triangle;
use std::path::Path;
use stl_io::read_stl;
use std::fs::OpenOptions;


use crate::linear::{Point, Line};

pub fn convert_stl_to_vec(file_name: &str, scale: f32) -> Vec<Line> {
    let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
    let stl = read_stl(&mut file).unwrap();
    let mut shape: Vec<Line> = Vec::new();
    let verts = stl.vertices;
    for i in stl.faces.iter() {
        let mut line: Line = Line{
            start: Point{
                x: verts[i.vertices[0]][0]*scale,
                y: verts[i.vertices[0]][1]*scale,
                z: verts[i.vertices[0]][2]*scale
            },
            end: Point{
                x:  verts[i.vertices[1]][0]*scale,
                y:  verts[i.vertices[1]][1]*scale,
                z:  verts[i.vertices[1]][2]*scale
            }
        };
        shape.push(line);
        line = Line{
            start: Point{
                x:  verts[i.vertices[1]][0]*scale,
                y:  verts[i.vertices[1]][1]*scale,
                z:  verts[i.vertices[1]][2]*scale
            },
            end: Point{
                x:  verts[i.vertices[2]][0]*scale,
                y:  verts[i.vertices[2]][1]*scale,
                z:  verts[i.vertices[2]][2]*scale
            }
        };
        shape.push(line);
        line = Line{
            start: Point{
                x:  verts[i.vertices[2]][0]*scale,
                y:  verts[i.vertices[2]][1]*scale,
                z:  verts[i.vertices[2]][2]*scale
            },
            end: Point{
                x:  verts[i.vertices[0]][0]*scale,
                y:  verts[i.vertices[0]][1]*scale,
                z:  verts[i.vertices[0]][2]*scale
            }
        };
        shape.push(line);
    }
    return shape;
}

