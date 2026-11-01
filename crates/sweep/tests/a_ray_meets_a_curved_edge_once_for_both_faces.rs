//! **A ray within the band of a curved edge meets it once, for both
//! faces it bounds** (D10, Booleans): half a crossing for each face, at
//! the carrier crossing of each it stands for, decided once from where
//! the ray passes the edge (`topo::boolean::solid_contain::curved_edges`).
//! The curved half of the edge-meeting ray
//! (`work/flush/a-ray-meets-a-curved-faces-edge-in-band-by-each-faces-own-trim.md`).
//!
//! Every crossing of a ray is read here, not only the first cluster a
//! reading of the side stops at: each cluster's count is held to the
//! analytic body's membership on either side of it
//! (`curved_edge_corpus`), so a crossing counted twice, or by no face,
//! reads wrong wherever along the ray it lies.

#![allow(clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{
    bored_block_of_arcs, boss_of_arcs, disc_of_arcs, hemisphere_on_flat_base, prism,
    revolved_about_y,
};
use topo::{Body, ContactRecords};

use super::curved_edge_corpus::{Truth, corpus, fast, kinked_bore, revolved, robust};

test_utils::gated_to![
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/topo/src/boolean/solid_contain/curved_edges.rs",
    "crates/sweep/tests/curved_edge_corpus.rs",
];

/// A ray's clusters as [`clusters`] reads them, or the walk's fault.
type Read = Result<(Vec<(f64, f64, i32)>, bool), String>;

/// Asserts `body` legal at tier 3′ with no contacts at the run's ε.
fn at_rest(name: &str, body: &Body<f64>, tol: Tol) {
    assert_eq!(
        topo::validate_pseudomanifold(body, &ContactRecords::default(), tol),
        Ok(()),
        "{name} at ε {:e}: tier 3′ with no contacts",
        tol.eps()
    );
}

/// A ray's crossings gathered into clusters (crossings within `gap` of
/// each other along the ray), each `(from, to, count)`, up to the first
/// cluster holding a sideless crossing, and whether one stopped them.
fn clusters(mut hits: Vec<(f64, f64, i32, bool)>, gap: f64) -> (Vec<(f64, f64, i32)>, bool) {
    hits.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut out: Vec<(f64, f64, i32, bool)> = Vec::new();
    for (lo, hi, c, s) in hits {
        match out.last_mut() {
            Some(last) if lo <= last.1 + gap => {
                *last = (last.0, last.1.max(hi), last.2 + c, last.3 || s);
            }
            _ => out.push((lo, hi, c, s)),
        }
    }
    let stop = out.iter().position(|c| c.3);
    let read = out[..stop.unwrap_or(out.len())]
        .iter()
        .map(|&(lo, hi, c, _)| (lo, hi, c))
        .collect();
    (read, stop.is_some())
}

/// **Whether every cluster of the ray `q + s·d` counts right** on `body`,
/// whose material is `truth` (its complement where `inside_out`): the
/// ray's crossings in two face orders read alike, and each cluster's
/// count is the material it leaves, `+1`, or enters, `−1`, between points
/// either side of it clear of the boundary by `clear`. A ray whose walk
/// faults reads nothing and is right. `Err` names what read wrong.
fn whole_ray(
    body: &Body<f64>,
    face: topo::FaceKey,
    (q, d): (Point3<f64>, Vec3<f64>),
    (truth, inside_out): (&Truth, bool),
    band: Band,
    clear: f64,
) -> Result<(), String> {
    let material = |p: Point3<f64>| robust(truth, p, clear).map(|m| m != inside_out);
    let orders = [(0, false), (3, true)].map(|order| {
        topo::test_support::ray_counts(body, face, (q, d), band, order)
            .map(|hits| clusters(hits, 2.0 * clear))
    });
    let counts = |r: &Read| {
        r.as_ref()
            .map(|(c, s)| (c.iter().map(|x| x.2).collect::<Vec<_>>(), *s))
            .ok()
    };
    let kind = |r: &Read| {
        r.as_ref()
            .err()
            .map(|e| e.split('(').next().unwrap_or("").to_owned())
    };
    if counts(&orders[0]) != counts(&orders[1]) || kind(&orders[0]) != kind(&orders[1]) {
        return Err(format!(
            "two face orders read {:?} and {:?}",
            counts(&orders[0]),
            counts(&orders[1])
        ));
    }
    let Ok((cl, stopped)) = &orders[0] else {
        return Ok(());
    };
    let Some(mut m) = material(q) else {
        return Ok(());
    };
    let mut prev = 0.0;
    for &(lo, hi, c) in cl {
        if lo - prev > 4.0 * clear
            && let Some(at) = material(q + d * (0.5 * (prev + lo)))
            && at != m
        {
            return Err(format!(
                "{cl:?}: material {at} at {}, counted {m}",
                0.5 * (prev + lo)
            ));
        }
        m = match (m, c) {
            (_, 0) => m,
            (true, 1) => false,
            (false, -1) => true,
            _ => return Err(format!("{cl:?}: a count of {c} from material {m}")),
        };
        prev = hi;
    }
    if !stopped
        && let Some(at) = material(q + d * (prev + 1.0))
        && at != m
    {
        return Err(format!("{cl:?}: ends counted {m}, material {at} beyond"));
    }
    Ok(())
}

/// Rays about every edge of `body`, both ways out, each read whole
/// ([`whole_ray`]): at `per` random points of each edge, offsets within
/// 3Kε along the face and off it, in a random direction, one shallow to
/// the face and one nearly along the edge, each from 8 m and from 5 cm
/// back. Returns the rays read and the wrong ones.
fn rays_about_edges(
    name: &str,
    body: &Body<f64>,
    truth: &Truth,
    rng: &mut test_utils::fuzz::Rng,
    per: usize,
    tol: Tol,
) -> (usize, Vec<String>) {
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let reverted = body.revert().expect("the body reverts");
    let (mut n, mut wrong) = (0, Vec::new());
    for (inside_out, b) in [(false, body), (true, &reverted)] {
        let fractions: Vec<f64> = (0..per).map(|_| rng.range(0.02, 0.98)).collect();
        for f in topo::test_support::edge_frames(b, &fractions, band) {
            let p = f.point
                + f.into * (rng.range(-3.0, 3.0) * k_eps)
                + f.normal * (rng.range(-3.0, 3.0) * k_eps);
            let along = f.normal.cross(f.into);
            let random = Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
            let shallow = f.into * rng.range(-1.0, 1.0)
                + along * rng.range(-1.0, 1.0)
                + f.normal * rng.range(-0.02, 0.02);
            for d in [random, shallow, along + random * 0.05] {
                let d = d.normalize();
                for back in [8.0, 0.05] {
                    n += 1;
                    if let Err(w) = whole_ray(
                        b,
                        f.face,
                        (p - d * back, d),
                        (truth, inside_out),
                        band,
                        10.0 * k_eps,
                    ) {
                        wrong.push(format!(
                            "{name} (inside out: {inside_out}) at ε {:e}, {} q={:?} d={d:?}: {w}",
                            tol.eps(),
                            f.label,
                            p - d * back
                        ));
                    }
                }
            }
        }
    }
    (n, wrong)
}

/// **Every crossing of a ray about a curved edge counts right** (shape
/// 1, a counterexample search) on the corpus's cheap part: a cylinder,
/// discs 12ε, 21ε and 30ε thick, a filleted prism and the kinked bore.
/// On PR 5697's first head a crossing two faces on one carrier each stood
/// for counted twice on the discs, and a planar face's half read at a
/// plane crossing outside it on the kinked bore.
#[test]
fn every_crossing_of_a_ray_about_a_curved_edge_counts_right() {
    let tol = Tol::witness();
    let mut rng =
        test_utils::fuzz::start("every_crossing_of_a_ray_about_a_curved_edge_counts_right");
    let per = test_utils::fuzz::scaled(2);
    for (name, body, truth) in fast(tol) {
        at_rest(&name, &body, tol);
        let (n, wrong) = rays_about_edges(&name, &body, &truth, &mut rng, per, tol);
        assert!(n > 0, "{name}: no ray was cast");
        assert!(
            wrong.is_empty(),
            "{} of {n} rays read wrong: {:#?}; {}",
            wrong.len(),
            wrong,
            test_utils::fuzz::replay()
        );
    }
}

/// [`every_crossing_of_a_ray_about_a_curved_edge_counts_right`] over the
/// whole corpus: revolves (whole and partial), booleans of curved and
/// planar bodies in both operand orders, and the cheap part.
#[test]
fn every_crossing_of_a_ray_about_the_corpuss_edges_counts_right() {
    let tol = Tol::witness();
    let mut rng =
        test_utils::fuzz::start("every_crossing_of_a_ray_about_the_corpuss_edges_counts_right");
    let per = test_utils::fuzz::scaled(1);
    let mut wrong = Vec::new();
    for (name, body, truth) in corpus(tol) {
        at_rest(&name, &body, tol);
        wrong.extend(rays_about_edges(&name, &body, &truth, &mut rng, per, tol).1);
    }
    assert!(
        wrong.is_empty(),
        "{} rays read wrong: {:#?}; {}",
        wrong.len(),
        wrong,
        test_utils::fuzz::replay()
    );
}

/// The thin discs' rays (21ε and 30ε thick, whose half-walls share each
/// root of the one cylinder), read whole at `per` points of each edge
/// from the stream `rng`: the wrong ones.
fn discs_rays(rng: &mut test_utils::fuzz::Rng, per: usize, tol: Tol) -> Vec<String> {
    fast(tol)
        .into_iter()
        .filter(|(name, ..)| name == "disc 21ε thick" || name == "disc 30ε thick")
        .flat_map(|(name, body, truth)| rays_about_edges(&name, &body, &truth, rng, per, tol).1)
        .collect()
}

/// **A root two faces on one carrier share is counted once** (PR 5697's
/// first review, MAJOR 2), on the thin discs. The seed is a pinned
/// counterexample: on PR 5697's first head its draws read wrong at ε
/// 1e-9, 1e-6 and 1e-12 alike, half-walls' rims each standing for one
/// root of the cylinder. A sweep, in the slow set; the counterexample's
/// one ray is pinned fast by [`one_root_two_rims_stand_for_reads_nothing_twice`].
#[test]
fn a_root_two_half_walls_share_is_counted_once() {
    let tol = Tol::witness();
    let mut rng =
        test_utils::fuzz::pinned("a_root_two_half_walls_share_is_counted_once", DISC_SEED);
    let wrong = discs_rays(&mut rng, 8, tol);
    assert!(
        wrong.is_empty(),
        "{} rays read wrong: {wrong:#?}",
        wrong.len()
    );
}

/// [`a_root_two_half_walls_share_is_counted_once`]'s seed: 16, 14 and 16 rays
/// read wrong on PR 5697's first head at ε 1e-9, 1e-6 and 1e-12.
const DISC_SEED: u64 = 34;

/// The wrong rays [`discs_rays`] reads at each of the seeds 1 to 40: the
/// search that picked [`DISC_SEED`]. Run by hand.
#[test]
#[ignore = "probe: run by hand"]
fn probe_disc_seeds() {
    let tol = Tol::witness();
    for seed in 1..=40 {
        let mut rng = test_utils::fuzz::Rng::from_seed(seed);
        println!(
            "DISC SEED {seed} eps={:e}: {} wrong",
            tol.eps(),
            discs_rays(&mut rng, 8, tol).len()
        );
    }
}

/// **A ray shallow to a face past a curved edge of it reads right**
/// (PR 5697's first review, MAJOR 1): rays just inside the kinked bore's
/// face `x = 1`, shallow to it, passing the bore's mouth on that face
/// within the band, each read whole. The face's plane crossing lies past
/// the kink edge, which reads clear, outside the face; on PR 5697's first
/// head the mouth, met in band, stood for it, and 140 of the 3042 rays
/// read wrong.
#[test]
fn a_ray_shallow_to_a_face_past_its_curved_edge_reads_right() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let (body, truth) = kinked_bore(tol);
    at_rest("the kinked bore", &body, tol);
    let face = body.faces().next().expect("a face").0;
    let phi = 0.6f64;
    let mut wrong = Vec::new();
    for di in 1..40 {
        let delta = f64::from(di) * 0.025 * k_eps;
        for si in 0..13 {
            let s = (0.8 + f64::from(si) * 0.1) * k_eps / (0.4 * phi.cos());
            for tz in [1e-3, -2e-3, 0.0] {
                let mouth = Point3::new(1.0 - delta, 0.4 + 0.2 * phi.cos(), 0.5 + 0.2 * phi.sin());
                let d = Vec3::new(s, 1.0, tz).normalize();
                for back in [3.0, 0.05] {
                    if let Err(w) = whole_ray(
                        &body,
                        face,
                        (mouth - d * back, d),
                        (&truth, false),
                        band,
                        10.0 * k_eps,
                    ) {
                        wrong.push(format!(
                            "δ {:.3}Kε, s {si}, tz {tz}, back {back}: {w}",
                            delta / k_eps
                        ));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "at ε {:e}, {} of 3042 rays read wrong: {wrong:#?}",
        tol.eps(),
        wrong.len()
    );
}

/// **One crossing two met edges stand for reads nothing** (PR 5697's
/// first review, MAJOR 2): a ray from 8 m out through the 30ε disc,
/// meeting its top rim on one half-wall and its bottom rim on the other,
/// each half-wall's root the same root of the one cylinder. On PR 5697's
/// first head both rims stood for it, and the ray read `In`.
#[test]
fn one_root_two_rims_stand_for_reads_nothing_twice() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let (name, body, truth) = fast(tol)
        .into_iter()
        .find(|(name, ..)| name == "disc 30ε thick")
        .expect("the 30ε disc");
    at_rest(&name, &body, tol);
    let face = body.faces().next().expect("a face").0;
    let q = Point3::new(0.431777029158541, 7.738305151126824, 2.028294585136746);
    let d = Vec3::new(
        0.008527870324942689,
        -0.9672881451929304,
        -0.25353682098847374,
    );
    let k_eps = tol.k() * tol.eps();
    assert_eq!(
        whole_ray(&body, face, (q, d), (&truth, false), band, 10.0 * k_eps),
        Ok(()),
        "{name} at ε {:e}",
        tol.eps()
    );
}

/// The bodies, each with a curved boundary, built through the public
/// doors, each asserted legal at tier 3′ with no contacts.
fn aimed_bodies(tol: Tol) -> Vec<(&'static str, Body<f64>)> {
    let quarter = (PI / 8.0).tan();
    let tube = || vec![(Point2::new(2.5, 0.0), 1.0), (Point2::new(1.5, 0.0), 1.0)];
    let out = vec![
        ("cylinder 2 arcs", disc_of_arcs(2, 0.5, 1.0, tol)),
        ("cylinder 3 arcs", disc_of_arcs(3, 0.5, 1.0, tol)),
        ("boss on a box", boss_of_arcs(2, 1.0, 0.3, 0.5, 1.0, tol)),
        ("bored block", bored_block_of_arcs(2, 1.0, 1.0, 0.3, tol)),
        (
            "filleted edge",
            prism(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.8), quarter),
                    (Point2::new(0.8, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                1.0,
                tol,
            ),
        ),
        (
            "cone frustum",
            revolved_about_y(
                vec![
                    (Point2::new(0.0, 0.0), 0.0),
                    (Point2::new(1.0, 0.0), 0.0),
                    (Point2::new(0.6, 1.0), 0.0),
                    (Point2::new(0.0, 1.0), 0.0),
                ],
                Revolution::Full,
                tol,
            ),
        ),
        ("hemisphere", hemisphere_on_flat_base(1.0, tol)),
        (
            "half torus",
            revolved_about_y(tube(), Revolution::Partial(PI), tol),
        ),
    ];
    for (name, body) in &out {
        assert_eq!(
            topo::validate_pseudomanifold(body, &ContactRecords::default(), tol),
            Ok(()),
            "{name} at ε {:e}: tier 3′ with no contacts",
            tol.eps()
        );
    }
    out
}

/// **Rays aimed within the band of a curved edge read by it.** At a
/// third of the band and at the band from the edge's middle, into each
/// face, every ray reads the side at infinity right or reads nothing
/// (a ray that clips the edge's corner crosses nothing net there), and
/// none is set aside on a crossing in band of a face's boundary, which
/// each face's own trim read before the curved edge-meeting ray. At
/// least 40% of each body's rays read `Out` (42–88% at ε 1e-9, 1e-6 and
/// 1e-12), where 12–46% did before, most of the rest set aside on such a
/// crossing.
#[test]
fn rays_in_band_of_a_curved_edge_read_it_once() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    for (name, body) in aimed_bodies(tol) {
        let (mut read, mut aimed) = (0, 0);
        for f in topo::test_support::edge_frames(&body, &[0.5], band) {
            if f.label.starts_with("Line") && f.label.contains("[Plane | Plane]") {
                continue;
            }
            for o in [0.3 * k_eps, k_eps] {
                let r =
                    topo::test_support::ray_at(&body, f.face, f.point + f.into * o, f.normal, band);
                let what = format!("{name} at ε {:e}, {} o={o:e}: {r}", tol.eps(), f.label);
                assert!(!r.contains("Side(In)"), "{what}: a wrong side");
                assert!(
                    !r.contains("SetAside(Crossing"),
                    "{what}: set aside in band of a face's own boundary"
                );
                aimed += 1;
                read += usize::from(r.contains("Side(Out)"));
            }
        }
        assert!(
            5 * read >= 2 * aimed,
            "{name} at ε {:e}: {read} of {aimed} rays in band of a curved edge read",
            tol.eps()
        );
    }
}

/// The tube of [`rays_grazing_a_torus_rim_count_no_crossing_of_the_tube_elsewhere`]:
/// radius 0.5 about `r = 2`.
fn tube() -> Vec<(Point2<f64>, f64)> {
    vec![(Point2::new(1.5, 0.0), 1.0), (Point2::new(2.5, 0.0), 1.0)]
}

/// **The nightly's ray grazing a partial torus's rim counts no crossing
/// of the tube elsewhere for it**
/// (`work/flush/main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond.md`;
/// nightly 2026-10-29 at ε 1e-6, `CAD_FUZZ_SEED=0x003481ce0ba06522`). It
/// passes the 1.5-turn torus's rim within the band, just outside the
/// tube's bottom, and crosses the carrier nowhere near it. The rim's
/// torus half read nearest the root where the ray leaves the tube
/// across the axis, 3.75 back, and counted that exit as half the rim's.
/// Its family is [`rays_grazing_a_torus_rim_count_no_crossing_of_the_tube_elsewhere`].
#[test]
fn the_nightlys_ray_grazing_a_torus_rim_reads_right() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let (body, truth) = revolved(
        "rev torus 1.5pi",
        tube(),
        Revolution::Partial(1.5 * PI),
        tol,
    );
    at_rest("rev torus 1.5pi", &body, tol);
    let face = body.faces().next().expect("a face").0;
    let q = Point3::new(0.1654750844081492, -0.3689933861942981, -6.0031985322182555);
    let d = Vec3::new(
        -0.020687219868856334,
        -0.01637304530398302,
        0.9996519205811447,
    );
    assert_eq!(
        whole_ray(&body, face, (q, d), (&truth, false), band, 10.0 * k_eps),
        Ok(()),
        "at ε {:e}",
        tol.eps()
    );
}

/// **A ray grazing a partial torus's rim counts no crossing of the tube
/// elsewhere for it**. A ray along a rim's tangent, just outside the
/// tube and tilted off the cap's plane, passes the rim within the band
/// and crosses the carrier nowhere near it. Past a half turn it crosses
/// the tube again across the axis, and that root, held by its face, is
/// the one the met edge's torus half reads nearest. That root is a
/// crossing of its own, and the rim stands for no root of the tube that
/// far off. Three partial turns, each rim of each, both ways along it,
/// from 8 m and from 5 cm back, each read whole. Before the fix, 72 of
/// 9024 rays read wrong at each of ε 1e-9, 1e-6 and 1e-12.
#[test]
fn rays_grazing_a_torus_rim_count_no_crossing_of_the_tube_elsewhere() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let clear = 10.0 * k_eps;
    let mut wrong = Vec::new();
    let fractions: Vec<f64> = (1..48).map(|i| f64::from(i) / 48.0).collect();
    let (mut n, mut across) = (0, 0);
    for turn in [1.25, 1.5, 1.75] {
        let name = format!("torus {turn}π");
        let (body, truth) = revolved(&name, tube(), Revolution::Partial(turn * PI), tol);
        at_rest(&name, &body, tol);
        for f in topo::test_support::edge_frames(&body, &fractions, band) {
            if !(f.label.starts_with("Circle") && f.label.contains("[Torus | Plane]")) {
                continue;
            }
            let along = f.normal.cross(f.into);
            for off in [0.3, 0.9] {
                let p = f.point + f.normal * (off * k_eps);
                // Tilted off the cap's plane, still tangent to the tube.
                for d in [along, -along]
                    .into_iter()
                    .flat_map(|a| [0.02, -0.02].map(|tilt| (a + f.into * tilt).normalize()))
                {
                    // Whether the ray's line crosses the tube across the axis.
                    across +=
                        usize::from((200..1200).any(|i| truth(p + d * (f64::from(i) * 0.005))));
                    for back in [8.0, 0.05] {
                        n += 1;
                        if let Err(w) = whole_ray(
                            &body,
                            f.face,
                            (p - d * back, d),
                            (&truth, false),
                            band,
                            clear,
                        ) {
                            wrong.push(format!("{name}, {} p={p:?} d={d:?}: {w}", f.label));
                        }
                    }
                }
            }
        }
    }
    assert!(
        across >= 8,
        "at ε {:e}: {across} of the family's lines cross the tube across the axis",
        tol.eps()
    );
    assert!(
        wrong.is_empty(),
        "at ε {:e}, {} of {n} rays read wrong: {wrong:#?}",
        tol.eps(),
        wrong.len()
    );
}

/// Review probe (PR 6168): a ray just INSIDE the tube at a rim's
/// bottom or top point, along the parallel's tangent, where the tube is
/// parabolic (`f ≈ −δ + s⁴/16`): its far exit root has `off·|n·d| ≈ 4δ`.
#[test]
#[ignore = "review probe"]
fn review_probe_rays_inside_a_rims_parabolic_point() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let clear = 10.0 * k_eps;
    let (mut n, mut wrong, mut aside) = (0usize, Vec::new(), 0usize);
    for turn in [1.25, 1.5, 1.75] {
        let name = format!("torus {turn}π");
        let (body, truth) = revolved(&name, tube(), Revolution::Partial(turn * PI), tol);
        let fractions = [0.5];
        let mut seen: Vec<f64> = Vec::new();
        for f in topo::test_support::edge_frames(&body, &fractions, band) {
            if !(f.label.starts_with("Circle") && f.label.contains("[Torus | Plane]")) {
                continue;
            }
            let th = f.point.z.atan2(f.point.x);
            if seen.iter().any(|s| (s - th).abs() < 1e-6) {
                continue;
            }
            seen.push(th);
            let tangent = Vec3::new(-th.sin(), 0.0, th.cos());
            for ny in [-1.0, 1.0] {
                // `ny` the outward normal's y at the point: bottom −1, top +1.
                let b = Point3::new(2.0 * th.cos(), 0.5 * ny, 2.0 * th.sin());
                for frac in [0.02, 0.05, 0.1, 0.15, 0.2, 0.24, 0.3, 0.5, 0.9] {
                    let p = b - Vec3::new(0.0, ny, 0.0) * (frac * k_eps);
                    for sgn in [-1.0, 1.0] {
                        for a in [0.0, 1e-6, 3e-6, 1e-5, -1e-6, 1e-4] {
                            let d = (tangent * sgn + Vec3::new(0.0, ny * a, 0.0)).normalize();
                            for back in [8.0, 0.05] {
                                n += 1;
                                let q = p - d * back;
                                match whole_ray(&body, f.face, (q, d), (&truth, false), band, clear) {
                                    Err(w) => wrong.push(format!(
                                        "{name} {} δ={frac}Kε sgn={sgn} a={a} back={back}: {w}",
                                        f.label
                                    )),
                                    Ok(()) => {
                                        if let Err(e) = topo::test_support::ray_counts(&body, f.face, (q, d), band, (0, false))
                                        {
                                            if aside < 12 {
                                                eprintln!("ASIDE δ={frac} sgn={sgn} a={a} back={back} ny={ny}: {}", &e[..e.len().min(300)]);
                                            }
                                            aside += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("PROBE at ε {:e}: {} of {n} wrong, {aside} set aside", tol.eps(), wrong.len());
    for w in &wrong {
        eprintln!("  {w}");
    }
    assert!(wrong.is_empty());
}

/// Review probe (PR 6168): a near-full turn. A ray grazes rim A just
/// outside the tube's bottom (top), crosses cap A's plane leaving the
/// face's side, crosses the missing wedge, and dips into the tube
/// near-tangent just past cap B: an entering root `off < r` from the
/// passage, held by the face, with `off·|n·d| ≤ Kε` and a decided
/// incidence. The rim's halves would pair the plane's exit with that
/// entry.
#[test]
#[ignore = "review probe"]
fn review_probe_near_full_turn_tangent_entry_past_the_other_cap() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let clear = 10.0 * k_eps;
    let (mut n, mut wrong, mut aside) = (0usize, Vec::new(), 0usize);
    for turn in [1.9, 1.95, 1.97, 1.98, 1.99] {
        let name = format!("torus {turn}π");
        let Ok((body, truth)) = std::panic::catch_unwind(|| {
            revolved(&name, tube(), Revolution::Partial(turn * PI), tol)
        }) else {
            eprintln!("{name}: no build");
            continue;
        };
        let gap = (2.0 - turn) * PI;
        let deep = |th: f64| truth(Point3::new(2.0 * th.cos(), 0.013, 2.0 * th.sin()));
        let mut seen: Vec<f64> = Vec::new();
        let mut face = None;
        for f in topo::test_support::edge_frames(&body, &[0.5], band) {
            if f.label.starts_with("Circle") && f.label.contains("[Torus | Plane]") {
                let th = f.point.z.atan2(f.point.x);
                face = Some(f.face);
                if !seen.iter().any(|s| (s - th).abs() < 1e-6) {
                    seen.push(th);
                }
            }
        }
        let face = face.expect("a torus face");
        for &tha in &seen {
            // The gap's side of cap A.
            let sg = if deep(tha + 0.01) { -1.0 } else { 1.0 };
            if !(deep(tha - 0.01 * sg) && !deep(tha + 0.01 * sg)) {
                eprintln!("{name}: θA={tha} unclear: {} {} {} {}", deep(tha - 0.01), deep(tha + 0.01), truth(Point3::new(2.0,0.0,0.0)), truth(Point3::new(0.0,0.0,2.0)));
                continue;
            }
            for ny in [-1.0f64, 1.0] {
                for extra in [0.02, 0.04, 0.08] {
                    let th1 = tha + sg * (gap + extra);
                    let span = gap + extra;
                    let off = 4.0 * (span / 2.0).sin();
                    let kappa = 2.0 * (span / 2.0).sin().powi(2);
                    for dfrac in [0.3, 0.9, 3.0, 30.0] {
                        for m in [1.2, 1.5, 2.0, 2.2, 2.5, 3.0, 3.5, 4.0, 5.0, 8.0] {
                            let slope = m * k_eps;
                            let h = slope * slope / (2.0 * kappa);
                            let a = Point3::new(2.0 * tha.cos(), 0.5 * ny, 2.0 * tha.sin())
                                + Vec3::new(0.0, ny, 0.0) * (dfrac * k_eps);
                            let p1 = Point3::new(2.0 * th1.cos(), 0.5 * ny, 2.0 * th1.sin())
                                - Vec3::new(0.0, ny, 0.0) * h;
                            let d = (p1 - a).normalize();
                            for back in [8.0, 0.05] {
                                n += 1;
                                let q = a - d * back;
                                match whole_ray(&body, face, (q, d), (&truth, false), band, clear) {
                                    Err(w) => wrong.push(format!(
                                        "{name} θA={tha:.4} ny={ny} extra={extra} δ={dfrac}Kε m={m} off={off:.3} back={back} q={q:?} d={d:?}: {w}"
                                    )),
                                    Ok(()) => {
                                        if let Err(e) = topo::test_support::ray_counts(&body, face, (q, d), band, (0, false)) {
                                            if back == 8.0 && dfrac > 1.0 && extra == 0.04 && ny < 0.0 && turn == 1.97 {
                                                eprintln!("ASIDE2 {name} θA={tha:.3} m={m} off={off:.3}: {}", &e[..e.len().min(240)]);
                                            }
                                            aside += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("PROBE2 at ε {:e}: {} of {n} wrong, {aside} set aside", tol.eps(), wrong.len());
    for w in wrong.iter().take(40) {
        eprintln!("  {w}");
    }
    assert!(wrong.is_empty());
}

/// Review probe (PR 6168): every corpus body's rays about its edges, as
/// `rays_about_edges` casts them, each ray's reading written out line by
/// line (read counts, set aside, or wrong) to `$REVIEW_OUT`, and a grid
/// of `point_in_solid` readings against the analytic truth beside it, so
/// base and head can be line-diffed.
#[test]
#[ignore = "review probe"]
fn review_probe_corpus_readings() {
    use std::io::Write;
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let k_eps = tol.k() * tol.eps();
    let clear = 10.0 * k_eps;
    let path = std::env::var("REVIEW_OUT").expect("REVIEW_OUT");
    let mut out = std::io::BufWriter::new(std::fs::File::create(path).expect("out"));
    let mut rng = test_utils::fuzz::Rng::from_seed(0x6168_0001);
    let (mut n, mut wrong, mut aside) = (0usize, 0usize, 0usize);
    let (mut pn, mut pwrong, mut prefused) = (0usize, 0usize, 0usize);
    for (name, body, truth) in corpus(tol) {
        let reverted = body.revert().expect("the body reverts");
        for (inside_out, b) in [(false, &body), (true, &reverted)] {
            let fractions: Vec<f64> = (0..4).map(|_| rng.range(0.02, 0.98)).collect();
            for f in topo::test_support::edge_frames(b, &fractions, band) {
                let p = f.point
                    + f.into * (rng.range(-3.0, 3.0) * k_eps)
                    + f.normal * (rng.range(-3.0, 3.0) * k_eps);
                let along = f.normal.cross(f.into);
                let random = Vec3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
                let shallow = f.into * rng.range(-1.0, 1.0)
                    + along * rng.range(-1.0, 1.0)
                    + f.normal * rng.range(-0.02, 0.02);
                let inside_chord = along + f.normal * rng.range(-0.01, 0.01);
                for (kind, d) in [("random", random), ("shallow", shallow), ("along", along + random * 0.05), ("chord", inside_chord)] {
                    let d = d.normalize();
                    for back in [8.0, 0.05] {
                        n += 1;
                        let q = p - d * back;
                        let w = whole_ray(b, f.face, (q, d), (&truth, inside_out), band, clear);
                        let r = topo::test_support::ray_counts(b, f.face, (q, d), band, (0, false));
                        let tag = match (&w, &r) {
                            (Err(e), _) => {
                                wrong += 1;
                                format!("WRONG {e}")
                            }
                            (Ok(()), Err(e)) => {
                                aside += 1;
                                format!("ASIDE {}", e.split('(').next().unwrap_or(""))
                            }
                            (Ok(()), Ok(h)) => format!(
                                "READ {:?}",
                                h.iter().map(|x| (x.2, x.3)).collect::<Vec<_>>()
                            ),
                        };
                        writeln!(out, "RAY {name} io={inside_out} {} {kind} back={back}: {tag}", f.label).ok();
                    }
                }
            }
        }
        // A grid of points about the body against the analytic truth.
        for i in 0..9 {
            for j in 0..9 {
                for k in 0..9 {
                    let pt = Point3::new(
                        -3.1 + 6.2 * f64::from(i) / 8.0 + 1e-3,
                        -1.1 + 2.2 * f64::from(j) / 8.0 + 2e-3,
                        -3.1 + 6.2 * f64::from(k) / 8.0 + 3e-3,
                    );
                    let Some(t) = robust(&truth, pt, clear) else { continue };
                    pn += 1;
                    let got = topo::point_in_solid(&body, pt, band, tol);
                    let tag = match got {
                        Ok(topo::SolidContainment::In) if t => "in".to_owned(),
                        Ok(topo::SolidContainment::Out) if !t => "out".to_owned(),
                        Ok(c) => {
                            pwrong += 1;
                            format!("WRONG {c:?} truth {t}")
                        }
                        Err(e) => {
                            prefused += 1;
                            format!("REFUSED {}", format!("{e:?}").split('{').next().unwrap_or(""))
                        }
                    };
                    writeln!(out, "PT {name} {i} {j} {k}: {tag}").ok();
                }
            }
        }
    }
    eprintln!("PROBE3 at ε {:e}: rays {n}, wrong {wrong}, aside {aside}; points {pn}, wrong {pwrong}, refused {prefused}", tol.eps());
}
