//! **A shared row is certified per wall** (D2's structural selector, C2):
//! a rim that is a row on each of two spline walls' chart boundaries is
//! the curve known in advance, recovered as a
//! [`CurveSelector::Structural`] shared row and certified through each wall's
//! own row certificate, with no uniqueness tube.
//!
//! The walls are rings of revolution about `z`, as in
//! `spline_pair_arc_rim.rs`: a cylinder and a flat annulus meeting at
//! right angles along their shared unit circle, the `v = 0` row of each.
//! A witness over that rim refuses at limb 3 (that file's rows); the
//! shared row certifies.
//!
//! Every row reads the run's ε: the near-row carrier sits a fixed number
//! of bands off the row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::edge_nurbs::{pick_arc_rim, sub_arc_joint};
use geom_brep::keys::SurfaceKey;
use geom_brep::pcurve_cache::{Pcurve, PcurveCache};
use geom_brep::{
    CertifyError, CurveSelector, EdgeCurve, EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec,
    IntersectionDraft, NurbsLane, SharedRow, Structural, SurfacePair,
};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use slotmap::SlotMap;

/// The rim's turn on the open rings.
const TURN: f64 = 1.2;

/// A ring of revolution about `z` through `turn` from `x̂`, in `m`
/// rational quadratic sub-arcs (`u`), its `v` rows the profile points
/// `(radius, height)` of `profile`, joined linearly.
fn ring(m: usize, turn: f64, profile: [(f64, f64); 2]) -> Surface<f64> {
    let h = turn / m as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..m {
        ku.extend([sub_arc_joint(k, m); 2]);
    }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * m {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for (r, z) in profile {
            let r = r * reach;
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(ku, 2).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            weights,
        )
        .unwrap(),
    ))
}

/// The unit cylinder `z ∈ [0, 1]`: its `v = 0` row is the rim.
fn cylinder(m: usize, turn: f64) -> Surface<f64> {
    ring(m, turn, [(1.0, 0.0), (1.0, 1.0)])
}

/// The annulus `r ∈ [1, 2]` in `z = 0`: its `v = 0` row is the rim.
fn annulus(m: usize, turn: f64) -> Surface<f64> {
    ring(m, turn, [(1.0, 0.0), (2.0, 0.0)])
}

/// The annulus `r ∈ [1, 2]` in `z = 0` run inward: its `v = 1` row is the
/// rim, so its row image is not the cylinder's.
fn outward_annulus(m: usize, turn: f64) -> Surface<f64> {
    ring(m, turn, [(2.0, 0.0), (1.0, 0.0)])
}

/// The circle of radius `r` about `z` in `z = 0`, `t` its angle from `x̂`.
fn rim(r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// `wall`'s row image of `carrier` over `(t0, t1)`, as the walls' row pick
/// finds it.
fn row(carrier: &Curve3<f64>, (t0, t1): (f64, f64), wall: &Surface<f64>) -> Pcurve<f64> {
    pick_arc_rim(carrier, (t0, t1 - t0), wall, band())
        .expect("the pick decides")
        .expect("the rim is a row of the wall")
}

/// Two walls under keys, `first` inserted first so its key is the lower,
/// and the plane `z = 0` every rim here lies in.
struct Pair {
    arena: SlotMap<SurfaceKey, Surface<f64>>,
    keys: [SurfaceKey; 2],
    plane: SurfaceKey,
}

impl Pair {
    fn of(first: Surface<f64>, second: Surface<f64>) -> Self {
        let mut arena = SlotMap::with_key();
        let keys = [arena.insert(first), arena.insert(second)];
        let plane = arena.insert(Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        });
        Self { arena, keys, plane }
    }

    fn pair(&self) -> SurfacePair {
        SurfacePair::new(self.keys[0], self.keys[1])
    }

    fn wall(&self, i: usize) -> &Surface<f64> {
        &self.arena[self.keys[i]]
    }

    /// The edge `selector` picks over `carrier` on `params`, certified.
    fn certify(
        &self,
        selector: CurveSelector<f64>,
        carrier: &Curve3<f64>,
        (t0, t1): (f64, f64),
    ) -> Result<EdgeCurve<f64>, CertifyError> {
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: self.pair(),
                selector,
            },
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        };
        EdgeCurve::certify(
            spec,
            carrier.eval(t0),
            carrier.eval(t1),
            |k| self.arena.get(k).cloned(),
            1.0,
            band(),
            NurbsLane::certified(),
        )
    }

    /// D2's question asked of an edge over `carrier` on `params`, each
    /// wall's image `images[i]`: the draft is the edge certified on the
    /// plane `z = 0` it lies in, its carrier kept.
    fn recover(
        &self,
        carrier: &Curve3<f64>,
        params: (f64, f64),
        images: [Pcurve<f64>; 2],
    ) -> Option<CurveSelector<f64>> {
        self.recover_on(
            EdgeDescriptionSpec::chart(self.plane),
            carrier,
            params,
            images,
        )
    }

    /// [`Pair::recover`] with the draft's edge certified as `description`.
    fn recover_on(
        &self,
        description: EdgeDescriptionSpec<f64>,
        carrier: &Curve3<f64>,
        (t0, t1): (f64, f64),
        images: [Pcurve<f64>; 2],
    ) -> Option<CurveSelector<f64>> {
        let spec = EdgeCurveSpec {
            description,
            carrier: carrier.clone(),
            param_start: t0,
            param_end: t1,
        };
        let (p0, p1) = (carrier.eval(t0), carrier.eval(t1));
        let edge = EdgeCurve::certify(
            spec,
            p0,
            p1,
            |k| self.arena.get(k).cloned(),
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the draft's edge certifies");
        let [ia, ib] = images;
        IntersectionDraft::of(Some(&edge), p0, p1).shared_row(
            self.pair(),
            [
                (self.keys[0], self.wall(0), ia),
                (self.keys[1], self.wall(1), ib),
            ],
            band(),
        )
    }
}

/// The structural selector stating `images[i]` on `pair.keys[i]`.
fn shared(pair: &Pair, images: [Pcurve<f64>; 2]) -> CurveSelector<f64> {
    let [ia, ib] = images;
    let row = SharedRow::new(pair.pair(), (pair.keys[0], ia), (pair.keys[1], ib))
        .expect("two row images on the pair's two walls");
    CurveSelector::Structural(Structural::SharedRow(row))
}

/// The largest of each wall's row certificate numbers: the residual at
/// its samples and its envelope.
fn wall_bound(pair: &Pair, carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> f64 {
    (0..2)
        .map(|i| {
            let image = row(carrier, (t0, t1), pair.wall(i));
            let cache = PcurveCache::certify(image, t0, t1, carrier, pair.wall(i), band())
                .expect("each wall's row certifies");
            let c = cache.certificate();
            c.max_residual.abs().max(c.envelope.abs())
        })
        .fold(0.0, f64::max)
}

/// **The rim is a shared row, and it certifies per wall.** In both key
/// orders, over the whole turn and a sub-span, on one and on four
/// sub-arcs: D2's question recovers the structural selector, and the
/// edge it describes certifies, recording each wall's row certificate
/// numbers as its residual bound.
#[test]
fn a_rim_on_both_walls_rows_is_a_certified_shared_row() {
    let carrier = rim(1.0);
    for m in [1, 4] {
        for (what, pair) in [
            (
                "cylinder first",
                Pair::of(cylinder(m, TURN), annulus(m, TURN)),
            ),
            (
                "annulus first",
                Pair::of(annulus(m, TURN), cylinder(m, TURN)),
            ),
            (
                "outward annulus",
                Pair::of(cylinder(m, TURN), outward_annulus(m, TURN)),
            ),
        ] {
            for params in [(0.0, TURN), (0.3, 0.9)] {
                let what = format!("{what}, m = {m}, {params:?}");
                let images = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
                let selector = pair
                    .recover(&carrier, params, images.clone())
                    .unwrap_or_else(|| panic!("{what}: the rim is recovered as a shared row"));
                assert!(
                    matches!(selector, CurveSelector::Structural(_)),
                    "{what}: structural"
                );
                let edge = pair
                    .certify(selector, &carrier, params)
                    .unwrap_or_else(|e| panic!("{what}: the shared row certifies: {e}"));
                let bound = wall_bound(&pair, &carrier, params);
                assert!(bound > 0.0, "{what}: the walls' rows bound a real residual");
                assert!(
                    edge.certificate().max_residual >= bound,
                    "{what}: the edge's residual bound {:e} reads each wall's row certificate \
                     ({bound:e})",
                    edge.certificate().max_residual
                );
                assert!(edge.certificate().max_residual <= eps(), "{what}: within ε");
                let EdgeDescription::Intersection {
                    selector: CurveSelector::Structural(Structural::SharedRow(row)),
                    ..
                } = edge.description()
                else {
                    panic!("{what}: the certified description keeps the shared row");
                };
                for (i, image) in images.iter().enumerate() {
                    assert_eq!(
                        format!("{:?}", row.image_on(pair.pair(), pair.keys[i])),
                        format!("{:?}", Some(image)),
                        "{what}: wall {i} keeps its own row"
                    );
                }
            }
        }
    }
}

/// **The pair's order does not decide the row.** On the cylinder and an
/// annulus whose rim is its `v = 1` row, so the two images differ. Stated in either
/// argument order, the shared row holds each image against its own wall;
/// re-keyed so the pair's order flips, the edge still certifies, its
/// images following their walls. A row whose images answer for the
/// other wall refuses, on the wall whose row it is not.
#[test]
fn a_shared_row_is_symmetric_in_its_pair() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let pair = Pair::of(cylinder(1, TURN), outward_annulus(1, TURN));
    let [ia, ib] = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
    let forward = SharedRow::new(
        pair.pair(),
        (pair.keys[0], ia.clone()),
        (pair.keys[1], ib.clone()),
    );
    let backward = SharedRow::new(
        pair.pair(),
        (pair.keys[1], ib.clone()),
        (pair.keys[0], ia.clone()),
    );
    assert_eq!(
        format!("{forward:?}"),
        format!("{backward:?}"),
        "argument order is not the row"
    );
    let edge = pair
        .certify(shared(&pair, [ia.clone(), ib.clone()]), &carrier, params)
        .expect("the shared row certifies");

    // Re-key into an arena whose order flips: the annulus first.
    let flipped = Pair::of(outward_annulus(1, TURN), cylinder(1, TURN));
    let bridge = |k: SurfaceKey| {
        Some(if k == pair.keys[0] {
            flipped.keys[1]
        } else {
            flipped.keys[0]
        })
    };
    assert!(
        pair.keys[0] < pair.keys[1] && flipped.keys[0] < flipped.keys[1],
        "the bridge flips the pair's key order"
    );
    let moved = edge.with_remapped_surfaces(bridge).expect("both keys map");
    let EdgeDescription::Intersection {
        pair: moved_pair,
        selector,
    } = moved.description()
    else {
        panic!("still an intersection");
    };
    assert_eq!(*moved_pair, flipped.pair());
    flipped
        .certify(selector.clone(), &carrier, params)
        .expect("re-keyed, each image still answers for its own wall");

    // The images swapped against their walls: the cylinder's row stated
    // on the annulus refuses on the wall whose row it is not.
    let swapped = SharedRow::new(pair.pair(), (pair.keys[0], ib), (pair.keys[1], ia)).unwrap();
    let got = pair.certify(
        CurveSelector::Structural(Structural::SharedRow(swapped)),
        &carrier,
        params,
    );
    assert!(
        matches!(
            got,
            Err(CertifyError::RowImage { .. } | CertifyError::Escalated { .. })
        ),
        "a row stated on the wrong wall refuses: {got:?}"
    );
}

/// **A rim a few bands off the row is no shared row.** Two hundred ε
/// outside the unit circle, a score of escalation bands, the rim is no row of the cylinder: D2's
/// question finds no shared row, so the boolean keeps today's witness,
/// and a shared row stated for it anyway refuses typed on the wall
/// whose row it is not.
#[test]
fn a_rim_off_the_row_is_no_shared_row() {
    let params = (0.0, TURN);
    let pair = Pair::of(cylinder(1, TURN), annulus(1, TURN));
    let on = rim(1.0);
    let images = [0, 1].map(|i| row(&on, params, pair.wall(i)));
    let off = rim(1.0 + 200.0 * eps());
    assert!(
        pair.recover(&off, params, images.clone()).is_none(),
        "a rim off the cylinder's row reads no shared row"
    );
    let got = pair.certify(shared(&pair, images), &off, params);
    assert!(
        matches!(got, Err(CertifyError::RowImage { surface, .. }) if surface == pair.keys[0]),
        "the cylinder refuses the off rim, definitely: {got:?}"
    );
}

/// The bilinear patch through `corners`, `[(u₀, v₀), (u₀, v₁), (u₁, v₀),
/// (u₁, v₁)]`.
fn bilinear(corners: [Point3<f64>; 4]) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(k.clone(), k, corners.to_vec(), vec![1.0; 4]).unwrap(),
    ))
}

/// **A line row: on both walls' sides, a shared row; inside one wall, no
/// rim.** A straight degree-1 spline carrier (a line carrier is a chord to
/// the boolean, which states no row over one). The floor `z = 0` over `[0, 1]²` (its chart `u = x`, `v = y`)
/// and an upright wall in the plane `x = c`, rising from the floor (its
/// chart `u = y`, `v = z`), meet along the line `x = c` on the floor, the
/// upright's `v = 0` row and the floor's `u = c` column. At `c = 1` that
/// column is the floor's side, and the line is recovered as a shared row
/// and certifies. At `c = ½` the column is inside the floor: it certifies
/// there, but it is no domain side of the chart, so D2's question, as the
/// boolean asks it of a rim, finds no shared row.
#[test]
fn a_line_row_is_shared_only_on_both_walls_sides() {
    let p = |x, y, z| Point3::new(x, y, z);
    let floor = bilinear([
        p(0.0, 0.0, 0.0),
        p(0.0, 1.0, 0.0),
        p(1.0, 0.0, 0.0),
        p(1.0, 1.0, 0.0),
    ]);
    let params = (0.0, 1.0);
    for (c, side) in [(1.0, true), (0.5, false)] {
        let upright = bilinear([
            p(c, 0.0, 0.0),
            p(c, 0.0, 1.0),
            p(c, 1.0, 0.0),
            p(c, 1.0, 1.0),
        ]);
        let pair = Pair::of(floor.clone(), upright);
        let line = Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![p(c, 0.0, 0.0), p(c, 1.0, 0.0)],
                vec![1.0, 1.0],
            )
            .unwrap(),
        ));
        let images = [
            Pcurve::IsoLine {
                p0: geom_core::Point2::new(c, 0.0),
                pl: geom_core::Vec2::new(0.0, 1.0),
            },
            Pcurve::IsoLine {
                p0: geom_core::Point2::new(0.0, 0.0),
                pl: geom_core::Vec2::new(1.0, 0.0),
            },
        ];
        for (i, image) in images.iter().enumerate() {
            PcurveCache::certify(
                image.clone(),
                params.0,
                params.1,
                &line,
                pair.wall(i),
                band(),
            )
            .unwrap_or_else(|e| panic!("c = {c}: wall {i}'s row is the line: {e:?}"));
        }
        // The draft's edge, as the upright's side row states it.
        let upright = EdgeDescriptionSpec::chart_image(pair.keys[1], images[1].clone());
        let got = pair.recover_on(upright, &line, params, images.clone());
        assert_eq!(
            got.is_some(),
            side,
            "c = {c}: a shared row exactly where the line is a side of both charts"
        );
        if side {
            pair.certify(got.unwrap(), &line, params)
                .expect("the line on both walls' sides certifies as their shared row");
        }
    }
}

/// **The sheet: a closed chart.** On rings that close (a whole turn), a
/// rim across the closure line lies on no single arc of a row, and a row
/// shifted a period along the carrier states another arc: neither is
/// recovered as a shared row, and a shifted row stated anyway refuses.
#[test]
fn a_rim_across_a_closed_charts_closure_is_no_shared_row() {
    let tau = std::f64::consts::TAU;
    let carrier = rim(1.0);
    let pair = Pair::of(cylinder(4, tau), annulus(4, tau));

    // Across the closure line at `u = 0`: no row of either wall holds it,
    // and the whole turn's row, stated over it, is no shared row.
    let across = (tau - 0.3, tau + 0.3);
    for i in 0..2 {
        let picked = pick_arc_rim(
            &carrier,
            (across.0, across.1 - across.0),
            pair.wall(i),
            band(),
        );
        assert!(
            matches!(picked, Ok(None)),
            "wall {i}: no row holds a rim across the closure line: {picked:?}"
        );
    }
    let whole = [0, 1].map(|i| row(&carrier, (0.0, tau), pair.wall(i)));
    pair.certify(shared(&pair, whole.clone()), &carrier, (0.0, tau))
        .expect("the whole turn is the walls' shared row");
    assert!(
        pair.recover(&carrier, across, whole).is_none(),
        "a rim across the closure line reads no shared row"
    );

    // A period-shifted row: the image of `(0.2, 1.0)` with its arc moved
    // a whole turn along the carrier.
    let inside = (0.2, 1.0);
    let images = [0, 1].map(|i| row(&carrier, inside, pair.wall(i)));
    pair.certify(shared(&pair, images.clone()), &carrier, inside)
        .expect("inside one arc, the shared row certifies");
    let shifted = images.map(|image| match image {
        Pcurve::IsoArc {
            p0,
            pd,
            t0,
            angle,
            breaks,
        } => Pcurve::IsoArc {
            p0,
            pd,
            t0: t0 + tau,
            angle,
            breaks,
        },
        other => panic!("an arc row: {other:?}"),
    });
    assert!(
        pair.recover(&carrier, inside, shifted.clone()).is_none(),
        "a period-shifted row reads no shared row"
    );
    let got = pair.certify(shared(&pair, shifted), &carrier, inside);
    assert!(
        matches!(
            got,
            Err(CertifyError::RowImage { .. } | CertifyError::Escalated { .. })
        ),
        "a period-shifted row refuses: {got:?}"
    );
}

/// **A shared row still owes transversality.** Two coaxial cylinders
/// meeting end to end share the unit circle as a row of each, smoothly:
/// each row certifies, and the edge refuses, the walls not definitely
/// crossing along it.
#[test]
fn a_shared_row_between_tangent_walls_refuses_as_not_transverse() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let below = ring(1, TURN, [(1.0, 0.0), (1.0, -1.0)]);
    let pair = Pair::of(cylinder(1, TURN), below);
    let images = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
    let got = pair.certify(shared(&pair, images), &carrier, params);
    assert!(
        matches!(
            got,
            Err(CertifyError::NotTransverse { .. } | CertifyError::Escalated { .. })
        ),
        "tangent walls are not definitely transverse along the row: {got:?}"
    );
}

// ---- review r1 (PR 5959) counterexample rows: each is red at 52b56438 ----

fn k2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// The floor `z = 0` over `[0, 1]²` (degree 2 along `y`), the line
/// `x = 1` on it (its `u = 1` side), and an upright on that line whose top
/// row is `(lean, y, h(y))`, `h(y) = (y - y*)²`: at `y*` the upright's
/// ruling lies in `z = 0`, so the walls are tangent there.
fn tangent_at(ys: f64, lean: f64) -> (Pair, Curve3<f64>, [Pcurve<f64>; 2]) {
    let p = |x, y, z| Point3::new(x, y, z);
    let floor = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            k2(),
            [0.0, 1.0]
                .into_iter()
                .flat_map(|x| [0.0, 0.5, 1.0].map(|y| p(x, y, 0.0)))
                .collect(),
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    let h = [ys * ys, ys * ys - ys, (1.0 - ys) * (1.0 - ys)];
    let mut control = Vec::new();
    for (j, y) in [0.0, 0.5, 1.0].into_iter().enumerate() {
        control.push(p(1.0, y, 0.0));
        control.push(p(lean, y, h[j]));
    }
    let upright = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            k2(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    let line = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            k2(),
            vec![p(1.0, 0.0, 0.0), p(1.0, 0.5, 0.0), p(1.0, 1.0, 0.0)],
            vec![1.0; 3],
        )
        .unwrap(),
    ));
    let images = [
        Pcurve::IsoLine {
            p0: geom_core::Point2::new(1.0, 0.0),
            pl: geom_core::Vec2::new(0.0, 1.0),
        },
        Pcurve::IsoLine {
            p0: geom_core::Point2::new(0.0, 0.0),
            pl: geom_core::Vec2::new(1.0, 0.0),
        },
    ];
    (Pair::of(floor, upright), line, images)
}

/// **Claim 2 (class a).** Walls tangent between the transversality
/// samples (`y* = 0.19`, between 0.125 and 0.25) or at an end (never
/// sampled). With `lean = 0` the upright folds back over the floor and its
/// ruling at `y*` lies inside both charts: a second branch of the pair, a
/// unit long, crossing the rim. The witness lane refuses every case (its
/// tube's crossing-angle bound); the shared row certifies all but the
/// sampled one, and D2's question recovers it.
#[test]
fn r1_a_shared_row_between_walls_tangent_off_the_samples_refuses() {
    for (ys, lean) in [(0.19, 0.0), (0.19, 2.0), (0.0, 0.0), (1.0, 0.0), (0.5, 0.0)] {
        let (pair, line, images) = tangent_at(ys, lean);
        let witness = pair.certify(CurveSelector::Witness(line.eval(0.5)), &line, (0.0, 1.0));
        assert!(witness.is_err(), "y* = {ys}: the witness lane refuses");
        let got = pair.certify(shared(&pair, images), &line, (0.0, 1.0));
        assert!(
            got.is_err(),
            "y* = {ys}, lean {lean}: walls tangent on the row certify as a transverse shared row"
        );
    }
}

/// **Claims 3 and (c).** A floor creased along its interior column
/// `u = ½`, one sheet folded up to within `0.1 ε` of the upright `x = ½`:
/// the certifier, which (unlike recovery) admits an interior row, reads
/// one span's derivative at the crease, so its verdict depends on which
/// side the hugging sheet is on. Mirror images must get one verdict.
#[test]
fn r1_a_creased_interior_row_reads_one_verdict_for_mirror_images() {
    let p = |x, y, z| Point3::new(x, y, z);
    let upright = bilinear([
        p(0.5, 0.0, 0.0),
        p(0.5, 0.0, 1.0),
        p(0.5, 1.0, 0.0),
        p(0.5, 1.0, 1.0),
    ]);
    let line = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![p(0.5, 0.0, 0.0), p(0.5, 1.0, 0.0)],
            vec![1.0, 1.0],
        )
        .unwrap(),
    ));
    let images = [
        Pcurve::IsoLine {
            p0: geom_core::Point2::new(0.5, 0.0),
            pl: geom_core::Vec2::new(0.0, 1.0),
        },
        Pcurve::IsoLine {
            p0: geom_core::Point2::new(0.0, 0.0),
            pl: geom_core::Vec2::new(1.0, 0.0),
        },
    ];
    let s = 0.1 * eps();
    let verdict = |left_up: bool| {
        let col = |x: f64, z: f64| [p(x, 0.0, z), p(x, 1.0, z)];
        let rows = if left_up {
            [col(0.5 - s, 1.0), col(0.5, 0.0), col(1.0, 0.0)]
        } else {
            [col(0.0, 0.0), col(0.5, 0.0), col(0.5 + s, 1.0)]
        };
        let creased = Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap(),
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                rows.iter().flatten().copied().collect(),
                vec![1.0; 6],
            )
            .unwrap(),
        ));
        let pair = Pair::of(creased, upright.clone());
        pair.certify(shared(&pair, images.clone()), &line, (0.0, 1.0))
            .is_ok()
    };
    assert_eq!(
        verdict(false),
        verdict(true),
        "the crease's verdict depends on the span read"
    );
}
