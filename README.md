# ternary-fire

**Forest fire dynamics on a ternary grid. Sparks catch, fires spread, forests regrow, and the cycle never stops.**

A forest fire is a chain reaction: a single spark ignites a tree, the burning tree ignites its neighbors, the fire spreads until there's nothing left to burn, and then — slowly — the forest grows back. This cycle (growth → ignition → spread → burnout → growth) is one of the fundamental rhythms of natural systems.

This crate implements the Drossel-Schwabl forest fire model on a ternary grid. Three states: `+1 = living tree`, `0 = empty (burned or never grown)`, `-1 = burning`. At each tick, empty cells might grow a tree (probability p), trees next to fires catch fire, and burning cells become empty. The result is self-organized criticality — just like the sandpile, but with a biological clock.

## What's Inside

- **`FireGrid`** — ternary grid with states: `Tree (1)`, `Empty (0)`, `Burning (-1)`
- **`new(width, height, growth_prob, ignition_prob)`** — configure the fire model
- **`tick()`** — one generation: grow trees, spread fire, burn out
- **`plant(x, y)`** — manually plant a tree
- **`ignite(x, y)`** — set a tree on fire
- **`count(state)`** — how many cells in each state?
- **`burn_history()`** — record of how many cells burned each tick (the fire cycle)

## Quick Example

```rust
use ternary_fire::*;

let mut grid = FireGrid::new(30, 30, 0.05, 0.001);
// 5% chance of tree growth per empty cell per tick
// 0.1% chance of spontaneous ignition per tree per tick

// Start with a dense forest
for y in 0..30 {
    for x in 0..30 {
        grid.plant(x, y);
    }
}

// Drop a match
grid.ignite(15, 15);

// Watch it burn
for _ in 0..50 {
    grid.tick();
    let c = grid.count();
    println!("Trees: {}, Burning: {}, Empty: {}", c.trees, c.burning, c.empty);
}
// Fire spreads outward from center, burns everything, leaves emptiness
// Then slowly: trees regrow from the edges
```

## The Deeper Truth

**Fire is percolation with a clock.** Ternary percolation asks "can the +1 cells connect?" Fire asks "do the +1 cells connect *fast enough* to sustain a blaze?" The growth probability sets how dense the forest gets before the next fire. The ignition probability sets how often fires start. The ratio between them determines the fire regime:

- High growth, rare ignition → dense forests, catastrophic megafires
- Low growth, frequent ignition → sparse scrubland, small fires
- The sweet spot → patchy forests with regular, moderate fires

The Drossel-Schwabl model is another example of self-organized criticality (like the sandpile), but with a *memory* — the time since the last fire determines the current fuel load. The fire cycle is a natural rhythm: growth, accumulation, conflagration, reset. The ternary mapping is perfect: trees are +1 (building energy), empty is 0 (waiting), fire is -1 (releasing energy).

**Use cases:**
- **Ecological modeling** — forest fire dynamics and fire regime analysis
- **Risk assessment** — how does fuel load affect fire severity?
- **Generative art** — fire patterns create dramatic visual textures
- **Game design** — fire spreading mechanics for strategy games
- **Education** — the simplest model of a self-organizing natural cycle

## See Also

- **ternary-sandpile** — another SOC model (mechanical, not biological)
- **ternary-life** — Life is growth without fire
- **ternary-percolation** — fire is percolation with dynamics
- **ternary-irradiate** — radiation cascades (fire with inverse-square falloff)
- **ternary-morph** — morphological analysis of fire scars
- **ternary-color** — visualize fire with warm/cool palettes

## Install

```bash
cargo add ternary-fire
```

## License

MIT
