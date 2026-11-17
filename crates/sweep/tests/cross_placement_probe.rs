//! **PROBE (not for merge): cross-placement readings over a boxed rigid
//! motion, up front against per reading** (ONEHOME fork 2xx evidence,
//! `[ev]` PR 7079).
//!
//! One operand exact, the other moved by a boxed motion: a turn of
//! 30° ± δ about x, or a translation ± δ. (i) up front: today's path,
//! the whole body moved (`FinishedBody::moved`) and the reading or the
//! whole boolean run on the image. (ii) per reading: the body kept, the
//! reading taken through its seat (`topo::Seat`): classification at the
//! point (the query mapped into the body), the carrier-pair verdict at
//! the carrier (desc, extent ball, witnesses mapped). Readings that take
//! carriers only (section, edge×face, section certificate) are
//! crate-private in `topo`; for them the probe checks that a carrier
//! mapped at the reading is bit-identical to the moved body's, so (ii)
//! there reads what (i) reads.
//!
//! Run: `cargo nextest run -p sweep --test all -E 'test(cross_placement_probe)'
//! --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use geom_core::{
    Arc2, Band, Bounds, CertifiedBounds, Decide, Interval, ParamSymbol, Point2, Point3,
    RigidMotion, Sym, SymBudget, Tol, UnitVec3, Vec3,
};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::FinishedBody as _;
use topo::LeafBody as _;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

/// A cylinder of radius `r`, height `h`, centred on `(cx, cy)` at `z0`.
fn cylinder<T: Decide + topo::AtRestPolicy>(r: f64, h: f64, (cx, cy, z0): (f64, f64, f64)) -> topo::Body<T> {
    let l = T::from_f64;
    let rim = RawLoop::new([(
        Point2::new(l(r), l(0.0)),
        Segment::Arc(Arc2 {
            centre: Point2::new(l(0.0), l(0.0)),
            radius: l(r),
            sweep: l(core::f64::consts::TAU),
        }),
    )]);
    let plane = SketchPlane::from_frame(RigidMotion::axes_xy(Point3::new(l(cx), l(cy), l(z0))));
    let profile = Profile::new(plane, vec![rim]).validate(tol()).unwrap();
    let depth = Extrusion::Distance {
        depth: l(h),
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body.into_body()
}

/// A box `[0, a] × [0, b] × [0, c]`.
fn slab<T: Decide + topo::AtRestPolicy>(a: f64, b: f64, c: f64) -> topo::Body<T> {
    let l = T::from_f64;
    let rim = RawLoop::new([
        (Point2::new(l(0.0), l(0.0)), Segment::Line),
        (Point2::new(l(a), l(0.0)), Segment::Line),
        (Point2::new(l(a), l(b)), Segment::Line),
        (Point2::new(l(0.0), l(b)), Segment::Line),
    ]);
    let profile = Profile::new(SketchPlane::<T>::xy(), vec![rim]).validate(tol()).unwrap();
    let depth = Extrusion::Distance {
        depth: l(c),
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body.into_body()
}

/// What one reading did: decided, escalated (by predicate), or refused
/// another way.
#[derive(Default, Debug)]
struct Tally {
    decided: usize,
    escalated: BTreeMap<&'static str, usize>,
    other: BTreeMap<String, usize>,
}

impl Tally {
    fn show(&self) -> String {
        let esc: Vec<String> = self.escalated.iter().map(|(k, n)| format!("{k}×{n}")).collect();
        let oth: Vec<String> = self.other.iter().map(|(k, n)| format!("{k}×{n}")).collect();
        format!(
            "ok {} | esc [{}] | other [{}]",
            self.decided,
            esc.join(" "),
            oth.join(" ")
        )
    }
}

/// The escalations a bracket recorded, by predicate.
fn escalations(rec: &geom_core::k_stats::Recorded) -> BTreeMap<&'static str, usize> {
    let mut out = BTreeMap::new();
    for e in &rec.escalations {
        *out.entry(e.predicate()).or_insert(0) += 1;
    }
    out
}

fn bracketed<R>(f: impl FnOnce() -> R) -> (R, BTreeMap<&'static str, usize>) {
    let b = geom_core::k_stats::Bracket::open();
    let r = f();
    (r, escalations(&b.finish()))
}

/// One case: `a` exact, `b` read at `motion`.
struct Case<T: topo::AtRestPolicy> {
    name: &'static str,
    a: T::Finished,
    b: T::Finished,
    op: topo::boolean::BooleanOp,
}

fn probe<T>(lane: &str, what: &str, motion: &RigidMotion<T>, case: &Case<T>) -> Vec<String>
where
    T: Decide + CertifiedBounds + Bounds + topo::AtRestPolicy,
    T::Finished: topo::FinishedBody<Scalar = T> + core::ops::Deref<Target = topo::Body<T>>,
{
    let mut lines = Vec::new();
    let row = |reading: &str, how: &str, t: &str| format!("{lane} | {what} | {} | {reading} | {how} | {t}", case.name);
    // (i) the move itself.
    let (moved, esc) = bracketed(|| case.b.moved(motion, tol()));
    let moved = match moved {
        Ok(m) => {
            lines.push(row("move", "(i)", &format!("ok | esc {esc:?}")));
            Some(m)
        }
        Err(e) => {
            let kinds: Vec<String> = e
                .into_findings()
                .iter()
                .map(|f| format!("{f:?}").split([' ', '{', '(']).next().unwrap_or("").to_owned())
                .collect();
            lines.push(row("move", "(i)", &format!("REFUSED {kinds:?} | esc {esc:?}")));
            None
        }
    };
    let own_b: &topo::Body<T> = &case.b;
    let seat = topo::Seat::at(own_b, *motion).unwrap();
    // Carrier bits: a carrier mapped at the reading against the moved body's.
    if let Some(m) = &moved {
        let mut same = 0;
        let mut differ = 0;
        for (k, _) in own_b.faces() {
            let at = seat.surface(own_b.face_surface(k).unwrap());
            let held = m.face_surface(k).cloned();
            if format!("{at:?}") == format!("{held:?}") {
                same += 1;
            } else {
                differ += 1;
            }
        }
        for ((_, p), (_, q)) in own_b.points().zip(m.points()) {
            if format!("{:?}", seat.point(*p)) == format!("{q:?}") {
                same += 1;
            } else {
                differ += 1;
            }
        }
        lines.push(row(
            "carrier bits (surfaces, points)",
            "(ii) vs (i)",
            &format!("identical {same}, differ {differ}"),
        ));
    }
    // Point-in-body: A's vertices asked of B.
    let queries: Vec<Point3<T>> = case.a.vertex_points().map(|(_, p)| p).collect();
    let mut up = Tally::default();
    let mut per = Tally::default();
    for q in &queries {
        if let Some(m) = &moved {
            let (r, esc) = bracketed(|| topo::boolean::point_in_solid(m, *q, band(), tol()));
            tally(&mut up, r.map(|_| ()).map_err(|e| format!("{e:?}")), esc);
        }
        let own_q = seat.into_own(*q);
        let (r, esc) = bracketed(|| topo::boolean::point_in_solid(own_b, own_q, band(), tol()));
        tally(&mut per, r.map(|_| ()).map_err(|e| format!("{e:?}")), esc);
    }
    if moved.is_some() {
        lines.push(row("point-in-body (A's vertices in B)", "(i) moved B", &up.show()));
    }
    lines.push(row("point-in-body (A's vertices in B)", "(ii) at the point", &per.show()));
    // Carrier pair, every face pair.
    let mut up = Tally::default();
    let mut per = Tally::default();
    for (fa, _) in case.a.faces() {
        for (fb, _) in own_b.faces() {
            if let Some(m) = &moved {
                let (r, esc) = bracketed(|| topo::boolean::carrier_pair_decided(&case.a, fa, m, fb, band()));
                tally(&mut up, pair(r), esc);
            }
            let (r, esc) = bracketed(|| {
                topo::boolean::seated_pair_decided((topo::Seat::own(&case.a), fa), (seat, fb), band())
            });
            tally(&mut per, pair(r), esc);
        }
    }
    if moved.is_some() {
        lines.push(row("carrier pair (every face pair)", "(i) moved B", &up.show()));
    }
    lines.push(row("carrier pair (every face pair)", "(ii) at the carrier", &per.show()));
    // The whole boolean, (i) only: the kernel takes one frame.
    if let Some(m) = &moved {
        let (r, esc) = bracketed(|| {
            topo::boolean::boolean_op_with(
                case.op,
                &case.a,
                m,
                &topo::boolean::BooleanInputs::default(),
                topo::SweepStrategy::Realized,
                tol(),
            )
        });
        let out = match r {
            Ok(_) => "BUILT".to_owned(),
            Err(e) => {
                let s = format!("{e:?}");
                format!("REFUSED {}", s.chars().take(90).collect::<String>())
            }
        };
        lines.push(row("whole boolean", "(i)", &format!("{out} | esc {esc:?}")));
    }
    lines
}

fn pair(
    r: Result<Result<geom_brep::CarrierVerdict, geom_brep::CarrierEqError>, topo::boolean::PairUnread>,
) -> Result<(), String> {
    match r {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(geom_brep::CarrierEqError::Undecided { .. })) => Err("Undecided".to_owned()),
        Ok(Err(e)) => Err(format!("{e:?}").split([' ', '{', '(']).next().unwrap_or("").to_owned()),
        Err(topo::boolean::PairUnread::OutsideInventory) => Ok(()),
        Err(e) => Err(format!("{e:?}")),
    }
}

fn tally(t: &mut Tally, r: Result<(), String>, esc: BTreeMap<&'static str, usize>) {
    for (k, n) in esc {
        *t.escalated.entry(k).or_insert(0) += n;
    }
    match r {
        Ok(()) => t.decided += 1,
        Err(e) => {
            let key: String = e.chars().take(40).collect();
            *t.other.entry(key).or_insert(0) += 1;
        }
    }
}

const DELTAS_DEG: [f64; 5] = [1e-8, 1e-6, 1e-4, 1e-2, 1.0];

fn cases<T>() -> Vec<(Case<T>, (f64, f64, f64))>
where
    T: Decide + topo::AtRestPolicy,
{
    let fin = |what: &str, b: topo::Body<T>| topo::test_support::finished(what, b, tol());
    vec![
        // The turned disc and a clear copy: B about its own x through the
        // origin, then 3 along x, apart from A.
        (
            Case {
                name: "turned disc ∪ clear copy",
                a: fin("disc A", cylinder::<T>(1.0, 0.2, (0.0, 0.0, 0.0))),
                b: fin("disc B", cylinder::<T>(1.0, 0.2, (0.0, 0.0, 0.0))),
                op: topo::boolean::BooleanOp::Union,
            },
            (3.0, 0.0, 0.0),
        ),
        // A placed pocket: base − pocket, the pocket through the top face.
        (
            Case {
                name: "base − placed pocket",
                a: fin("base", slab::<T>(4.0, 4.0, 1.0)),
                b: fin("pocket", cylinder::<T>(0.5, 1.0, (0.0, 0.0, 0.0))),
                op: topo::boolean::BooleanOp::Subtract,
            },
            (2.0, 2.0, 0.5),
        ),
        // Touching: B stacked on A's top cap, coincident planes.
        (
            Case {
                name: "stacked on the top cap (coincident)",
                a: fin("block A", slab::<T>(1.0, 1.0, 1.0)),
                b: fin("block B", slab::<T>(1.0, 1.0, 1.0)),
                op: topo::boolean::BooleanOp::Union,
            },
            (0.0, 0.0, 1.0),
        ),
    ]
}

/// The motions for one δ at lane `T`: a turn of 30° ± δ about x then the
/// case's offset, and the case's offset ± δ·(π/180) along every axis.
fn motions<T: Decide>(
    lift: &impl Fn(&'static str, f64, f64) -> T,
    delta_deg: f64,
    at: (f64, f64, f64),
) -> Vec<(String, RigidMotion<T>)> {
    let l = T::from_f64;
    let d = delta_deg.to_radians();
    let deg30 = 30f64.to_radians();
    let axis = UnitVec3::new(Vec3::new(l(1.0), l(0.0), l(0.0)), "probe axis", band()).unwrap();
    let turn = RigidMotion::rotation(axis, lift("probe_turn", deg30 - d, deg30 + d)).unwrap();
    let off = RigidMotion::translation(Vec3::new(l(at.0), l(at.1), l(at.2))).unwrap();
    let turned = off.compose(turn).unwrap();
    let shift = RigidMotion::translation(Vec3::new(
        lift("probe_dx", at.0 - d, at.0 + d),
        lift("probe_dy", at.1 - d, at.1 + d),
        lift("probe_dz", at.2 - d, at.2 + d),
    ))
    .unwrap();
    vec![
        (format!("turn 30°±{delta_deg:e}°"), turned),
        (format!("shift ±{d:.1e} m"), shift),
    ]
}

#[test]
fn cross_placement_probe_interval() {
    let lift = |_: &'static str, lo: f64, hi: f64| Interval::from_bounds(lo, hi);
    for (case, at) in cases::<Interval>() {
        for &delta in &DELTAS_DEG {
            for (what, m) in motions(&lift, delta, at) {
                for line in probe("Interval", &what, &m, &case) {
                    eprintln!("PROBE {line}");
                }
            }
        }
    }
}

#[test]
fn cross_placement_probe_sym_interval() {
    let budget = SymBudget {
        max_terms: 4096,
        max_degree: 128,
    };
    geom_core::sym::with_session(budget, || {
        let lift = |name: &'static str, lo: f64, hi: f64| {
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id(name)),
                Interval::from_bounds(lo, hi),
                lo,
                hi,
            )
        };
        for (case, at) in cases::<Sym<Interval>>() {
            for &delta in &DELTAS_DEG {
                for (what, m) in motions(&lift, delta, at) {
                    for line in probe("Sym<Interval>", &what, &m, &case) {
                        eprintln!("PROBE {line}");
                    }
                }
            }
        }
    });
}
