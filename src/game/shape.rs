
pub enum Shape {
    Polygon {number_of_sides: u32, width: u32, height: u32, vertices: Vec<u32>, indices: Vec<u32>},
    Circle {radius: u32, vertices: Vec<u32>, indices: Vec<u32>}
}
