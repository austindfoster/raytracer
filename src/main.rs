use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use ndarray::Array1;
use ndarray::array;

const FOV: f32 = 90.0;
const ASPECT_RATIO: f32 = 1.0 / 1.0; // 16.0 / 9.0;
const IMG_WIDTH: u32 = 500;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;

// const VIEW_H: f32 = 2.0;
// const VIEW_W: f32 = ASPECT_RATIO * VIEW_H;
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

fn create_ppm(filename: &str) -> std::io::Result<()> {
    let mut output = File::create(filename)?;
    write!(output, "P3\n{} {}\n255\n",IMG_WIDTH, IMG_HEIGHT)?;
    Ok(())
}

fn write_to_ppm(filename: &str, color: Array1<f32>) {
    let r = (color[0] * 255.999) as u32;
    let g = (color[1] * 255.999) as u32;
    let b = (color[2] * 255.999) as u32;
    let mut file = OpenOptions::new().append(true).open(filename).expect("Unable to open file");
    let pixel_color = format!("{} {} {}\n", r, g, b);
    file.write_all(pixel_color.as_bytes()).expect("write failed");
}

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
        direction: array![1.0,1.0,1.0],
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
    let s2 = Sphere {
        center: array![0.45,0.0,-0.15],
        radius: 0.15,
        shader: Shader {
            od: array![1.0,1.0,1.0],
            kd: 0.8,
            os: array![1.0,1.0,1.0],
            ks: 0.1,
            kgls: 4.0,
            ao: array![0.2,0.2,0.2],
            ka: 0.3,
        }
    };
    let s3 = Sphere {
        center: array![0.0,0.0,-0.1],
        radius: 0.2,
        shader: Shader {
            od: array![1.0,0.0,0.0],
            kd: 0.6,
            os: array![1.0,1.0,1.0],
            ks: 0.3,
            kgls: 32.0,
            ao: array![0.2,0.2,0.2],
            ka: 0.1,
        }
    };
    let s4 = Sphere {
        center: array![-0.6,0.0,0.0],
        radius: 0.3,
        shader: Shader {
            od: array![0.0,1.0,0.0],
            kd: 0.7,
            os: array![1.0,1.0,1.0],
            ks: 0.2,
            kgls: 64.0,
            ao: array![0.2,0.2,0.2],
            ka: 0.1,
        }
    };
    let s5 = Sphere {
        center: array![0.0,-10000.5,0.0],
        radius: 10000.0,
        shader: Shader {
            od: array![0.0,0.0,1.0],
            kd: 0.9,
            os: array![1.0,1.0,1.0],
            ks: 0.0,
            kgls: 16.0,
            ao: array![0.2,0.2,0.2],
            ka: 0.1,
        }
    };

    let mut objects = Vec::new();
    //let o1 = Object::Sphere(Sphere::from(s1));
    // objects.push(s1);
    objects.push(s2);
    objects.push(s3);
    objects.push(s4);
    objects.push(s5);
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn normalize_vector(v: &Array1<f32>) -> Array1<f32> {
    let norm = v[0].powi(2) + v[1].powi(2) + v[2].powi(2);
    v / norm.sqrt()
}

fn calculate_ray(origin: Array1<f32>, direction: Array1<f32>) -> Ray {
    let r = Ray {
        origin,
        direction,
    };
    r
}

fn trace_rays(scene: Scene, filename: &str) {
    let screen_w = FOCAL_LENGTH * (FOV / 2.0).to_radians().tan();
    let screen_h = screen_w / ASPECT_RATIO;
    let h = array![screen_w / 2.0, 0.0, 0.0];
    let v = array![0.0, screen_h / 2.0, 0.0];
    let origin = &scene.camera.look_from;
    let window_loc = origin - array![0.0,0.0,FOCAL_LENGTH];
    let tlc: Array1<f32> = window_loc - &h + &v;
    let x_inc = screen_w / (IMG_WIDTH as  f32) as f32;
    let y_inc = screen_h / (IMG_HEIGHT as  f32) as f32;
    let mut prev_y:f32 = 0.0;
    for _j in 0..IMG_HEIGHT {
        let mut prev_x: f32 = 0.0;
        for _i in 0..IMG_WIDTH {
            // let dir = &tlc + (i as f32 / (IMG_WIDTH - 1) as f32) as f32 * &h - j as f32 / (IMG_HEIGHT - 1) as f32 * &v - origin;
            let dir = &tlc + array![prev_x, 0.0, 0.0] - array![0.0, prev_y, 0.0] - origin;
            let r = calculate_ray(origin.clone(), normalize_vector(&dir));
            let mut pixel_objects: Vec<(&Sphere,Array1<f32>)> = Vec::new();
            for o in &scene.objects {
                let (b,p) = sphere_intersections(&r,o);
                if b {
                    pixel_objects.push((o,p.clone()));
                }
                // match o  {
                //     Object::Sphere => {
                //         let s = Object::Sphere()
                //     }
                // }
            }
            if !pixel_objects.is_empty() {
                let (mut object_in_front,mut point) = pixel_objects[0].clone();
                for po in &pixel_objects {
                    let (current_object,p) = po;
                    if p[2] > point[2] {
                        object_in_front = current_object;
                        point = p.clone();
                    }
                }
                let mut color: Array1<f32> = array![0.0,0.0,0.0];
                for l in &scene.lights {
                    color = color.clone() + calculate_lighting(&point, &object_in_front.shader, object_in_front, l, &scene.camera.look_from);
                }
                // write_to_ppm(filename, array![0.2,0.2,0.2]);
                write_to_ppm(filename, color);
                // print!("{}", color);
            } else {
                write_to_ppm(filename, array![0.2,0.2,0.2]);
            }
            prev_x += x_inc;
        }
        prev_y += y_inc;
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
    + r.origin[1].powi(2) - 2.0 * r.origin[1] * s.center[1] + s.center[1].powi(2) 
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
    normalize_vector(&n)
}

fn calculate_reflection(n: &Array1<f32>, l: &Array1<f32>) -> Array1<f32> {
    normalize_vector(&(2.0 * n * n.dot(l) - l))
}

fn calculate_diffuse(kd: f32, plc: &Array1<f32>, od: &Array1<f32>, n: &Array1<f32>, l: &Array1<f32>) -> Array1<f32> {
    kd * plc * od * f32::max(0.0, n.dot(l)) as f32
}

fn calculate_ambient(ka: f32, ao: &Array1<f32>, od: &Array1<f32>) -> Array1<f32> {
    ka * ao * od
}

fn calculate_specular(ks: f32, plc: &Array1<f32>, os: &Array1<f32>, v: &Array1<f32>, r: &Array1<f32>, kgls: f32) -> Array1<f32> {
    ks * plc * os * (f32::max(0.0, v.dot(r)) as f32).powf(kgls)
}

fn calculate_lighting(p: &Array1<f32>, sh: &Shader, s: &Sphere, l: &Light, v: &Array1<f32>) -> Array1<f32> {
    let n = calculate_normal(p,s);
    let r = calculate_reflection(&n, &l.direction);
    let dif = calculate_diffuse(sh.kd, &l.color, &sh.od, &n, &l.direction);
    let amb = calculate_ambient(sh.ka, &sh.ao, &sh.od);
    let spec = calculate_specular(sh.ks, &l.color, &sh.os, v, &r, sh.kgls);
    dif + amb + spec
    // calculate_diffuse(sh.kd, &l.color, &sh.od, &n, &l.direction) 
    // + calculate_ambient(sh.ka, &sh.ao, &sh.od) 
    // + calculate_specular(sh.ks, &array![l.color[0], l.color[1], l.color[2]], &sh.os, v, &r, sh.kgls)
}

// fn render() {

// }

// -> Result<(), std::io::Error> 
fn main() -> Result<(), std::io::Error> {
    let scene = construct_scene();
    let filename = "image2.ppm";
    let result = create_ppm(filename);
    trace_rays(scene, filename);
    result
    // println!("P3\n256 256\n255");
    // create_ppm(256,256);
}
