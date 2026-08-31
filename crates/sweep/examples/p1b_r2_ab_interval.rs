//! **PCURVE P-1b, reviewer R2 — the `#1116` A/B in the INTERVAL lane.**
//!
//! The f64 A/B (`p1b_r2_ab.rs`) showed the conversion the spec ordered
//! certifying at ε = 1e-6, 1e-9 and 1e-12 on `die_fillet`'s shape. But
//! the PR sources its escalation from `m4_pr6_roundtrip_interval` — the
//! INTERVAL lane — so the f64 result cannot settle it alone. This is
//! the same fixture at the certified `Interval` scalar.
//!
//! Run with `--features interval`; ε comes from `CAD_TOLERANCE_EPS`.
//! Prints; asserts nothing.

// A reviewer's evidence binary, not a library door: it fails LOUDLY on
// any unexpected state, which is the point. Same allowance the test
// targets carry, for the same reason.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[cfg(feature = "interval")]
fn main() {
    use geom::Surface;
    use geom_core::{Band, Bounds, Interval, Point2, Real, Tol};
    use profile::RawLoop;
    use profile::{Profile, ProfileLoop, SketchPlane};
    use sweep::blend::fillet_edges;
    use sweep::{Extrusion, extrude};
    use topo::{Body, EdgeKey};

    let _ = <Interval as Bounds>::lo;
    let band = Band::linear(Tol::witness()).unwrap();
    let i = Interval::from_f64;
    let (l, r) = (1.0_f64, 0.15_f64);

    let lp = ProfileLoop::polygon([
        Point2::new(i(0.0), i(0.0)),
        Point2::new(i(l), i(0.0)),
        Point2::new(i(l), i(l)),
        Point2::new(i(0.0), i(l)),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .expect("the square validates");
    let blank: Body<Interval> = extrude(&vp, Extrusion::Distance(i(l)), Tol::witness())
        .expect("the square extrudes")
        .body;

    let planes = blank
        .faces()
        .filter(|(_, f)| matches!(blank.get_surface(f.surface), Some(Surface::Plane { .. })))
        .count();
    println!(
        "[ABI] eps = {:e} | die blank: {} faces, {planes} planes, {} edges",
        Tol::witness().get().eps,
        blank.faces().count(),
        blank.edges().count()
    );

    let edges: Vec<EdgeKey> = blank.edges().map(|(k, _)| k).collect();
    match fillet_edges(&blank, &edges, i(r), Tol::witness()) {
        Err(e) => println!("[ABI] fillet_edges REFUSED: {e:?}"),
        Ok(f) => {
            let scaffolds = f
                .body
                .edges()
                .filter(|(_, e)| {
                    f.body
                        .get_curve_geom(e.curve)
                        .and_then(topo::CurveGeom::certified)
                        .is_some_and(|c| {
                            matches!(c.description(), geom_brep::EdgeDescription::Scaffold(_))
                        })
                })
                .count();
            let t3 = topo::validate_geometric(&f.body, Tol::witness());
            let (n_err, n_fence) = match &t3 {
                Ok(()) => (0usize, 0usize),
                Err(v) => (
                    v.len(),
                    v.iter()
                        .filter(|e| matches!(e, topo::ValidationError::ScaffoldAtRest { .. }))
                        .count(),
                ),
            };
            println!(
                "[ABI] fillet_edges OK: {} faces, {} edges | scaffolds = {scaffolds} | \
                 tier3 errors = {n_err} (ScaffoldAtRest = {n_fence})",
                f.body.faces().count(),
                f.body.edges().count()
            );
            if let Err(v) = &t3 {
                for e in v.iter().take(4) {
                    println!("[ABI]   tier3: {e:?}");
                }
            }
        }
    }
}

#[cfg(not(feature = "interval"))]
fn main() {
    println!("[ABI] built without --features interval; nothing measured");
}
