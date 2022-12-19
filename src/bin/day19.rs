use std::collections::HashSet;
use anyhow::Result;
use itertools::Itertools;

const TIME: u32 = 24;

#[derive(Debug)]
struct Blueprint {
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32)
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct State {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    time: u32
}

impl State {
    fn update(self: &mut Self) -> () {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
        self.time += 1;
    }

    fn timestep(self: &mut Self, blueprint: &Blueprint) -> Vec<State> {
       let mut successor_states = Vec::new();
       let mut base_state = *self;
       base_state.update();
       if base_state.time == TIME { return successor_states; }
       successor_states.push(base_state);
       if self.ore >= blueprint.ore {
           let mut state = base_state;
           state.ore_robots += 1;
           state.ore -= blueprint.ore;
           successor_states.push(state);
       }
       if self.ore >= blueprint.clay {
           let mut state = base_state;
           state.clay_robots += 1;
           state.ore -= blueprint.clay;
           successor_states.push(state);
       }
       if self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1 {
           let mut state = base_state;
           state.obsidian_robots += 1;
           state.ore -= blueprint.obsidian.0;
           state.clay -= blueprint.obsidian.1;
           successor_states.push(state);
       }
       if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
           let mut state = base_state;
           state.geode_robots += 1;
           state.ore -= blueprint.geode.0;
           state.obsidian -= blueprint.geode.1;
           successor_states.push(state);
       }
       successor_states
    }

    fn can_prune(self: &Self, geode_threshold: u32) -> bool {
        self.geode_lower_bound() + (0..TIME - self.time).sum::<u32>() < geode_threshold
    }

    fn geode_lower_bound(self: &Self) -> u32 {
        self.geode + self.geode_robots * (TIME - self.time)
    }

}

fn part01() -> Result<u32> {
    let blueprints = std::fs::read_to_string("./data/day19.example")?
        .trim()
        .lines()
        .map(|line| {
            let costs = line.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect_vec();
            Blueprint { ore: costs[0], clay: costs[1], obsidian: (costs[2], costs[3]), geode: (costs[4], costs[5]) }
        })
    .collect::<Vec<Blueprint>>();
    let start_state = State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0, time: 0 };
    for blueprint in &blueprints {
        let mut states = HashSet::new();
        let mut geode_lower_bound = 0;
        states.insert(start_state);
        while let Some(mut state) = states.iter().next().cloned() {
            states.remove(&state);
            // println!("{state:?}");
            if state.can_prune(geode_lower_bound) { continue; }
            geode_lower_bound = geode_lower_bound.max(state.geode_lower_bound());
            let new_states = state.timestep(&blueprint);
            states.extend(new_states);
        }
        println!("{geode_lower_bound}");
    }
    Ok(0)
}

fn part02() -> Result<u32> {
    Ok(0)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
