use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::ops::{Add, Sub, Mul, Div};

impl Add for Point {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Self {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, b: Self) -> Point {
        Point {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl Add<Point> for &Point {
    type Output = Point;
    fn add(self, b: Point) -> Point {
        Point {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl Add<&Point> for Point {
    type Output = Self;
    fn add(self, b: &Self) -> Self {
        Self {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl Add<f32> for Point {
    type Output = Self;
    fn add(self, b: f32) -> Self {
        Self { x: self.x + b, y: self.y + b, z: self.z + b }
    }
}

impl Add<f32> for &Point {
    type Output = Point;
    fn add(self, b: f32) -> Point {
        Point { x: self.x + b, y: self.y + b, z: self.z + b }
    }
}

impl Add<&Point> for f32 {
    type Output = Point;
    fn add(self, b: &Point) -> Point {
        Point { x: self + b.x, y: self + b.y, z: self + b.z }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Self {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}

impl Sub for &Point {
    type Output = Point;
    fn sub(self, b: Self) -> Point {
        Point {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}

impl Sub<&Point> for Point {
    type Output = Self;
    fn sub(self, b: &Self) -> Self {
        Self {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}

impl Sub<Point> for &Point {
    type Output = Point;
    fn sub(self, b: Point) -> Point {
        Point {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}

impl Sub<f32> for Point {
    type Output = Self;
    fn sub(self, b: f32) -> Self {
        Self { x: self.x - b, y: self.y - b, z: self.z - b }
    }
}

impl Sub<f32> for &Point {
    type Output = Point;
    fn sub(self, b: f32) -> Point {
        Point { x: self.x - b, y: self.y - b, z: self.z - b }
    }
}

impl Sub<&Point> for f32 {
    type Output = Point;
    fn sub(self, b: &Point) -> Point {
        Point { x: self - b.x, y: self - b.y, z: self - b.z }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Self {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }
}

impl Mul for &Point {
    type Output = Point;
    fn mul(self, b: Self) -> Point {
        Point {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }
}

impl Mul<Point> for &Point {
    type Output = Point;
    fn mul(self, b: Point) -> Point {
        Point {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }
}

impl Mul<&Point> for Point {
    type Output = Self;
    fn mul(self, b: &Self) -> Self {
        Self {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }
}

impl Mul<f32> for Point {
    type Output = Self;
    fn mul(self, b: f32) -> Self {
        Self { x: self.x * b, y: self.y * b, z: self.z * b }
    }
}

impl Mul<f32> for &Point {
    type Output = Point;
    fn mul(self, b: f32) -> Point {
        Point { x: self.x * b, y: self.y * b, z: self.z * b }
    }
}

impl Mul<&Point> for f32 {
    type Output = Point;
    fn mul(self, b: &Point) -> Point {
        Point { x: self * b.x, y: self * b.y, z: self * b.z }
    }
}

impl Div for Point {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        Self {x: self.x / b.x, y: self.y / b.y, z: self.z / b.z}
    }
}

impl Div for &Point {
    type Output = Point;
    fn div(self, b: Self) -> Point {
        Point {x: self.x / b.x, y: self.y / b.y, z: self.z / b.z}
    }
}

impl Div<Point> for &Point {
    type Output = Point;
    fn div(self, b: Point) -> Point {
        Point {x: self.x / b.x, y: self.y / b.y, z: self.z / b.z}
    }
}

impl Div<&Point> for Point {
    type Output = Self;
    fn div(self, b: &Self) -> Self {
        Self {x: self.x / b.x, y: self.y / b.y, z: self.z / b.z}
    }
}

impl Div<f32> for Point {
    type Output = Self;
    fn div(self, b: f32) -> Self {
        Self { x: self.x / b, y: self.y / b, z: self.z / b }
    }
}

impl Div<f32> for &Point {
    type Output = Point;
    fn div(self, b: f32) -> Point {
        Point { x: self.x / b, y: self.y / b, z: self.z / b }
    }
}

impl Div<&Point> for f32 {
    type Output = Point;
    fn div(self, b: &Point) -> Point {
        Point { x: self / b.x, y: self / b.y, z: self / b.z }
    }
}

const FOV: f32 = 90.0;
const ASPECT_RATIO: f32 = 1.0 / 1.0; // 16.0 / 9.0;
const IMG_WIDTH: u32 = 500;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const BACKGROUND_COLOR: Point = Point{x: 0.2, y: 0.2, z: 0.2};
const FOCAL_LENGTH: f32 = 1.0;

struct Point {
    x: f32,
    y: f32,
    z: f32,
}

struct Shader {
    kd: f32,
    ks: f32,
    ka: f32,
    od: Point,
    os: Point,
    kgls: f32,
    ao: Point,
}

struct Sphere {
    center: Point,
    radius: f32,
    shader: Shader,
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

struct Triangle2D {
    p1: [f32;2],
    p2: [f32;2],
    p3: [f32;2],
}

struct Polygon {
    triangles: Vec<Triangle>,
    shader: Shader,
}

struct Polygon2D {
    triangles: Vec<Triangle2D>,
    shader: Shader,
}

trait Object {
    fn intersect(self, r: Ray) -> (bool, Point);
}

impl Object for Sphere {
    fn intersect(self, r: Ray) -> (bool, Point) {
        return sphere_intersection(&r, &self);
    }
}

// impl Object for Polygon {
//     fn intersect(&self, r: Ray) -> (bool, Point) {
//         let d = compute_distance(r.origin + r.direction);
//         // compute_plane_normal();
//         return plane_intersection(d, n, r);
//     }
// }

struct Light {
    color: Point,
    direction: Point,
}

struct Camera {
    look_from: Point,
    look_at: Point,
    look_up: Point,
    fov: f32,
}

struct Ray<'a> {
    origin: &'a Point,
    direction: Point,
}

struct Scene {
    objects: Vec<Sphere>,
    lights: Vec<Light>,
    camera: Camera,
}

fn cross(a: Point, b:Point) -> Point {
    Point {x: a.y * b.z - a.z * b.y, y: a.z * b.x - a.x * b.z, z: a.x * b.y - a.y * b.x}
}

fn dot(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn create_ppm(filename: &str) -> std::io::Result<()> {
    let mut output = File::create(filename)?;
    write!(output, "P3\n{} {}\n255\n",IMG_WIDTH, IMG_HEIGHT)?;
    Ok(())
}

fn write_to_ppm(filename: &str, color: Point) {
    let r = (color.x * 255.999) as u32;
    let g = (color.y * 255.999) as u32;
    let b = (color.z * 255.999) as u32;
    let mut file = OpenOptions::new().append(true).open(filename).expect("Unable to open file");
    let pixel_color = format!("{} {} {}\n", r, g, b);
    file.write_all(pixel_color.as_bytes()).expect("write failed");
}

fn construct_scene() -> Scene {
    let c1 = Camera {
        look_from: Point { x: 0.0, y: 0.0, z: 1.0 },
        look_at: Point { x: 0.0, y: 0.0, z: 0.0 },
        look_up: Point { x: 0.0, y: 1.0, z: 0.0 },
        fov: 90.0,
    };
    let l1 = Light {
        // location: [0.0,0.0,0.0],
        color: Point { x: 1.0, y: 1.0, z: 1.0 },
        direction: normalize_vector(&Point { x: 1.0, y: 1.0, z: 1.0 }),
    };
    // let s0 = Sphere {
    //     center: array![0.0,0.0,0.0],
    //     radius: 0.4,
    //     shader: Shader {
    //         kd: 0.7,
    //         ks: 0.2,
    //         ka: 0.1,
    //         od: array![1.0,0.0,1.0],
    //         os: array![1.0,1.0,1.0],
    //         kgls: 16.0,
    //         ao: array![0.1,0.1,0.1],
    //     }
    // };
    let s1 = Sphere {
        center: Point { x: 0.45, y: 0.0, z: -0.15 },
        radius: 0.15,
        shader: Shader {
            kd: 0.8,
            ks: 0.1,
            ka: 0.3,
            od: Point { x: 1.0, y: 1.0, z: 1.0 },
            os: Point { x: 1.0, y: 1.0, z: 1.0 },
            kgls: 4.0,
            ao: Point { x: 0.1, y: 0.1, z: 0.1 },
        }
    };
    let s2 = Sphere {
        center: Point { x: 0.0, y: 0.0, z: -0.1 },
        radius: 0.2,
        shader: Shader {
            kd: 0.6,
            ks: 0.3,
            ka: 0.1,
            od: Point { x: 1.0, y: 0.0, z: 0.0 },
            os: Point { x: 1.0, y: 1.0, z: 1.0 },
            kgls: 32.0,
            ao: Point { x: 0.1, y: 0.1, z: 0.1 },
        }
    };
    let s3 = Sphere {
        center: Point { x: -0.6, y: 0.0, z: 0.0 },
        radius: 0.3,
        shader: Shader {
            kd: 0.7,
            ks: 0.2,
            ka: 0.1,
            od: Point { x: 0.0, y: 1.0, z: 0.0 },
            os: Point { x: 0.5, y: 1.0, z: 0.5 },
            kgls: 64.0,
            ao: Point { x: 0.1, y: 0.1, z: 0.1 },
        }
    };
    let s4 = Sphere {
        center: Point { x: 0.0, y: -10000.5, z: 0.0 },
        radius: 10000.0,
        shader: Shader {
            kd: 0.9,
            ks: 0.0,
            ka: 0.1,
            od: Point { x: 0.0, y: 0.0, z: 1.0 },
            os: Point { x: 1.0, y: 1.0, z: 1.0 },
            kgls: 16.0,
            ao: Point { x: 0.1, y: 0.1, z: 0.1 },
        }
    };

    let mut objects = Vec::new();
    //let o1 = Object::Sphere(Sphere::from(s1));
    // objects.push(s0);
    objects.push(s1);
    objects.push(s2);
    objects.push(s3);
    objects.push(s4);
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn calculate_diffuse(kd: f32, plc: &Point, od: &Point, n: &Point, l: &Point) -> Point {
    kd * plc * od * f32::max(0.0, dot(&n,&l)) as f32
}

fn calculate_ambient(ka: f32, ao: &Point, od: &Point) -> Point {
    ka * ao * od
}

fn calculate_specular(ks: f32, plc: &Point, os: &Point, v: &Point, r: &Point, kgls: f32) -> Point {
    ks * plc * os * (f32::max(0.0, dot(&v,&r)) as f32).powf(kgls)
}

fn calculate_sphere_normal(p: &Point, s: &Sphere) -> Point {
    let n = Point {x: (p.x - s.center.x) / s.radius, y: (p.y - s.center.y) / s.radius, z: (p.z - s.center.z) / s.radius};
    n
}

fn calculate_reflection(n: &Point, l: &Point) -> Point {
    2.0 * n * dot(n,l) - l
}

fn show_normals(p: &Point, s: &Sphere) -> Point {
    let n = calculate_sphere_normal(p, s);
    n
}

fn calculate_lighting(p: &Point, sh: &Shader, s: &Sphere, l: &Light, v: &Point) -> Point {
    let n = calculate_sphere_normal(p,s);
    let r = calculate_reflection(&n, &l.direction);
    let dif = calculate_diffuse(sh.kd, &l.color, &sh.od, &n, &l.direction);
    let amb = calculate_ambient(sh.ka, &sh.ao, &sh.od);
    let spec = calculate_specular(sh.ks, &l.color, &sh.os, &v, &r, sh.kgls);
    dif + amb + spec
}

fn sphere_intersection(r: &Ray, s: &Sphere) -> (bool, Point) {
    let oc = r.origin - &s.center;
    let a = dot(&r.direction,&r.direction);
    let b = 2.0 * dot(&oc,&r.direction);
    let c = dot(&oc,&oc) - s.radius * s.radius;
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return (false, Point {x: 0.0, y: 0.0, z: 0.0});
    }
    let t: f32;
    let t0: f32 = (-b - d.sqrt()) / (2.0 * a);
    if t0 <= 0.0 {
        let t1: f32 = (-b + d.sqrt()) / (2.0 * a);
        if t1 <= 0.0 {
            return (false, Point {x: 0.0, y: 0.0, z: 0.0});
        }
        t = t1;
    } else {
        t = t0;
    }
    return (true, r.origin + &r.direction * t);
}

fn compute_triangle_normal(tri: Triangle) -> Point {
    let v1 = tri.p1 - &tri.p2;
    let v2 = tri.p3 - &tri.p2;
    v2 * v1
}

// fn compute_plane_normal() -> Point {
    
// }

fn get_dominant_coordinate(n: Point) -> usize {
    let v = [n.x, n.y,n.z];
    let mut dom: usize = 0;
    for axis in 0..=2 {
        if v[axis].abs() > v[dom].abs() {
            dom = axis;
        }
    }
    return dom;
}

fn project_verticies(tri: Triangle, axis: usize) -> Triangle2D {
    let mut u = 0;
    let mut v = 1;
    let tp1 = [tri.p1.x,tri.p1.y,tri.p1.z];
    let tp2 = [tri.p2.x,tri.p2.y,tri.p2.z];
    let tp3 = [tri.p3.x,tri.p3.y,tri.p3.z];
    if axis == v {
        v = 2;
    }
    if axis == u {
        u = 1;
        v = 2;
    }
    Triangle2D { p1: [tp1[u],tp1[v]], p2: [tp2[u],tp2[v]], p3: [tp3[u],tp3[v]] }
}

fn project_intersection_point(p: Point) -> [f32;2] {
    [p.x,p.z]
}

fn compute_distance(p: Point) -> f32 {
    (p.x.powi(2) + p.y.powi(2) + p.z.powi(2)).sqrt()
}

fn polygon_intersection(r: Ray, poly: Polygon) {
    let axis = 2;
    let mut poly2d = Polygon2D {
        triangles: Vec::new(),
        shader: poly.shader,
    };
    for tri in poly.triangles {
        poly2d.triangles.push(project_verticies(tri, axis));
    }
}

fn plane_intersection(d: f32, n: Point,r: Ray) -> (bool, Point) {
    let vd: f32 = dot(&n,&r.direction);
    if vd >= 0.0 {
        return (false, Point {x: 0.0, y: 0.0, z: 0.0});
    }
    let t: f32 = -(dot(&n,r.origin) + d) / vd;
    if t <= 0.0 {
        return (false, Point {x: 0.0, y: 0.0, z: 0.0});
    }
    return (true, r.origin + r.direction * t);
}

fn normalize_vector(v: &Point) -> Point {
    let norm = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
    v / norm.sqrt()
}

fn calculate_ray(origin: &Point, direction: Point) -> Ray {
    let r = Ray {
        origin,
        direction,
    };
    r
}

fn trace_rays(scene: Scene, filename: &str) {
    let screen_w = FOCAL_LENGTH * (FOV / 2.0).to_radians().tan();
    let screen_h = screen_w / ASPECT_RATIO;
    let h = Point{x: screen_w / 2.0, y: 0.0, z: FOCAL_LENGTH};
    let v = Point{x: 0.0, y: screen_h / 2.0, z: 0.0};
    let tlc = v - h;
    let x_inc = screen_w / (IMG_WIDTH as f32) as f32;
    let y_inc = screen_h / (IMG_HEIGHT as f32) as f32;
    let mut prev_y:f32 = 0.0;
    for _j in 0..IMG_HEIGHT {
        let mut prev_x: f32 = 0.0;
        for _i in 0..IMG_WIDTH {
            let dir = &tlc + Point{x: prev_x, y: -prev_y, z: 0.0};
            let r = calculate_ray(&scene.camera.look_from, dir);
            let mut pixel_objects: Vec<&Sphere> = Vec::new();
            let mut pixel_points: Vec<Point> = Vec::new();
            for o in &scene.objects {
                let (b,p) = sphere_intersection(&r,o);
                if b {
                    pixel_objects.push(o);
                    pixel_points.push(p);
                }
            }
            if !pixel_objects.is_empty() {
                let mut object_in_front = pixel_objects[0];
                let mut point = &pixel_points[0];
                for i in 0..(pixel_objects.len()-1) {
                    let current_object = pixel_objects[i];
                    if pixel_points[i].z > point.z {
                        object_in_front = &current_object;
                        point = &pixel_points[i];
                    }
                }
                let mut color = Point{x: 0.0, y: 0.0, z: 0.0};
                for l in &scene.lights {
                    // color = show_normals(point, object_in_front);
                    color = color + calculate_lighting(point, &object_in_front.shader, object_in_front, l, &scene.camera.look_from);
                }
                write_to_ppm(filename, color);
            } else {
                write_to_ppm(filename, BACKGROUND_COLOR);
            }
            prev_x += x_inc;
        }
        prev_y += y_inc;
    }
}

fn main() -> Result<(), std::io::Error> {
    let scene = construct_scene();
    let filename = "image2.ppm";
    let result = create_ppm(filename);
    trace_rays(scene, filename);
    result
}
