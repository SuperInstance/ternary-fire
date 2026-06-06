# ternary-fire

**Forest fire cellular automaton on ternary grids — modeling propagation dynamics with balanced ternary states.**

## Background

The forest fire model is a classic cellular automaton in complexity science, first studied by Drossel and Schwabl (1992). It demonstrates self-organized criticality: a system that naturally evolves toward a critical state where small perturbations can trigger cascading events of any size. The traditional model uses three states (empty, tree, burning) — making it a natural fit for balanced ternary representation.

`ternary-fire` implements the forest fire model on a 2D ternary grid where each cell takes one of three values:

| State | Trit Value | Meaning |
|-------|-----------|---------|
| Burning | −1 | Currently on fire |
| Empty | 0 | No tree present |
| Tree | +1 | Living tree |

The model captures the interplay between growth (empty → tree), ignition (tree → burning via lightning or neighbor spread), and combustion (burning → empty). This cycle produces emergent behavior: periodic oscillations, power-law fire-size distributions, and phase transitions dependent on density and spread probability.

## How It Works

### Grid Initialization

`new_grid(width, height, tree_density)` creates a grid where each cell is independently set to tree (+1) with probability `tree_density`, or empty (0) otherwise. A seeded PRNG (xorshift64) ensures reproducibility.

### Simulation Step

Each `step()` applies four rules simultaneously:

1. **Ignition** — Explicit ignition points `[(x, y)]` set tree cells to burning
2. **Combustion** — Burning cells (−1) become empty (0) in the next generation
3. **Spread** — Tree cells (+1) adjacent to burning cells catch fire with probability `spread_prob` (von Neumann neighborhood)
4. **Growth** — Empty cells (0) spontaneously grow trees with probability `growth_prob`

The step function uses double-buffering (old grid read, new grid written) to ensure synchronous updates — all cells observe the same state during transition.

### Analysis Functions

- **`count_states(grid)`** — returns `(burning, empty, tree)` counts
- **`burn_rate(history)`** — computes the fraction of burning cells at each timestep
- **`cycle_period(history)`** — detects periodic behavior by searching for repeating burn-count patterns

## Experimental Results

The test suite validates core dynamics:

- **Density control** — `tree_density = 1.0` produces all trees; `0.0` produces all empty
- **Ignition** — a lightning strike on a tree cell converts it to burning
- **Fire spread** — with `spread_prob = 1.0`, fire propagates to all adjacent trees
- **Combustion** — burning cells become empty in the next step
- **Growth** — with `growth_prob = 1.0`, empty cells regrow trees
- **Cycle detection** — repeating grid patterns are identified with correct period

### Theoretical Behavior

At equilibrium, the model exhibits three regimes:
- **Subcritical** (low density, low spread): isolated fires, sparse tree cover
- **Critical** (moderate density, moderate spread): power-law fire-size distribution, scale-invariant patterns
- **Supercritical** (high density, high spread): catastrophic fires that clear large regions, followed by regrowth cycles

The `cycle_period()` function can detect these regimes by analyzing the periodicity of burn counts over time.

## Impact

`ternary-fire` demonstrates that balanced ternary is a natural representation for multi-state cellular automata. The three-state fire model maps directly to {−1, 0, +1} without encoding overhead. This has implications beyond academic simulation:

- **Epidemiological modeling** — the same model applies to disease spread (susceptible/infected/recovered), information cascades in social networks, and failure propagation in distributed systems.
- **Chaos engineering** — by varying spread probability and growth rate, the model generates controlled cascade scenarios for testing fault-tolerance mechanisms.

## Use Cases

1. **Cascade simulation in ternary fleets** — Model how failures propagate through a fleet of rooms. Rooms in state +1 (healthy), 0 (idle), or −1 (failing). Tune spread probability to study failure cascade dynamics and validate that fault isolation mechanisms prevent supercritical cascades.

2. **Chaos engineering scenarios** — Generate test scenarios for `ternary-chaos` by running fire simulations with specific parameters. The resulting grid histories serve as input for sensitivity analysis and Lyapunov exponent estimation.

3. **Educational visualization** — The ternary grid maps directly to RGB color channels (−1 → red, 0 → black, +1 → green), enabling real-time visualization of fire dynamics. The `burn_rate()` function produces time-series data for plotting.

4. **Phase transition research** — Sweep parameters (density, spread_prob) to map the phase diagram of the ternary fire model. The `cycle_period()` function identifies transitions between periodic and chaotic regimes.

5. **Epidemiological modeling** — Adapt the model for SIR (susceptible-infected-recovered) dynamics: trees = susceptible, burning = infected, empty = recovered. Study how different spread probabilities affect epidemic curves.

## Open Questions

- **Stochastic seeding:** The current implementation uses a fixed seed (42). Should the API accept arbitrary seeds or a randomness trait for proper Monte Carlo studies?
- **Larger neighborhoods:** The von Neumann neighborhood (4 neighbors) is simple. Should the model support Moore neighborhood (8 neighbors) or arbitrary neighborhood definitions for studying different propagation topologies?
- **Three-dimensional grids:** The current model is 2D. Can the same framework extend to 3D grids for modeling volumetric propagation (e.g., fire in a multi-story building, failure cascades in a layered network)?

## Connection to Oxide Stack

`ternary-fire` is a simulation tool within the SuperInstance ecosystem:

- **`ternary-chaos`** — fire model outputs feed into chaos analysis (Lyapunov exponents, bifurcation detection)
- **`ternary-event`** — fire state changes can be published as events for real-time monitoring
- **`ternary-game-theory`** — fire scenarios can be framed as cooperative games where agents choose to invest in fire prevention
- **`ternary-voting`** — consensus on fire response priorities uses ternary voting mechanisms

The ternary grid representation (−1, 0, +1) ensures compatibility with all other crates in the ecosystem, enabling seamless data flow between simulation, analysis, and decision-making layers.
