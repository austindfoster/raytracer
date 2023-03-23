use std::fs::{File,OpenOptions};
use std::io::Write;

use structs::*;

pub mod overload;
pub mod structs;

const FOV: f32 = 90.0;
const ASPECT_RATIO: f32 = 1.0 / 1.0; // 16.0 / 9.0;
const IMG_WIDTH: u32 = 500;
const IMG_HEIGHT: u32 = (IMG_WIDTH as f32 / ASPECT_RATIO) as u32;
const MAX_DEPTH: u32 = 1;
const BACKGROUND_COLOR: Point = Point {
    x: 0.2,
    y: 0.2,
    z: 0.2,
};
// const AMBIENT_LIGHT: Point = Point {
//     x: 0.0,
//     y: 0.0,
//     z: 0.0,
// };
const AMBIENT_LIGHT: Point = Point {
    x: 0.1,
    y: 0.1,
    z: 0.1,
};
const FOCAL_LENGTH: f32 = 1.0;

fn cross(a: &Point, b: &Point) -> Point {
    Point {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
    }
}

fn dot(a: &Point, b: &Point) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

fn create_ppm(filename: &str) -> std::io::Result<()> {
    let mut output = File::create(filename)?;
    write!(output, "P3\n{} {}\n255\n", IMG_WIDTH, IMG_HEIGHT)?;
    Ok(())
}

fn save_colors(mut colors: String, color: Point) -> String {
    let r = (color.x * 255.999) as u32;
    let g = (color.y * 255.999) as u32;
    let b = (color.z * 255.999) as u32;
    // let r = u32::min((color.x * 255.999) as u32, 255);
    // let g = u32::min((color.y * 255.999) as u32, 255);
    // let b = u32::min((color.z * 255.999) as u32, 255);
    let pixel_color = format!("{} {} {}\n", r, g, b);
    colors.push_str(&pixel_color);
    return colors;
}

fn write_to_ppm(filename: &str, colors: String) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)
        .expect("Unable to open file");
    file.write_all(colors.as_bytes()).expect("write failed");
}

fn construct_scene0() -> Scene {
    let c1 = Camera {
        look_from: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        look_at: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        look_up: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fov: 90.0,
    };
    let l1 = Light {
        // location: [0.0,0.0,0.0],
        color: Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        direction: normalize_vector(&Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }),
    };
    let s1 = Sphere {
        center: Point {
            x: 0.45,
            y: 0.0,
            z: -0.15,
        },
        radius: 0.15,
        shader: Shader {
            kd: 0.8,
            ks: 0.1,
            ka: 0.3,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 4.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s2 = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -0.1,
        },
        radius: 0.2,
        shader: Shader {
            kd: 0.6,
            ks: 0.3,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 32.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s3 = Sphere {
        center: Point {
            x: -0.6,
            y: 0.0,
            z: 0.0,
        },
        radius: 0.3,
        shader: Shader {
            kd: 0.7,
            ks: 0.2,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            kgls: 64.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s4 = Sphere {
        center: Point {
            x: 0.0,
            y: -10000.5,
            z: 0.0,
        },
        radius: 10000.0,
        shader: Shader {
            kd: 0.9,
            ks: 0.0,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 16.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    // objects.push(Box::new(s0));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(s4));
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn construct_scene1() -> Scene {
    let c1 = Camera {
        look_from: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        look_at: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        look_up: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fov: 90.0,
    };
    let l1 = Light {
        // location: [0.0,0.0,0.0],
        color: Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        direction: normalize_vector(&Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }),
    };
    let s0 = Sphere {
        center: Point {
            x: 0.0,
            y: 0.3,
            z: -1.0,
        },
        radius: 0.25,
        shader: Shader {
            kd: 0.0,
            ks: 0.1,
            ka: 0.1,
            od: Point {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 10.0,
            kr: 0.9,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let t0: Triangle = Triangle {
        p1: Point {
            x: 0.0,
            y: -0.7,
            z: -0.5,
        },
        p2: Point {
            x: 1.0,
            y: 0.4,
            z: -1.0,
        },
        p3: Point {
            x: 0.0,
            y: -0.7,
            z: -1.5,
        },
    };
    let t1: Triangle = Triangle {
        p1: Point {
            x: 0.0,
            y: -0.7,
            z: -0.5,
        },
        p2: Point {
            x: 0.0,
            y: -0.7,
            z: -1.5,
        },
        p3: Point {
            x: -1.0,
            y: 0.4,
            z: -1.0,
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t0);
    let p0: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 1.0,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 16.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t1);
    let p1: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 1.0,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 16.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    objects.push(Box::new(s0));
    objects.push(Box::new(p0));
    objects.push(Box::new(p1));
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn construct_scene2() -> Scene {
    let c1 = Camera {
        look_from: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        look_at: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        look_up: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fov: 90.0,
    };
    let l1 = Light {
        // location: [0.0,0.0,0.0],
        color: Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        direction: normalize_vector(&Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    };
    let s0 = Sphere {
        center: Point {
            x: 0.5,
            y: 0.0,
            z: -0.15,
        },
        radius: 0.05,
        shader: Shader {
            kd: 0.8,
            ks: 0.1,
            ka: 0.3,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 4.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let s1 = Sphere {
        center: Point {
            x: 0.3,
            y: 0.0,
            z: -0.1,
        },
        radius: 0.08,
        shader: Shader {
            kd: 0.8,
            ks: 0.8,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            os: Point {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            kgls: 32.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s2 = Sphere {
        center: Point {
            x: -0.6,
            y: 0.0,
            z: 0.0,
        },
        radius: 0.3,
        shader: Shader {
            kd: 0.7,
            ks: 0.5,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            kgls: 64.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s3 = Sphere {
        center: Point {
            x: 0.1,
            y: -0.55,
            z: 0.25,
        },
        radius: 0.3,
        shader: Shader {
            kd: 0.0,
            ks: 0.1,
            ka: 0.1,
            od: Point {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 10.0,
            kr: 0.9,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let t0: Triangle = Triangle {
        p1: Point {
            x: 0.3,
            y: -0.3,
            z: -0.4,
        },
        p2: Point {
            x: 0.0,
            y: 0.3,
            z: -0.1,
        },
        p3: Point {
            x: -0.3,
            y: -0.3,
            z: 0.2,
        },
    };
    let t1: Triangle = Triangle {
        p1: Point {
            x: -0.2,
            y: 0.1,
            z: 0.1,
        },
        p2: Point {
            x: -0.2,
            y: -0.5,
            z: 0.2,
        },
        p3: Point {
            x: -0.2,
            y: 0.1,
            z: -0.3,
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t0);
    let p0: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 0.9,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 32.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t1);
    let p1: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 0.5,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 4.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    //let o1 = Object::Sphere(Sphere::from(s1));
    objects.push(Box::new(s0));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(p0));
    objects.push(Box::new(p1));
    let mut lights = Vec::new();
    lights.push(l1);
    let scene = Scene {
        objects,
        lights,
        camera: c1,
    };
    scene
}

fn construct_scene3() -> Scene {
    let c1 = Camera {
        look_from: Point {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        look_at: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        look_up: Point {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        fov: 90.0,
    };
    let l1 = Light {
        // location: [0.0,0.0,0.0],
        color: Point {
            x: 0.5,
            y: 0.0,
            z: 1.0,
        },
        direction: normalize_vector(&Point {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }),
    };
    let s0 = Sphere {
        center: Point {
            x: 0.5,
            y: 0.0,
            z: -0.15,
        },
        radius: 0.05,
        shader: Shader {
            kd: 0.8,
            ks: 0.1,
            ka: 0.3,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 4.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let s1 = Sphere {
        center: Point {
            x: 0.3,
            y: 0.0,
            z: -0.1,
        },
        radius: 0.08,
        shader: Shader {
            kd: 0.8,
            ks: 0.8,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            os: Point {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            kgls: 32.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s2 = Sphere {
        center: Point {
            x: -0.6,
            y: 0.0,
            z: 0.0,
        },
        radius: 0.3,
        shader: Shader {
            kd: 0.7,
            ks: 0.5,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 0.5,
                y: 1.0,
                z: 0.5,
            },
            kgls: 64.0,
            kr: 0.0,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let s3 = Sphere {
        center: Point {
            x: 0.1,
            y: -0.55,
            z: 0.25,
        },
        radius: 0.3,
        shader: Shader {
            kd: 0.0,
            ks: 0.1,
            ka: 0.1,
            od: Point {
                x: 0.75,
                y: 0.75,
                z: 0.75,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 10.0,
            kr: 0.9,
            ao: Point {
                x: 0.1,
                y: 0.1,
                z: 0.1,
            },
        },
    };
    let t0: Triangle = Triangle {
        p1: Point {
            x: 0.3,
            y: -0.3,
            z: -0.4,
        },
        p2: Point {
            x: 0.0,
            y: 0.3,
            z: -0.1,
        },
        p3: Point {
            x: -0.3,
            y: -0.3,
            z: 0.2,
        },
    };
    let t1: Triangle = Triangle {
        p1: Point {
            x: -0.2,
            y: 0.1,
            z: 0.1,
        },
        p2: Point {
            x: -0.2,
            y: -0.5,
            z: 0.2,
        },
        p3: Point {
            x: -0.2,
            y: 0.1,
            z: -0.3,
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t0);
    let p0: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 0.9,
            ka: 0.1,
            od: Point {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 32.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };
    let mut triangles = Vec::new();
    triangles.push(t1);
    let p1: Polygon = Polygon {
        triangles,
        shader: Shader {
            kd: 0.9,
            ks: 0.5,
            ka: 0.1,
            od: Point {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            os: Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            kgls: 4.0,
            kr: 0.0,
            ao: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    };

    let mut objects: Vec<Box<dyn Object>> = Vec::new();
    //let o1 = Object::Sphere(Sphere::from(s1));
    objects.push(Box::new(s0));
    objects.push(Box::new(s1));
    objects.push(Box::new(s2));
    objects.push(Box::new(s3));
    objects.push(Box::new(p0));
    objects.push(Box::new(p1));
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
    kd * plc * od * f32::max(0.0, dot(&n, &l)) as f32
}

fn calculate_ambient(ka: f32, ao: &Point, od: &Point) -> Point {
    ka * AMBIENT_LIGHT * od
}

fn calculate_specular(ks: f32, plc: &Point, os: &Point, v: &Point, r: &Point, kgls: f32) -> Point {
    ks * plc * os * (f32::max(0.0, dot(&normalize_vector(&v), &normalize_vector(&r))) as f32).powf(kgls)
}

fn calculate_sphere_normal(p: &Point, s: &Sphere) -> Point {
    let n = Point {
        x: (p.x - s.center.x) / s.radius,
        y: (p.y - s.center.y) / s.radius,
        z: (p.z - s.center.z) / s.radius,
    };
    n
}

fn calculate_reflection(n: &Point, l: &Point) -> Point {
    2.0 * n * dot(n, l) - l
}

fn show_normals(p: &Point, s: &Sphere) -> Point {
    let n = calculate_sphere_normal(p, s);
    n
}

fn calculate_lighting(
    n: &Point,
    sh: &Shader,
    l: &Light,
    v: &Point,
    r: &Point,
    in_shadow: &bool,
) -> Point {
    let amb = calculate_ambient(sh.ka, &sh.ao, &sh.od);
    if *in_shadow {
        return amb;
    }
    let spec = calculate_specular(sh.ks, &l.color, &sh.os, &v, &r, sh.kgls);
    // println!("view: ({}, {})", sh.kgls, sh.ks);
    let dif = calculate_diffuse(sh.kd, &l.color, &sh.od, &n, &l.direction);
    return dif + amb + spec;
}

fn sphere_intersection(r: &Ray, s: &Sphere) -> (bool, Point) {
    let oc = r.origin - &s.center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - s.radius * s.radius;
    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return (false, BACKGROUND_COLOR);
    }
    let t: f32;
    let t0: f32 = (-b - d.sqrt()) / (2.0 * a);
    if t0 <= 0.0 {
        let t1: f32 = (-b + d.sqrt()) / (2.0 * a);
        if t1 <= 0.0 {
            return (false, BACKGROUND_COLOR);
        }
        t = t1;
    } else {
        t = t0;
    }
    return (true, r.origin + &r.direction * t);
}

fn compute_triangle_normal(tri: &Triangle) -> Point {
    let v1 = &tri.p2 - &tri.p1;
    let v2 = &tri.p3 - &tri.p1;
    let n = cross(&v1, &v2);
    return normalize_vector(&n);
    // let v1 = tri.p1 - &tri.p2;
    // let v2 = tri.p3 - &tri.p2;
    // v2 * v1
}

fn is_in_shadow(p: &Point, scene: &Scene, l: &Point) -> bool {
    for o in &scene.objects {
        let r = calculate_ray(p, normalize_vector(&l));
        let (b, _z) = o.intersect(&r);
        if b {
            return true;
        }
    }
    return false;
}

fn next_intersection<'a>(
    p: &'a Point,
    scene: &'a Scene,
    reflection: &'a Point,
) -> (bool, Point, &'a Box<dyn Object>, Ray<'a>) {
    for o in &scene.objects {
        let r = calculate_ray(p, normalize_vector(&reflection));
        let (b, point) = o.intersect(&r);
        if b {
            return (true, point, o, r);
        }
    }
    return (false, BACKGROUND_COLOR, &scene.objects[0], Ray { origin: &Point { x: 0.0, y: 0.0, z: 0.0 } , direction: Point { x: 0.0, y: 0.0, z: 0.0 } });
}

fn triangle_intersection(r: &Ray, tri: &Triangle) -> (bool, Point) {
    let mut p = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let eps: f32 = 0.0000001;
    let e1 = &tri.p2 - &tri.p1;
    let e2 = &tri.p3 - &tri.p1;
    let h = cross(&r.direction, &e2);
    let a = dot(&e1, &h);
    if a > -eps && a < eps {
        return (false, p);
    }
    let f = 1.0 / a;
    let s = r.origin - &tri.p1;
    let u = f * dot(&s, &h);
    if u < 0.0 || u > 1.0 {
        return (false, p);
    }
    let q = cross(&s, &e1);
    let v = f * dot(&r.direction, &q);
    if v < 0.0 || u + v > 1.0 {
        return (false, p);
    }
    let t = f * dot(&e2, &q);
    if t > eps {
        p = r.origin + &r.direction * t;
        return (true, p);
    }
    return (false, p);
}

fn polygon_intersection(r: &Ray, poly: &Polygon) -> (bool, Point) {
    // let mut index = 0;
    let p = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    for tri in &poly.triangles {
        let (b, ip) = triangle_intersection(r, tri);
        if b {
            return (b, ip);
        }
    }
    return (false, p);
}

fn normalize_vector(v: &Point) -> Point {
    let norm = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
    v / norm.sqrt()
}

fn calculate_ray(origin: &Point, direction: Point) -> Ray {
    let r = Ray { origin, direction };
    r
}

fn get_ray_color(
    scene: &Scene,
    o: &dyn Object,
    point: &Point,
    depth: u32,
    ray: &Ray,
) -> Point {
    let n = &o.calculate_normal(point);
    let mut color = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    for l in &scene.lights {
        let ray_reflection = calculate_reflection(&n, &normalize_vector(&(ray.origin - &ray.direction)));
        let sp = point + &l.direction * 0.001;
        let reflection = calculate_reflection(&n, &normalize_vector(&l.direction));
        println!("{} {} {}", reflection.x, reflection.y, reflection.z);
        color = calculate_lighting(
            &n,
            &o.get_shader(),
            l,
            &scene.camera.look_from,
            &reflection,
            &is_in_shadow(&sp, &scene, &l.direction),
        );
        if o.get_shader().kr > 0.0 && depth > 0 {
            // let ray_reflection = calculate_reflection(&n, &normalize_vector(&(ray.origin - &ray.direction)));
            let rp = point + &ray_reflection * 0.001;
            let (b, np, no, nr) = next_intersection(&rp, scene, &ray_reflection);
            if b {
                color = &color
                    + o.get_shader().kr
                        * get_ray_color(
                            scene,
                            no.as_ref(),
                            &np,
                            depth - 1,
                            &nr,
                        );
            } else {
                return &color + o.get_shader().kr * BACKGROUND_COLOR;
            }
        }
        // color = color + reflection_color(&sp, &scene, &reflection, MAX_DEPTH);
    }
    return color;
}

fn get_front_object(pixel_objects: &Vec<&dyn Object>, pixel_points: &Vec<Point>) -> usize {
    // let mut object_in_front = pixel_objects[0];
    let mut point = &pixel_points[0];
    let mut index = 0;
    for i in 0..(pixel_objects.len() - 1) {
        // let current_object = pixel_objects[i];
        if pixel_points[i].z > point.z {
            // object_in_front = current_object;
            point = &pixel_points[i];
            index = i;
        }
    }
    return index;
}

fn get_all_intersected_objects<'a>(
    scene: &'a Scene,
    r: &'a Ray<'a>,
) -> (Vec<&'a dyn Object>, Vec<Point>) {
    let mut pixel_objects: Vec<&dyn Object> = Vec::new();
    let mut pixel_points: Vec<Point> = Vec::new();
    for o in &scene.objects {
        let (b, p) = o.intersect(&r);
        if b {
            pixel_objects.push(o.as_ref());
            pixel_points.push(p);
        }
    }
    return (pixel_objects, pixel_points);
}

fn trace_rays(scene: Scene, filename: &str) {
    let screen_w = FOCAL_LENGTH * (FOV / 2.0).to_radians().tan();
    let screen_h = screen_w / ASPECT_RATIO;
    let h = Point {
        x: screen_w / 2.0,
        y: 0.0,
        z: FOCAL_LENGTH,
    };
    let v = Point {
        x: 0.0,
        y: screen_h / 2.0,
        z: 0.0,
    };
    let tlc = v - h;
    let x_inc = screen_w / (IMG_WIDTH as f32) as f32;
    let y_inc = screen_h / (IMG_HEIGHT as f32) as f32;
    let mut colors: String = String::new();
    let mut prev_y: f32 = 0.0;
    for _j in 0..IMG_HEIGHT {
        let mut prev_x: f32 = 0.0;
        for _i in 0..IMG_WIDTH {
            let dir = &tlc
                + Point {
                    x: prev_x,
                    y: -prev_y,
                    z: 0.0,
                };
            let r = calculate_ray(&scene.camera.look_from, normalize_vector(&dir));
            let (pixel_objects, pixel_points) = get_all_intersected_objects(&scene, &r);
            if !pixel_objects.is_empty() {
                let index = get_front_object(&pixel_objects, &pixel_points);
                let color = get_ray_color(&scene, pixel_objects[index], &pixel_points[index], MAX_DEPTH, &r);
                colors = save_colors(colors, color)
            } else {
                colors = save_colors(colors, BACKGROUND_COLOR);
            }
            prev_x += x_inc;
        }
        prev_y += y_inc;
    }
    write_to_ppm(filename, colors);
}

fn main() -> Result<(), std::io::Error> {
    let scene = construct_scene2();
    let filename = "image2.ppm";
    let result = create_ppm(filename);
    trace_rays(scene, filename);
    result
}
