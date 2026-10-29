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
    IntersectionDraft, NurbsLane, RowRefusal, SharedRow, Structural, SurfacePair,
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
            Err(CertifyError::Row { surface, refusal: RowRefusal::Image(_) }) if surface == pair.keys[0]
        ),
        "a row stated on the wrong wall refuses: {got:?}"
    );
}

/// **The span is gated per wall.** The cylinder turns 1.4 rad, the
/// annulus 1.2: over `(0, 1.3)` the rim runs past the annulus's row. D2's
/// question finds no shared row, and the annulus's whole row stated over
/// that span refuses, on the annulus.
#[test]
fn a_carrier_past_one_walls_row_refuses_on_that_wall() {
    let carrier = rim(1.0);
    let pair = Pair::of(cylinder(1, 1.4), annulus(1, TURN));
    let past = (0.0, 1.3);
    let images = [
        row(&carrier, past, pair.wall(0)),
        row(&carrier, (0.0, TURN), pair.wall(1)),
    ];
    assert!(
        pair.recover(&carrier, past, images.clone()).is_none(),
        "a rim past the annulus's row reads no shared row"
    );
    let got = pair.certify(shared(&pair, images), &carrier, past);
    assert!(
        matches!(got, Err(CertifyError::Row { surface, refusal: RowRefusal::Image(_) }) if surface == pair.keys[1]),
        "the annulus refuses the span past its row, definitely: {got:?}"
    );
}

/// **Both operand orders recover one description.** The cylinder and the
/// annulus whose rim is its `v = 1` row, keyed in either order: each
/// wall's row is the same image whichever order recovered it, and the
/// first order's edge, re-keyed into the second's arena (which flips the
/// pair's order), states each wall's row exactly as the second recovered.
#[test]
fn both_operand_orders_recover_one_description() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let forward = Pair::of(cylinder(1, TURN), outward_annulus(1, TURN));
    let backward = Pair::of(outward_annulus(1, TURN), cylinder(1, TURN));
    let recovered = |pair: &Pair| {
        let images = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
        let selector = pair
            .recover(&carrier, params, images)
            .expect("a shared row");
        pair.certify(selector, &carrier, params)
            .expect("it certifies")
    };
    let rows = |pair: &Pair, edge: &EdgeCurve<f64>, cylinder: SurfaceKey, annulus: SurfaceKey| {
        let EdgeDescription::Intersection {
            pair: stated,
            selector: CurveSelector::Structural(Structural::SharedRow(row)),
        } = edge.description()
        else {
            panic!("a shared row");
        };
        assert_eq!(*stated, pair.pair());
        [cylinder, annulus].map(|k| format!("{:?}", row.image_on(*stated, k)))
    };
    let (a, b) = (recovered(&forward), recovered(&backward));
    let want = rows(&backward, &b, backward.keys[1], backward.keys[0]);
    assert_eq!(
        rows(&forward, &a, forward.keys[0], forward.keys[1]),
        want,
        "each wall's row, whichever order recovered it"
    );
    let bridge = |k: SurfaceKey| {
        Some(if k == forward.keys[0] {
            backward.keys[1]
        } else {
            backward.keys[0]
        })
    };
    let moved = a.with_remapped_surfaces(bridge).expect("both keys map");
    assert_eq!(
        rows(&backward, &moved, backward.keys[1], backward.keys[0]),
        want,
        "re-keyed across the flip, each wall keeps its own row"
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
        matches!(got, Err(CertifyError::Row { surface, refusal: RowRefusal::Image(_) }) if surface == pair.keys[0]),
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
            Err(CertifyError::Row { surface, refusal: RowRefusal::Image(None) }) if surface == pair.keys[0]
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
        matches!(got, Err(CertifyError::NotTransverse { sample: 1, .. })),
        "tangent walls are not definitely transverse along the row: {got:?}"
    );
}

/// A degree-2 clamped knot vector on `[0, 1]` with no interior knot.
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

/// **A crossing angle that vanishes between the samples or at an end
/// refuses.** The upright meets the floor along `x = 1` and leans
/// `(y − y*)²` toward it, so the walls are tangent at `y*` alone: between
/// the transversality samples (`y* = 0.19`), at either end, or on a sample
/// (`y* = ½`); with `lean = 0` the upright folds back over the floor and
/// its ruling at `y*` is a second branch crossing the rim. The cover of
/// the row reads every one: where its least margin is definitely zero it
/// refuses `NotTransverse`, where a piece cannot be read zero-free it
/// escalates, and a sample on the tangency refuses there first.
#[test]
fn a_shared_row_whose_walls_touch_off_its_samples_refuses() {
    for (ys, lean, escalates) in [
        (0.19, 0.0, true),
        (0.19, 2.0, true),
        (0.0, 0.0, true),
        (1.0, 0.0, true),
        (0.5, 0.0, false),
    ] {
        let (pair, line, images) = tangent_at(ys, lean);
        let got = pair.certify(shared(&pair, images), &line, (0.0, 1.0));
        let read = match got {
            Err(CertifyError::Escalated {
                check: geom_brep::CertCheck::Transversality,
                ..
            }) => Some(true),
            Err(CertifyError::NotTransverse { .. }) => Some(false),
            _ => None,
        };
        assert_eq!(
            read,
            Some(escalates),
            "y* = {ys}, lean {lean}: walls touching on the row refuse their crossing: {got:?}"
        );
    }
}

/// The floor and an upright on its `x = 1` side whose `v` direction is
/// `(1, 0, c·(y − y*)² + δ)`: the walls cross along the whole row, at an
/// angle of about `δ` at `y*` and opening quadratically, at rate `c`,
/// away from it.
fn dipping_at(ys: f64, c: f64, delta: f64) -> (Pair, Curve3<f64>, [Pcurve<f64>; 2]) {
    let (pair, line, images) = tangent_at(ys, 2.0);
    let p = |x, y, z| Point3::new(x, y, z);
    let h = [ys * ys, ys * ys - ys, (1.0 - ys) * (1.0 - ys)].map(|q| c * q + delta);
    let mut control = Vec::new();
    for (j, y) in [0.0, 0.5, 1.0].into_iter().enumerate() {
        control.push(p(1.0, y, 0.0));
        control.push(p(2.0, y, h[j]));
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
    (Pair::of(pair.wall(0).clone(), upright), line, images)
}

/// **A crossing whose least bound lies in the band escalates; it is not a
/// definite zero.** The walls dip to an angle of about `0.3 ε` at
/// `y* = 0.19`, between the transversality samples, where every sample
/// reads them clear of the band. The cover reads every piece there
/// zero-free and positive, with no sign change: its least margin is a
/// lower bound in the band, which tells no tangency from a crossing too
/// shallow to read, so it escalates.
#[test]
fn a_crossing_bounded_in_the_band_with_no_turn_escalates() {
    let e = eps();
    let (pair, line, images) = dipping_at(0.19, 2e6 * e, 0.3 * e);
    let got = pair.certify(shared(&pair, images), &line, (0.0, 1.0));
    assert!(
        matches!(
            got,
            Err(CertifyError::Escalated {
                check: geom_brep::CertCheck::Transversality,
                sample: geom_brep::certify::NOT_A_SAMPLE,
                ..
            })
        ),
        "a dip into the band between samples escalates at no sample: {got:?}"
    );
}

/// **A sign change across a crease is no tangency.** The row is a
/// polyline that turns 135° at its knot `t = ½`, the `v = 0` row of two
/// degree-1 walls on that knot: a floor in `z = 0` whose cross-row edges
/// all run along `−y` (so it folds back over itself past the corner) and
/// a vertical upright. The walls cross at right angles along all of it,
/// but past the corner the floor's chart turns over, and the cover's
/// margin changes sign across the knot. The partials jump there, so the
/// sign change says nothing about the tangent planes, and the row
/// certifies.
#[test]
fn a_sign_change_across_a_crease_is_no_tangency() {
    let p = |x, y, z| Point3::new(x, y, z);
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let k1 = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let row = [p(0.0, 0.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0)];
    let wall = |cross: Vec3<f64>| {
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(
                k(),
                k1(),
                row.iter().flat_map(|&r| [r, r + cross]).collect(),
                vec![1.0; 6],
            )
            .unwrap(),
        ))
    };
    let line = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(k(), row.to_vec(), vec![1.0; 3]).unwrap(),
    ));
    let image = || Pcurve::IsoLine {
        p0: geom_core::Point2::new(0.0, 0.0),
        pl: geom_core::Vec2::new(1.0, 0.0),
    };
    let pair = Pair::of(
        wall(Vec3::new(0.0, -1.0, 0.0)),
        wall(Vec3::new(0.0, 0.0, 1.0)),
    );
    let got = pair.certify(shared(&pair, [image(), image()]), &line, (0.0, 1.0));
    assert!(
        got.is_ok(),
        "walls crossing at right angles certify across the crease: {got:?}"
    );
}

/// A degree-2 clamped knot vector on `[s, s + 1]` with no interior knot.
fn k2_at(s: f64) -> KnotVector {
    KnotVector::clamped(vec![s, s, s, s + 1.0, s + 1.0, s + 1.0], 2).unwrap()
}

/// The floor `z = 0` over `x, y ∈ [0, 1]` and an upright on its `x = 1`
/// side whose `v` direction is `(1, 0, y − y*)`: the walls are tangent at
/// `y*` alone, and the upright crosses the floor's plane there (its
/// crossing angle linear in `y − y*`, as on lily's edge 9). Both walls'
/// `y` coordinate, and the line's parameter, run over `[s, s + 1]`: the
/// same geometry, its charts read away from 0.
fn swapping_at(ys: f64, s: f64) -> (Pair, Curve3<f64>, [Pcurve<f64>; 2]) {
    let p = |x, y, z| Point3::new(x, y, z);
    let floor = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            k2_at(s),
            [0.0, 1.0]
                .into_iter()
                .flat_map(|x| [0.0, 0.5, 1.0].map(|y| p(x, y, 0.0)))
                .collect(),
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    let mut control = Vec::new();
    for y in [0.0, 0.5, 1.0] {
        control.push(p(1.0, y, 0.0));
        control.push(p(2.0, y, y - ys));
    }
    let upright = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            k2_at(s),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            vec![1.0; 6],
        )
        .unwrap(),
    ));
    let line = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            k2_at(s),
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

/// **A crossing that vanishes where a one-ulp nudge would skip is still
/// read** (the PR 5959 re-review's row). Walls tangent at the row's start,
/// at its end, and at a halving join (`y* = 3/16`, no sample), with both
/// charts' row coordinate on `[s, s + 1]`: the same geometry as at `s = 0`
/// (where every case refuses), but one ulp of the chart coordinate is now
/// `s·2⁻⁵²`, wide enough that the margin one ulp inside an end clears the
/// band. `s` is the least power of two whose ulp is `1000 ε`.
#[test]
fn rr_a_tangency_the_ulp_nudge_skips_refuses() {
    let s = 2f64.powi(52 + (1000.0 * eps()).log2().ceil() as i32);
    for ys in [0.0, 1.0, 0.1875] {
        let at_zero = {
            let (pair, line, images) = swapping_at(ys, 0.0);
            pair.certify(shared(&pair, images), &line, (0.0, 1.0))
        };
        assert!(at_zero.is_err(), "y* = {ys}, s = 0: refuses");
        let (pair, line, images) = swapping_at(ys, s);
        let got = pair.certify(shared(&pair, images), &line, (s, s + 1.0));
        assert!(
            got.is_err(),
            "y* = {ys}, s = {s}: walls tangent on the row certify as transverse"
        );
    }
}

/// **The cover reads the same verdict wherever the charts sit.** The
/// swapping walls, their row coordinate on `[s, s + 1]` for `s` from 0 to
/// the least power of two whose ulp is `1000 ε`: tangent inside the row,
/// at a halving join (`y* = 3/16`) or between joins (`y* = 0.3`), the
/// walls swap sides and the cover reads the turn, definitely; tangent at
/// an end, the crossing's lower bound reaches the band with no sign change
/// to read, and escalates; tangent off the row (`y* = −1`, `y* = 2`), the
/// walls cross along all of it and the row certifies.
#[test]
fn a_shared_row_reads_one_verdict_wherever_its_charts_sit() {
    let far = 2f64.powi(52 + (1000.0 * eps()).log2().ceil() as i32);
    for s in [0.0, 1.0, 1024.0, 2f64.powi(30), far] {
        for (ys, want) in [
            (0.1875, "NotTransverse"),
            (0.3, "NotTransverse"),
            (0.0, "Escalated"),
            (1.0, "Escalated"),
            (-1.0, "Ok"),
            (2.0, "Ok"),
        ] {
            let (pair, line, images) = swapping_at(ys, s);
            let got = pair.certify(shared(&pair, images), &line, (s, s + 1.0));
            let read = match &got {
                Ok(_) => "Ok",
                Err(CertifyError::NotTransverse { .. }) => "NotTransverse",
                Err(CertifyError::Escalated {
                    check: geom_brep::CertCheck::Transversality,
                    ..
                }) => "Escalated",
                Err(_) => "other",
            };
            assert_eq!(read, want, "y* = {ys}, s = {s}: {got:?}");
        }
    }
}

/// **An interior row refuses, whichever way a crease beside it folds.** A
/// floor creased along its interior column `u = ½`, one sheet folded up to
/// within `0.1 ε` of the upright `x = ½`, on either side: the shared row
/// is built for boundary rows, so both mirror images refuse
/// `CertifyError::Row { refusal: RowRefusal::OffSide, .. }` on the floor, before any reading that
/// could tell the two sides apart.
#[test]
fn an_interior_row_refuses_whichever_way_its_crease_folds() {
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
    for left_up in [false, true] {
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
        let got = pair.certify(shared(&pair, images.clone()), &line, (0.0, 1.0));
        assert!(
            matches!(got, Err(CertifyError::Row { surface, refusal: RowRefusal::OffSide }) if surface == pair.keys[0]),
            "left up {left_up}: the floor's interior column is no side: {got:?}"
        );
    }
}

/// The flange rings' sub-arcs.
const M: usize = 24;

/// A ring about `z` through `TURN` in `M` rational quadratic sub-arcs:
/// its `v = 0` row the unit circle, its `v = 1` row column `j`'s control
/// point plus `offset(j)`.
fn flange_ring(offset: impl Fn(usize, Vec3<f64>) -> Vec3<f64>) -> Surface<f64> {
    let h = TURN / M as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..M {
        ku.extend([sub_arc_joint(k, M); 2]);
    }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * M {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        let a = Point3::new(reach * theta.cos(), reach * theta.sin(), 0.0);
        let radial = Vec3::new(reach * theta.cos(), reach * theta.sin(), 0.0);
        control.push(a);
        control.push(a + offset(j, radial));
        weights.extend([w, w]);
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

/// The cylinder `z ∈ [0, 1]` and a flange running outward in `z = 0`
/// except on sub-arc 4 (carrier `t ∈ [0.2, 0.25]`), where its columns
/// 8–10 turn straight down: there `∂S/∂v = −ẑ`, so the flange's normal is
/// the cylinder's and the walls are tangent along the rim on that whole
/// sub-span. The edge over the whole turn, its selector their shared row.
fn tangent_along_a_sub_span() -> Result<EdgeCurve<f64>, CertifyError> {
    let cylinder = flange_ring(|_, _| Vec3::new(0.0, 0.0, 1.0));
    let flange = flange_ring(|j, radial| {
        if (8..=10).contains(&j) {
            Vec3::new(0.0, 0.0, -1.0)
        } else {
            radial
        }
    });
    let Surface::Nurbs(f) = &flange else {
        unreachable!("a ring is a spline wall")
    };
    let d = f.ders((sub_arc_joint(4, M) + sub_arc_joint(5, M)) / 2.0, 0.0);
    let n = d.du.cross(d.dv);
    assert!(
        (n * (1.0 / n.norm())).z.abs() < 1e-12,
        "the fixture's tangency: the flange's normal is horizontal on sub-arc 4"
    );
    let pair = Pair::of(cylinder, flange);
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let images = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
    pair.certify(shared(&pair, images), &carrier, params)
}

/// **Walls tangent along a whole sub-span refuse.** The cylinder and a
/// flange that turns straight down on one of its 24 sub-arcs, where the
/// two walls are tangent along the rim; no certification sample falls on
/// that sub-arc. The cover of the row cannot read it zero-free, and
/// escalates.
#[test]
fn a_shared_row_tangent_along_a_sub_span_escalates() {
    let got = tangent_along_a_sub_span();
    assert!(
        matches!(
            got,
            Err(CertifyError::Escalated {
                check: geom_brep::CertCheck::Transversality,
                ..
            })
        ),
        "walls tangent along t ∈ [0.2, 0.25] escalate their crossing: {:?}",
        got.map(|e| e.certificate().max_residual)
    );
}

/// A ring of revolution about `z` through `turn` in `m` rational quadratic
/// sub-arcs, its `v` direction quadratic through the three profile points
/// `(radius, height)`.
fn curled_ring(m: usize, turn: f64, profile: [(f64, f64); 3]) -> Surface<f64> {
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
            control.push(Point3::new(
                r * reach * theta.cos(),
                r * reach * theta.sin(),
                z,
            ));
            weights.push(w);
        }
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(ku, 2).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            control,
            weights,
        )
        .unwrap(),
    ))
}

/// **The crossing is read beside the row, not over the whole wall.** The
/// cylinder and a flange that leaves the rim outward and curls down and
/// back under itself, each one sub-arc 3 rad wide: across the flange's
/// whole cell its tangents turn through the cylinder's, both across the
/// row (the curl) and along it (the sub-arc's swing), but at the rim the
/// two walls cross square. The row's crossing reading, cut along both
/// axes to a sliver of each wall beside its piece of the row, certifies
/// the rim.
#[test]
fn a_rim_beside_a_curling_wall_reads_its_crossing_at_the_row() {
    const WIDE: f64 = 3.0;
    let (carrier, params) = (rim(1.0), (0.0, WIDE));
    let pair = Pair::of(
        cylinder(1, WIDE),
        curled_ring(1, WIDE, [(1.0, 0.0), (2.0, 0.0), (1.0, -1.0)]),
    );
    let images = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
    let selector = pair
        .recover(&carrier, params, images)
        .expect("a shared row");
    pair.certify(selector, &carrier, params)
        .expect("the walls cross square at the rim");
}

/// **An arc rim runs along `u`.** An arc-rim image moving along `v` is no
/// row image any certificate states, and the shared row refuses it, as it
/// refuses an image that moves along neither coordinate.
#[test]
fn a_shared_row_admits_only_row_images() {
    let (carrier, params) = (rim(1.0), (0.0, TURN));
    let pair = Pair::of(cylinder(1, TURN), annulus(1, TURN));
    let [ia, ib] = [0, 1].map(|i| row(&carrier, params, pair.wall(i)));
    assert!(
        SharedRow::new(
            pair.pair(),
            (pair.keys[0], ia.clone()),
            (pair.keys[1], ib.clone())
        )
        .is_some(),
        "two arc rims along u are a shared row"
    );
    let Pcurve::IsoArc {
        p0,
        pd,
        t0,
        angle,
        breaks,
    } = ib
    else {
        panic!("an arc rim");
    };
    for (what, pd) in [
        ("along v", geom_core::Vec2::new(0.0, pd.x)),
        ("along neither", geom_core::Vec2::new(pd.x, pd.x)),
    ] {
        let moved = Pcurve::IsoArc {
            p0,
            pd,
            t0,
            angle,
            breaks: breaks.clone(),
        };
        assert!(
            SharedRow::new(
                pair.pair(),
                (pair.keys[0], ia.clone()),
                (pair.keys[1], moved)
            )
            .is_none(),
            "an arc image {what} is no row"
        );
    }
}

// ---- the PR 5959 re-review 2 rows, ported ----

/// The swapping geometry with the upright's row coordinate on the knot
/// vector `knots` (local `[0, 1]`, shifted by `s`), its controls at the
/// abscissae `at` and its outward lean `lean` there.
fn swapping_on(
    s: f64,
    knots: Vec<f64>,
    degree: usize,
    at: &[f64],
    lean: &[f64],
) -> (Pair, Curve3<f64>, [Pcurve<f64>; 2]) {
    let (pair, line, images) = swapping_at(0.5, s);
    let p = |x, y, z| Point3::new(x, y, z);
    let mut control = Vec::new();
    for (y, l) in at.iter().zip(lean) {
        control.push(p(1.0, *y, 0.0));
        control.push(p(2.0, *y, *l));
    }
    let upright = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(knots.iter().map(|k| k + s).collect(), degree).unwrap(),
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            control,
            vec![1.0; 2 * at.len()],
        )
        .unwrap(),
    ));
    let kv = || KnotVector::clamped(knots.iter().map(|k| k + s).collect(), degree).unwrap();
    let floor = Surface::Nurbs(Arc::new(
        NurbsSurface::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            kv(),
            [0.0, 1.0]
                .into_iter()
                .flat_map(|x| at.iter().map(move |&y| p(x, y, 0.0)))
                .collect(),
            vec![1.0; 2 * at.len()],
        )
        .unwrap(),
    ));
    let _ = (pair, line);
    let line = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            kv(),
            at.iter().map(|&y| p(1.0, y, 0.0)).collect(),
            vec![1.0; at.len()],
        )
        .unwrap(),
    ));
    (Pair::of(floor, upright), line, images)
}

fn verdict(got: &Result<EdgeCurve<f64>, CertifyError>) -> String {
    match got {
        Ok(_) => "Ok".into(),
        Err(CertifyError::NotTransverse {
            sample, verdict, ..
        }) => format!("NotTransverse({sample}, {verdict:?})"),
        Err(CertifyError::Escalated { check, sample, .. }) => {
            format!("Escalated({check:?}, {sample})")
        }
        Err(e) => format!("other: {e:?}"),
    }
}

/// RR2: a tangency exactly on an interior C¹ knot of the upright (degree
/// 2, one interior knot at `k`), the walls swapping sides there: the
/// same geometry as `swapping_at(k, s)`, the knot only re-spelling it.
/// It must not certify, at any `s`.
#[test]
fn rr2_a_tangency_on_a_c1_knot_refuses() {
    let far = 2f64.powi(52 + (1000.0 * eps()).log2().ceil() as i32);
    let mut bad = Vec::new();
    for s in [0.0, 1.0, 1024.0, far] {
        for k in [0.25, 0.3, 0.5, 0.6875, 0.9] {
            let at = [0.0, k / 2.0, (k + 1.0) / 2.0, 1.0];
            let lean = at.map(|y| y - k);
            let (pair, line, images) =
                swapping_on(s, vec![0.0, 0.0, 0.0, k, 1.0, 1.0, 1.0], 2, &at, &lean);
            let got = pair.certify(shared(&pair, images), &line, (s, s + 1.0));
            let v = verdict(&got);
            eprintln!("c1 knot k={k} s={s}: {v}");
            if v == "Ok" {
                bad.push((k, s));
            }
        }
    }
    assert!(bad.is_empty(), "tangent on a C¹ knot certifies: {bad:?}");
}

/// RR2: a tangency exactly on a crease of the upright (degree 1 along the
/// row, interior knot at `k`): its lean is piecewise linear and zero at
/// `k`, crossing (`swap`) or touching (`!swap`). The upright's tangent
/// plane from either side lies in the floor's at `k`: it must not certify.
#[test]
fn rr2_a_tangency_on_a_crease_refuses() {
    let far = 2f64.powi(52 + (1000.0 * eps()).log2().ceil() as i32);
    let mut bad = Vec::new();
    for s in [0.0, 1024.0, far] {
        for k in [0.25, 0.3, 0.5, 0.7] {
            for (name, l0, l2) in [
                ("swap", -k, 2.0 * (1.0 - k)),
                ("touch", -k, -(1.0 - k)),
                ("touch+", k, 3.0 * (1.0 - k)),
            ] {
                let (pair, line, images) = swapping_on(
                    s,
                    vec![0.0, 0.0, k, 1.0, 1.0],
                    1,
                    &[0.0, k, 1.0],
                    &[l0, 0.0, l2],
                );
                let got = pair.certify(shared(&pair, images), &line, (s, s + 1.0));
                let v = verdict(&got);
                eprintln!("crease {name} k={k} s={s}: {v}");
                if v == "Ok" {
                    bad.push((name, k, s));
                }
            }
        }
    }
    assert!(bad.is_empty(), "tangent on a crease certifies: {bad:?}");
}

/// RR2: tangent just past an end of the row (by δ), the walls swapping
/// sides off the row: the closest crossing on the row is δ-shallow; at
/// δ inside the band the row must not certify clear.
#[test]
fn rr2_a_tangency_a_hair_past_an_end_does_not_certify_clear() {
    let far = 2f64.powi(52 + (1000.0 * eps()).log2().ceil() as i32);
    let mut bad = Vec::new();
    for s in [0.0, far] {
        for ys in [-0.01 * eps(), 1.0 + 0.01 * eps()] {
            let (pair, line, images) = swapping_at(ys, s);
            let got = pair.certify(shared(&pair, images), &line, (s, s + 1.0));
            let v = verdict(&got);
            eprintln!("past end ys={ys} s={s}: {v}");
            if v == "Ok" {
                bad.push((ys, s));
            }
        }
    }
    assert!(bad.is_empty(), "{bad:?}");
}
