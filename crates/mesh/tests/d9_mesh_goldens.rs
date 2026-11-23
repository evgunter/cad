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
//!   (Torus), and the filleted holes' bands (thin torus strips with one
//!   slit whose chord points fall between the rows,
//!   `the_one_slit_band.rs`) at the δ their rows read.
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
//!   by an oblique plane — the cut rim is an ellipse, whose stored image
//!   is no iso line of the chart, so the route sends the wall through
//!   the pcurve-driven lane with `cert::cert_cylinder` certificates.
//!   `notched_wall` and `holed_wall` (`common::rowless::walls`: a pipe
//!   notched across its seam, and one with a bar through it) are walls
//!   bounded by rims and rulings, with a ring on the second; the lane lays
//!   no rows on a cylinder, so these and the tilted halves are its
//!   boundary-point triangulations, and their digests at the three ε are
//!   that triangulation's bytes, which no ε moves.
//! - trimmed on NURBS: `loft_prism` (walls of degree 1×2) and
//!   `swept_elbow` (1×3), the described-NURBS lane with its
//!   hull-derived certificate and its correction;
//!   `pinned_wedge` and `corner_pinned_through_three`, walls with a
//!   collapsed side, which the lane walks through its vertex. A
//!   `Surface::Approx` face shares the `Nurbs` dispatch arm and the
//!   same lane, on its fit, so these rows cover its code path; what no
//!   fixture in this crate carries is a body that reaches it through
//!   the `Approx` half of the arm.
//! - typed refusals: `keyway`, `slit`, `oblique_lens`,
//!   `pole_crossing_half_cap` and `apex_crossing_bowtie` mesh to no
//!   triangles at all. Hand-built, they store no chart image on their
//!   curved faces; the route sends a face its doors decide no rectangle
//!   to the chart lane, whose domain read finds no image, so each but
//!   the slit refuses `DomainUnread` where the kind route gave the
//!   doors' verdict (moved by PR 7041, `tesslane/route-by-domain`: the
//!   keyway from `0x9d8e_44fd_e323_88de`, the lens from
//!   `0xfdb2_0be5_e9aa_8077`, the half cap from `0xdd6f_a9f2_0af3_083a`,
//!   the bow tie from `0x9cdd_b7a3_543d_b550`). The lens and the half cap
//!   digest alike (`0x72fb_aee1_b444_dff6`): the digest reads a refusal
//!   by its `Debug` alone, and both refuse `DomainUnread` on face `1v1`
//!   at half-edge `3v1`, keys their hand-built Euler sequences assign
//!   alike, so the two refusals are equal in every field it reads. Their
//!   minted forms, which refuse differently, are what tells them apart
//!   here. The same four with their
//!   images minted (`witness_bodies::minted`), as a finished body
//!   carries them, are bodies of their own (`*_minted`): the keyway's
//!   complement wound against its face (the keyway face itself meshes),
//!   the lens's outer loop open in the chart, the half cap's branch
//!   premise and the bow tie's rim sides, the last two the kind route's
//!   digests. Their digest is the refusal, so a change that turns one
//!   into a mesh — or into a DIFFERENT refusal — moves it.
//!   δ does not reach the doors that refuse them, which is why their
//!   two slots agree.
//!
//! **Thread count.** `the_corpus_digests_the_same_bytes_at_one_and_four_threads`
//! runs the same corpus under an explicit 1-thread and an explicit
//! 4-thread rayon pool: `tessellate`'s per-face dispatch is D9 idiom 1,
//! and a digest that moved between the two widths would be the map
//! leaking its schedule into the bytes.
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
use sweep::ExtrudeSide;
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
        (
            "notched_wall",
            rowless_wall("a notch across the seam"),
            [0.1, 0.01],
        ),
        ("holed_wall", rowless_wall("a bar through"), [0.1, 0.01]),
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
        (
            "keyway_minted",
            witness_bodies::minted(witness_bodies::keyway().0),
            [0.05, 0.15],
        ),
        (
            "oblique_lens_minted",
            witness_bodies::minted(witness_bodies::oblique_lens().0),
            [0.05, 0.15],
        ),
        (
            "pole_crossing_half_cap_minted",
            witness_bodies::minted(witness_bodies::pole_crossing_half_cap().0),
            [0.05, 0.15],
        ),
        (
            "apex_crossing_bowtie_minted",
            witness_bodies::minted(witness_bodies::apex_crossing_bowtie().0),
            [0.05, 0.15],
        ),
        ("loft_prism", nurbs_bodies::loft_prism(), [6e-3, 2e-2]),
        ("pinned_wedge", nurbs_bodies::pinned_wedge(), [1e-2, 3e-2]),
        (
            "corner_pinned_through_three",
            nurbs_bodies::corner_pinned_through_three(),
            [1e-2, 3e-2],
        ),
        (
            "swept_elbow",
            sweep::test_support::swept_elbow(Tol::witness()),
            [3e-3, 1e-2],
        ),
        (
            "filleted_hole_three_arcs",
            filleted_plate(0.3, 1e-3, 3),
            [1e-3, 3e-4],
        ),
        (
            "filleted_hole_five_arcs",
            filleted_plate(0.3, 5e-3, 5),
            [3e-3, 1e-3],
        ),
    ]
}

/// One of `common::rowless::walls`, by name.
fn rowless_wall(what: &str) -> Body<f64> {
    rowless::walls()
        .into_iter()
        .find(|(w, _)| *w == what)
        .unwrap()
        .1
}

/// The oblique cut of a cylinder, whose two halves carry an elliptical
/// trim rim on an analytic wall — `m5_pr11_trimmed`'s fixture, whose
/// header carries its provenance.
pub(crate) fn tilted_halves() -> (Body<f64>, Body<f64>) {
    use geom_core::{Point2, Point3, Vec3};
    use profile::{Profile, SketchPlane, test_support::bulge_loop};
    use sweep::{Extrusion, extrude};
    use topo::splitting::{SplitPart, split};

    const R: f64 = 1.0;
    const H: f64 = 2.5;
    const PHI: f64 = 0.3;

    let disc = Profile::new(
        SketchPlane::xy(),
        vec![bulge_loop(vec![
            (Point2::new(-R, 0.0), 1.0),
            (Point2::new(R, 0.0), 1.0),
        ])],
    )
    .validate(Tol::witness())
    .expect("the disc validates");
    let cylinder = extrude(
        &disc,
        Extrusion::Distance {
            depth: H,
            side: ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .expect("the disc extrudes")
    .body;
    let cylinder = topo::test_support::finished("the cylinder", cylinder, Tol::witness());
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, H / 2.0),
        Vec3::new(PHI.sin(), 0.0, PHI.cos()),
        geom_core::Tol::witness(),
    );
    let result = split(&cylinder, &plane, Tol::witness()).expect("the oblique cut splits");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&result.above, &result.below) else {
        panic!("both sides of the oblique cut carry material");
    };
    (Body::clone(above), Body::clone(below))
}

/// The NURBS-walled corpus bodies. One name per body, kept because
/// `patch_memo` reads them by it; the bodies themselves are
/// `sweep::test_support`'s, whose doc comments carry the provenance
/// and the derived volumes.
pub(crate) mod nurbs_bodies {
    use geom_core::Tol;
    use topo::Body;

    pub(crate) fn loft_prism() -> Body<f64> {
        sweep::test_support::loft_prism(Tol::witness())
    }

    /// A two-section loft of `sweep::test_support::pinned_square` placed
    /// by `ms`, at `v_degree`: `sweep`'s `pinned_corners_build` bodies.
    fn pinned(ms: &[geom_core::Affine3<f64>], v_degree: usize) -> Body<f64> {
        use sweep::test_support::{pinned_square, rigid_places};
        let tol = Tol::witness();
        sweep::loft_body::<f64>(
            &vec![pinned_square(); ms.len()],
            &rigid_places(ms, tol),
            v_degree,
            tol,
        )
        .expect("the pinned loft builds")
        .body
        .into_body()
    }

    /// The square turned half a radian about its edge x = −1: a hinge,
    /// its two triangle walls each with a collapsed side.
    pub(crate) fn pinned_wedge() -> Body<f64> {
        let hinge = sweep::test_support::turn_about([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5);
        pinned(&[geom_core::Affine3::identity(), hinge], 1)
    }

    /// The square's corner (−1, −1) pinned through three sections at
    /// v-degree 2: one wall over the three, collapsed at the corner.
    pub(crate) fn corner_pinned_through_three() -> Body<f64> {
        let corner = |a| sweep::test_support::turn_about([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], a);
        pinned(
            &[geom_core::Affine3::identity(), corner(0.3), corner(0.6)],
            2,
        )
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
    ("wedge", [0xc3cc_46f2_c9a9_d6ce, 0xe4e2_5c53_3441_c482]),
    ("axis_wedge", [0xe181_9a80_1d61_3a02, 0xc22a_36c7_a287_583e]),
    (
        "rounded_prism",
        [0x0d7a_8827_002b_0ba9, 0x0d7a_8827_002b_0ba9],
    ),
    ("ball", [0x2b6c_6bff_a4ee_cfff, 0xc18b_5c43_8a0d_64ab]),
    (
        "sphere_wedge",
        [0xc0e4_3c76_081e_10f3, 0xd0e1_7c1a_5ff9_9fee],
    ),
    ("cone", [0x9ec9_51c3_d7a6_353c, 0x5db8_4478_29ca_bfdf]),
    ("cone_wedge", [0x71ec_6787_8f4c_ac2d, 0x71ec_6787_8f4c_ac2d]),
    ("donut", [0xacba_ea8e_c424_d559, 0x0a73_f71a_a4fa_1618]),
    ("washer", [0x965c_cafc_821b_7545, 0x569e_8f0f_f9a7_3ad5]),
    (
        "tilted_above",
        [0xe75f_4c20_8757_5320, 0x7287_50f8_fda6_89b9],
    ),
    (
        "tilted_below",
        [0x9f02_f8b1_d709_be52, 0x066c_5095_98ac_554d],
    ),
    (
        "notched_wall",
        [0x4950_051c_3ec1_9c1d, 0x9372_d3b9_4167_c97a],
    ),
    ("holed_wall", [0xe1ff_b951_d586_0e3c, 0xb30d_7e3b_965c_37c6]),
    ("keyway", [0xe457_7191_dbec_3540, 0xe457_7191_dbec_3540]),
    ("slit", [0xaed6_09ed_f923_833b, 0xaed6_09ed_f923_833b]),
    (
        "oblique_lens",
        [0x72fb_aee1_b444_dff6, 0x72fb_aee1_b444_dff6],
    ),
    (
        "pole_crossing_half_cap",
        [0x72fb_aee1_b444_dff6, 0x72fb_aee1_b444_dff6],
    ),
    (
        "apex_crossing_bowtie",
        [0xbd72_11b8_010f_3772, 0xbd72_11b8_010f_3772],
    ),
    (
        "keyway_minted",
        [0xa110_eb9f_658b_9246, 0xa110_eb9f_658b_9246],
    ),
    (
        "oblique_lens_minted",
        [0x1d3b_f531_e34e_7286, 0x1d3b_f531_e34e_7286],
    ),
    (
        "pole_crossing_half_cap_minted",
        [0xdd6f_a9f2_0af3_083a, 0xdd6f_a9f2_0af3_083a],
    ),
    (
        "apex_crossing_bowtie_minted",
        [0x9cdd_b7a3_543d_b550, 0x9cdd_b7a3_543d_b550],
    ),
    ("loft_prism", [0xd3ce_dece_149d_e356, 0x6d45_152a_ef26_f951]),
    (
        "pinned_wedge",
        [0x12ff_02e0_5d1f_4e27, 0xb6d0_baab_eae4_fad2],
    ),
    (
        "corner_pinned_through_three",
        [0x9789_7a1f_84e0_bbe9, 0x656e_bbf8_8ac8_da74],
    ),
    (
        "swept_elbow",
        [0xca97_2f70_8e56_d5a9, 0xe8f8_263b_c340_6dd9],
    ),
    (
        "filleted_hole_three_arcs",
        [0x4093_41dc_b499_5952, 0x72f0_8b60_2ed6_4ac5],
    ),
    (
        "filleted_hole_five_arcs",
        [0x3fa0_ec1e_8060_4c49, 0xf006_75e0_3bb3_dc82],
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

/// Every corpus digest, in corpus order, labelled `body at delta=δ`.
///
/// One derivation for both rows above and below: the committed-table
/// row and the thread-count row must digest the same bodies at the same
/// δ in the same order, or the second proves nothing about the first.
fn corpus_digests(corpus: &[(&'static str, Body<f64>, [f64; 2])]) -> Vec<(String, u64)> {
    corpus
        .iter()
        .flat_map(|(name, body, deltas)| {
            deltas
                .iter()
                .map(move |&delta| (format!("{name} at delta={delta}"), digest(body, delta)))
        })
        .collect()
}

/// The committed table flattened into [`corpus_digests`]' order.
fn golden_digests() -> Vec<u64> {
    GOLDEN.iter().flat_map(|(_, d)| *d).collect()
}

/// **The thread-count pin.** The corpus digests to the SAME bytes under
/// an explicit 1-thread rayon pool, under an explicit 4-thread one, and
/// against the committed table.
///
/// `tessellate`'s per-face dispatch is D9 idiom 1 — an indexed parallel
/// map into a pre-sized buffer, combined by the sequential arena-order
/// fold that places the patches (idiom 2). Both idioms are
/// schedule-invariant by construction, so this row is not a search for
/// a race: it is the standing evidence that the construction holds, at
/// the two widths that distinguish "no parallelism ran" from "the map
/// ran on several threads".
///
/// **Why an explicit pool rather than `RAYON_NUM_THREADS`.** The
/// environment variable configures the GLOBAL pool, once, at its first
/// use — so a second value in the same process is ignored and a row
/// that set it would silently measure one width twice. A
/// `ThreadPoolBuilder` pool is per-call and `install` binds it for the
/// closure, which is what lets one test binary run both widths.
///
/// The row above digests the same corpus at whatever width the run's
/// ambient pool has, so between them the corpus is pinned at three
/// points, one of them the machine's.
#[test]
fn the_corpus_digests_the_same_bytes_at_one_and_four_threads() {
    let corpus = corpus();
    let at = |threads: usize| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("a rayon pool of the requested width")
            .install(|| corpus_digests(&corpus))
    };
    let one = at(1);
    let four = at(4);
    let want = golden_digests();
    // Both lengths, because every comparison below is a `zip` and a
    // `zip` over a short side is silently a shorter comparison.
    assert_eq!(
        one.len(),
        four.len(),
        "the two pools digested different numbers of body/δ pairs"
    );
    assert_eq!(
        one.len(),
        want.len(),
        "the committed digest table and the corpus must name the same bodies"
    );

    let mut moved: Vec<String> = Vec::new();
    for ((label, got1), (_, got4)) in one.iter().zip(&four) {
        if got1 != got4 {
            moved.push(format!(
                "{label}: 1 thread {got1:016x}, 4 threads {got4:016x} — the map's \
                 result depends on its schedule"
            ));
        }
    }
    for ((label, got1), want) in one.iter().zip(&want) {
        if got1 != want {
            moved.push(format!(
                "{label}: 1 thread {got1:016x}, committed {want:016x}"
            ));
        }
    }
    assert!(
        moved.is_empty(),
        "the mesh is not a function of (body, delta) alone at eps={}:\n  {}",
        common::eps(),
        moved.join("\n  ")
    );
}
