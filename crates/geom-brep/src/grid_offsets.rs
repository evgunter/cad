//! The knot offsets the domain-grid sites' no-cliff rows sweep: a
//! stated knot placed at every distance from a grid point that a
//! clearance rule could get wrong, each row reading what its site's
//! consumer reads.

use geom_core::spline::algebra::{grid_clearance, range_grid_points};

/// Grid point `k` (`0 < k < pieces`) of the `pieces`-span grid on
/// `[0, 1]`, as the production grid computes it, with knots at it and
/// at every offset from it a skip rule has a boundary at, labelled:
///
/// * one ulp and 129 ulps (`≈ 1.8e-15` at `1/16`) above — gaps of a
///   few ulps, where an inserted span's `1/gap` excess is largest;
/// * `1e-14`, `1e-13`, `1e-12` and `1e-9` above, and `1e-13` below —
///   the gaps over which that excess decays;
/// * ON the grid's [`grid_clearance`] (the grid point is skipped) and
///   one ulp past it (the grid point is taken);
/// * `1e-3` above, a knot no rule ties to the grid point.
pub(crate) fn knot_offsets(pieces: usize, k: usize) -> (f64, Vec<(String, f64)>) {
    let g = range_grid_points(0.0, 1.0, pieces, &[])[k - 1];
    let clearance = grid_clearance(0.0, 1.0, pieces);
    let bits = |n: u64| f64::from_bits(g.to_bits() + n);
    let rows = vec![
        ("on the grid point".to_string(), g),
        ("1 ulp above".to_string(), bits(1)),
        ("129 ulps above".to_string(), bits(129)),
        ("1e-14 above".to_string(), g + 1e-14),
        ("1e-13 above".to_string(), g + 1e-13),
        ("1e-13 below".to_string(), g - 1e-13),
        ("1e-12 above".to_string(), g + 1e-12),
        ("1e-9 above".to_string(), g + 1e-9),
        ("on the clearance".to_string(), g + clearance),
        (
            "1 ulp past the clearance".to_string(),
            (g + clearance).next_up(),
        ),
        ("1e-3 above".to_string(), g + 1e-3),
    ];
    (g, rows)
}
