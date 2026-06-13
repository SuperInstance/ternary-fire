# ternary-fire

Forest fire simulation on ternary grids using the Drossel-Schwabl model. Each cell holds a ternary state: **−1 = burning, 0 = empty, +1 = tree**, enabling compact representation of the three-phase fire dynamics that govern wildfire spread, tree regrowth, and cyclic behavior analysis.

## Why It Matters

Wildfire modeling drives critical decisions in forestry management, climate science, and emergency response. Classical forest fire models (Bak, Chen & Tang, 1992) use at least 2 bits per cell for state encoding. The ternary representation uses a natural 3-state mapping that fits in a signed byte, enabling:

- **3× state density** over separate boolean grids (tree/empty + burning/not-burning)
- **Compact serialization** — one `i8` per cell, directly serializable
- **Phase analysis** — built-in tools for detecting periodic behavior (limit cycles) in fire dynamics
- **Burn-rate tracking** — time-series of the fraction of burning cells, useful for parametric studies

The model exhibits **self-organized criticality** — it naturally evolves toward a critical state where fires of all sizes occur, without any parameter tuning.

## How It Works

### The Drossel-Schwabl Model

The grid evolves according to stochastic cellular automaton rules. At each time step:

1. **Ignition**: A burning cell (−1) becomes empty (0). `s(t+1) = 0`
2. **Spread**: A tree (+1) with at least one burning neighbor catches fire with probability p. `s(t+1) = −1`
3. **Growth**: An empty cell (0) grows a tree with probability q. `s(t+1) = +1`

The state transition function is:

```
s(t+1) = {
    0                    if s(t) = −1                    (burning → empty)
    −1                   if s(t) = +1  ∧  ∃ burning neighbor  ∧  ξ < p
    +1                   if s(t) = 0   ∧  ξ < q           (empty → tree)
    s(t)                 otherwise                       (no change)
}
```

where ξ ~ Uniform(0,1) and p, q are the spread and growth probabilities.

### Critical Behavior

The ratio p/q determines the fire regime:
- **p/q >> 1** (fast spread, slow growth): Large devastating fires, low tree density
- **p/q << 1** (slow spread, fast growth): Small fires, near-full forest
- **p/q ~ √N** (critical): Scale-free fire size distribution — power law P(size) ~ size^(−τ), τ ≈ 1.19 in 2D

### Neighbor Topology

Uses 4-connected von Neumann neighborhood with non-wrapping boundaries:

```
    ┌───┐
    │ N │
┌───┼───┼───┐
│ W │ C │ E │
└───┼───┼───┘
    │ S │
    └───┘
```

Boundary cells have fewer neighbors, naturally constraining fire spread at edges.

### Cycle Detection

The `cycle_period` function searches for periodicity in the burn-rate time series using autocorrelation:

```
For period τ in [2, T/2]:
    If burn_count(t) == burn_count(t + τ) for all t in [0, τ):
        Return τ
```

### Complexity

| Operation | Time | Space |
|-----------|------|-------|
| `new_grid(w, h, d)` | O(w·h) | O(w·h) |
| `step(grid, p, q)` | O(w·h) | O(w·h) |
| `count_states(grid)` | O(w·h) | O(1) |
| `burn_rate(history)` | O(T·N) | O(T) |
| `cycle_period(history)` | O(T²/4) | O(T) |

Where w × h = grid dimensions, T = history length, N = cells per grid.

### Random Number Generation

Uses xorshift64 (Marsaglia, 2003):

```
x₁ = x₀ ⊕ (x₀ << 13)
x₂ = x₁ ⊕ (x₁ >> 7)
x₃ = x₂ ⊕ (x₂ << 17)
```

Period: 2⁶⁴ − 1. Each step is 3 XOR + 3 shift operations — the fastest practical PRNG for simulation.

## Quick Start

```rust
use ternary_fire::{new_grid, step, count_states, burn_rate, cycle_period};

// Create a 50×50 grid with 70% tree density
let mut grid = new_grid(50, 50, 0.7);

// Simulate 100 time steps
let mut history = vec![grid.clone()];
for t in 0..100 {
    // Ignite center cell
    let ignitions = if t == 0 { vec![(25, 25)] } else { vec![] };
    grid = step(&grid, &ignitions, 0.8, 0.01, 50, 50);
    history.push(grid.clone());
}

// Analyze results
let (burning, empty, tree) = count_states(&grid);
println!("Burning: {}, Empty: {}, Trees: {}", burning, empty, tree);

let rates = burn_rate(&history);
if let Some(period) = cycle_period(&history) {
    println!("Detected cycle period: {} steps", period);
}
```

## API

| Function | Description |
|----------|-------------|
| `new_grid(w, h, density)` | Initialize grid with given tree density |
| `step(grid, ignitions, p_spread, p_growth, w, h)` | Advance one time step |
| `count_states(grid)` | Count (burning, empty, tree) cells |
| `burn_rate(history)` | Fraction of burning cells per time step |
| `cycle_period(history)` | Detect periodicity in burn-rate dynamics |

## Architecture Notes

This crate implements **η (eta) layer** dynamics in the γ + η = C framework:

- **η (eta)**: The simulation engine — cellular automaton rules, random number generation, state transitions. This crate provides the η-layer fire model.
- **γ (gamma)**: External coordination — parallel grid evaluation, multi-region coupling, and checkpoint/restore would be provided by ecosystem crates.
- **C**: The complete wildfire simulation system. The ternary state encoding is the bridge: {-1, 0, +1} maps directly to the ternary compute primitives used throughout the ecosystem.

## References

- **Drossel-Schwabl Model**: Drossel, B. & Schwabl, F., "Self-Organized Criticality in a Forest-Fire Model," Physica A: Statistical Mechanics, 1992.
- **SOC Theory**: Bak, P., Tang, C. & Wiesenfeld, K., "Self-Organized Criticality," Physical Review A, 38(1), 364, 1988.
- **Forest Fire CA**: Bak, P., Chen, K. & Tang, C., "A Forest-Fire Model and Some Thoughts on Turbulence," Physics Letters A, 147(5-6), 297-300, 1990.
- **Power-Law Distributions**: Clar, S., Drossel, B. & Schwabl, F., "Forest Fires and Other Examples of Self-Organized Criticality," Journal de Physique I, 6(5), 603-610, 1996.
- **Xorshift PRNG**: Marsaglia, G., "Xorshift RNGs," Journal of Statistical Software, 8(14), 1-6, 2003.

## License

MIT
