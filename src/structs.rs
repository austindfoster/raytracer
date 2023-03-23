use crate::sphere_intersection;
use crate::polygon_intersection;
use crate::calculate_sphere_normal;
use crate::compute_triangle_normal;
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Shader {
    pub kd: f32,
    pub ks: f32,
    pub ka: f32,
    pub od: Point,
    pub os: Point,
    pub kgls: f32,
    pub kr: f32,
    pub ao: Point,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub shader: Shader,
}

pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

pub struct Polygon {
    pub triangles: Vec<Triangle>,
    pub shader: Shader,
}

// enum Object {
//     Sphere(Sphere),
//     Polygon(Polygon),
// }

pub trait Object {
    
    fn intersect(&self, r: &Ray) -> (bool, Point);
    
    fn calculate_normal(&self, p: &Point) -> Point;
    
    fn get_shader(&self) -> &Shader;
}

impl Object for Sphere {
    
    fn intersect(&self, r: &Ray) -> (bool, Point) {
        return sphere_intersection(r, self);
    }

    
    fn calculate_normal(&self, p: &Point) -> Point {
        return calculate_sphere_normal(p, &self);
    }

    
    fn get_shader(&self) -> &Shader {
        return &self.shader;
    }
}

impl Object for Polygon {
    
    fn intersect(&self, r: &Ray) -> (bool, Point) {
        return polygon_intersection(r, self);
    }

    
    fn calculate_normal(&self, p: &Point) -> Point {
        let tn = compute_triangle_normal(&self.triangles[p.x as usize]);
        return tn;
    }

    
    fn get_shader(&self) -> &Shader {
        return &self.shader;
    }
}

pub struct Light {
    pub color: Point,
    pub direction: Point,
}

pub struct Camera {
    pub look_from: Point,
    pub look_at: Point,
    pub look_up: Point,
    pub fov: f32,
}

pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: Point,
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub camera: Camera,
}
