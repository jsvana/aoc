use std::cmp::{max, min, Ordering};
use std::thread;

use anyhow::Result;

#[derive(Copy, Clone, Debug)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialEq for Point3d {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Point3d {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

#[derive(Copy, Clone, Debug)]
struct Moon {
    position: Point3d,
    velocity: Point3d,
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.velocity == other.velocity
    }
}

impl Moon {
    fn new(initial_position: Point3d) -> Self {
        Self {
            position: initial_position,
            velocity: Point3d::new(),
        }
    }

    fn apply_gravity_from_moon(&mut self, other: &mut Self) {
        let x_delta = match self.position.x.cmp(&other.position.x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.velocity.x += x_delta;
        other.velocity.x -= x_delta;

        let y_delta = match self.position.y.cmp(&other.position.y) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.velocity.y += y_delta;
        other.velocity.y -= y_delta;

        let z_delta = match self.position.z.cmp(&other.position.z) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };

        self.velocity.z += z_delta;
        other.velocity.z -= z_delta;
    }

    fn x_equal(&self, other: &Self) -> bool {
        self.position.x == other.position.x && self.velocity.x == other.velocity.x
    }

    fn y_equal(&self, other: &Self) -> bool {
        self.position.y == other.position.y && self.velocity.y == other.velocity.y
    }

    fn z_equal(&self, other: &Self) -> bool {
        self.position.z == other.position.z && self.velocity.z == other.velocity.z
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn potential_energy(&self) -> i64 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn print(&self) {
        println!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.position.x,
            self.position.y,
            self.position.z,
            self.velocity.x,
            self.velocity.y,
            self.velocity.z,
        );
    }
}

fn step(io: &mut Moon, ganymede: &mut Moon, callisto: &mut Moon, europa: &mut Moon) {
    io.apply_gravity_from_moon(ganymede);
    io.apply_gravity_from_moon(callisto);
    io.apply_gravity_from_moon(europa);
    ganymede.apply_gravity_from_moon(europa);
    ganymede.apply_gravity_from_moon(callisto);
    europa.apply_gravity_from_moon(callisto);

    io.apply_velocity();
    ganymede.apply_velocity();
    callisto.apply_velocity();
    europa.apply_velocity();
}

fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();
    // REAL INPUT
    let io = Moon::new(Point3d { x: -7, y: -1, z: 6 });
    let ganymede = Moon::new(Point3d { x: 6, y: -9, z: -9 });
    let callisto = Moon::new(Point3d {
        x: -12,
        y: 2,
        z: -7,
    });
    let europa = Moon::new(Point3d {
        x: 4,
        y: -17,
        z: -12,
    });

    let mut x_io = io.clone();
    let mut x_ganymede = ganymede.clone();
    let mut x_callisto = callisto.clone();
    let mut x_europa = europa.clone();

    let x_thread = thread::spawn(move || {
        let mut counter: u64 = 0;
        let initial_x_io = x_io.clone();
        let initial_x_ganymede = x_ganymede.clone();
        let initial_x_callisto = x_callisto.clone();
        let initial_x_europa = x_europa.clone();

        loop {
            step(&mut x_io, &mut x_ganymede, &mut x_callisto, &mut x_europa);

            counter += 1;

            if counter % 1000000 == 0 {
                println!("COUNT {} IN X", counter);
            }

            if initial_x_io.x_equal(&x_io)
                && initial_x_callisto.x_equal(&x_callisto)
                && initial_x_ganymede.x_equal(&x_ganymede)
                && initial_x_europa.x_equal(&x_europa)
            {
                break;
            }
        }

        counter
    });

    let mut y_io = io.clone();
    let mut y_ganymede = ganymede.clone();
    let mut y_callisto = callisto.clone();
    let mut y_europa = europa.clone();

    let y_thread = thread::spawn(move || {
        let mut counter: u64 = 0;
        let initial_y_io = y_io.clone();
        let initial_y_ganymede = y_ganymede.clone();
        let initial_y_callisto = y_callisto.clone();
        let initial_y_europa = y_europa.clone();

        loop {
            step(&mut y_io, &mut y_ganymede, &mut y_callisto, &mut y_europa);

            counter += 1;

            if counter % 1000000 == 0 {
                println!("COUNT {} IN X", counter);
            }

            if initial_y_io.y_equal(&y_io)
                && initial_y_callisto.y_equal(&y_callisto)
                && initial_y_ganymede.y_equal(&y_ganymede)
                && initial_y_europa.y_equal(&y_europa)
            {
                break;
            }
        }

        counter
    });

    let mut z_io = io.clone();
    let mut z_ganymede = ganymede.clone();
    let mut z_callisto = callisto.clone();
    let mut z_europa = europa.clone();

    let z_thread = thread::spawn(move || {
        let mut counter: u64 = 0;
        let initial_z_io = z_io.clone();
        let initial_z_ganymede = z_ganymede.clone();
        let initial_z_callisto = z_callisto.clone();
        let initial_z_europa = z_europa.clone();

        loop {
            step(&mut z_io, &mut z_ganymede, &mut z_callisto, &mut z_europa);

            counter += 1;

            if counter % 1000000 == 0 {
                println!("COUNT {} IN X", counter);
            }

            if initial_z_io.z_equal(&z_io)
                && initial_z_callisto.z_equal(&z_callisto)
                && initial_z_ganymede.z_equal(&z_ganymede)
                && initial_z_europa.z_equal(&z_europa)
            {
                break;
            }
        }

        counter
    });

    let x_counter = x_thread.join().unwrap();
    let y_counter = y_thread.join().unwrap();
    let z_counter = z_thread.join().unwrap();

    println!("Period: {}", lcm(lcm(x_counter, y_counter), z_counter));

    Ok(())
}
