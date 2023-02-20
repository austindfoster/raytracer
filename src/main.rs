// use std::fs::File;
// use std::fs::OpenOptions;
// use std::io::Write;

use ndarray::Array1;
use ndarray::array;

// const FOV: f32 = 90.0;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMG_WIDTH: u32 = 400;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;

const VIEW_H: f32 = 2.0;
const VIEW_W: f32 = ASPECT_RATIO * VIEW_H;
const FOCAL_LENGTH: f32 = 1.0;

// const VERTICAL: [f32;3] = [0.0, VIEW_H, 0.0];
// const HORIZONTAL: [f32;3] = [VIEW_W, 0.0, 0.0];

struct Shader {
    od: Array1<f32>,
    kd: f32,
    os: Array1<f32>,
    ks: f32,
    kgls: f32,
    ao: Array1<f32>,
    ka: f32,
}

struct Sphere {
    center: Array1<f32>,
    radius: f32,
    shader: Shader,
}

// struct Polygon {
//     triangles: Vec<Triagle>,

// }

// enum Object {
//     Sphere(Sphere),
//     //Polygon(Polygon),
// }

struct Light {
    // location: Array1<f32>,
    color: Array1<f32>,
    direction: Array1<f32>,
}

struct Camera {
    look_from: Array1<f32>,
    // look_at: Array1<f32>,
    // look_up: Array1<f32>,
    // fov: f32,
}

struct Ray {
    origin: Array1<f32>,
    direction: Array1<f32>,
}

struct Scene {
    objects: Vec<Sphere>,
    lights: Vec<Light>,
    camera: Camera,
}

// fn create_ppm(filename: &str) -> std::io::Result<()> {
//     let mut output = File::create(filename)?;
//     write!(output, "P3\n256 256\n255\n")?;
//     Ok(())
//     // for j in (0..height).rev() {
//     //     for i in 0..width {
//     //         let r: f32 = (i as f32 / (width-1) as f32) as f32;
//     //         let g: f32 = (j as f32 / (height-1) as f32) as f32;
//     //         let b: f32 = 0.25;

//     //         let ir: u32 = (255.999 * r) as u32;
//     //         let ig: u32 = (255.999 * g) as u32;
//     //         let ib: u32 = (255.999 * b) as u32;
//     //         println!("{ir} {ig} {ib}");
//     //     }
//     // }
// }

// fn write_to_ppm(filename: &str, color: Array1<f32>) {
//     let r = (color[0] * 255.999) as u32;
//     let g = (color[1] * 255.999) as u32;
//     let b = (color[2] * 255.999) as u32;
//     let mut file = OpenOptions::new().append(true).open(filename).expect("Unable to open file");
//     let pixel_color = format!("{} {} {}\n", r, g, b);
//     file.write_all(pixel_color.as_bytes()).expect("write failed");
// }

fn construct_scene() -> Scene {
    let c1 = Camera {
        look_from: array![0.0,0.0,1.0],
        // look_at: array![0.0,0.0,0.0],
        // look_up: array![0.0,1.0,0.0],
        // fov: 90.0,
    };
    let l1 = Light {
        // location: array![0.0,0.0,0.0],
        color: array![1.0,1.0,1.0],
        direction: array![0.0,1.0,0.0],
    };
    let s1 = Sphere {
        center: array![0.0,0.0,0.0],
        radius: 0.4,
        shader: Shader {
            od: array![1.0,0.0,1.0],
            kd: 0.7,
            os: array![1.0,1.0,1.0],
            ks: 0.2,
            kgls: 16.0,
            ao: array![0.2,0.2,0.2],
            ka: 0.1,
        }
    };
    let mut objects = Vec::new();
    //let o1 = Object::Sphere(Sphere::from(s1));
    objects.push(s1);
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn calculate_ray(origin: Array1<f32>, direction: Array1<f32>) -> Ray {
    let r = Ray {
        origin,
        direction,
    };
    r
}

fn trace_rays(scene: Scene, filename: &str) {
    let h = array![VIEW_W, 0.0, 0.0];
    let v = array![0.0, VIEW_H, 0.0];
    let origin = &scene.camera.look_from;
    let tlc: Array1<f32> = origin - &h / 2.0 + &v / 2.0 - array![0.0, 0.0, FOCAL_LENGTH];
    for j in (0..IMG_HEIGHT).rev() {
        for i in 0..IMG_WIDTH {
            let r = calculate_ray(origin.clone(), &tlc + (i / (IMG_WIDTH - 1)) as f32 * &h + j as f32 / (IMG_HEIGHT - 1) as f32 * &v - origin);
            for o in &scene.objects {
                let (b,p) = sphere_intersections(&r,o);
                if b {
                    let mut color: Array1<f32> = array![0.0,0.0,0.0];
                    for l in &scene.lights {
                        color = color.clone() + calculate_lighting(&p, &o.shader, o, l, &scene.camera.look_from);
                    }
                    // write_to_ppm(filename, color);
                    print!("{}", color);
                }
                // match o  {
                //     Object::Sphere => {
                //         let s = Object::Sphere()
                //     }
                // }
            }
        }

    }
}

fn sphere_intersections(r: &Ray, s: &Sphere) -> (bool, Array1<f32>) {
    let b: f32 = 2.0 * (r.direction[0] * r.origin[0] 
        - r.direction[0] * s.center[0] 
        + r.direction[1] * r.origin[1] 
        - r.direction[1] * s.center[1]
        + r.direction[2] * r.origin[2] 
        - r.direction[2] * s.center[2]);
    let c: f32 = r.origin[0].powi(2) - 2.0 * r.origin[0] * s.center[0] + s.center[0].powi(2) 
    + r.origin[1].powi(2) - 2.0 * r.origin[1] * s.center[1].powi(2) 
    + r.origin[2].powi(2) - 2.0 * r.origin[2] * s.center[2] + s.center[2].powi(2) - s.radius.powi(2);
    let d: f32 = b.powi(2) - 4.0 * c;
    if d < 0.0 {
        return (false, array![0.0,0.0,0.0]);
    }
    let t: f32;
    let t0: f32 = (-b - d.sqrt()) / 2.0;
    if t0 <= 0.0 {
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        if t1 <= 0.0 {
            return (false, array![0.0,0.0,0.0]);
        }
        t = t1;
    } else {
        t = t0;
    }
    return (true, r.origin.clone() + r.direction.clone() * t);
}

fn calculate_normal(p: &Array1<f32>, s: &Sphere) -> Array1<f32> {
    let n = array![(p[0] - s.center[0]) / s.radius, (p[1] - s.center[1]) / s.radius, (p[2] - s.center[2]) / s.radius];
    n
}

fn calculate_reflection(n: &Array1<f32>, l: &Array1<f32>) -> Array1<f32> {
    2.0 * n * n.dot(l) - l
}

fn calculate_diffuse(kd: f32, plc: &Array1<f32>, od: &Array1<f32>, n: &Array1<f32>, l: &Array1<f32>) -> Array1<f32> {
    kd * plc * od * std::cmp::max(0, n.dot(l) as i32) as f32
}

fn calculate_ambient(ka: f32, ao: &Array1<f32>, od: &Array1<f32>) -> Array1<f32> {
    ka * ao * od
}

fn calculate_specular(ks: f32, plc: &Array1<f32>, os: &Array1<f32>, v: &Array1<f32>, r: &Array1<f32>, kgls: f32) -> Array1<f32> {
    ks * plc * os * (std::cmp::max(0, v.dot(r) as i32) as f32).powf(kgls)
}

fn calculate_lighting(p: &Array1<f32>, sh: &Shader, s: &Sphere, l: &Light, v: &Array1<f32>) -> Array1<f32> {
    let n = calculate_normal(p,s);
    let ld = array![l.direction[0], l.direction[1], l.direction[2]];
    let r = calculate_reflection(&n, &ld);
    calculate_diffuse(sh.kd, &array![l.color[0], l.color[1], l.color[2]], &sh.od, &n, &ld) 
    + calculate_ambient(sh.ka, &sh.ao, &sh.od) 
    + calculate_specular(sh.ks, &array![l.color[0], l.color[1], l.color[2]], &sh.os, v, &r, sh.kgls)
}

// fn render() {

// }

// -> Result<(), std::io::Error> 
fn main() {
    let scene = construct_scene();
    let filename = "image.ppm";
    // let result = create_ppm(filename);
    trace_rays(scene, filename);
    //result
    // println!("P3\n256 256\n255");
    // create_ppm(256,256);
}
