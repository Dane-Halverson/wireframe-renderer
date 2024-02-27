extern crate glfw;
extern crate gl;
extern crate rayon;
use rayon::prelude::*;
use std::path::Path;


mod linear;
mod stl_to_shape;

use std::{thread, time};
use glfw::{Action, Context, Key};
use gl::types::*;
use crate::linear::{point_to_screen, rotate, Point, Line};
use crate::stl_to_shape::convert_stl_to_vec;


fn main() {
    std::thread::Builder::new().stack_size(1000000000).spawn(||{

        // Set up window
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create Window
    let (mut window, events) = glfw.create_window(800, 800, "WireFrame", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make context current
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| glfw.get_proc_address_raw(s));
    gl::Viewport::load_with(|s| glfw.get_proc_address_raw(s));

    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 0.0); }

    //the shape to render
    let mut shape: Vec<Line> = convert_stl_to_vec("./bulb.STL", 0.5);

    //initial rotation
    for i in shape.iter_mut() {
        rotate(&mut i.start, 0.0, 0.0, 0.0);
        rotate(&mut i.end, 0.0, 0.0, 0.0);
    }


    // Loop until window closes
    while !window.should_close() {

        unsafe { gl::Viewport(0, 0, window.get_size().0, window.get_size().1); }

        //resets buffer data
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindBuffer(gl::ARRAY_BUFFER, 1);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STATIC_DRAW);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BufferData(gl::ARRAY_BUFFER, 0, std::ptr::null(), gl::STATIC_DRAW);
        }

        //fills buffer with line data
        create_lines(&mut shape, Point{
            x: 0.0,
            y: 0.0,
            z: -200.0,
        }, -15.0, 16.0);

        // Draws all lines
        unsafe {
            for i in 0..shape.len() {
                gl::DrawArrays(gl::LINES, i as GLint*2, 2);
            }
        }

        // Swap buffers
        window.swap_buffers();
        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
        let dur = time::Duration::from_millis(10);

        thread::sleep(dur);
    }
    }).unwrap().join().unwrap();

}

fn create_lines(line_list: &mut Vec<Line>, cam: Point, screen: f32, screen_width: f32) {
    // Define line data for the line
    let mut lines: Vec<f32> = vec![];

    let mut lines: Vec<f32> = line_list
        .par_iter_mut()
        .fold(Vec::new, |mut acc, i| {
            // turns lines into screen coords
            let mut point = point_to_screen(i.start, screen, cam);
            acc.push(point.x / screen_width);
            acc.push(point.y / screen_width);
            point = point_to_screen(i.end, screen, cam);
            acc.push(point.x / screen_width);
            acc.push(point.y / screen_width);

            // rotates the points
            rotate(&mut i.start, 0.0, 0.01, 0.0);
            rotate(&mut i.end, 0.0, 0.01, 0.0);

            acc
        })
        .reduce(Vec::new, |mut acc1, acc2| {
            acc1.extend(acc2);
            acc1
        });

    // create a vertex object and vertex buffer object
    let
        mut vao = 1;
    let
        mut vbo = 1;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (lines.len() * std::mem::size_of::<f32>()) as GLsizeiptr,
                       lines.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW);

        // Specify vertex attribute layout
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 2 * std::mem::size_of::<f32>() as GLsizei, std::ptr::null());
        gl::EnableVertexAttribArray(1);
        lines.clear();
    }

}