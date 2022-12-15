use std::collections::HashSet;

use anyhow::Result;

#[derive(Debug)]
struct SensorBeaconPair {
    sensor: (i32, i32),
    beacon: (i32, i32),
    distance: u32
}

fn part01() -> Result<usize> {
    let y = 2000000;
    let mut y_covered_by_sensors: HashSet<i32> = HashSet::new();
    let input: Vec<Vec<String>> = std::fs::read_to_string("./data/day15.input")?
        .trim()
        .lines()
        .map(|line| line.split_terminator(&[':', ',', '=', ' ']).collect::<Vec<&str>>().into_iter().map(|e| e.to_string()).collect())
        .collect();
    let sensor_beacon_pairs: Vec<SensorBeaconPair> = input
        .into_iter()
        .map(|pair| SensorBeaconPair {
            sensor: (pair[3].parse().unwrap(), pair[6].parse().unwrap()),
            beacon: (pair[13].parse().unwrap(), pair[16].parse().unwrap()),
            distance: pair[3].parse::<i32>().unwrap().abs_diff(pair[13].parse::<i32>().unwrap())
                + pair[6].parse::<i32>().unwrap().abs_diff(pair[16].parse::<i32>().unwrap())
        }).collect();

    let mut beacons_on_y: HashSet<i32> = HashSet::new();
    for pair in &sensor_beacon_pairs {
        let dist_to_y = pair.sensor.1.abs_diff(y);
        if dist_to_y <= pair.distance {
            let remaining_dist = pair.distance - dist_to_y;
            for i in -(remaining_dist as i32)..=(remaining_dist as i32) {
                y_covered_by_sensors.insert(pair.sensor.0 + i);
            }
        }
        if pair.beacon.1 == y { beacons_on_y.insert(pair.beacon.0); }
    }

    Ok(y_covered_by_sensors.len() - beacons_on_y.len())
}


fn get_outer_sensor_boundary_iterator(pair: &SensorBeaconPair) -> impl Iterator<Item = (i32, i32)> {
    let mut positions = Vec::new();
    for i in 0..pair.distance + 1 {
        positions.push((pair.sensor.0 + ((pair.distance as i32) + 1) - (i as i32), pair.sensor.1 + (i as i32))); // E->S
        positions.push((pair.sensor.0 - (i as i32), pair.sensor.1 + ((pair.distance as i32) + 1) - (i as i32))); // S->W
        positions.push((pair.sensor.0 - ((pair.distance as i32) + 1) + (i as i32), pair.sensor.1 - (i as i32))); // W->N
        positions.push((pair.sensor.0 + (i as i32), pair.sensor.1 - ((pair.distance as i32) + 1) + (i as i32))); // N->E
    }
    positions.into_iter()
}

fn manhatten_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn part02() -> Result<u64> {
    let dim = 4000000;
    let input: Vec<Vec<String>> = std::fs::read_to_string("./data/day15.input")?
        .trim()
        .lines()
        .map(|line| line.split_terminator(&[':', ',', '=', ' ']).collect::<Vec<&str>>().into_iter().map(|e| e.to_string()).collect())
        .collect();
    let sensor_beacon_pairs: Vec<SensorBeaconPair> = input
        .into_iter()
        .map(|pair| SensorBeaconPair {
            sensor: (pair[3].parse().unwrap(), pair[6].parse().unwrap()),
            beacon: (pair[13].parse().unwrap(), pair[16].parse().unwrap()),
            distance: pair[3].parse::<i32>().unwrap().abs_diff(pair[13].parse::<i32>().unwrap())
                + pair[6].parse::<i32>().unwrap().abs_diff(pair[16].parse::<i32>().unwrap())
        }).collect();

    for pair in &sensor_beacon_pairs {
        for pos in get_outer_sensor_boundary_iterator(&pair) {
            if !(0..dim).contains(&pos.0) || !(0..dim).contains(&pos.1) { continue; }
            match sensor_beacon_pairs.iter().find(|other_pair| manhatten_distance(pos, other_pair.sensor) <= other_pair.distance) {
                Some(_) => continue,
                None => return Ok((pos.0 as u64) * dim as u64 + pos.1 as u64)
            }
        }
    }

    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
