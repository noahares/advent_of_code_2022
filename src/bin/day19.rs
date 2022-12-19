use std::collections::HashSet;
use anyhow::Result;
use itertools::Itertools;

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
    geode_robots: u32
}

impl State {
    fn update(self: &mut Self) -> () {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn timestep(self: &Self, blueprint: &Blueprint, geode_threshold: u32, remaining_time: u32, next_states: &mut HashSet<State>) -> () {
       let mut base_state = *self;
       base_state.update();
       // geode first for pruning
       if self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1 {
           let mut state = base_state;
           state.geode_robots += 1;
           state.ore -= blueprint.geode.0;
           state.obsidian -= blueprint.geode.1;
           if state.can_prune(geode_threshold, remaining_time - 1) { return; }
           next_states.insert(state);
       }
       if base_state.can_prune(geode_threshold, remaining_time - 1) { return; }
       next_states.insert(base_state);
       if self.ore >= blueprint.ore {
           let mut state = base_state;
           state.ore_robots += 1;
           state.ore -= blueprint.ore;
           next_states.insert(state);
       }
       if self.ore >= blueprint.clay {
           let mut state = base_state;
           state.clay_robots += 1;
           state.ore -= blueprint.clay;
           next_states.insert(state);
       }
       if self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1 {
           let mut state = base_state;
           state.obsidian_robots += 1;
           state.ore -= blueprint.obsidian.0;
           state.clay -= blueprint.obsidian.1;
           next_states.insert(state);
       }
    }

    fn can_prune(self: &Self, geode_threshold: u32, remaining_time: u32) -> bool {
        self.geode_lower_bound(remaining_time) + (0..remaining_time).sum::<u32>() < geode_threshold
    }

    fn geode_lower_bound(self: &Self, remaining_time: u32) -> u32 {
        self.geode + self.geode_robots * remaining_time
    }

}

fn part01() -> Result<u32> {
    const TIME: u32 = 24;
    let blueprints = std::fs::read_to_string("./data/day19.input")?
        .trim()
        .lines()
        .map(|line| {
            let costs = line.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect_vec();
            Blueprint { ore: costs[0], clay: costs[1], obsidian: (costs[2], costs[3]), geode: (costs[4], costs[5]) }
        })
    .collect::<Vec<Blueprint>>();
    let mut score = 0;
    let start_state = State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0 };
    for (i, blueprint) in blueprints.iter().enumerate() {
        let mut states = HashSet::new();
        let mut next_states = HashSet::new();
        let mut geode_lower_bound = 0;
        states.insert(start_state);
        for t in (1..=TIME).rev() {
            for state in &states {
                if state.can_prune(geode_lower_bound, t) { continue; }
                geode_lower_bound = geode_lower_bound.max(state.geode_lower_bound(t));
                state.timestep(&blueprint, geode_lower_bound, t, &mut next_states);
            }
            std::mem::swap(&mut states, &mut next_states);
            next_states.clear();
        }
        score += (i+1) as u32 * geode_lower_bound;
    }
    Ok(score)
}

fn part02() -> Result<u32> {
    const TIME: u32 = 32;
    let blueprints = std::fs::read_to_string("./data/day19.input")?
        .trim()
        .lines()
        .take(3)
        .map(|line| {
            let costs = line.split_whitespace().filter_map(|word| word.parse::<u32>().ok()).collect_vec();
            Blueprint { ore: costs[0], clay: costs[1], obsidian: (costs[2], costs[3]), geode: (costs[4], costs[5]) }
        })
    .collect::<Vec<Blueprint>>();
    let mut score = 1;
    let start_state = State { ore: 0, clay: 0, obsidian: 0, geode: 0, ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0 };
    for blueprint in &blueprints {
        let mut states = HashSet::new();
        let mut next_states = HashSet::new();
        let mut geode_lower_bound = 0;
        states.insert(start_state);
        for t in (1..=TIME).rev() {
            for state in &states {
                if state.can_prune(geode_lower_bound, t) { continue; }
                geode_lower_bound = geode_lower_bound.max(state.geode_lower_bound(t));
                state.timestep(&blueprint, geode_lower_bound, t, &mut next_states);
            }
            std::mem::swap(&mut states, &mut next_states);
            next_states.clear();
        }
        score *= geode_lower_bound;
    }
    Ok(score)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
