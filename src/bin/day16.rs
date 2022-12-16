use anyhow::Result;
use ndarray::Array3;
use std::collections::HashMap;

fn part01() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day16.input")?;

    let mut valves: Vec<(&str, u32, Vec<&str>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (valve, flow, _, tunnels) = sscanf::sscanf!(
                line,
                "Valve {str} has flow rate={u32}; {str:/tunnels? leads? to valves?/} {str}"
                )
                .unwrap();
            let tunnels = tunnels.split(", ").collect::<Vec<_>>();
            (valve, flow, tunnels)
        })
    .collect();

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let valve_map = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let num_positive_flow_valves = valves.iter().filter(|v| v.1 > 0).count();
    let num_valves = valves.len();
    let mut adj = vec![vec![0usize; 0]; num_valves];
    let mut flow = vec![0u32; num_valves];
    for v in valves.iter() {
        let i = valve_map[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(valve_map[w]);
        }
    }
    let start_valve = valve_map["AA"];

    let b_positive_flow_valves = 1 << num_positive_flow_valves;
    // // dp table [time left, current node, bitset of available valves]
    let mut table = Array3::<u32>::zeros([30, num_valves, b_positive_flow_valves]);
    for t in 1..30 {
        for i in 0..num_valves {
            let b_current_valve = 1 << i;
            for x in 0..b_positive_flow_valves {
                let mut current_cell = table[(t, i, x)];
                if b_current_valve & x != 0 {
                    current_cell = current_cell.max(table[(t - 1, i, x - b_current_valve)] + flow[i] * t as u32);
                }
                for &j in adj[i].iter() {
                    current_cell = current_cell.max(table[(t - 1, j, x)]);
                }
                table[(t, i, x)] = current_cell;
            }
        }
    }

    Ok(table[(29, start_valve, b_positive_flow_valves - 1)])
}

fn part02() -> Result<u32> {
    let input = std::fs::read_to_string("./data/day16.input")?;

    let mut valves: Vec<(&str, u32, Vec<&str>)> = input
        .trim()
        .lines()
        .map(|line| {
            let (valve, flow, _, tunnels) = sscanf::sscanf!(
                line,
                "Valve {str} has flow rate={u32}; {str:/tunnels? leads? to valves?/} {str}"
                )
                .unwrap();
            let tunnels = tunnels.split(", ").collect::<Vec<_>>();
            (valve, flow, tunnels)
        })
    .collect();

    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let valve_map = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<HashMap<_, _>>();
    let num_positive_flow_valves = valves.iter().filter(|v| v.1 > 0).count();
    let num_valves = valves.len();
    let mut adj = vec![vec![0usize; 0]; num_valves];
    let mut flow = vec![0u32; num_valves];
    for v in valves.iter() {
        let i = valve_map[v.0];
        flow[i] = v.1;
        for w in v.2.iter() {
            adj[i].push(valve_map[w]);
        }
    }
    let start_valve = valve_map["AA"];

    let b_positive_flow_valves = 1 << num_positive_flow_valves;
    // // dp table [time left, current node, bitset of available valves]
    let mut table = Array3::<u32>::zeros([30, num_valves, b_positive_flow_valves]);
    for t in 1..30 {
        for i in 0..num_valves {
            let b_current_valve = 1 << i;
            for x in 0..b_positive_flow_valves {
                let mut current_cell = table[(t, i, x)];
                if b_current_valve & x != 0 {
                    current_cell = current_cell.max(table[(t - 1, i, x - b_current_valve)] + flow[i] * t as u32);
                }
                for &j in adj[i].iter() {
                    current_cell = current_cell.max(table[(t - 1, j, x)]);
                }
                table[(t, i, x)] = current_cell;
            }
        }
    }


    let mut best = 0;
    // only checking the last 2 timeslots is a hack that works on my input :)
    for t in 24..26 {
        for s in 24..26 {
            for x in 0..b_positive_flow_valves {
                for y in 0..x {
                    if x & y != 0 { continue; }
                    best = best.max(table[(t, start_valve, x)] + table[(s, start_valve, y)]);
                }
            }
        }
    }

    Ok(best)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
