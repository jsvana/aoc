use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::convert::{TryFrom, TryInto};

use anyhow::{format_err, Error, Result};
use log::{debug, info};

type Orbits = BTreeMap<String, OrbitInfo>;

#[derive(Debug, PartialEq, Eq)]
struct Orbit {
    center_of_mass: String,
    planet: String,
}

#[derive(Debug)]
struct OrbitInfo {
    distance_from_center: usize,
    total_distances: usize,
    planets: BTreeSet<String>,
}

impl OrbitInfo {
    fn new() -> Self {
        OrbitInfo {
            distance_from_center: 0,
            total_distances: 0,
            planets: BTreeSet::new(),
        }
    }

    fn add_planet(&mut self, planet: &str) {
        self.planets.insert(planet.to_string());
    }
}

impl TryFrom<&str> for Orbit {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.splitn(2, ")").collect();

        let center_of_mass = parts.get(0).ok_or(format_err!(
            "Somehow didn't get single string from Orbit (passed \"{}\")",
            value
        ))?;

        match parts.get(1) {
            Some(planet) => Ok(Orbit {
                center_of_mass: center_of_mass.to_string(),
                planet: planet.to_string(),
            }),
            None => Err(format_err!(
                "Orbit was not passed an outer planet (passed \"{}\")",
                value
            )),
        }
    }
}

fn string_to_orbits(input: &str) -> Result<Orbits> {
    let mut orbits = BTreeMap::new();
    for line in input.trim().split("\n") {
        let orbit: Orbit = line.try_into()?;

        orbits
            .entry(orbit.center_of_mass.clone())
            .or_insert(OrbitInfo::new())
            .add_planet(&orbit.planet);

        orbits
            .entry(orbit.planet.clone())
            .or_insert(OrbitInfo::new())
            .add_planet(&orbit.center_of_mass);
    }
    Ok(orbits)
}

fn read_input(filename: &str) -> Result<Orbits> {
    let data = std::fs::read_to_string(filename)?;

    string_to_orbits(&data)
}

fn shortest_path_length(orbits: &Orbits, start: &str, end: &str) -> Result<usize> {
    let mut to_check = VecDeque::new();
    to_check.push_back(start);

    let mut counts = BTreeMap::new();

    for planet in orbits.keys() {
        counts.insert(planet.to_string(), std::usize::MAX);
    }

    counts.insert(start.to_string(), 0);

    let mut visited = BTreeSet::new();

    while !to_check.is_empty() {
        let next = to_check.pop_front().unwrap();
        debug!("Checking {}", next);

        if visited.contains(next) {
            debug!("Already visted {}", next);
            continue;
        }

        visited.insert(next);

        let count = counts
            .get(next)
            .ok_or(format_err!("Count not found for {}", next))?;
        let next_hop = count + 1;

        let info = orbits
            .get(next)
            .ok_or(format_err!("Orbit info not found for {}", next))?;

        for planet in info.planets.iter() {
            let next_planet_count = *counts
                .get(planet)
                .ok_or(format_err!("Comparison count not found for {}", planet))?;
            if next_hop < next_planet_count {
                counts.insert(planet.to_string(), next_hop);
                to_check.push_back(planet);
            }
        }
    }

    Ok(*counts
        .get(end)
        .ok_or(format_err!("End count not found for {}", end))?)
}

fn main() -> Result<()> {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let orbits = read_input("input.txt")?;

    debug!("Orbits: {:?}", orbits);

    info!(
        "Shortest path: {}",
        shortest_path_length(&orbits, "YOU", "SAN")?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_orbit() -> Result<()> {
        let orbit: Orbit = "A)B".try_into()?;
        assert_eq!(
            orbit,
            Orbit {
                center_of_mass: "A".to_string(),
                planet: "B".to_string(),
            }
        );
        Ok(())
    }
}
