//! **The node's de-risking probe**: the bodies the node is to produce,
//! built by hand through the Euler operators and handed to the at-rest
//! gate (tier 3), before any boolean builds them.
//!
//! Two unit cylinders whose axes meet at right angles, `A` about `z`,
//! `B` about `y`, cut each other in the two bisector-plane ellipses `e₁`
//! (`z = y`) and `e₂` (`z = −y`), which cross at the nodes `N± = ±x̂`.
//! Each ellipse is two arcs between the nodes, `hi` (`y > 0`) and `lo`
//! (`y < 0`).
//!
//! - The bicylinder `A ∩ B` is V2 E4 F4: four lunes, each bounded by one
//!   arc of each ellipse, alternating `A` and `B` round each node, which
//!   has valence 4.
//! - `A ∖ B` is two solids, the `z > 0` and `z < 0` pieces of `A`'s
//!   rod less the bicylinder, touching at both nodes, each with its own
//!   valence-2 vertex there. The top piece is built here, and both
//!   pieces as one body for the census lane's measurement.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI};

use crate::Body;

use crate::construction::{FaceSurface, MefSite, MevSite};
use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, EqualCylinderSection, SurfacePair};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn wall(axis: Vec3<f64>, u_ref: Vec3<f64>) -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis,
        radius: 1.0,
        u_ref,
    }
}

/// The two walls and the table's two ellipses, `e₁` then `e₂`. On `e₁`
/// the nodes stand at `t = ±π/2` (`N₊` at `π/2`), on `e₂` at `t = ∓π/2`.
fn pair() -> (Surface<f64>, Surface<f64>, Curve3<f64>, Curve3<f64>) {
    let (sa, sb) = (
        wall(Vec3::unit_z(), Vec3::unit_x()),
        wall(Vec3::unit_y(), Vec3::unit_z()),
    );
    let reach = geom_brep::Reach::Measured {
        at: Point3::origin(),
        lever: 1.0,
    };
    let Ok(EqualCylinderSection::TwoEllipses { e1, e2, .. }) =
        geom_brep::cylinder_cylinder_section(&sa, &sb, &reach.ball(), band())
    else {
        panic!("the table constructs the two ellipses");
    };
    (sa, sb, e1, e2)
}

/// An arc of `carrier` over `[t0, t1]`, described as the walls' section
/// through its midpoint.
fn arc(pair: SurfacePair, carrier: &Curve3<f64>, (t0, t1): (f64, f64)) -> EdgeCurveSpec<f64> {
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Intersection {
            pair,
            selector: geom_brep::CurveSelector::Witness(carrier.mid_point(t0, t1)),
        },
        carrier: carrier.clone(),
        param_start: t0,
        param_end: t1,
    }
}

/// Each boundary edge of `face`: its carrier and its midpoint.
fn arcs(body: &Body<f64>, face: crate::FaceKey) -> Vec<(Curve3<f64>, Point3<f64>)> {
    let fd = body.get_face(face).expect("the face");
    body.face_boundary_linked(face, fd)
        .filter_map(|m| match m {
            crate::live::BoundaryMember::Edge { ek, .. } => Some(ek),
            crate::live::BoundaryMember::Isolated { .. } => None,
        })
        .map(|ek| {
            let c = body
                .get_edge(ek)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(crate::null::CurveGeom::certified)
                .expect("a certified arc");
            let (t0, t1) = c.params();
            (c.carrier().clone(), c.carrier().mid_point(t0, t1))
        })
        .collect()
}

/// **The bicylinder by hand**: `mvfs` at `N₊`, `mev` along `e₁`'s `hi`
/// arc to `N₋`, then three `mef`s closing `e₂`'s `lo` arc, `e₂`'s `hi`
/// arc and `e₁`'s `lo` arc, every face on `B` until the end, when each
/// lune takes the wall it lies on: `A` where its arcs are both `hi` or
/// both `lo`. `flip` mints the seed the other way round.
fn bicylinder(flip: bool) -> Body<f64> {
    let tol = Tol::witness();
    let (sa, sb, e1, e2) = pair();
    let mut body = Body::new();
    let (np, nm) = (Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
    let seed = body.mvfs(np, !flip).unwrap();
    let kb = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: sb,
                sense: true,
            },
        )
        .unwrap();
    let ka = body.add_surface(sa);
    let sp = SurfacePair::new(ka, kb);
    let on_b = || FaceSurface::Shared {
        key: kb,
        sense: true,
    };
    let e1_hi = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            nm,
            arc(sp, &e1, (FRAC_PI_2, 3.0 * FRAC_PI_2)),
            tol,
        )
        .unwrap();
    let (vp, vm) = (seed.vertex, e1_hi.vertex);
    // e₂'s lo arc, N₋ → N₊.
    let lune = body
        .mef(
            MefSite::Chords {
                he1: e1_hi.he_minus,
                he2: e1_hi.he_plus,
            },
            arc(sp, &e2, (FRAC_PI_2, 3.0 * FRAC_PI_2)),
            on_b(),
            tol,
        )
        .unwrap();
    // e₂'s hi arc, N₊ → N₋, in one of the two lamina faces.
    let split = |body: &mut Body<f64>, face, from, to, spec| {
        let he1 = body.find_half_edge(face, from, to).expect("a half");
        let he2 = body.find_half_edge(face, to, from).expect("its partner");
        body.mef(MefSite::Chords { he1, he2 }, spec, on_b(), tol)
            .unwrap()
    };
    let host = if flip { lune.face } else { seed.face };
    let second = split(
        &mut body,
        host,
        vp,
        vm,
        arc(sp, &e2, (-FRAC_PI_2, FRAC_PI_2)),
    );
    // e₁'s lo arc, N₋ → N₊, splitting the face that holds both e₂ arcs.
    let both_e2 = [host, second.face]
        .into_iter()
        .find(|&f| {
            arcs(&body, f)
                .iter()
                .all(|(c, _)| format!("{c:?}") == format!("{e2:?}"))
        })
        .expect("a face bounded by e₂'s two arcs");
    split(
        &mut body,
        both_e2,
        vm,
        vp,
        arc(sp, &e1, (-FRAC_PI_2, FRAC_PI_2)),
    );
    // Each lune takes its wall.
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    for f in faces {
        let his: Vec<bool> = arcs(&body, f).iter().map(|(_, mid)| mid.y > 0.0).collect();
        if his.iter().all(|h| *h) || his.iter().all(|h| !*h) {
            body.set_face_surface(
                f,
                FaceSurface::Shared {
                    key: ka,
                    sense: true,
                },
            )
            .unwrap();
        }
    }
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// **The bicylinder passes the at-rest gate**, V2 E4 F4, its volume
/// the closed form `16r³/3`. Minted the other way round it is the same
/// complex inside out, which the gate refuses as such.
#[test]
fn the_bicylinder_built_by_hand_is_a_finished_body() {
    let tol = Tol::witness();
    let done = crate::test_support::finished("the bicylinder", bicylinder(true), tol);
    let v =
        crate::mass_properties(&done, tol, crate::test_support::pinned_resolution(tol)).unwrap();
    assert!(
        (v.enclosure.volume - 16.0 / 3.0).abs() <= v.enclosure.volume_pad + 1e-12,
        "volume {} ± {}",
        v.enclosure.volume,
        v.enclosure.volume_pad
    );
    let c = crate::readback::euler_counts(&done);
    assert_eq!((c.v, c.e, c.f), (2, 4, 4));
    assert!(
        matches!(
            crate::AtRestPolicy::gate_at_rest_kept(bicylinder(false), tol)
                .as_ref()
                .map_err(Vec::as_slice),
            Err([crate::ValidationError::NegativeVolume { .. }])
        ),
        "inside out, the gate refuses the volume's sign"
    );
}

/// **The top piece of `A ∖ B`**: `A`'s rod above `z = |y|` up to its
/// cap at `z = 2`. Its faces are `A`'s wall above the ellipses, closed
/// along the ruling `(0, 1, z)` from `P = (0, 1, 1)` on `e₁` to
/// `Q = (0, 1, 2)` on the rim; `B`'s lune `z > 0`, material outside
/// `B`; and the cap. Its nodes are valence-2 vertices: `e₁`'s `hi` arc
/// and `e₂`'s `lo` arc meet there. `flip` mints the seed the other way
/// round.
fn top_piece(flip: bool, rim_axis: Vec3<f64>) -> Body<f64> {
    let tol = Tol::witness();
    let (sa, sb, e1, e2) = pair();
    let mut body = Body::new();
    let (np, nm) = (Point3::new(1.0, 0.0, 0.0), Point3::new(-1.0, 0.0, 0.0));
    let (p, q) = (Point3::new(0.0, 1.0, 1.0), Point3::new(0.0, 1.0, 2.0));
    let seed = body.mvfs(np, !flip).unwrap();
    let kb = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: sb,
                sense: true,
            },
        )
        .unwrap();
    let ka = body.add_surface(sa);
    let sp = SurfacePair::new(ka, kb);
    let on_b = || FaceSurface::Shared {
        key: kb,
        sense: true,
    };
    let to_p = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p,
            arc(sp, &e1, (FRAC_PI_2, PI)),
            tol,
        )
        .unwrap();
    let to_nm = body
        .mev(
            MevSite::Fan {
                he1: to_p.he_minus,
                he2: to_p.he_minus,
            },
            nm,
            arc(sp, &e1, (PI, 3.0 * FRAC_PI_2)),
            tol,
        )
        .unwrap();
    let lune = body
        .mef(
            MefSite::Chords {
                he1: to_nm.he_minus,
                he2: to_p.he_plus,
            },
            arc(sp, &e2, (FRAC_PI_2, 3.0 * FRAC_PI_2)),
            on_b(),
            tol,
        )
        .unwrap();
    let wall = if flip { lune.face } else { seed.face };
    body.set_face_surface(
        wall,
        FaceSurface::Shared {
            key: ka,
            sense: true,
        },
    )
    .unwrap();
    let at_p = [to_p.vertex, seed.vertex, to_nm.vertex]
        .into_iter()
        .find_map(|to| body.find_half_edge(wall, to_p.vertex, to))
        .expect("a half from P in the wall face");
    let seam = body
        .mev(
            MevSite::Fan {
                he1: at_p,
                he2: at_p,
            },
            q,
            EdgeCurveSpec::line_between(p, q).at_rest_in_chart(ka, true),
            tol,
        )
        .unwrap();
    let cap = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 2.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let kc = body.add_surface(cap);
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 2.0),
        axis: rim_axis,
        radius: 1.0,
        u_ref: Vec3::unit_y(),
    };
    body.mef(
        MefSite::Chords {
            he1: seam.he_minus,
            he2: seam.he_minus,
        },
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(ka, kc),
                selector: geom_brep::CurveSelector::Witness(rim.mid_point(0.0, 2.0 * PI)),
            },
            carrier: rim,
            param_start: 0.0,
            param_end: 2.0 * PI,
        },
        FaceSurface::Shared {
            key: kc,
            sense: true,
        },
        tol,
    )
    .unwrap();
    // Each face takes its surface and side: the cap's edges are all
    // circles, the wall's carry the ruling, and the lune is B's, its
    // material outside B.
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    for f in faces {
        let kinds: Vec<_> = arcs(&body, f).into_iter().map(|(c, _)| c).collect();
        let (key, sense) = if kinds.iter().all(|c| matches!(c, Curve3::Circle { .. })) {
            (kc, true)
        } else if kinds.iter().any(|c| matches!(c, Curve3::Line { .. })) {
            (ka, true)
        } else {
            (kb, false)
        };
        if body.get_face(f).expect("the face").surface == key {
            body.set_face_sense(f, sense).unwrap();
        } else {
            body.set_face_surface(f, FaceSurface::Shared { key, sense })
                .unwrap();
        }
    }
    crate::pcurves::mint_pcurves(&mut body, tol).unwrap();
    body
}

/// **The top piece of `A ∖ B` passes the at-rest gate**, its volume
/// `A`'s rod above `z = 0` (`2π`) less the bicylinder's upper half
/// (`8/3`). Its rim runs about `−z`, against the ellipses' winding, so
/// the wall face's loop closes on `A`'s chart.
#[test]
fn a_piece_of_the_subtraction_built_by_hand_is_a_finished_body() {
    let tol = Tol::witness();
    let done = crate::test_support::finished(
        "the top piece",
        top_piece(false, Vec3::new(0.0, 0.0, -1.0)),
        tol,
    );
    let v =
        crate::mass_properties(&done, tol, crate::test_support::pinned_resolution(tol)).unwrap();
    let want = 2.0 * PI - 8.0 / 3.0;
    assert!(
        (v.enclosure.volume - want).abs() <= v.enclosure.volume_pad + 1e-12,
        "volume {} ± {}, want {want}",
        v.enclosure.volume,
        v.enclosure.volume_pad
    );
}

/// **Both pieces of `A ∖ B` as one body**, the bottom the top turned a
/// half-turn about `x`: two solids touching at the two nodes. Each
/// piece is finished on its own (above); what the gate says of the
/// pair is the census's reading of their touch, measured here.
#[test]
#[ignore = "a measurement for the census lane: cargo nextest run -p topo --run-ignored only -E 'test(/both_pieces/)' --no-capture"]
fn both_pieces_of_the_subtraction_as_one_body() {
    let tol = Tol::witness();
    let top = top_piece(false, Vec3::new(0.0, 0.0, -1.0));
    let half_turn = geom_core::test_support::rigid(
        &geom_core::Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), PI),
        tol,
    );
    let bottom = crate::transform_rigid(&top, &half_turn, tol).unwrap();
    let mut both = top;
    crate::graft_disjoint(&mut both, &bottom).unwrap();
    let census = crate::census::census_and_certify(
        &both,
        &crate::ContactRecords::default(),
        band(),
        tol,
        None,
    );
    eprintln!("PROBE both pieces, census: {census:?}");
    eprintln!(
        "PROBE both pieces, gate: {:?}",
        crate::AtRestPolicy::gate_at_rest_kept(both, tol).map(|_| "finished")
    );
}
