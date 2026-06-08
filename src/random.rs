//! # Randomized Algorithms — Correlated Pair
//!
//! This module provides primitives for generating **correlated random pairs**,
//! a fundamental building block in randomized algorithms including:
//!
//! - Variance-reduction techniques (antithetic / control-variate sampling)
//! - Correlated sketching and locality-sensitive hashing
//! - Monte-Carlo simulations that require dependent random inputs
//!
//! ## Algorithm overview
//!
//! Given a desired Pearson correlation coefficient ρ ∈ \[-1, 1\], two
//! standard-normal random variables `(X, Y)` with `Corr(X, Y) = ρ` are
//! produced using the **linear combination method**:
//!
//! ```text
//! X  =  Z₁
//! Y  =  ρ·Z₁  +  √(1 − ρ²)·Z₂
//! ```
//!
//! where `Z₁` and `Z₂` are **independent** standard-normal samples obtained
//! via the [Box–Muller transform] applied to uniform random numbers from a
//! [Xorshift64] generator.
//!
//! [Box–Muller transform]: https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform
//! [Xorshift64]: https://en.wikipedia.org/wiki/Xorshift

use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// Xorshift64 — lightweight, seedable PRNG (no external dependencies)
// ---------------------------------------------------------------------------

/// A fast, seedable pseudo-random number generator based on the 64-bit
/// [xorshift](https://en.wikipedia.org/wiki/Xorshift) algorithm.
///
/// # Example
/// ```
/// use aps_2026::random::Xorshift64;
///
/// let mut rng = Xorshift64::new(42);
/// let u = rng.next_f64(); // uniform in [0, 1)
/// assert!(u >= 0.0 && u < 1.0);
/// ```
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Creates a new RNG seeded with `seed`.
    ///
    /// The seed must be non-zero; if `0` is passed it is replaced with `1`.
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    /// Returns the next raw 64-bit pseudo-random integer.
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Returns a pseudo-random `f64` uniformly distributed in `[0, 1)`.
    pub fn next_f64(&mut self) -> f64 {
        // Use the top 53 bits (mantissa width of f64).
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
}

// ---------------------------------------------------------------------------
// Box–Muller transform — convert uniform pairs to independent normals
// ---------------------------------------------------------------------------

/// Converts two independent uniform samples `(u1, u2)` in `(0, 1)` to two
/// independent standard-normal samples `(z0, z1)` using the
/// [Box–Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform).
///
/// Both inputs must be strictly positive; passing `0.0` triggers a panic in
/// debug builds (the logarithm is undefined at zero).
fn box_muller(u1: f64, u2: f64) -> (f64, f64) {
    debug_assert!(u1 > 0.0 && u1 < 1.0, "u1 must be in (0, 1)");
    debug_assert!(u2 > 0.0 && u2 < 1.0, "u2 must be in (0, 1)");
    let r = (-2.0 * u1.ln()).sqrt();
    let theta = 2.0 * PI * u2;
    (r * theta.cos(), r * theta.sin())
}

// ---------------------------------------------------------------------------
// CorrelatedPair — the main public API
// ---------------------------------------------------------------------------

/// Generator of **correlated standard-normal pairs** `(X, Y)`.
///
/// Each call to [`CorrelatedPair::sample`] returns `(X, Y)` where both
/// marginals are `N(0, 1)` and the Pearson correlation is the `rho` supplied
/// at construction time.
///
/// # Construction
///
/// ```
/// use aps_2026::random::CorrelatedPair;
///
/// // Pairs with correlation ρ = 0.8
/// let cpgen = CorrelatedPair::new(0.8, 42);
/// ```
///
/// # Panics
///
/// Panics if `rho` is outside `[-1, 1]`.
pub struct CorrelatedPair {
    rho: f64,
    /// √(1 − ρ²) — precomputed for efficiency.
    rho_perp: f64,
    rng: Xorshift64,
}

impl CorrelatedPair {
    /// Creates a new generator targeting correlation `rho` with the given
    /// RNG `seed`.
    ///
    /// # Panics
    ///
    /// Panics if `rho` is not in `[-1.0, 1.0]`.
    pub fn new(rho: f64, seed: u64) -> Self {
        assert!(
            (-1.0..=1.0).contains(&rho),
            "rho must be in [-1, 1], got {rho}"
        );
        Self {
            rho,
            rho_perp: (1.0 - rho * rho).sqrt(),
            rng: Xorshift64::new(seed),
        }
    }

    /// Samples a correlated pair `(X, Y)` of standard-normal random variables
    /// satisfying `Corr(X, Y) ≈ ρ`.
    ///
    /// # Construction
    ///
    /// Internally generates two independent normals `Z₁` and `Z₂` via
    /// Box–Muller, then sets:
    ///
    /// ```text
    /// X = Z₁
    /// Y = ρ·Z₁ + √(1 − ρ²)·Z₂
    /// ```
    ///
    /// # Example
    ///
    /// ```
    /// use aps_2026::random::CorrelatedPair;
    ///
    /// let mut cpgen = CorrelatedPair::new(0.9, 0xDEAD);
    /// for _ in 0..5 {
    ///     let (x, y) = gen.sample();
    ///     println!("x = {x:.4}, y = {y:.4}");
    /// }
    /// ```
    pub fn sample(&mut self) -> (f64, f64) {
        // Avoid ln(0) by resampling the rare u == 0 case.
        let u1 = loop {
            let u = self.rng.next_f64();
            if u > 0.0 {
                break u;
            }
        };
        let u2 = loop {
            let u = self.rng.next_f64();
            if u > 0.0 {
                break u;
            }
        };

        let (z1, z2) = box_muller(u1, u2);
        let x = z1;
        let y = self.rho * z1 + self.rho_perp * z2;
        (x, y)
    }

    /// Returns an iterator that yields correlated pairs indefinitely.
    ///
    /// # Example
    ///
    /// ```
    /// use aps_2026::random::CorrelatedPair;
    ///
    /// let cpgen = CorrelatedPair::new(-0.5, 7);
    /// let samples: Vec<(f64, f64)> = gen.iter().take(100).collect();
    /// assert_eq!(samples.len(), 100);
    /// ```
    pub fn iter(self) -> CorrelatedPairIter {
        CorrelatedPairIter { inner: self }
    }
}

/// An iterator over correlated pairs produced by a [`CorrelatedPair`] generator.
pub struct CorrelatedPairIter {
    inner: CorrelatedPair,
}

impl Iterator for CorrelatedPairIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.sample())
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: estimate Pearson correlation from a slice of pairs.
    fn pearson(pairs: &[(f64, f64)]) -> f64 {
        let n = pairs.len() as f64;
        let mean_x = pairs.iter().map(|(x, _)| x).sum::<f64>() / n;
        let mean_y = pairs.iter().map(|(_, y)| y).sum::<f64>() / n;
        let cov = pairs
            .iter()
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum::<f64>();
        let var_x = pairs.iter().map(|(x, _)| (x - mean_x).powi(2)).sum::<f64>();
        let var_y = pairs.iter().map(|(_, y)| (y - mean_y).powi(2)).sum::<f64>();
        cov / (var_x * var_y).sqrt()
    }

    /// The uniform samples from Xorshift64 must lie in [0, 1).
    #[test]
    fn test_xorshift64_range() {
        let mut rng = Xorshift64::new(1);
        for _ in 0..10_000 {
            let u = rng.next_f64();
            assert!(u >= 0.0 && u < 1.0, "out of range: {u}");
        }
    }

    /// Consecutive seeds must yield different sequences.
    #[test]
    fn test_xorshift64_different_seeds() {
        let mut rng_a = Xorshift64::new(1);
        let mut rng_b = Xorshift64::new(2);
        let a: Vec<u64> = (0..10).map(|_| rng_a.next_u64()).collect();
        let b: Vec<u64> = (0..10).map(|_| rng_b.next_u64()).collect();
        assert_ne!(a, b);
    }

    /// The same seed must always produce the same sequence (determinism).
    #[test]
    fn test_xorshift64_deterministic() {
        let mut rng1 = Xorshift64::new(99);
        let mut rng2 = Xorshift64::new(99);
        for _ in 0..1_000 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    /// A zero seed should not crash or produce an all-zero stream.
    #[test]
    fn test_xorshift64_zero_seed_replaced() {
        let mut rng = Xorshift64::new(0);
        // state must have been set to 1, so first output is non-zero
        assert_ne!(rng.next_u64(), 0);
    }

    /// Box–Muller should produce samples with mean ≈ 0 and variance ≈ 1.
    #[test]
    fn test_box_muller_moments() {
        let mut rng = Xorshift64::new(42);
        let n = 100_000usize;
        let mut samples = Vec::with_capacity(n * 2);
        for _ in 0..n {
            let u1 = loop {
                let u = rng.next_f64();
                if u > 0.0 {
                    break u;
                }
            };
            let u2 = loop {
                let u = rng.next_f64();
                if u > 0.0 {
                    break u;
                }
            };
            let (z0, z1) = box_muller(u1, u2);
            samples.push(z0);
            samples.push(z1);
        }
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        let var = samples.iter().map(|z| (z - mean).powi(2)).sum::<f64>() / samples.len() as f64;
        assert!(mean.abs() < 0.02, "mean too far from 0: {mean}");
        assert!((var - 1.0).abs() < 0.02, "variance too far from 1: {var}");
    }

    /// Sampling with ρ = 0 should yield (nearly) uncorrelated pairs.
    #[test]
    fn test_uncorrelated_pairs() {
        let mut cpgen = CorrelatedPair::new(0.0, 1);
        let samples: Vec<_> = (0..50_000).map(|_| cpgen.sample()).collect();
        let r = pearson(&samples);
        assert!(r.abs() < 0.02, "expected r ≈ 0, got {r}");
    }

    /// Sampling with ρ = 1 should yield pairs where X ≈ Y.
    #[test]
    fn test_perfectly_correlated_pairs() {
        let mut cpgen = CorrelatedPair::new(1.0, 2);
        let samples: Vec<_> = (0..1_000).map(|_| cpgen.sample()).collect();
        for (x, y) in &samples {
            assert!(
                (x - y).abs() < 1e-10,
                "expected x == y for rho=1, got x={x}, y={y}"
            );
        }
    }

    /// Sampling with ρ = -1 should yield pairs where X ≈ -Y.
    #[test]
    fn test_perfectly_anticorrelated_pairs() {
        let mut cpgen = CorrelatedPair::new(-1.0, 3);
        let samples: Vec<_> = (0..1_000).map(|_| cpgen.sample()).collect();
        for (x, y) in &samples {
            assert!(
                (x + y).abs() < 1e-10,
                "expected x == -y for rho=-1, got x={x}, y={y}"
            );
        }
    }

    /// Measured correlation should be within 2% of the requested ρ.
    #[test]
    fn test_positive_correlation_accuracy() {
        let mut cpgen = CorrelatedPair::new(0.7, 100);
        let samples: Vec<_> = (0..100_000).map(|_| cpgen.sample()).collect();
        let r = pearson(&samples);
        assert!((r - 0.7).abs() < 0.02, "expected r ≈ 0.7, got {r}");
    }

    /// Measured correlation should be within 2% of the requested negative ρ.
    #[test]
    fn test_negative_correlation_accuracy() {
        let mut cpgen = CorrelatedPair::new(-0.6, 77);
        let samples: Vec<_> = (0..100_000).map(|_| cpgen.sample()).collect();
        let r = pearson(&samples);
        assert!((r + 0.6).abs() < 0.02, "expected r ≈ -0.6, got {r}");
    }

    /// The iterator adapter must produce the same values as repeated `sample` calls
    /// from an identically-seeded generator.
    #[test]
    fn test_iter_matches_sample() {
        let mut cpgen_a = CorrelatedPair::new(0.5, 55);
        let cpgen_b = CorrelatedPair::new(0.5, 55);
        let from_sample: Vec<_> = (0..20).map(|_| cpgen_a.sample()).collect();
        let from_iter: Vec<_> = cpgen_b.iter().take(20).collect();
        assert_eq!(from_sample, from_iter);
    }

    /// Constructing with rho outside [-1, 1] must panic.
    #[test]
    #[should_panic]
    fn test_invalid_rho_panics() {
        CorrelatedPair::new(1.5, 1);
    }
}
