//! **Reviewer bit-identity dump (claim C1 of every blend PR).** Builds
//! the fixtures a PR claims are bit-identical — the die (open chains +
//! corners), the pipped die's pip rim (the N-link closed LADDER path),
//! the chamfered cube, and one convex closed rim per coaxial arm family
//! the annulus door reaches (the dome's plane–sphere equator, the
//! sphere zone's sphere–sphere rim pair, the lantern's sphere–plane,
//! sphere–cone and cone–plane rims, the waisted body's cone–plane rims)
//! — and writes a bit-faithful text dump of every output body to
//! `$BITDUMP_DIR/<name>.txt`. Run at the merge base and at the head,
//! then `diff` the files: any moved bit shows as a text change
//! (shortest-roundtrip f64 formatting is injective on non-NaN
//! values, and `-0.0` prints signed).
//!
//! **Unarmed by default.** With `BITDUMP_DIR` unset every row returns
//! immediately — an explicit clean skip, so the suite is neither a red
//! nor a silent green in the aggregated matrix. See `dump_dir` for why
//! an environment read is admissible in this file at all.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    dead_code,
    missing_docs
)]

use std::fmt::Write as _;

use geom::Surface;
use geom_brep::SurfaceKind;
use geom_core::{Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::Revolution;
use sweep::blend::build::fillet_edges;
use sweep::chamfer::chamfer_edges;
use sweep::test_support::{
    ball_poled_z, closed_plane_sphere_rim, cube, dome, lantern, rim_arcs_at, sphere_zone, waisted,
};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::query::{self, SurfaceKindSet};
use topo::{Body, BooleanDeclarations, EdgeKey};

/// Dump one body, bit for bit, in key iteration order (identical
/// operation sequences produce identical key orders).
fn dump(body: &Body<f64>) -> String {
    let mut s = String::new();
    let _ = writeln!(
        s,
        "census V={} E={} F={}",
        body.vertices().count(),
        body.edges().count(),
        body.faces().count()
    );
    for (k, _) in body.vertices() {
        let p = body
            .get_vertex(k)
            .and_then(|v| body.get_point(v.point))
            .unwrap();
        let _ = writeln!(s, "V {k:?} ({:?}, {:?}, {:?})", p.x, p.y, p.z);
    }
    for (k, e) in body.edges() {
        let _ = write!(s, "E {k:?} he+={:?} he-={:?}", e.he_plus, e.he_minus);
        match body.get_curve_geom(e.curve).and_then(|g| g.certified()) {
            Some(c) => {
                let (t0, t1) = c.params();
                let _ = writeln!(
                    s,
                    " carrier={:?} params=({t0:?}, {t1:?}) desc={:?}",
                    c.carrier(),
                    c.description()
                );
            }
            None => {
                let _ = writeln!(s, " UNCERTIFIED");
            }
        }
    }
    for (k, _) in body.faces() {
        let fd = body.get_face(k).unwrap();
        let surf = body.get_surface(fd.surface).unwrap();
        let _ = writeln!(
            s,
            "F {k:?} sense={:?} rings={} surface={surf:?}",
            fd.sense,
            fd.rings.len()
        );
    }
    let props = topo::mass_properties(body, Tol::witness()).unwrap();
    let _ = writeln!(
        s,
        "props volume={:?} pad={:?} area={:?} apad={:?}",
        props.volume, props.volume_pad, props.surface_area, props.area_pad
    );
    s
}

/// The dump directory, or `None` when this suite is not armed.
///
/// **Why an env read is admissible here, stated rather than assumed**
/// (the fix pass; `sweep`'s manifest warns that a suite rolling its own
/// dial would be a second `CAD_FUZZ`-style channel). The gate that bans
/// ambient environment scans `crates/*/src` and this is a `tests/`
/// file, so no shipped build can reach it — the same REACHABILITY
/// argument that allowlists `test-utils`' fuzz dial. And unlike a dial,
/// this one gates no assertion: armed, the rows write a file and assert
/// nothing about it; unarmed, they return before building anything. It
/// selects an artifact's destination, never a behaviour.
fn dump_dir() -> Option<String> {
    std::env::var("BITDUMP_DIR").ok().filter(|d| !d.is_empty())
}

fn save(dir: &str, name: &str, text: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{dir}/{name}.txt"), text).unwrap();
}

// --- fixtures, verbatim from the merge-base suites -----------------

fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&k| {
            query::edge_adjacent_matches(
                body,
                k,
                SurfaceKindSet::just(SurfaceKind::Plane),
                SurfaceKindSet::just(SurfaceKind::Sphere),
            )
        })
        .collect()
}

fn pipped_die() -> (Body<f64>, Vec<EdgeKey>, Vec<EdgeKey>) {
    const DIE_L: f64 = 1.0;
    const PIP_R: f64 = 0.09;
    const PIP_H: f64 = 0.05;
    let cube0 = cube(DIE_L, Tol::witness());
    let box_keys: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pip = ball_poled_z(
        PIP_R,
        Vec3::new(0.5, 0.5, DIE_L + (PIP_R - PIP_H)),
        Tol::witness(),
    );
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &cube0,
        &pip,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    )
    .unwrap()
    .body()
    .expect("a body")
    .body
    .clone();
    let box_edges: Vec<_> = box_keys
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    let rims = rim_edges(&pipped);
    (pipped, box_edges, rims)
}

// --- the dumps -----------------------------------------------------

/// The die: twelve open chains + eight corners, fillet r = 0.15.
#[test]
fn bitdump_die() {
    // An explicit CLEAN SKIP when unarmed: this row must never enter
    // the aggregated matrix as a red (a panicking env read) or as a
    // silent green (a body built and nothing checked).
    let Some(dir) = dump_dir() else {
        return;
    };
    let body = cube(1.0, Tol::witness());
    let out = fillet_edges(&body, &query::all_edges(&body), 0.15, Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save(&dir, "die", &text);
}

/// The pip rim: the two-arc closed LADDER chain plus the twelve box
/// edges, in one call (the F-e form), fillet r = 0.05.
#[test]
fn bitdump_pip_rims() {
    // An explicit CLEAN SKIP when unarmed: this row must never enter
    // the aggregated matrix as a red (a panicking env read) or as a
    // silent green (a body built and nothing checked).
    let Some(dir) = dump_dir() else {
        return;
    };
    let (pipped, box_edges, rims) = pipped_die();
    assert_eq!(rims.len(), 2, "the pip rim is two arcs");
    let mut all = box_edges;
    all.extend(rims);
    let out = fillet_edges(&pipped, &all, 0.05, Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save(&dir, "pip_rims", &text);
}

/// The chamfered cube: twelve strips + eight corner planes, d = 0.1.
#[test]
fn bitdump_chamfered_cube() {
    // An explicit CLEAN SKIP when unarmed: this row must never enter
    // the aggregated matrix as a red (a panicking env read) or as a
    // silent green (a body built and nothing checked).
    let Some(dir) = dump_dir() else {
        return;
    };
    let body = cube(1.0, Tol::witness());
    let out = chamfer_edges(&body, &query::all_edges(&body), 0.1, Tol::witness()).unwrap();
    let mut text = dump(&out.body);
    let _ = writeln!(
        text,
        "blend={:?} corner={:?} band={:?}",
        out.blend_faces, out.corner_faces, out.band_faces
    );
    save(&dir, "chamfered_cube", &text);
}

/// One rim's carve, dumped with its band face named.
fn dump_rim(name: &str, body: &Body<f64>, arcs: &[EdgeKey], r: f64) -> String {
    let out = fillet_edges(body, arcs, r, Tol::witness())
        .unwrap_or_else(|e| panic!("{name} carves on both sides of the differential: {e:?}"));
    let mut text = format!("== {name} ==\n");
    text.push_str(&dump(&out.body));
    let _ = writeln!(text, "band={:?}", out.band_faces);
    text
}

/// **Every convex closed rim the annulus door reaches, one per coaxial
/// arm family**: the dome's plane–sphere equator (one closed edge), the
/// sphere zone's two sphere–sphere rims in ONE call (the shared-wall
/// composition), the lantern's neck (sphere–plane), shoulder
/// (sphere–cone) and lip (cone–plane) rims — each seam-split — and the
/// waisted body's convex base and top (cone–plane) rims, whose concave
/// waist is the material-adding fixture. A change to how the arms rest
/// the ball, or to either rim walk, that moved any convex carve shows
/// here.
#[test]
fn bitdump_convex_closed_rims() {
    let Some(dir) = dump_dir() else {
        return;
    };
    let tol = Tol::witness();
    let r = 0.05;
    let mut text = String::new();
    let body = dome(1.0, tol);
    text.push_str(&dump_rim(
        "dome equator",
        &body,
        &[closed_plane_sphere_rim(&body, 1.0)],
        r,
    ));
    let body = sphere_zone(0.5, Revolution::Full, tol);
    let mut pair = rim_arcs_at(&body, 3.75f64.sqrt(), -0.5);
    pair.extend(rim_arcs_at(&body, 3f64.sqrt(), 1.0));
    assert_eq!(
        pair.len(),
        2,
        "the zone's two sphere rims are one closed edge each"
    );
    text.push_str(&dump_rim("sphere zone rim pair", &body, &pair, r));
    let body = lantern(tol);
    for (name, rim_r, rim_y) in [
        ("lantern neck", 1.0, 0.0),
        ("lantern shoulder", 0.8, 0.6),
        ("lantern lip", 0.2, 1.2),
    ] {
        let arcs = rim_arcs_at(&body, rim_r, rim_y);
        assert_eq!(arcs.len(), 2, "{name} is seam-split");
        text.push_str(&dump_rim(name, &body, &arcs, r));
    }
    let body = waisted(tol);
    for (name, rim_y) in [("waist base", 0.0), ("waist top", 1.0)] {
        let arcs = rim_arcs_at(&body, 1.0, rim_y);
        assert_eq!(arcs.len(), 2, "{name} is seam-split");
        text.push_str(&dump_rim(name, &body, &arcs, r));
    }
    save(&dir, "convex_closed_rims", &text);
}

// --- R2 review (ordinal 101, VERBS-SHELLFIX PR-1) -------------------

/// The `#1048` acceptance corpus of `shell_open`: the box cup and the
/// box TUBE (both caps designated). The PR claims a one-face unslit
/// chart canonicalises to itself and the box is **bit-identical**;
/// run this at the merge base and at the head and diff.
#[test]
fn bitdump_shell_open_box_corpus() {
    let Some(dir) = dump_dir() else {
        return;
    };
    let tol = Tol::witness();
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(2.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(2.0, 3.0), 0.0),
        ProfileVertex::new(Point2::new(0.0, 3.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let body = sweep::extrude(&profile, sweep::Extrusion::Distance(4.0), tol)
        .unwrap()
        .body;
    let cap_at = |b: &Body<f64>, z: f64| -> Vec<topo::FaceKey> {
        b.faces()
            .filter(|(_, f)| {
                matches!(b.get_surface(f.surface),
                    Some(Surface::Plane { origin, normal, .. })
                        if (origin.z - z).abs() < 1e-12
                            && normal.x.abs() < 1e-9
                            && normal.y.abs() < 1e-9)
            })
            .map(|(k, _)| k)
            .collect()
    };
    let top = cap_at(&body, 4.0);
    let bottom = cap_at(&body, 0.0);
    let both: Vec<topo::FaceKey> = top.iter().chain(&bottom).copied().collect();

    let mut text = String::new();
    let _ = writeln!(text, "== box cup (top designated, t = 0.25) ==");
    let cup = topo::shell_open(&body, 0.25, &top, 1e-6, tol).unwrap().body;
    text.push_str(&dump(&cup));
    let _ = writeln!(text, "== box tube (both caps designated, t = 0.25) ==");
    let tubey = topo::shell_open(&body, 0.25, &both, 1e-6, tol)
        .unwrap()
        .body;
    text.push_str(&dump(&tubey));
    let _ = writeln!(text, "== the SEALED box (t = 0.25) ==");
    let sealed = topo::shell(&body, 0.25, 1e-6, tol).unwrap().body;
    text.push_str(&dump(&sealed));
    save(&dir, "shell_open_box_corpus", &text);
}

/// The **extrude/revolve corpus**: every body kind whose construction
/// runs a description upgrade — extrude's cap rims and strut joins over
/// each profile leg kind, revolve's meridian and latitude joins over
/// each elementary wall. `dump` writes each edge's stored description,
/// so a PR claiming the descriptions do not move runs this at the merge
/// base and at the head and diffs the files.
#[test]
fn bitdump_extrude_revolve_corpus() {
    let Some(dir) = dump_dir() else {
        return;
    };
    let tol = Tol::witness();
    let p2 = Point2::<f64>::new;
    let b = core::f64::consts::FRAC_PI_8.tan();
    let extruded = |name: &str, loops: Vec<ProfileLoop<f64>>, h: f64| -> (String, Body<f64>) {
        let profile = Profile::new(SketchPlane::xy(), loops)
            .validate(tol)
            .unwrap();
        let body = sweep::extrude(&profile, sweep::Extrusion::Distance(h), tol)
            .unwrap()
            .body;
        (name.to_owned(), body)
    };
    let circle = |cx: f64, cy: f64, r: f64| {
        <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
            ProfileVertex::new(p2(cx - r, cy), 1.0),
            ProfileVertex::new(p2(cx + r, cy), 1.0),
        ])
    };

    let mut rows: Vec<(String, Body<f64>)> = vec![
        extruded(
            "L prism (all-line, one concave corner)",
            vec![ProfileLoop::polygon([
                p2(0.0, 0.0),
                p2(2.0, 0.0),
                p2(2.0, 1.0),
                p2(1.0, 1.0),
                p2(1.0, 2.0),
                p2(0.0, 2.0),
            ])],
            0.75,
        ),
        extruded(
            "cylinder (two semicircle arcs)",
            vec![circle(0.0, 0.0, 1.5)],
            2.0,
        ),
        extruded(
            "holed prism (square + circular ring)",
            vec![
                ProfileLoop::polygon([p2(0.0, 0.0), p2(2.0, 0.0), p2(2.0, 2.0), p2(0.0, 2.0)]),
                circle(1.0, 1.0, 0.5),
            ],
            1.0,
        ),
        extruded(
            "rounded square (tangent line-arc joins)",
            vec![
                <ProfileLoop<f64> as RawLoop<f64>>::new(vec![
                    ProfileVertex::new(p2(0.25, 0.0), 0.0),
                    ProfileVertex::new(p2(0.75, 0.0), b),
                    ProfileVertex::new(p2(1.0, 0.25), 0.0),
                    ProfileVertex::new(p2(1.0, 0.75), b),
                    ProfileVertex::new(p2(0.75, 1.0), 0.0),
                    ProfileVertex::new(p2(0.25, 1.0), b),
                    ProfileVertex::new(p2(0.0, 0.75), 0.0),
                    ProfileVertex::new(p2(0.0, 0.25), b),
                ])
                .with_tangent_joints(vec![0, 1, 2, 3, 4, 5, 6, 7]),
            ],
            0.5,
        ),
        extruded(
            "concave arc leg",
            vec![<ProfileLoop<f64> as RawLoop<f64>>::new(vec![
                ProfileVertex::new(p2(0.0, 0.0), 0.0),
                ProfileVertex::new(p2(3.0, 0.0), 0.0),
                ProfileVertex::new(p2(3.0, 2.0), 0.0),
                ProfileVertex::new(p2(0.0, 2.0), -0.4),
            ])],
            1.25,
        ),
    ];
    rows.push(("dome (plane-sphere equator)".to_owned(), dome(1.0, tol)));
    rows.push(("waisted (cone-plane rims)".to_owned(), waisted(tol)));
    rows.push((
        "sphere zone (sphere-sphere rim pair)".to_owned(),
        sphere_zone(0.4, Revolution::Full, tol),
    ));
    rows.push(("lantern (sphere-cone-plane)".to_owned(), lantern(tol)));
    rows.push((
        "poled ball (full revolve, meridian seam)".to_owned(),
        ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol),
    ));

    let mut text = String::new();
    for (name, body) in &rows {
        let _ = writeln!(text, "== {name} ==");
        text.push_str(&dump(body));
    }
    save(&dir, "extrude_revolve_corpus", &text);
}
