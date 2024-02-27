use crate::linear::{Point, Vertex};

pub(crate) fn get_cube() -> Vec<Vertex> {
    let mut line: Vec<Vertex> = vec![
        //near bottom line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: -8.0,
        }},
        //near top line
        Vertex{start: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: -8.0,
        }},
        //far bottom line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }},
        //far top line
        Vertex{start: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //left bottom line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }},

        //right bottom line
        Vertex{start: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: -8.0,
        }},
        //left top line
        Vertex{start: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }},
        //right top line
        Vertex{start: Point{
            x: 8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //far left line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }},
        //far right line
        Vertex{start: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //near left line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }},
        //near right line
        Vertex{start: Point{
            x: 8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: -8.0,
        }},
    ];
    return line;
}



pub(crate) fn get_square() -> Vec<Vertex> {
    let mut line: Vec<Vertex> = vec![

        //near bottom line
        Vertex{start: Point{
            x: -8.0,
            y: -8.0,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: 0.0,
        }},
        //near top line
        Vertex{start: Point{
            x: -8.0,
            y: 8.0,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: -0.0,
        }},
        //far bottom line
        Vertex{start: Point{
            x: -8.0,
            y: 8.0,
            z: 0.0,
        }, end: Point{
            x: -8.0,
            y: -8.0,
            z: 0.0,
        }},
        //far top line
        Vertex{start: Point{
            x: 8.0,
            y: 8.0,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: 0.0,
        }},


    ];
    return line;
}


pub(crate) fn get_tranglur_pyrimid() -> Vec<Vertex> {
    let mut line: Vec<Vertex> = vec![
        Vertex{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }},
        Vertex{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: -8.0,
            y: -6.92620,
            z: -6.92620,

        }},
        Vertex{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: 0.0,
            y: -6.92620,
            z: 6.92620,
        }},
        //base
        Vertex{start: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }, end: Point{
            x: -8.0,
            y: -6.92620,
            z: -6.92620,
        }},
        Vertex{start: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }, end: Point{
            x: 0.0,
            y: -6.92620,
            z: 6.92620,
        }},
        Vertex{start: Point{
            x: 0.0,
            y: -6.92620,
            z: 6.92620,
        }, end: Point{
            x: -8.0,
            y: -6.92620,
            z: -6.92620,
        }},
    ];
    return line;
}

