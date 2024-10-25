use crate::linear::{Point, Line};

pub(crate) fn get_cube() -> Vec<Line> {
    let mut line: Vec<Line> = vec![
        //near bottom line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: -8.0,
        }},
        //near top line
        Line{start: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: -8.0,
        }},
        //far bottom line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }},
        //far top line
        Line{start: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //left bottom line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }},

        //right bottom line
        Line{start: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: -8.0,
        }},
        //left top line
        Line{start: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }},
        //right top line
        Line{start: Point{
            x: 8.0,
            y: 8.0,
            z: -8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //far left line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: 8.0,
        }},
        //far right line
        Line{start: Point{
            x: 8.0,
            y: -8.0,
            z: 8.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: 8.0,
        }},
        //near left line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: -8.0,
        }, end: Point{
            x: -8.0,
            y: 8.0,
            z: -8.0,
        }},
        //near right line
        Line{start: Point{
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



pub(crate) fn get_square() -> Vec<Line> {
    let mut line: Vec<Line> = vec![

        //near bottom line
        Line{start: Point{
            x: -8.0,
            y: -8.0,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: -8.0,
            z: 0.0,
        }},
        //near top line
        Line{start: Point{
            x: -8.0,
            y: 8.0,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: 8.0,
            z: -0.0,
        }},
        //far bottom line
        Line{start: Point{
            x: -8.0,
            y: 8.0,
            z: 0.0,
        }, end: Point{
            x: -8.0,
            y: -8.0,
            z: 0.0,
        }},
        //far top line
        Line{start: Point{
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


pub(crate) fn get_tranglur_pyrimid() -> Vec<Line> {
    let mut line: Vec<Line> = vec![
        Line{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }},
        Line{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: -8.0,
            y: -6.92620,
            z: -6.92620,

        }},
        Line{start: Point{
            x: 0.0,
            y: 6.92620,
            z: 0.0,
        }, end: Point{
            x: 0.0,
            y: -6.92620,
            z: 6.92620,
        }},
        //base
        Line{start: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }, end: Point{
            x: -8.0,
            y: -6.92620,
            z: -6.92620,
        }},
        Line{start: Point{
            x: 8.0,
            y: -6.92620,
            z: -6.92620,
        }, end: Point{
            x: 0.0,
            y: -6.92620,
            z: 6.92620,
        }},
        Line{start: Point{
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

