// use crate::hittables::Hittable;
// use std::fs::File;
// use std::io::Write;
//
//
// fn recursive(world: &Box<dyn Hittable>, file: &mut File) {
//
// }
//
// pub fn render_graph(world: &Box<dyn Hittable>) {
//     let mut file = File::open("debug.dot").unwrap();
//     file.write_all(b"digraph G {\n");
//     recursive(world)
//     file.write_all(b"}\n");
//
// }