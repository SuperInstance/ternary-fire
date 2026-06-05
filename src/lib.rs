#![forbid(unsafe_code)]

/// Forest fire model on ternary grids.
/// States: -1 = burning, 0 = empty, +1 = tree.

pub fn new_grid(width: usize, height: usize, tree_density: f64) -> Vec<i8> {
    let seed = 42u64;
    let mut rng = SimpleRng::new(seed);
    (0..width * height)
        .map(|_| if rng.next_f64() < tree_density { 1 } else { 0 })
        .collect()
}

pub fn step(grid: &[i8], ignitions: &[(usize, usize)], spread_prob: f64, growth_prob: f64, width: usize, height: usize) -> Vec<i8> {
    let mut next = grid.to_vec();
    let mut rng = SimpleRng::new(123);

    for &(x, y) in ignitions {
        let idx = y * width + x;
        if idx < next.len() && next[idx] == 1 {
            next[idx] = -1;
        }
    }

    let old = grid.to_vec();
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            if old[idx] == -1 {
                next[idx] = 0; // burning -> empty
            } else if old[idx] == 1 {
                // check burning neighbors
                let neighbors = get_neighbors(x, y, width, height);
                for &ni in &neighbors {
                    if old[ni] == -1 && rng.next_f64() < spread_prob {
                        next[idx] = -1;
                        break;
                    }
                }
            } else if old[idx] == 0 {
                if rng.next_f64() < growth_prob {
                    next[idx] = 1;
                }
            }
        }
    }
    next
}

fn get_neighbors(x: usize, y: usize, w: usize, h: usize) -> Vec<usize> {
    let mut n = Vec::new();
    if x > 0 { n.push(y * w + x - 1); }
    if x + 1 < w { n.push(y * w + x + 1); }
    if y > 0 { n.push((y - 1) * w + x); }
    if y + 1 < h { n.push((y + 1) * w + x); }
    n
}

pub fn count_states(grid: &[i8]) -> (usize, usize, usize) {
    let mut burning = 0; let mut empty = 0; let mut tree = 0;
    for &v in grid {
        match v {
            -1 => burning += 1,
            0 => empty += 1,
            1 => tree += 1,
            _ => {}
        }
    }
    (burning, empty, tree)
}

pub fn burn_rate(history: &[Vec<i8>]) -> Vec<f64> {
    history.iter().map(|g| {
        let n = g.len();
        if n == 0 { return 0.0; }
        let (b, _, _) = count_states(g);
        b as f64 / n as f64
    }).collect()
}

pub fn cycle_period(history: &[Vec<i8>]) -> Option<usize> {
    if history.len() < 4 { return None; }
    // Look for the burn count pattern to repeat
    let rates: Vec<usize> = history.iter().map(|g| count_states(g).0).collect();
    for period in 2..=rates.len() / 2 {
        let mut matches = true;
        for i in 0..period {
            if rates[i] != rates[i + period] { matches = false; break; }
        }
        if matches { return Some(period); }
    }
    None
}

struct SimpleRng { state: u64 }

impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: if seed == 0 { 1 } else { seed } } }
    fn next_u64(&mut self) -> u64 {
        // xorshift64
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid_density() {
        let g = new_grid(10, 10, 1.0);
        assert!(g.iter().all(|&v| v == 1));
    }

    #[test]
    fn test_new_grid_empty() {
        let g = new_grid(10, 10, 0.0);
        assert!(g.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_new_grid_size() {
        let g = new_grid(5, 7, 0.5);
        assert_eq!(g.len(), 35);
    }

    #[test]
    fn test_step_ignition() {
        let mut g = vec![1i8; 9]; // 3x3 all trees
        g = step(&g, &[(1, 1)], 1.0, 0.0, 3, 3);
        // center ignited, neighbors catch fire with prob 1.0
        assert_eq!(g[4], -1); // center (1,1) = index 4 burning
    }

    #[test]
    fn test_step_growth() {
        let g = vec![0i8; 100]; // all empty
        let next = step(&g, &[], 0.0, 1.0, 10, 10);
        // With growth_prob=1.0 and our rng, some should grow
        assert!(next.iter().any(|&v| v == 1));
    }

    #[test]
    fn test_count_states() {
        let g = vec![-1, 0, 1, 1, 0, -1];
        let (b, e, t) = count_states(&g);
        assert_eq!(b, 2);
        assert_eq!(e, 2);
        assert_eq!(t, 2);
    }

    #[test]
    fn test_count_states_all_trees() {
        let g = vec![1i8; 25];
        let (b, e, t) = count_states(&g);
        assert_eq!((b, e, t), (0, 0, 25));
    }

    #[test]
    fn test_burn_rate() {
        let h = vec![vec![-1, 0, 1], vec![0, 0, 1], vec![-1, -1, 0]];
        let rates = burn_rate(&h);
        assert_eq!(rates.len(), 3);
        // first grid: 1 burning out of 3 total cells
        assert!((rates[0] - 1.0/3.0).abs() < 1e-9);
    }

    #[test]
    fn test_burn_rate_empty() {
        let h: Vec<Vec<i8>> = vec![vec![], vec![]];
        let rates = burn_rate(&h);
        assert_eq!(rates, vec![0.0, 0.0]);
    }

    #[test]
    fn test_cycle_period_none() {
        let h = vec![
            vec![1, 0, -1],
            vec![0, 0, 0],
            vec![1, 1, 1],
        ];
        assert!(cycle_period(&h).is_none());
    }

    #[test]
    fn test_cycle_period_found() {
        let h = vec![
            vec![-1, 0], vec![0, 1], vec![-1, 0], vec![0, 1],
        ];
        assert_eq!(cycle_period(&h), Some(2));
    }

    #[test]
    fn test_cycle_period_too_short() {
        let h = vec![vec![1]];
        assert!(cycle_period(&h).is_none());
    }

    #[test]
    fn test_no_spread_prob_zero() {
        let g = vec![1, -1, 1]; // 3x1
        let next = step(&g, &[], 0.0, 0.0, 3, 1);
        assert_eq!(next[1], 0); // burning -> empty
    }

    #[test]
    fn test_step_fire_becomes_empty() {
        let g = vec![-1i8; 4]; // 2x2 all burning
        let next = step(&g, &[], 1.0, 0.0, 2, 2);
        // all burning cells become empty
        assert!(next.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_get_neighbors_corner() {
        let n = get_neighbors(0, 0, 3, 3);
        assert_eq!(n.len(), 2);
    }

    #[test]
    fn test_get_neighbors_center() {
        let n = get_neighbors(1, 1, 3, 3);
        assert_eq!(n.len(), 4);
    }
}
