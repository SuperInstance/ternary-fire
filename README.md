# ternary-fire

**Forest fire model on ternary grids. Burning, empty, and growing.**

The forest fire model is one of the simplest examples of *self-organized criticality*. Trees grow (+1), lightning ignites them (-1), they burn to ash (0), and new trees grow again. No central control. No tuning. The system naturally finds a critical state where huge conflagrations are rare but possible — following a power law.

This crate implements the classic Drossel-Schwabl forest fire on a ternary grid: `+1 = tree`, `0 = empty`, `-1 = burning`. One step, one rule, endless emergent behavior.

## What's Inside

- **`new_grid(width, height, tree_density)`** — initialize a forest with given tree density
- **`step(grid, ignitions, spread_prob, growth_prob, width, height)`** — one tick: burn trees, spread fire, grow new trees
- **`count_states(grid)`** — how many burning, empty, tree cells
- **`burn_fraction(grid)`** — what fraction is on fire
- **`tree_fraction(grid)`** — forest coverage
- **`simulate(width, height, density, spread, growth, ticks)`** — full simulation, returns history of state counts

## Quick Example

```rust
use ternary_fire::*;

// Start with 60% tree coverage on a 50x50 grid
let mut grid = new_grid(50, 50, 0.6);

// Lightning strikes at (25, 25)
let ignitions = vec![(25, 25)];

// Run one step: fire spreads with 30% probability, trees grow with 1% probability
grid = step(&grid, &ignitions, 0.3, 0.01, 50, 50);

let (burning, empty, trees) = count_states(&grid);
println!("Burning: {}, Empty: {}, Trees: {}", burning, empty, trees);

// Full simulation: 500 ticks
let history = simulate(50, 50, 0.6, 0.3, 0.01, 500);
// Each entry: (tick, burning_count, empty_count, tree_count)
// Look for power-law distribution of fire sizes
```

## The Insight

**Criticality emerges for free.** You don't need to tune the forest fire model to get interesting behavior — it self-organizes. The ratio of growth rate to spread rate determines whether you get frequent small fires or rare catastrophic ones. In ternary agent systems, this maps directly to the question: how resilient is the population to cascading failures?

**Use cases:**
- **Self-organized criticality research** — the simplest SOC model
- **Epidemiology** — SIR model on a grid (Susceptible=tree, Infected=burning, Recovered=empty)
- **Risk analysis** — cascade failure modeling in infrastructure networks
- **Opinion dynamics** — spread of ideas through a population (contagion model)
- **Teaching** — the most accessible example of emergence

## See Also

- **ternary-sandpile** — another self-organized critical system on ternary grids
- **ternary-percolation** — when does fire spread become percolation?
- **ternary-cascade** — (related) avalanche dynamics
- **ternary-cell** — large-scale fire simulations

## Install

```bash
cargo add ternary-fire
```

## License

MIT
