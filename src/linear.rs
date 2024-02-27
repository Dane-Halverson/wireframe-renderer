use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



#[derive(Clone, Copy, PartialEq)]
pub(crate) struct Point {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32
}

#[derive(Clone, Copy, PartialEq)] // Add the PartialEq trait
pub(crate) struct Line {
    pub(crate) start: Point,
    pub(crate) end: Point
}



pub(crate) struct ScreenPoint {
    pub(crate) x: f32,
    pub(crate) y: f32
}



pub(crate) fn point_to_screen(point: Point, screen: f32, camera: Point) -> ScreenPoint {
    let x_point: f32 = (point.x - camera.x)*((screen - camera.z)/(point.z - camera.z));
    let y_point: f32 = (point.y - camera.y)*((screen - camera.z)/(point.z - camera.z));
    let screen_point: ScreenPoint = ScreenPoint{
        x: x_point,
        y: y_point
    };
    return screen_point;
}

pub(crate) fn rotate(point: &mut Point, yaw: f32, pitch: f32, roll: f32) {

    let col1: [f32; 3] = [
        f32::cos(pitch) * f32::cos(yaw),
        f32::cos(pitch) * f32::sin(yaw),
        f32::sin(pitch) * -1.0
    ];
    let col2: [f32; 3] = [
        f32::sin(roll) * f32::sin(pitch) * f32::cos(yaw) - f32::cos(roll) * f32::sin(yaw),
        f32::sin(roll) * f32::sin(pitch) * f32::sin(yaw) + f32::cos(roll) * f32::cos(yaw),
        f32::sin(roll) * f32::cos(pitch)
    ];
    let col3: [f32; 3] = [
        f32::cos(yaw)*f32::sin(pitch)*f32::cos(roll)+f32::sin(roll)*f32::sin(yaw),
        f32::cos(roll)*f32::sin(pitch)*f32::sin(yaw)-f32::sin(roll)*f32::cos(yaw),
        f32::cos(roll) * f32::cos(pitch)
    ];
    point.x = point.x*col1[0] + point.y*col1[1] + point.z*col1[2];
    point.y = point.x*col2[0] + point.y*col2[1] + point.z*col2[2];
    point.z = point.x*col3[0] + point.y*col3[1] + point.z*col3[2];

}



