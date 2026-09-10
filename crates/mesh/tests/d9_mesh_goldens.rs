//! **The D9 mesh contract, made checkable: committed digests of the
//! whole `Mesh` value for a named corpus at two δ each.**
//!
//! D9 promises a byte-identical mesh for identical `(body, δ, ε)`. The
//! rows elsewhere in this suite check PROPERTIES of a mesh
//! (watertightness, volume, certificate bounds); none of them pins the
//! bytes, so a refactor that reorders ids or renumbers a patch passes
//! all of them while breaking the promise. This row pins the bytes: an
//! FNV-1a digest over every position's bit pattern, every patch's face
//! key and triangle indices, and every boundary polyline, against a
//! constant committed beside it.
//!
//! **A moved digest is not a re-cut.** It says the mesh a body produces
//! changed. Decide whether the new mesh is right; if it is, move the
//! constant in the same change that moved the mesh and say in the PR
//! which bodies moved and why. The way to tell a mesh change from a
//! renumbering is to run this row on the tree WITHOUT the change and
//! on the tree with it — the constants are one tree's reading, and the
//! comparison is between trees.
//!
//! **Corpus coverage**, one line per lane the dispatch in
//! `tessellate` can take:
//!
//! - planar (`planar::tessellate_planar`): every body here has caps;
//!   `holed_prism` carries a ring, `wedge`/`axis_wedge` the degenerate
//!   corners.
//! - curved (`curved::tessellate_curved`), one body per `ChartKind`:
//!   `rounded_prism` + `washer` (Cylinder — the washer is two full
//!   cylinder walls closed by planar annuli, not a torus), `ball` +
//!   `sphere_wedge` (Sphere), `cone` + `cone_wedge` (Cone), `donut`
//!   (Torus).
//! - poles and apexes: `ball`'s two poles, `cone`'s apex,
//!   `pole_crossing_half_cap` and `apex_crossing_bowtie` — the
//!   witnesses whose meridian sides cross the singularity.
//!
//! **What the cylinder rows do NOT pin.** `grid_counts`' cylinder arm
//! answers `(nu, 1)` — a cylinder is ruled in v, so no interior row
//! exists — and the interior loop runs `1..nv`. A cylinder face
//! therefore mints no interior points at all, and its digest says
//! nothing about how interior ids are numbered. The bodies that do
//! carry that are the sphere, cone, torus and NURBS rows.
//! - trimmed (`trimmed::tessellate_trimmed`) on an ANALYTIC carrier:
//!   `tilted_above` / `tilted_below`, the two halves of a cylinder cut
//!   by an oblique plane — the cut rim is an ellipse, so
//!   `has_trim_carrier` routes the wall through the pcurve-driven lane
//!   with `cert::cert_cylinder` certificates.
//! - trimmed on NURBS: `loft_prism` (walls of degree 1×2) and
//!   `swept_elbow` (1×3), the described-NURBS lane with its
//!   hull-derived certificate and its cert-driven refinement. A
//!   `Surface::Approx` face shares the `Nurbs` dispatch arm and the
//!   same lane, on its fit, so these rows cover its code path; what no
//!   fixture in this crate carries is a body that reaches it through
//!   the `Approx` half of the arm.
//! - typed refusals: `keyway`, `slit`, `oblique_lens`,
//!   `pole_crossing_half_cap` and `apex_crossing_bowtie` mesh to no
//!   triangles at all. Their digest is the refusal, so a change that
//!   turns one into a mesh — or into a DIFFERENT refusal — moves it.
//!   δ does not reach the doors that refuse them, which is why their
//!   two slots agree.
//!
//! **ε.** ONE table, asserted at whatever ε the run committed. Every
//! digest below was taken at 1e-6, 1e-9 and 1e-12 — the three rows the
//! CI matrix gates — and all three agree, which is the claim
//! `review_m2_pr6_determinism::survives_eps_row_bitwise_independence`
//! makes and argues (the mesh is bitwise a function of `(body, δ)`
//! because ε's one structural consumer, pole/apex identification, is
//! unexercised by these bodies — unexercised, not impossible). This
//! corpus extends that measurement over the trimmed and NURBS bodies
//! that row does not carry. Asserting unconditionally is what keeps it
//! a standing claim: an ε-sensitivity anywhere this corpus reaches
//! turns the row red on the ε row that has it, and the failure message
//! names the committed ε.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::Body;

use crate::common;
use common::witness_bodies;
use common::*;

fn fnv(h: &mut u64, x: u64) {
    for b in x.to_le_bytes() {
        *h ^= u64::from(b);
        *h = h.wrapping_mul(0x0100_0000_01b3);
    }
}

fn fnv_str(h: &mut u64, s: &str) {
    fnv(h, s.len() as u64);
    for b in s.bytes() {
        fnv(h, u64::from(b));
    }
}

/// Every byte of the mesh value: positions, patches (face key and
/// triangle indices, in patch order) and boundaries (edge key,
/// polyline ids, endpoint vertex ids, in boundary order). A refusal
/// hashes its own typed `Debug`, so a body that stops meshing — or
/// starts — moves the digest.
fn digest(body: &Body<f64>, delta: f64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    match mesh::tessellate(body, delta, Tol::witness()) {
        Ok(m) => {
            fnv(&mut h, m.positions.len() as u64);
            for p in &m.positions {
                fnv(&mut h, p.x.to_bits());
                fnv(&mut h, p.y.to_bits());
                fnv(&mut h, p.z.to_bits());
            }
            fnv(&mut h, m.patches.len() as u64);
            for q in &m.patches {
                fnv_str(&mut h, &format!("{:?}", q.face));
                fnv(&mut h, q.triangles.len() as u64);
                for t in &q.triangles {
                    fnv(&mut h, u64::from(t[0]));
                    fnv(&mut h, u64::from(t[1]));
                    fnv(&mut h, u64::from(t[2]));
                }
            }
            fnv(&mut h, m.boundaries.len() as u64);
            for b in &m.boundaries {
                fnv_str(&mut h, &format!("{:?}", b.edge));
                fnv(&mut h, b.points.len() as u64);
                for id in &b.points {
                    fnv(&mut h, u64::from(*id));
                }
                fnv_str(&mut h, &format!("{:?}", b.start_vertex));
                fnv_str(&mut h, &format!("{:?}", b.end_vertex));
            }
        }
        Err(e) => {
            fnv(&mut h, 0xdead_beef);
            fnv_str(&mut h, &format!("{e:?}"));
        }
    }
    h
}

/// The corpus: name, body, and the two δ it is digested at. The
/// NURBS-walled bodies take the δ their own acceptance rows use — a
/// coarse δ there refuses on the certificate, which is a digest too
/// but a less informative one.
fn corpus() -> Vec<(&'static str, Body<f64>, [f64; 2])> {
    vec![
        ("l_prism", l_prism(), [0.05, 0.15]),
        ("holed_prism", holed_prism(), [0.05, 0.15]),
        ("wedge", wedge(), [0.05, 0.15]),
        ("axis_wedge", axis_wedge(), [0.05, 0.15]),
        ("rounded_prism", rounded_prism(), [0.05, 0.15]),
        ("ball", ball(), [0.05, 0.15]),
        ("sphere_wedge", sphere_wedge(1.2), [0.05, 0.15]),
        ("cone", cone(), [0.05, 0.15]),
        ("cone_wedge", cone_wedge(1.0, 1.2), [0.05, 0.15]),
        ("donut", donut(), [0.05, 0.15]),
        ("washer", washer(), [0.05, 0.15]),
        ("tilted_above", tilted_halves().0, [0.1, 0.01]),
        ("tilted_below", tilted_halves().1, [0.1, 0.01]),
        ("keyway", witness_bodies::keyway().0, [0.05, 0.15]),
        ("slit", witness_bodies::slit().0, [0.05, 0.15]),
        (
            "oblique_lens",
            witness_bodies::oblique_lens().0,
            [0.05, 0.15],
        ),
        (
            "pole_crossing_half_cap",
            witness_bodies::pole_crossing_half_cap().0,
            [0.05, 0.15],
        ),
        (
            "apex_crossing_bowtie",
            witness_bodies::apex_crossing_bowtie().0,
            [0.05, 0.15],
        ),
        ("loft_prism", nurbs_bodies::loft_prism(), [6e-3, 2e-2]),
        (
            "swept_elbow",
            sweep::test_support::swept_elbow(Tol::witness()),
            [3e-3, 1e-2],
        ),
    ]
}

/// The oblique cut of a cylinder, whose two halves carry an elliptical
/// trim rim on an analytic wall — `m5_pr11_trimmed`'s fixture, whose
/// header carries its provenance.
fn tilted_halves() -> (Body<f64>, Body<f64>) {
    use geom_core::{Point2, Point3, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
    use sweep::{Extrusion, extrude};
    use topo::splitting::{SplitPart, SplitPlane, split};

    const R: f64 = 1.0;
    const H: f64 = 2.5;
    const PHI: f64 = 0.3;

    let disc = Profile::new(
        SketchPlane::xy(),
        vec![ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(-R, 0.0), 1.0),
            ProfileVertex::new(Point2::new(R, 0.0), 1.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the disc validates");
    let cylinder = extrude(&disc, Extrusion::Distance(H), Tol::witness())
        .expect("the disc extrudes")
        .body;
    let plane = SplitPlane {
        origin: Point3::new(0.0, 0.0, H / 2.0),
        normal: Vec3::new(PHI.sin(), 0.0, PHI.cos()),
    };
    let result = split(&cylinder, &plane, Tol::witness()).expect("the oblique cut splits");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides of the oblique cut carry material");
    };
    (above.clone(), below.clone())
}

/// The NURBS-walled corpus bodies, built the way `m7_nurbs_trimmed`
/// builds them (its own header carries the provenance of each).
mod nurbs_bodies {
    use geom_core::{Affine3, Tol, Vec3};
    use sweep::loft_body;
    use topo::Body;

    use crate::common::quad;

    pub(super) fn loft_prism() -> Body<f64> {
        let sections = vec![
            quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
            quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
            quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        ];
        let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.0]
            .iter()
            .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
            .collect();
        loft_body::<f64>(&sections, &places, 2, Tol::witness())
            .expect("the loft builds")
            .body
    }
}

/// The committed digest of each corpus body at each of its two δ, in
/// corpus order. Taken on the tree that introduced this row; see the
/// module docs for what moving one means.
const GOLDEN: &[(&str, [u64; 2])] = &[
    ("l_prism", [0x56fe_6d85_7e52_407c, 0x56fe_6d85_7e52_407c]),
    (
        "holed_prism",
        [0x7f08_3826_6bd7_5197, 0x7f08_3826_6bd7_5197],
    ),
    ("wedge", [0x9b26_5e0c_a25b_fb06, 0x4ac0_18e4_90e4_602f]),
    ("axis_wedge", [0x240f_d14f_cb3f_6bea, 0xe181_9a80_1d61_3a02]),
    (
        "rounded_prism",
        [0xaf9b_9d73_07df_fbef, 0x0948_28db_26f7_5f65],
    ),
    ("ball", [0x3e4a_62a1_f1e3_d045, 0xefdc_3024_8193_e9e3]),
    (
        "sphere_wedge",
        [0x4dcf_6a46_d7e4_8afd, 0xdfa9_7fcf_d5a6_1f91],
    ),
    ("cone", [0xc8ac_e3fb_a915_38fa, 0x6376_0636_4757_7532]),
    ("cone_wedge", [0x2a10_1aee_9f5a_1b91, 0x83f2_5aac_9243_68ce]),
    ("donut", [0x7673_a909_57aa_a0f3, 0xe549_2aa4_78a6_d185]),
    ("washer", [0xb5e6_4707_7081_1521, 0xd261_a4a3_e2d2_f19d]),
    (
        "tilted_above",
        [0x05de_30f5_be30_29e1, 0xb957_d220_dd6d_cd8e],
    ),
    (
        "tilted_below",
        [0xedb7_92c4_4d03_cfde, 0xd33f_a391_6fc1_9740],
    ),
    ("keyway", [0x9d8e_44fd_e323_88de, 0x9d8e_44fd_e323_88de]),
    ("slit", [0xaed6_09ed_f923_833b, 0xaed6_09ed_f923_833b]),
    (
        "oblique_lens",
        [0xfdb2_0be5_e9aa_8077, 0xfdb2_0be5_e9aa_8077],
    ),
    (
        "pole_crossing_half_cap",
        [0xdd6f_a9f2_0af3_083a, 0xdd6f_a9f2_0af3_083a],
    ),
    (
        "apex_crossing_bowtie",
        [0xd64f_3d06_9373_61f5, 0xd64f_3d06_9373_61f5],
    ),
    ("loft_prism", [0x2d6a_6bd0_bdce_2300, 0x6109_b327_f166_6647]),
    (
        "swept_elbow",
        [0xd90e_27e6_5762_0090, 0xb1a5_2588_0e2c_329d],
    ),
];

#[test]
fn every_corpus_body_meshes_to_its_committed_digest() {
    let eps = common::eps();
    let corpus = corpus();
    assert_eq!(
        GOLDEN.len(),
        corpus.len(),
        "the committed digest table and the corpus must name the same bodies"
    );
    let mut moved: Vec<String> = Vec::new();
    for ((name, body, deltas), (gname, want)) in corpus.iter().zip(GOLDEN) {
        assert_eq!(name, gname, "corpus and golden table disagree on order");
        for (k, &delta) in deltas.iter().enumerate() {
            let got = digest(body, delta);
            if got != want[k] {
                moved.push(format!(
                    "{name} at delta={delta}: {got:016x}, committed {:016x}",
                    want[k]
                ));
            }
        }
    }
    assert!(
        moved.is_empty(),
        "the mesh moved for {} of {} body/δ pairs at eps={eps}:\n  {}",
        moved.len(),
        2 * corpus.len(),
        moved.join("\n  ")
    );
}
