//! **The iso-rectangle SHAPE door** (`require_iso_rectangle`): the S58
//! predicate and the per-kind boundary classification, public and
//! flux-free, for a consumer whose lane rests on the premise without
//! wanting a volume.
//!
//! Every row is built as key-free `LoopEdge`s and run through the door
//! AND through `curved_face`, so the rows state where the two agree
//! (a rectangle passes both, a notch refuses both by `props_rim_level`,
//! an oblique sphere section refuses both by the same incidence name)
//! and the rimless lune, a chart rectangle the door admits on the
//! shape alone while the flux lane measures it at the width its loop
//! bounds — two premises, one face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    CarrierId, LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face,
    require_iso_rectangle,
};

/// The unit cylinder about +Z with its rim (coaxial circle at height
/// `v`) and meridian (axial line at azimuth `u`) edge factories.
fn cylinder() -> Surface<f64> {
    surf::cylinder(1.0)
}
/// **Deliberately not `shared::topo::sphere_rim`.** This is a
/// CYLINDER's rim: `v` is a height along the axis, not a latitude, so
/// the circle's radius does not fall off as `cos v`. Same word, other
/// surface.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}
fn mer(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Line {
            origin: p3(u.cos(), u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        v0,
        v1,
        a,
        b,
    )
}

fn sphere() -> Surface<f64> {
    surf::sphere(1.0)
}
/// The great circle whose plane contains the axis at azimuth `u`; its
/// parameter IS the latitude.
fn great(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_great(1.0, u, v0, v1, a, b)
}

/// A cylinder rectangle passes the door and measures; the U-shaped
/// keyway (a notch cut into the top rim) refuses at the door AND at the
/// flux lane, both by `props_rim_level` — one predicate, two callers.
#[test]
fn a_keyway_refuses_at_the_door_by_the_same_name_the_flux_lane_uses() {
    let rect = vec![
        rim(0.0, 0.0, 1.5, 0, 1),
        mer(1.5, 0.0, 1.0, 1, 2),
        rim(1.0, 1.5, 0.0, 2, 3),
        mer(0.0, 1.0, 0.0, 3, 0),
    ];
    assert_eq!(require_iso_rectangle(&cylinder(), &rect, band()), Ok(()));
    assert!(curved_face(&cylinder(), &rect, true, band()).is_ok());
    let keyway = vec![
        rim(0.0, 0.0, 1.5, 0, 1),
        mer(1.5, 0.0, 1.0, 1, 2),
        rim(1.0, 1.5, 1.0, 2, 3),
        mer(1.0, 1.0, 0.6, 3, 4),
        rim(0.6, 1.0, 0.5, 4, 5),
        mer(0.5, 0.6, 1.0, 5, 6),
        rim(1.0, 0.5, 0.0, 6, 7),
        mer(0.0, 1.0, 0.0, 7, 0),
    ];
    let want = Err(PropsError::NotIsoRectangle {
        what: "props_rim_level",
    });
    assert_eq!(require_iso_rectangle(&cylinder(), &keyway, band()), want);
    assert_eq!(
        curved_face(&cylinder(), &keyway, true, band()).map(|_| ()),
        want
    );
}

/// **Two homes, one lune.** A lune between two great circles a
/// quarter turn apart is a chart rectangle in azimuth × latitude, so
/// the door admits it; the flux lane measures it at the width the
/// loop bounds (`props_wedge_azimuth`). The door's answer is the
/// shape's and does not depend on which lunes the flux lane measures.
#[test]
fn a_rimless_lune_passes_the_door_and_measures() {
    let half = core::f64::consts::FRAC_PI_2;
    let lune = vec![
        great(0.0, -half, half, 0, 1),
        great(half, half, -half, 1, 0),
    ];
    assert_eq!(require_iso_rectangle(&sphere(), &lune, band()), Ok(()));
    // Interior-left about the outward normal: with `sense = true` this
    // loop (northward at `u = 0`) bounds the three-quarter lune of the
    // unit sphere.
    let fc = curved_face(&sphere(), &lune, true, band()).expect("the flux lane measures the lune");
    let exact = 2.0 * 3.0 * half;
    assert!(
        (fc.area - exact).abs() / exact < 1e-12,
        "area {:.15e} != {exact:.15e}",
        fc.area
    );
}

/// An oblique plane section of the sphere — tilted 0.6 rad off the
/// polar axis, offset 0.3 from the centre — is neither a coaxial rim
/// nor a great circle. The door classifies it a rim (`n·â` definite)
/// and refuses its incidence: the circle's axis is not the sphere's.
/// This is the `walk::iso_side_starts` qualification's face, refused
/// on rim structure before any walk could collapse it.
#[test]
fn an_oblique_sphere_section_is_refused_on_rim_incidence() {
    let (a, d) = (0.6_f64, 0.3_f64);
    let r = (1.0 - d * d).sqrt();
    let n = v3(a.sin(), 0.0, a.cos());
    let section = |t0: f64, t1: f64, s: u32, e: u32| {
        edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, 0.0) + n * d,
                axis: n,
                radius: r,
                u_ref: v3(a.cos(), 0.0, -a.sin()),
            },
            t0,
            t1,
            s,
            e,
        )
    };
    // Two arcs of the one oblique circle closing a loop on their own:
    // the door refuses on the FIRST edge's incidence, so the loop's
    // closure is not what is under test here.
    let lens = vec![section(0.0, 3.0, 0, 1), section(3.0, 6.0, 1, 0)];
    let want = Err(PropsError::NotIsoRectangle {
        what: "props_rim_axis_parallel",
    });
    assert_eq!(require_iso_rectangle(&sphere(), &lens, band()), want);
    assert_eq!(
        curved_face(&sphere(), &lens, true, band()).map(|_| ()),
        want
    );
}

/// A plane is not the door's question and refuses typed, as
/// `curved_face` does.
#[test]
fn a_plane_is_refused_typed_not_answered() {
    let plane: Surface<f64> = Surface::Plane {
        origin: p3(0.0, 0.0, 0.0),
        normal: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    assert_eq!(
        require_iso_rectangle(&plane, &[], band()),
        Err(PropsError::NotIsoRectangle {
            what: "require_iso_rectangle called on a plane (a planar loop is not a chart rectangle)",
        })
    );
}

// ---------------------------------------------------------------------
// The torus: a meridian carried by the pieces of one split edge
// ---------------------------------------------------------------------

/// The torus under the lineage rows: `R = 20 mm`, `r = 5 mm` about +Z.
const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn torus() -> Surface<f64> {
    surf::torus(RR, R0)
}
/// A torus rim: the coaxial circle at minor angle `v`.
fn trim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(topo::torus_rim_circle(RR, R0, v), u0, u1, a, b)
}
/// A torus meridian ARC on the minor circle at azimuth `u`, stamped
/// with the identity of the edge it is a piece of (`None`: a loop
/// built without a body records none).
/// **Deliberately not shared with `mesh10r1_probes.rs`'s `tmer`,**
/// which is this text: the reviewer-pair independence above. The
/// geometry IS shared — `topo::torus_meridian_circle` — so what stays
/// apart is only how each suite stamps the carrier id.
fn tmer(u: f64, v0: f64, v1: f64, a: u32, b: u32, id: Option<u64>) -> LoopEdge<f64> {
    LoopEdge {
        carrier_id: id.map(CarrierId::minted),
        ..edge(topo::torus_meridian_circle(RR, R0, u), v0, v1, a, b)
    }
}

/// The torus rectangle `[v0, v1] × [u0, u1]` with each meridian one
/// edge — the control — and the same rectangle with the `u1` meridian
/// in two pieces (a halving split) and the `u0` meridian in three,
/// each piece stamped with the identity `(id1, id0)` the caller gives
/// it. Interval endpoints are the same literals on both, exactly as
/// `split_edge` keeps the parent's `t0`/`t1` on its outer children.
const V0: f64 = 0.2;
const V1: f64 = 1.2;
const U0: f64 = -1.0;
const U1: f64 = 1.0;
/// **Deliberately not shared with `mesh10r1_probes.rs` /
/// `mesh10r2_probes.rs`, whose `control` is this text.** This file is
/// the shipped door suite; those two are an R1/R2 reviewer pair whose
/// value is that they rebuild the rectangle for themselves. A shared
/// `control` would make all three agree by construction about the loop
/// the door is supposed to accept.
fn control() -> Vec<LoopEdge<f64>> {
    vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, V1, 1, 2, Some(1)),
        trim(V1, U1, U0, 2, 3),
        tmer(U0, V1, V0, 3, 0, Some(2)),
    ]
}
fn pieced(id1: Option<u64>, id0: Option<u64>) -> Vec<LoopEdge<f64>> {
    let (vm, va, vb) = (0.7, 0.9, 0.5);
    vec![
        trim(V0, U0, U1, 0, 1),
        tmer(U1, V0, vm, 1, 2, id1),
        tmer(U1, vm, V1, 2, 3, id1),
        trim(V1, U1, U0, 3, 4),
        tmer(U0, V1, va, 4, 5, id0),
        tmer(U0, va, vb, 5, 6, id0),
        tmer(U0, vb, V0, 6, 0, id0),
    ]
}
fn bits(c: geom_brep::props::FaceContribution<f64>) -> (u64, u64) {
    (c.flux.to_bits(), c.area.to_bits())
}

/// **A meridian in pieces folds back into one by lineage, and the
/// folded face IS the control** — through the door, the flux lane and
/// the material side, bitwise: a forward two-piece chain, a reversed
/// three-piece chain, and every rotation of the loop, including the
/// ones where a chain straddles the loop's first edge.
#[test]
fn a_meridian_in_pieces_folds_by_lineage_into_the_edge_it_came_from() {
    let s = torus();
    let ctl = bits(curved_face(&s, &control(), true, band()).expect("the control rectangle"));
    let side = boundary_material_sign(&s, &control(), band()).expect("the control's side");
    assert!(matches!(side, MaterialSign::Encoded(_)));
    let loop_ = pieced(Some(1), Some(2));
    assert_eq!(require_iso_rectangle(&s, &loop_, band()), Ok(()));
    assert_eq!(
        bits(curved_face(&s, &loop_, true, band()).expect("the pieced rectangle")),
        ctl,
        "flux and area bitwise: the fold reads the edge's own interval and anchor"
    );
    assert_eq!(boundary_material_sign(&s, &loop_, band()), Ok(side));
    for k in 0..loop_.len() {
        let mut rot = loop_[k..].to_vec();
        rot.extend_from_slice(&loop_[..k]);
        assert_eq!(
            require_iso_rectangle(&s, &rot, band()),
            Ok(()),
            "rotation {k}"
        );
        let c = curved_face(&s, &rot, true, band())
            .unwrap_or_else(|e| panic!("rotation {k}: refused {e:?}"));
        assert_eq!(bits(c), ctl, "rotation {k}: bitwise the control");
    }
}

/// **Identity is the whole test: pieces from distinct edges never
/// fold, however their stored circles compare.** Bit-identical arcs
/// stamped with two identities, or with none, stay two meridians —
/// the anchor then reads one piece's span, the far rim is not at an
/// extreme, and every consumer refuses `props_rim_level` as it does
/// on any non-rectangle. So does a genuine corner: two arcs on
/// different minor circles from two edges. Reds under a fold that
/// keys on anything but the lineage. The stamp itself is the body's
/// record and is trusted as such — a loop that stamps two circles
/// with one identity lies the way lying tags lie, and its author owns
/// that (the `LoopEdge` contract).
#[test]
fn pieces_from_distinct_edges_never_fold() {
    let s = torus();
    let refuses = |name: &str, loop_: &[LoopEdge<f64>]| {
        let rim_level = PropsError::NotIsoRectangle {
            what: "props_rim_level",
        };
        assert_eq!(
            require_iso_rectangle(&s, loop_, band()),
            Err(rim_level.clone()),
            "{name}: door"
        );
        assert_eq!(
            curved_face(&s, loop_, true, band()).map(|_| ()),
            Err(rim_level.clone()),
            "{name}: flux lane"
        );
        assert_eq!(
            boundary_material_sign(&s, loop_, band()),
            Err(rim_level),
            "{name}: side"
        );
    };
    // The same values, distinct identities on one meridian's pieces.
    refuses(
        "distinct ids",
        &pieced(Some(1), Some(2))
            .into_iter()
            .enumerate()
            .map(|(i, mut e)| {
                if i == 2 {
                    e.carrier_id = Some(CarrierId::minted(9));
                }
                e
            })
            .collect::<Vec<_>>(),
    );
    // No identity at all.
    refuses("no ids", &pieced(None, None));
    // A corner: arcs on two minor circles from two edges.
    let mut corner = pieced(Some(1), Some(2));
    corner[2] = tmer(0.5, 0.7, V1, 2, 3, Some(9));
    refuses("corner", &corner);
}

// ---------------------------------------------------------------------
// The sense-free residue of the flux lane's interior-side premise
// ---------------------------------------------------------------------

/// The sphere rim (a coaxial circle) at latitude `v`.
fn srim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_rim(1.0, v, u0, u1, a, b)
}

/// **The door takes the residue: every rim must encode the SAME
/// material side** (`work/props/the-shape-door-could-take-the-sense-
/// free-rim-side-residue.md`).
///
/// The staircase face carries a rim at `lo` and a rim at `hi`
/// traversed the same way. Its rims contradict each other about which
/// side the material is on, so `linear_rim_side` answers a definite ±1
/// whose value depends on which rim the owning body's loop walk hands
/// over first — and the door used to answer `Ok(())` for it, because
/// every rim does sit at an extreme. `unanimous_rim_side` needs no
/// sense bit to see the contradiction, and the door now asks it: the
/// same `props_rim_side` name the flux lane and the material-side gate
/// refuse it by.
#[test]
fn the_door_refuses_a_face_whose_rims_encode_different_sides() {
    let (lo, hi) = (-0.3_f64, 0.5_f64);
    let pi = core::f64::consts::PI;
    let tau = core::f64::consts::TAU;
    let want = || {
        Err(PropsError::NotIsoRectangle {
            what: "props_rim_side",
        })
    };
    let stair = [
        srim(lo, 0.0, pi, 0, 1),
        great(pi, lo, hi, 1, 2),
        srim(hi, pi, tau, 2, 3),
        great(0.0, hi, lo, 3, 0),
    ];
    for k in 0..stair.len() {
        let e: Vec<LoopEdge<f64>> = (0..stair.len())
            .map(|i| stair[(i + k) % stair.len()].clone())
            .collect();
        assert_eq!(
            require_iso_rectangle(&sphere(), &e, band()),
            want(),
            "k={k}: the door reads the contradiction whatever the anchor"
        );
        // The gate refused it already; the door now agrees with it.
        assert_eq!(
            boundary_material_sign(&sphere(), &e, band()).map(|_| ()),
            want()
        );
    }
    // The consistent zone — the same two rims traversed OPPOSITE ways —
    // still passes: unanimity is not "refuse every two-rim face".
    let zone = vec![srim(lo, 0.0, tau, 0, 0), srim(hi, tau, 0.0, 1, 1)];
    assert_eq!(require_iso_rectangle(&sphere(), &zone, band()), Ok(()));
    assert!(curved_face(&sphere(), &zone, true, band()).is_ok());
    assert_eq!(
        boundary_material_sign(&sphere(), &zone, band()),
        Ok(MaterialSign::Encoded(geom_core::Sign::Positive))
    );
}

/// **The residue does not close the door's documented divergence.**
/// The L-shaped complement of a half-cap has ONE rim, so there is
/// nothing for unanimity to compare: the door still admits it and the
/// flux lane still refuses it `props_rim_interior_side`. That residue
/// is σ's, and σ needs the face's sense bit the door does not take.
#[test]
fn the_one_rim_divergence_survives_the_residue() {
    let v0 = 0.5_f64;
    let pi = core::f64::consts::PI;
    let complement = vec![srim(v0, pi, 0.0, 1, 0), great(0.0, v0, pi - v0, 0, 1)];
    assert_eq!(
        require_iso_rectangle(&sphere(), &complement, band()),
        Ok(()),
        "one rim states no contradiction"
    );
    assert_eq!(
        curved_face(&sphere(), &complement, true, band()).map(|_| ()),
        Err(PropsError::NotIsoRectangle {
            what: "props_rim_interior_side"
        })
    );
}

/// **The residue is not band-continuous with the door's zero-extent
/// charter, and this row is that seam measured.**
///
/// "A zero-extent face passes" is true at EXACTLY zero: there no rim
/// sits at one extreme rather than the other, `rim_side` answers
/// `DegenerateFace`, and the door admits it. Move the extent into the
/// ambiguity band and the same question becomes undecidable rather
/// than vacuous — `props_rim_side` escalates, typed — and only above
/// the band does it answer again. So every linearly-levelled face
/// whose extent lands in the band now refuses at the door that gates
/// `mesh`'s walk, which the corpus measurement could not see because
/// no corpus body has a band-scale extent.
///
/// The posture is the ratified one (escalate, never guess) and this
/// row does not argue with it; it states the class, on a cylinder
/// wall, with every offset taken from the run's own `Band` so it holds
/// at each ε on the matrix.
#[test]
fn the_doors_zero_extent_charter_is_exactly_at_zero() {
    let z = band().zero();
    let wall = |dv: f64| {
        vec![
            rim(0.0, 0.0, 1.5, 0, 1),
            mer(1.5, 0.0, dv, 1, 2),
            rim(dv, 1.5, 0.0, 2, 3),
            mer(0.0, dv, 0.0, 3, 0),
        ]
    };
    for (dv, decidable) in [
        (0.0, true),
        (1.5 * z, false),
        (5.0 * z, false),
        (100.0 * z, true),
    ] {
        let door = require_iso_rectangle(&cylinder(), &wall(dv), band());
        println!("  extent {dv:.3e} ({:.1} x zero): door = {door:?}", dv / z);
        if decidable {
            assert_eq!(door, Ok(()), "extent {dv:.3e}: the door answers");
        } else {
            assert!(
                matches!(
                    &door,
                    Err(PropsError::Escalated { cause })
                        if cause.predicate == Some("props_rim_side")
                ),
                "extent {dv:.3e}: an in-band extent has no extreme to place a rim at: {door:?}"
            );
        }
    }
}
