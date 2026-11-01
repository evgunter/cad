//! **An arc rim's row, compared**: the one comparison two suites make of
//! a `Pcurve::IsoArc` image against the row it should be (`geom-brep`'s
//! `onpair_plane_nurbs`, `sweep`'s `a_rim_covering_part_of_its_row_…`).
//! This crate has no dependencies, so the image arrives as its parts.

/// An `IsoArc` image's parts, each as the variant holds it.
#[derive(Clone, Debug)]
pub struct ArcRow {
    /// The chart point at `g = 0`, `(u, v)`.
    pub p0: [f64; 2],
    /// The chart displacement over the whole arc.
    pub pd: [f64; 2],
    /// The sub-arc breaks, as their `Debug` text: compared, never read.
    pub breaks: String,
    /// The carrier parameter at `g = 0`.
    pub t0: f64,
    /// The arc's turn.
    pub angle: f64,
}

/// `got` is `want`'s row, direction and sub-arcs bit for bit, over
/// `want`'s arc to `1e-12`: a measured arc is read off the geometry, so
/// it agrees to rounding, never to the bit.
///
/// # Panics
///
/// Where either part differs, naming `what`.
pub fn assert_same_row(got: &ArcRow, want: &ArcRow, what: &str) {
    assert_eq!(
        (got.p0, got.pd, &got.breaks),
        (want.p0, want.pd, &want.breaks),
        "{what}: the row, its direction and its sub-arcs"
    );
    assert!(
        (got.t0 - want.t0).abs() < 1e-12 && (got.angle - want.angle).abs() < 1e-12,
        "{what}: the arc ({}, {}) is ({}, {})",
        got.t0,
        got.angle,
        want.t0,
        want.angle
    );
}
