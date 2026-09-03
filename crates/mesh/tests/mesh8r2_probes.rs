//! **R2 review probes for MESH-8** (issue 868, the coherence-detector
//! relocation). Independent instruments, written to falsify the PR's
//! claims rather than to re-assert them.
//!
//! Every row here PRINTS what it measured; the assertions are only on
//! the parts a reviewer is confident about, so a row that goes red is
//! a finding and not a re-baselining chore.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

test_utils::gated_to![
    "crates/mesh/src/",
    "crates/topo/src/coherence.rs",
    "crates/topo/src/splitting/",
];

mod common;
use common::witness_bodies::{keyway, oblique_lens, slit};
use common::*;
use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, FaceSurface, MefSite, MevSite};

// ---------------------------------------------------------------
// D9: an INDEPENDENT byte digest (claim 4)
// ---------------------------------------------------------------

/// A different mixer from `r2_bytes`' FNV-1a, on purpose: two
/// instruments agreeing is evidence, one instrument run twice is a
/// tautology. 64-bit xorshift-multiply over every position bit
/// pattern, every index and every boundary id, order-sensitive.
fn mixdigest(words: impl IntoIterator<Item = u64>) -> u64 {
    let mut h: u64 = 0x243f_6a88_85a3_08d3;
    for w in words {
        h ^= w.wrapping_add(0x9e37_79b9_7f4a_7c15);
        h = h.rotate_left(23).wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 29;
    }
    h
}

fn body_tour() -> Vec<(&'static str, Body<f64>)> {
    vec![
        ("ball", ball()),
        ("cone", cone()),
        ("l_prism", l_prism()),
        ("washer", washer()),
        ("donut", donut()),
        ("sphere_wedge", sphere_wedge(2.0)),
        ("cone_wedge", cone_wedge(2.0, 1.3)),
        ("rounded_prism", rounded_prism()),
        ("holed_prism", holed_prism()),
        ("wedge", wedge()),
        ("axis_wedge", axis_wedge()),
    ]
}

/// **The D9 leg, independently.** Print one line per (body, δ): the
/// counts and a digest over the mesh's bytes. The detectors gated
/// nothing, so every line must be identical at the merge base and at
/// the head — diff the two runs' output.
#[test]
fn r2r_independent_byte_digest() {
    println!("# eps = {:e}", Tol::witness().eps());
    for (name, b) in body_tour() {
        for d in [0.1f64, 0.02, 0.004] {
            let m = mesh::tessellate(&b, d, Tol::witness()).unwrap();
            let mut words: Vec<u64> = Vec::new();
            for p in &m.positions {
                words.push(p.x.to_bits());
                words.push(p.y.to_bits());
                words.push(p.z.to_bits());
            }
            let mut tris = 0usize;
            for patch in &m.patches {
                tris += patch.triangles.len();
                for t in &patch.triangles {
                    words.extend(t.iter().map(|i| u64::from(*i)));
                }
            }
            for pl in &m.boundaries {
                words.push(pl.points.len() as u64);
                words.extend(pl.points.iter().map(|i| u64::from(*i)));
            }
            println!(
                "DIGEST {name} d={d} pos={} tri={tris} bnd={} h={:016x}",
                m.positions.len(),
                m.boundaries.len(),
                mixdigest(words)
            );
        }
    }
}

// ---------------------------------------------------------------
// The examination on a WIDER corpus than the unit's 14 (claim 6)
// ---------------------------------------------------------------

/// A tilted split of a cylinder: the section is an ELLIPSE, so both
/// halves mesh through the pcurve-driven trimmed lane rather than
/// through the iso walk. `m5_pr11_trimmed` pins that they tessellate
/// watertight at every δ.
fn tilted_halves() -> (Body<f64>, Body<f64>) {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(-1.0, 0.0), 1.0),
        ProfileVertex::new(p2(1.0, 0.0), 1.0),
    ]);
    let disc = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let cylinder = extrude(&disc, Extrusion::Distance(2.5), Tol::witness())
        .unwrap()
        .body;
    let plane = topo::splitting::SplitPlane {
        origin: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos()),
    };
    let r = topo::splitting::split(&cylinder, &plane, Tol::witness()).unwrap();
    let (topo::splitting::SplitPart::Body(a), topo::splitting::SplitPart::Body(b)) =
        (&r.above, &r.below)
    else {
        panic!("both sides carry material");
    };
    (a.clone(), b.clone())
}

/// **Does the examination stay quiet on bodies that MESH CLEANLY but
/// are not in the unit's 14-body corpus?** The corpus row asserts
/// `0 findings, 0 unexamined` over bodies the iso walk handles; the
/// examination has no shape door in front of it, so it also reads
/// faces the walk never walks.
///
/// This row PRINTS rather than asserts: a nonzero count here is a
/// finding about the report's reach, not necessarily a defect.
#[test]
fn r2r_the_examination_on_bodies_outside_the_units_corpus() {
    let tol = Tol::witness();
    let (above, below) = tilted_halves();
    let corpus: Vec<(&str, Body<f64>)> = vec![
        ("tilted_half_above", above),
        ("tilted_half_below", below),
        ("keyway", keyway().0),
        ("oblique_lens", oblique_lens().0),
        ("slit", slit().0),
    ];
    for (name, body) in &corpus {
        let report = topo::examine_chart_coherence(body, tol);
        let meshes = mesh::tessellate(body, 0.05, tol).is_ok();
        println!(
            "WIDER {name}: meshes={meshes} findings={} unexamined={}",
            report.findings.len(),
            report.unexamined.len()
        );
        for f in &report.findings {
            println!("   finding {:?} metres={:e}", f.condition, f.metres);
        }
        for u in &report.unexamined {
            println!("   unexamined {:?}", u.why);
        }
    }
}

// ---------------------------------------------------------------
// The behaviour change: what was loud, what is now quiet (claim 8)
// ---------------------------------------------------------------

/// The `nist_ftc_09` shape at an arbitrary scale: a cylinder sliver
/// whose meridian side is a LINE whose two endpoints the source states
/// non-co-azimuthally, opening `metres` of arc at the closure.
///
/// Same construction as the unit's `chord_wobble`, re-derived here so
/// the two are independent instruments over one shape.
fn wobbled_sliver(metres: f64) -> Option<Body<f64>> {
    let tol = Tol::witness();
    let rr = 0.05_f64;
    let theta = 2.0 * metres / rr;
    let a = Point3::new(rr, 0.0, 0.0);
    let b = Point3::new(rr * theta.cos(), rr * theta.sin(), 0.0);
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rr,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(
        seed.face,
        FaceSurface::New(Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: rr,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }),
    )
    .unwrap();
    let arc = EdgeCurveSpec::arc_of_circle(rim, 0.0, theta)?;
    let e = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            arc,
            tol,
        )
        .ok()?;
    body.mef(
        MefSite::Chords {
            he1: e.he_minus,
            he2: e.he_plus,
        },
        EdgeCurveSpec::line_between(b, a),
        FaceSurface::Inherit,
        tol,
    )
    .ok()?;
    Some(body)
}

/// **Is there a body the deleted assertion caught that nothing catches
/// now?** Sweep the closure gap from just over the band to 1024 ε and
/// print, per scale, whether `tessellate` succeeds and what the
/// examination says.
///
/// At the merge base the same row shows what the mesh-side
/// `debug_assert` did on each of those bodies — run it at both
/// revisions and read the two outputs side by side.
#[test]
fn r2r_the_wobble_scale_sweep() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for k in [1.0f64, 2.0, 8.0, 64.0, 1024.0, 65536.0] {
        let Some(body) = wobbled_sliver(k * eps) else {
            println!("WOBBLE k={k} eps={eps:e} NOT CONSTRUCTIBLE (the arc does not certify)");
            continue;
        };
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mesh::tessellate(&body, 0.01, tol)
        }));
        let mesh_says = match &outcome {
            Ok(Ok(m)) => format!(
                "Ok({} tris)",
                m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
            ),
            Ok(Err(e)) => format!("{e:?}"),
            Err(_) => "PANIC".to_string(),
        };
        let report = topo::examine_chart_coherence(&body, tol);
        println!(
            "WOBBLE k={k} eps={eps:e} tessellate={mesh_says} findings={}",
            report.findings.len()
        );
    }
}

// ---------------------------------------------------------------
// Issue 1571's body through `tessellate` — the BEFORE/AFTER row
// (claims 5 and 8)
// ---------------------------------------------------------------

/// **Issue 1571's witness, re-derived here.** A unit sphere with a rim
/// at `z = 0.5` and ONE great-circle arc from the rim's `u = π` end
/// OVER the north pole to its `u = 0` end.
///
/// **Citation corrected (MESH-11):** this used to call itself "one of
/// four declared copies" and point at an ADOPTION NOTE in
/// `topo/tests/mesh8_coherence.rs` giving the reason there is no
/// single home. No such note exists there — the pointer was to a
/// document that was never written. What is true, and countable: the
/// body is declared as a `Body` in three places — `mesh/tests/common/
/// witness_bodies.rs` (the shared home MESH-11 made, which
/// `mesh7r1_probes` and `mesh11_arc_branch` both use), this file's
/// copy, and `topo/tests/mesh8_coherence.rs`'s — plus twice as
/// hand-built `LoopEdge` loops for the props doors
/// (`geom-brep/tests/cert1_sphere_polar.rs`,
/// `geom-brep/tests/mesh11_arc_branch.rs`), which are a different
/// thing and cannot share the `Body` home. The reason THIS copy stays
/// separate is the reviewer's own and is stated where it belongs: an
/// independent instrument that re-derives its subject is evidence in
/// a way one that imports it is not.
fn pole_crossing_half_cap() -> Body<f64> {
    let tol = Tol::witness();
    let rim_r = (1.0f64 - 0.25).sqrt();
    let a = Point3::new(rim_r, 0.0, 0.5);
    let b = Point3::new(-rim_r, 0.0, 0.5);
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let rim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.5),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: rim_r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let great = |axis: Vec3<f64>| Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis,
        radius: 1.0,
        u_ref: Vec3::new(-rim_r, 0.0, 0.5),
    };
    let mut g = great(Vec3::new(0.0, 1.0, 0.0));
    if g.eval(core::f64::consts::FRAC_PI_2).z < 0.5 {
        g = great(Vec3::new(0.0, -1.0, 0.0));
    }
    let t_end = g.param_near(a, 0.0).unwrap();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(sphere))
        .unwrap();
    let e_rim = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            EdgeCurveSpec::arc_of_circle(rim, 0.0, core::f64::consts::PI).unwrap(),
            tol,
        )
        .unwrap();
    body.mef(
        MefSite::Chords {
            he1: e_rim.he_minus,
            he2: e_rim.he_plus,
        },
        EdgeCurveSpec::arc_of_circle(g, 0.0, t_end).unwrap(),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    body
}

/// **What `tessellate` does on the π-rad witness, at both δ rows.**
///
/// At the merge base this row prints the `closing_column` panic and
/// its numbers at δ = 0.5; at this head it must print whatever the
/// deletion leaves — an `Ok` mesh, a typed refusal, or a panic from a
/// DIFFERENT guard (the issue-897 census). Run it with debug
/// assertions on and off.
///
/// It also prints what the relocated examination reports on the same
/// body, so the two halves of the relocation are read side by side.
#[test]
fn r2r_the_1571_body_through_tessellate() {
    let tol = Tol::witness();
    let body = pole_crossing_half_cap();
    println!("DEBUG_ASSERTIONS = {}", cfg!(debug_assertions));
    for d in [0.5f64, 0.1] {
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let out = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            mesh::tessellate(&body, d, tol)
        }));
        std::panic::set_hook(hook);
        let says = match out {
            Ok(Ok(m)) => {
                let watertight = mesh::validate::check_mesh(&m).is_ok();
                format!(
                    "Ok({} tris, watertight={watertight})",
                    m.patches.iter().map(|p| p.triangles.len()).sum::<usize>()
                )
            }
            Ok(Err(e)) => format!("Err({e:?})"),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_default();
                format!("PANIC({msg})")
            }
        };
        println!("W1571 delta={d} eps={:e} {says}", tol.eps());
    }
    let report = topo::examine_chart_coherence(&body, tol);
    println!(
        "W1571 examination: findings={} unexamined={}",
        report.findings.len(),
        report.unexamined.len()
    );
    for f in &report.findings {
        println!(
            "   {:?} gap={} lever={} metres={} eps={:e}",
            f.condition, f.gap, f.lever, f.metres, f.eps
        );
    }
}
