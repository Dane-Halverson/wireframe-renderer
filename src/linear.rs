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



pub(crate) fn point_to_screen(point: &Point, screen: f32, camera: &Point) -> ScreenPoint {
    let factor: f32 = (screen - camera.z)/(point.z - camera.z);
    let x_point: f32 = (point.x - camera.x)*factor;
    let y_point: f32 = (point.y - camera.y)*factor;
    let screen_point: ScreenPoint = ScreenPoint{
        x: x_point,
        y: y_point
    };
    return screen_point;
}

pub(crate) fn rotate(point: &mut Point, yaw: f32, pitch: f32, roll: f32) {
    let (sin_pitch, cos_pitch) = pitch.sin_cos();
    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    let (sin_roll, cos_roll) = roll.sin_cos();

    let col1: [f32; 3] = [
        cos_pitch * cos_yaw,
        cos_pitch * sin_yaw,
        -sin_pitch
    ];
    let col2: [f32; 3] = [
        sin_roll * sin_pitch * cos_yaw - cos_roll * sin_yaw,
        sin_roll * sin_pitch * sin_yaw + cos_roll * cos_yaw,
        sin_roll * cos_pitch
    ];
    let col3: [f32; 3] = [
        cos_roll * sin_pitch * cos_yaw + sin_roll * sin_yaw,
        cos_roll * sin_pitch * sin_yaw - sin_roll * cos_yaw,
        cos_roll * cos_pitch
    ];
    point.x = point.x*col1[0] + point.y*col1[1] + point.z*col1[2];
    point.y = point.x*col2[0] + point.y*col2[1] + point.z*col2[2];
    point.z = point.x*col3[0] + point.y*col3[1] + point.z*col3[2];
}




