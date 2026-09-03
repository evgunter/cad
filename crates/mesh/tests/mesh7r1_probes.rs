//! Review probes for the iso-rectangle shape door (PR 1565, issues
//! 727 / 726 / 1562). Each row prints what it measured with
//! `--nocapture`; the assertions pin only what the review verified.
//!
//! The half-cap row is the review's own construction: a sphere face
//! whose ONE meridian edge is a great-circle arc that crosses the north
//! pole mid-edge, so the traversed arc lies on TWO chart meridians
//! (`u` and `u + π`). Props' sphere parse certifies the CARRIER (a
//! great circle) and, since CERT-1, folds the pole into the extent, so
//! the SHAPE door admits both faces of the body — while the walk's
//! premise ("every boundary edge is an iso curve of the chart,
//! traversed on one branch of it") does not hold for that edge. The
//! row records what each lane says about it.
//!
//! **Since MESH-11 (issue 1571) a second door decides that half**:
//! `props::require_one_chart_branch`, cited beside the shape door, so
//! the body is refused typed before the walk. The half-cap row is
//! re-aimed onto the SEAM rather than deleted — shape door yes, branch
//! door no, on one face — because the seam is what this review found
//! and it is still the thing worth pinning.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::witness_bodies::{keyway, oblique_lens, pole_crossing_half_cap};
use common::*;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use mesh::TessellateError;
use topo::{Body, FaceKey, FaceSurface, MefSite, MevSite};

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

// R1-DOOR-ONLY-BEGIN
fn door_verdict(body: &Body<f64>, fk: FaceKey) -> Result<(), geom_brep::props::PropsError> {
    let face = body.get_face(fk).unwrap();
    let surface = body.get_surface(face.surface).unwrap();
    let (outer, _) = topo::props::loop_edges(body, face.outer).unwrap();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    geom_brep::props::require_iso_rectangle(surface, &outer, band)
}

// R1-DOOR-ONLY-END

/// **Reviewer's construction — does the door establish the walk's
/// premise?** Records the door verdict on both faces, the tessellate
/// outcome, `check_mesh` on any mesh, and `mass_properties` against
/// the exact volume 4π/3.
#[test]
fn a_pole_crossing_meridian_arc_passes_the_door_on_both_faces() {
    let (body, cap, rest) = pole_crossing_half_cap();
    let tol = Tol::witness();
    // R1-DOOR-ONLY-BEGIN
    let d_cap = door_verdict(&body, cap);
    let d_rest = door_verdict(&body, rest);
    println!("PROBE half-cap door: cap={d_cap:?} rest={d_rest:?}");
    // R1-DOOR-ONLY-END
    println!(
        "PROBE half-cap validate_geometric: {:?}",
        topo::validate_geometric(&body, tol)
    );
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05] {
        report_tessellate("half-cap (2 faces)", &body, delta, tol);
    }
    let mp = topo::mass_properties(&body, tol);
    println!(
        "PROBE half-cap mass_properties: {mp:?} (exact volume {})",
        4.0 * core::f64::consts::PI / 3.0
    );
    // R1-DOOR-ONLY-BEGIN
    assert_eq!(
        d_cap,
        Ok(()),
        "the half-cap is a chart rectangle and the door says so"
    );
    assert_eq!(
        d_rest,
        Ok(()),
        "the complement's chart domain is an L, and the door admits it: the sphere \
         parse certifies the carrier, not the traversed arc's chart membership"
    );
    // R1-DOOR-ONLY-END
}

fn report_tessellate(name: &str, body: &Body<f64>, delta: f64, tol: Tol) {
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh::tessellate(body, delta, tol)
    }));
    let got = match outcome {
        Ok(got) => got,
        Err(payload) => {
            let msg = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                .unwrap_or_default();
            let first = msg.lines().next().unwrap_or("");
            println!("PROBE {name} delta={delta}: PANIC {first}");
            return;
        }
    };
    match got {
        Ok(m) => println!(
            "PROBE {name} delta={delta}: Ok positions={} triangles/patch={:?} check_mesh={:?}",
            m.positions.len(),
            m.patches
                .iter()
                .map(|p| p.triangles.len())
                .collect::<Vec<_>>(),
            mesh::validate::check_mesh(&m)
        ),
        Err(e) => println!("PROBE {name} delta={delta}: Err({e:?})"),
    }
}

/// The same sphere cut into THREE faces, every one of them a chart
/// rectangle: the half-cap A `[0, π] × [asin 0.5, π/2]`, the other
/// half-cap B `[π, 2π] × [asin 0.5, π/2]`, and the southern part C
/// `[0, 2π] × [−π/2, asin 0.5]`. A and B share the ONE great-circle
/// arc that crosses the north pole mid-edge; no pole vertex exists.
fn pole_crossing_three_faces() -> (Body<f64>, [FaceKey; 3]) {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = p3(r, 0.0, z);
    let b = p3(-r, 0.0, z);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: v3(-r, 0.0, z),
    };
    let mut g = great(v3(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(v3(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim.clone(), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    // b -> a over the pole; the new face takes he1's side (b -> a along
    // the rim, westward), i.e. the complement of the half-cap.
    let over = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    // Inside the complement: b -> a along the OTHER rim half (u from π
    // to 2π). The new face takes the run [rim.minus .. over.minus): the
    // full rim traversed westward = the southern part C; the old loop
    // keeps the arc and the second rim half = B.
    let south = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: over.he_minus,
            },
            EdgeCurveSpec::arc_of_circle(rim, core::f64::consts::PI, core::f64::consts::TAU)
                .unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    (body, [seed.face, over.face, south.face])
}

/// **Three true chart rectangles, one pole-crossing shared arc.** The
/// door's verdict per face, tier 3, `mass_properties` against 4π/3,
/// and `tessellate` across δ.
#[test]
fn three_rectangles_sharing_a_pole_crossing_arc() {
    let (body, faces) = pole_crossing_three_faces();
    let tol = Tol::witness();
    // R1-DOOR-ONLY-BEGIN
    for (name, fk) in ["A", "B", "C"].iter().zip(faces) {
        println!(
            "PROBE 3-face door {name} {fk:?}: {:?}",
            door_verdict(&body, fk)
        );
    }
    // R1-DOOR-ONLY-END
    let _ = faces;
    println!(
        "PROBE 3-face validate_geometric: {:?}",
        topo::validate_geometric(&body, tol)
    );
    println!("PROBE 3-face validate: {:?}", topo::validate(&body));
    println!(
        "PROBE 3-face mass_properties: {:?} (exact {})",
        topo::mass_properties(&body, tol),
        4.0 * core::f64::consts::PI / 3.0
    );
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
        report_tessellate("3-face", &body, delta, tol);
    }
}

/// The three-face sphere with the southern part C given a seam
/// meridian down to a SOUTH POLE vertex (a strut `a -> S` walked both
/// ways), so every face is in props' closed-form inventory: A and B
/// are the two half-caps sharing the pole-crossing arc, C is the
/// revolved-cap shape (full rim + seam to the pole).
fn pole_crossing_three_faces_with_seam() -> (Body<f64>, [FaceKey; 3]) {
    let tol = Tol::witness();
    let z = 0.5_f64;
    let r = (1.0 - z * z).sqrt();
    let a = p3(r, 0.0, z);
    let b = p3(-r, 0.0, z);
    let s_pole = p3(0.0, 0.0, -1.0);
    let rim = Curve3::Circle {
        center: p3(0.0, 0.0, z),
        axis: v3(0.0, 0.0, 1.0),
        radius: r,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let great = |axis: Vec3<f64>, u_ref: Vec3<f64>| Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref,
    };
    // b -> a over the north pole.
    let mut g = great(v3(0.0, 1.0, 0.0), v3(-r, 0.0, z));
    if g.eval(core::f64::consts::FRAC_PI_2).z < z {
        g = great(v3(0.0, -1.0, 0.0), v3(-r, 0.0, z));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    // a -> south pole, descending.
    let mut gd = great(v3(0.0, 1.0, 0.0), v3(r, 0.0, z));
    if gd.eval(core::f64::consts::FRAC_PI_2).z > z {
        gd = great(v3(0.0, -1.0, 0.0), v3(r, 0.0, z));
    }
    let t_pole = gd.param_near(s_pole, 0.0).unwrap();
    assert!(t_pole > 0.0 && (gd.eval(t_pole) - s_pole).norm() < 1e-9);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Sphere {
            center: p3(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: v3(0.0, 0.0, 1.0),
            u_ref: v3(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim.clone(), 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    let over = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: e_rim.he_plus,
            },
            EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    let south = body
        .mef(
            MefSite::Chords {
                he1: e_rim.he_minus,
                he2: over.he_minus,
            },
            EdgeCurveSpec::arc_of_circle(rim, core::f64::consts::PI, core::f64::consts::TAU)
                .unwrap(),
            FaceSurface::Inherit,
            tol,
        )
        .unwrap();
    // The strut from `a` down to the south pole, inside C's loop
    // (`south.he_minus` starts at `a`).
    body.mev(
        MevSite::Fan {
            he1: south.he_minus,
            he2: south.he_minus,
        },
        s_pole,
        EdgeCurveSpec::arc_of_circle(gd, 0.0, t_pole).unwrap(),
        tol,
    )
    .unwrap();
    (body, [seed.face, over.face, south.face])
}

/// **Tier-3 status, mass properties and tessellation of the seamed
/// three-face sphere.** If `validate_geometric` is `Ok`, this body is
/// what `import_step`'s gate 3 would adopt.
#[test]
fn three_rectangles_with_a_seam_sharing_a_pole_crossing_arc() {
    let (body, faces) = pole_crossing_three_faces_with_seam();
    let tol = Tol::witness();
    // R1-DOOR-ONLY-BEGIN
    for (name, fk) in ["A", "B", "C"].iter().zip(faces) {
        println!(
            "PROBE seamed door {name} {fk:?}: {:?}",
            door_verdict(&body, fk)
        );
    }
    // R1-DOOR-ONLY-END
    let _ = faces;
    println!("PROBE seamed validate: {:?}", topo::validate(&body));
    println!(
        "PROBE seamed validate_geometric: {:?}",
        topo::validate_geometric(&body, tol)
    );
    println!(
        "PROBE seamed mass_properties: {:?} (exact {})",
        topo::mass_properties(&body, tol),
        4.0 * core::f64::consts::PI / 3.0
    );
    for delta in [0.5, 0.3, 0.2, 0.1, 0.05, 0.02] {
        report_tessellate("seamed", &body, delta, tol);
    }
}

/// PR claim: splitting a RIM of the donut is fine on both sides.
#[test]
fn a_split_rim_donut_meshes_and_measures() {
    let tol = Tol::witness();
    let mut body = donut();
    let rim = body
        .edges()
        .find(|(_, e)| {
            let c = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            matches!(c.carrier(), Curve3::Circle { radius, .. } if (*radius - 0.5).abs() > 1e-9)
        })
        .map(|(k, _)| k)
        .expect("a major-circle rim");
    let e = body.get_edge(rim).unwrap();
    let (t0, t1) = body
        .get_curve_geom(e.curve)
        .unwrap()
        .certified()
        .unwrap()
        .params();
    body.split_edge(rim, t0 + 0.5 * (t1 - t0), tol).unwrap();
    let m = mesh::tessellate(&body, 0.1, tol).expect("split-rim donut meshes");
    mesh::validate::check_mesh(&m).expect("watertight");
    let mp = topo::mass_properties(&body, tol).expect("split-rim donut measures");
    println!(
        "PROBE split-rim donut: positions={} props={mp:?}",
        m.positions.len()
    );
}

/// The raw outcomes on the two witnesses, printed — run on a tree with
/// the door removed (or on main) to record the pre-door behaviour.
#[test]
fn report_keyway_and_lens_raw_outcomes() {
    let tol = Tol::witness();
    for (name, (body, face)) in [("keyway", keyway()), ("lens", oblique_lens())] {
        let got = mesh::tessellate(&body, 0.05, tol);
        match &got {
            Ok(m) => println!(
                "PROBE {name} face {face:?}: Ok positions={} triangles/patch={:?} check_mesh={:?}",
                m.positions.len(),
                m.patches
                    .iter()
                    .map(|p| p.triangles.len())
                    .collect::<Vec<_>>(),
                mesh::validate::check_mesh(m)
            ),
            Err(e) => println!("PROBE {name} face {face:?}: Err({e:?})"),
        }
        println!(
            "PROBE {name} mass_properties: {:?}",
            topo::mass_properties(&body, tol)
        );
    }
}

// R1-DOOR-ONLY-BEGIN
/// `TessellateError::Band` reachability: run with
/// `CAD_TOLERANCE_EPS` near `f64::MAX / K` to see which arm answers.
#[test]
fn report_band_arm_under_the_runs_eps() {
    let tol = Tol::witness();
    println!("PROBE eps = {:e}", tol.eps());
    let got = mesh::tessellate(&Body::<f64>::new(), 0.1, tol).map(|m| m.positions.len());
    println!("PROBE empty body at this eps: {got:?}");
    let is_band = matches!(got, Err(TessellateError::Band { .. }));
    println!("PROBE band arm reached: {is_band}");
}
// R1-DOOR-ONLY-END

/// **Issue 1571, closed at the door.** The half-cap's one meridian
/// edge is a great-circle ARC that crosses the north pole mid-edge.
/// The SHAPE door still admits both faces — props certifies the
/// CARRIER (`props_meridian_great`) and folds the pole into the
/// extent, so `mass_properties` still answers, and what it answers is
/// this body's own defect: 0.0 for a closed unit sphere, because both
/// faces are bounded by the same two edges traversed opposite ways and
/// their fluxes cancel (issue 1598; the three-face split measures the
/// exact 4π/3 — the rows above print both). What has changed is that
/// the BRANCH door now stands beside the shape door in front of the
/// walk: `props::require_one_chart_branch` refuses the traversed arc
/// typed, at every δ, before a mesh is minted, so the walk no longer
/// reads the pole's `u` through `mid_azimuth` and no
/// `CertificateExceeded` — a refusal about the chord budget standing
/// in for a refusal about the premise — is reachable here.
///
/// The π itself is still REPORTED rather than asserted on, by
/// `topo::examine_chart_coherence` (issue 868), and
/// `topo/tests/mesh8_coherence.rs` is where this body's half-turn is
/// pinned as a value. The refusal's own rows are
/// `mesh/tests/mesh11_arc_branch.rs`; this one keeps the SEAM the
/// review found — shape door yes, branch door no, on one face.
#[test]
fn a_pole_crossing_arc_is_refused_by_the_branch_door_not_the_shape_door() {
    let (body, cap, rest) = pole_crossing_half_cap();
    let tol = Tol::witness();
    assert_eq!(door_verdict(&body, cap), Ok(()));
    assert_eq!(door_verdict(&body, rest), Ok(()));
    let mp = topo::mass_properties(&body, tol).expect("props answers for the pole-crossing body");
    assert_eq!(mp.volume, 0.0, "issue 1598, unmoved by the branch door");
    let got = mesh::tessellate(&body, 0.1, tol).map(|_| ());
    assert!(
        matches!(
            got,
            Err(TessellateError::UnsupportedCurvedShape {
                source: geom_brep::props::PropsError::NotOneChartBranch { .. },
                ..
            })
        ),
        "issue 1571: the arc premise is verified at props and refused typed; got {got:?}"
    );
}
