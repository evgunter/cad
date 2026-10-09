//! The knot offsets the domain-grid sites' no-cliff rows sweep: a
//! stated knot placed at every distance from a grid point that a
//! clearance rule could get wrong, one row per site reading what that
//! site's consumer reads.

/// Knots at `g` (a grid point of a grid with spacing `spacing`) and at
/// every offset from it a skip rule has a boundary at, labelled:
///
/// * one ulp of the knot, and 129 ulps (one past an 8-domain-ulp
///   clearance on `[0, 1]` at `1/16`);
/// * `1e-14`, `1e-13`, `1e-12` and `1e-9` above, and `1e-13` below —
///   the gaps over which a hairline span's excess decays as `1/gap`;
/// * ON the clearance `spacing·GRID_CLEARANCE` (the grid point is
///   skipped) and one ulp past it (the grid point is taken);
/// * `1e-3` above, a knot no rule ties to the grid point.
pub(crate) fn knot_offsets(g: f64, spacing: f64) -> Vec<(String, f64)> {
    let clearance = spacing * geom_core::spline::algebra::GRID_CLEARANCE;
    let bits = |n: u64| f64::from_bits(g.to_bits() + n);
    vec![
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
    ]
}
