#![allow(unused_variables)]
use glium;
fn main() {
    println!("Hello, world!");
    use glium::glutin;
    let events_loop = glutin::event_loop::EventLoop::new();
    let cb = glutin::ContextBuilder::new();
    let wb = glutin::window::WindowBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("Display could not be created.");
}
