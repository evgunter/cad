//! PROBE (not for merge): check 1's margins on a disc turned by a boxed
//! angle, `work/onehome/check-1-reads-a-turning-frames-length-off-its-bracket.md`.
//! Run by name with `--ignored --nocapture`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::{Arc2, Band, Bounds, Interval, Point2, Real, RigidMotion, Tol, UnitVec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

const RADIUS: f64 = 5e-3;

fn disc(plane: SketchPlane<Interval>) -> Result<topo::Body<Interval>, String> {
    let tol = Tol::witness();
    let c = Interval::from_f64;
    let rim = RawLoop::new([(
        Point2::new(c(RADIUS), c(0.0)),
        Segment::Arc(Arc2 {
            centre: Point2::new(c(0.0), c(0.0)),
            radius: c(RADIUS),
            sweep: c(TAU),
        }),
    )]);
    let profile = Profile::new(plane, vec![rim])
        .validate(tol)
        .map_err(|e| format!("profile: {e:?}"))?;
    extrude(
        &profile,
        Extrusion::Distance {
            depth: c(RADIUS),
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .map(|out| out.body.into_body())
    .map_err(|e| format!("extrude: {e:?}"))
}

/// Check 1's reading of every face's surface and edge's carrier: the
/// least lower end of any margin, the widest margin bracket, whether
/// every margin's lower end is above zero (check 1 decides), and the
/// first straddling margin.
fn check1(body: &topo::Body<Interval>, band: Band) -> String {
    let mut least = f64::INFINITY;
    let mut first_open = None;
    let mut all = true;
    let mut seen = |what: String, m: Interval| {
        let (lo, hi) = (m.lo(), m.hi());
        least = least.min(lo);
        if lo <= 0.0 {
            all = false;
            if first_open.is_none() {
                first_open = Some(format!("{what} [{lo:.4e}, {hi:.4e}]"));
            }
        }
    };
    for (f, _) in body.faces() {
        if let Some(s) = body.face_surface(f) {
            for m in s.representability_margins(band) {
                seen(
                    format!("{:?} {:?} {:?} {:?}", s.kind(), m.datum, m.measure, m.end),
                    m.margin,
                );
            }
        }
    }
    for (e, _) in body.edges() {
        if let Ok(c) = topo::readback::edge_carrier_ref(body, e) {
            for m in c.representability_margins(band) {
                seen(
                    format!("{:?} {:?} {:?} {:?}", c.kind(), m.datum, m.measure, m.end),
                    m.margin,
                );
            }
        }
    }
    format!(
        "decides {all}; least lower end {least:.4e}; first open {}",
        first_open.unwrap_or_else(|| "none".into())
    )
}

fn stored(body: &topo::Body<Interval>) -> String {
    for (f, _) in body.faces() {
        if let Some(s) = body.face_surface(f)
            && s.kind() == geom::SurfaceKind::Cylinder
        {
            return format!("{s:?}").chars().take(1500).collect();
        }
    }
    "no cylinder".into()
}

#[test]
#[ignore = "probe"]
fn check1_turned_disc() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let theta = 30f64.to_radians();
    let flat = disc(SketchPlane::xy()).expect("the flat disc builds");
    eprintln!("[c1] flat, at rest: {}", check1(&flat, band));
    for k in 0..=10 {
        let delta_deg = 10f64.powi(-k);
        let delta = delta_deg.to_radians();
        let angle = Interval::from_bounds(theta - delta, theta + delta);
        let turn = RigidMotion::rotation(UnitVec3::<Interval>::exact_y(), angle).expect("a turn");
        // (a) the flat disc moved by the boxed turn.
        match topo::transform_rigid(&flat, &turn) {
            Ok(moved) => {
                eprintln!("[c1] moved δ {delta_deg:e}°: {}", check1(&moved, band));
                if k == 0 || k == 6 {
                    eprintln!(
                        "[c1] moved δ {delta_deg:e}° stored cylinder: {}",
                        stored(&moved)
                    );
                }
            }
            Err(e) => eprintln!("[c1] moved δ {delta_deg:e}°: transform refused {e:?}"),
        }
        // (b) the disc extruded on the turned sketch plane.
        match disc(SketchPlane::from_frame(turn)) {
            Ok(b) => eprintln!("[c1] door δ {delta_deg:e}°: {}", check1(&b, band)),
            Err(e) => eprintln!(
                "[c1] door δ {delta_deg:e}°: {}",
                e.chars().take(400).collect::<String>()
            ),
        }
    }
}
