//! M3 PR 5.5 adversarial review suite (seam discipline). Falsification
//! targets: the sense theorem under non-axis-aligned operands (exact
//! dyadic linear maps: scaled rotations, shears, reflections), the
//! `bool_strut_order` angular rule (derivation says it is forced only
//! on half-plane sectors — reflex-corner attack), R1 closure depth
//! (pocket/boss matrices, six-face chains, near-edge and face-spanning
//! pockets), R2 generalization (6/8 collinear sites, mixed lanes, the
//! U-family under all ops), ring-lane role-order robustness (was the
//! `residual_side_in` probe; issue #93 replaced it with the intrinsic
//! `ring_run_ccw` winding rule — probes on the other operand's
//! boundary, nested pockets, ring+transversal same face), boundary-on-boundary refusal honesty
//! (UnpairedLooseEnds {4}/{8} + sharp perturbation boundary + null-edge
//! dump), THE DIE (21 pips, exact oracle), and the standing sweeps
//! (D9 replay, A minus B == A meet revert B, interval lane).
//! Every Ok carries an EXACT dyadic volume oracle.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::type_complexity)] // fixture tables read clearer inline

use crate::common;

use common::{flush_declarations, line, plane, prism_z};
use geom_core::Tol;
use geom_core::{Decide, Point3};
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, FaceSurface, MefSite,
    MevSite, mass_properties, subtract_with, union_with, validate, validate_closed,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// A right prism over `profile` x [z0, z1] pushed through the linear
/// map `m` (rows; dyadic entries, det > 0 so outward stays outward):
/// the prism_z construction generalized to mapped corner points, every
/// face a Newell plane of its mapped corners. Exact-volume oracle:
/// area(profile) * (z1 - z0) * det(m).
fn tprism<T: Decide>(profile: &[(f64, f64)], z0: f64, z1: f64, m: [[f64; 3]; 3]) -> Body<T> {
    assert!(profile.len() >= 3);
    let n = profile.len();
    let mp = |x: f64, y: f64, z: f64| -> Point3<T> {
        let w = [
            m[0][0] * x + m[0][1] * y + m[0][2] * z,
            m[1][0] * x + m[1][1] * y + m[1][2] * z,
            m[2][0] * x + m[2][1] * y + m[2][2] * z,
        ];
        Point3::new(T::from_f64(w[0]), T::from_f64(w[1]), T::from_f64(w[2]))
    };
    let bot: Vec<Point3<T>> = profile.iter().map(|&(x, y)| mp(x, y, z0)).collect();
    let top: Vec<Point3<T>> = profile.iter().map(|&(x, y)| mp(x, y, z1)).collect();
    let mut body = Body::<T>::new();
    let seed = body.mvfs(bot[0]).unwrap();
    let mut chain = Vec::new();
    chain.push(
        body.mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            bot[1],
            line(bot[0], bot[1]),
            Tol::witness(),
        )
        .unwrap(),
    );
    for i in 2..n {
        let at = chain[i - 2].he_minus;
        chain.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                bot[i],
                line(bot[i - 1], bot[i]),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    let bottom_vertices: Vec<_> = core::iter::once(seed.vertex)
        .chain(chain.iter().map(|c| c.vertex))
        .collect();
    let he_last = body
        .find_half_edge(seed.face, bottom_vertices[n - 1], bottom_vertices[n - 2])
        .unwrap();
    let rev: Vec<Point3<T>> = core::iter::once(bot[0])
        .chain(bot[1..].iter().rev().copied())
        .collect();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_last,
                he2: chain[0].he_plus,
            },
            line(bot[n - 1], bot[0]),
            FaceSurface::New(plane(&rev)),
            Tol::witness(),
        )
        .unwrap();
    let mut struts = Vec::new();
    for i in 0..n {
        let at = if i == 0 {
            chain[0].he_plus
        } else if i < n - 1 {
            chain[i].he_plus
        } else {
            f_bottom.he_plus
        };
        struts.push(
            body.mev(
                MevSite::Fan { he1: at, he2: at },
                top[i],
                line(bot[i], top[i]),
                Tol::witness(),
            )
            .unwrap(),
        );
    }
    let mut first_side_he_plus = None;
    for i in 0..n {
        let j = (i + 1) % n;
        let he2 = if i < n - 1 {
            struts[j].he_minus
        } else {
            first_side_he_plus.unwrap()
        };
        let f = body
            .mef(
                MefSite::Chords {
                    he1: struts[i].he_minus,
                    he2,
                },
                line(top[i], top[j]),
                FaceSurface::New(plane(&[bot[i], bot[j], top[j], top[i]])),
                Tol::witness(),
            )
            .unwrap();
        if i == 0 {
            first_side_he_plus = Some(f.he_plus);
        }
    }
    body.set_face_surface(seed.face, FaceSurface::New(plane(&top)))
        .unwrap();
    body
}

type BoolOp<T> = fn(
    &Body<T>,
    &Body<T>,
    &topo::BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<T>, BooleanError>;

/// Functional run with the author's flush contacts declared (M4
/// PR 5): operands bitwise untouched, result tier-1+2 valid, D9
/// bitwise replay.
fn run<T: Decide>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> BooleanResult<T> {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let decls = flush_declarations(a, b);
    let r = op(a, b, &decls, Tol::witness()).unwrap();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
    if let BooleanResult::Body(body) = &r {
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
        // D9: bitwise replay.
        let r2 = op(a, b, &decls, Tol::witness()).unwrap();
        if let BooleanResult::Body(body2) = &r2 {
            assert_eq!(
                format!("{:?}", body.body),
                format!("{:?}", body2.body),
                "D9 bitwise replay"
            );
        } else {
            panic!("replay changed result kind");
        }
    }
    r
}

fn body_of<T: Decide>(r: &BooleanResult<T>) -> &BooleanBody<T> {
    match r {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("expected a body"),
    }
}

fn vol(body: &Body<f64>) -> f64 {
    mass_properties(body, Tol::witness()).unwrap().volume
}

/// Subtract with the exact volume oracle AND the A minus B == A meet
/// revert(B) cross-oracle; returns the owned result body.
fn sub_exact(a: &Body<f64>, b: &Body<f64>, volume: f64) -> Body<f64> {
    let r = run(subtract_with, a, b);
    let out = body_of(&r);
    assert_eq!(vol(&out.body), volume, "exact subtract volume");
    let rb = b.revert().unwrap();
    let ri = run(topo::intersect_with, a, &rb);
    assert_eq!(vol(&body_of(&ri).body), volume, "revert-oracle volume");
    // The owned body is the one ALREADY computed above, cloned — not a
    // fresh `run`. INVARIANT: a third execution would be bit-identical
    // to `out.body` and so asserts nothing new. `run` above already
    // executed `subtract_with(a, b, &decls)` TWICE and asserted the two
    // results `Debug`-identical (the D9 replay), with both operands
    // asserted bitwise untouched by the call — i.e. determinism of this
    // op on these operands is established, in this process, by the
    // assertions we keep. Every check the discarded run performed
    // (operands untouched, tier 1, tier 2, D9 replay) is performed by
    // the kept one on the same inputs, so nothing is given up.
    out.body.clone()
}

/// A refusal must be typed, deterministic, and leave operands bitwise
/// untouched.
fn assert_typed_refusal<T: Decide>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> String {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let d = flush_declarations(a, b);
    let e1 = op(a, b, &d, Tol::witness()).map(|_| ()).unwrap_err();
    let e2 = op(a, b, &d, Tol::witness()).map(|_| ()).unwrap_err();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched by refusal");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched by refusal");
    assert_eq!(
        format!("{e1:?}"),
        format!("{e2:?}"),
        "refusal deterministic"
    );
    format!("{e1:?}")
}

// =====================================================================
// B. Sense theorem: direct census + non-axis-aligned operands.
// =====================================================================

/// Claim (1) as data. Census of sense_A(g) vs sense_B(g) over every
/// correspondence pair (sense read exactly as the join reads it:
/// membership of the facing half's start vertex in the solid's F9
/// side sets), plus the slot lock (same face pair + same direction at
/// matched slots) and the (d, -d) strut opposite-sense claim.
/// Returns the germ counts by how the two sides' senses relate.
fn sense_census(op: topo::BooleanOp, a: &Body<f64>, b: &Body<f64>) -> SenseCensus {
    let red =
        topo::boolean_reduce_declared(op, a, b, &flush_declarations(a, b), Tol::witness()).unwrap();
    // Edge keys are body-lineage-scoped (F9): the pair's a_edge must be
    // resolved among A-operand records only (keys can collide across
    // arenas).
    let rec_of = |edge, operand| {
        red.null_edges
            .iter()
            .find(|r| r.operand == operand && r.edge == edge)
            .expect("pair record references a minted null edge")
    };
    let side_sets = |op: topo::Operand| {
        let mut below = std::collections::BTreeSet::new();
        let mut above = std::collections::BTreeSet::new();
        for r in red.null_edges_of(op) {
            below.insert(r.attr.below_end);
            above.insert(r.attr.above_end);
        }
        Sets { below, above }
    };
    let (a_sets, b_sets) = (side_sets(topo::Operand::A), side_sets(topo::Operand::B));
    /// The solid's F9 side sets: the start vertices seen below and
    /// above each null edge.
    struct Sets {
        below: std::collections::BTreeSet<topo::VertexKey>,
        above: std::collections::BTreeSet<topo::VertexKey>,
    }
    let sense = |body: &Body<f64>, sets: &Sets, r: &topo::BoolNullEdgeRecord<f64>, i: usize| {
        let start = body.get_half_edge(r.germs[i].he).unwrap().start;
        if sets.below.contains(&start) {
            Some(true)
        } else if sets.above.contains(&start) {
            Some(false)
        } else {
            None // is_up here would be a JoinDesync
        }
    };
    assert!(!red.null_pairs.is_empty(), "fixture must mint pairs");
    let (mut anti, mut same, mut unres) = (0, 0, 0);
    for pair in &red.null_pairs {
        let ra = rec_of(pair.a_edge, topo::Operand::A);
        let rb = rec_of(pair.b_edge, topo::Operand::B);
        for i in 0..2 {
            // The slot lock: slot i of A and B is the SAME spatial
            // germ — same face pair, same outgoing direction.
            assert_eq!(ra.germs[i].a_face, rb.germs[i].a_face, "slot face lock (A)");
            assert_eq!(ra.germs[i].b_face, rb.germs[i].b_face, "slot face lock (B)");
            let d = ra.germs[i].dir.dot(rb.germs[i].dir);
            assert!(d > 0.5, "slot germ directions agree, dot = {d}");
            match (sense(&red.a, &a_sets, ra, i), sense(&red.b, &b_sets, rb, i)) {
                (Some(x), Some(y)) if x != y => anti += 1,
                (Some(_), Some(_)) => same += 1,
                _ => unres += 1,
            }
        }
        for (body, sets, r) in [(&red.a, &a_sets, ra), (&red.b, &b_sets, rb)] {
            // (d, -d) struts must carry opposite senses (pierce-lane
            // ring struts have non-opposite dirs and are exempt).
            if r.dangling && r.germs[0].dir.dot(r.germs[1].dir) < -0.5 {
                let (s0, s1) = (sense(body, sets, r, 0), sense(body, sets, r, 1));
                if let (Some(x), Some(y)) = (s0, s1) {
                    assert_ne!(x, y, "strut senses oppose");
                }
            }
        }
    }
    SenseCensus {
        anti,
        same,
        unresolvable: unres,
    }
}

/// Germ counts by how the A-side and B-side senses relate at a slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SenseCensus {
    anti: u32,
    same: u32,
    unresolvable: u32,
}

/// The sense theorem across lanes: the cross-solid anti-correlation
/// holds at literally EVERY germ — transversal corner overlaps
/// (pierce lane), the coplanar-cap Fig 15.1 lane (vv spikes),
/// the pocket ring lane, and tilted (sheared) operands — under all
/// three ops. (Reviewer note: a first census draft showed apparent
/// same-sense germs on fig151; that was a key-collision bug in the
/// census itself — pair edge keys are body-scoped and must be
/// resolved per operand. With scoped lookups the theorem is clean.)
#[test]
fn b_sense_theorem_census() {
    let ops = [
        topo::BooleanOp::Union,
        topo::BooleanOp::Intersect,
        topo::BooleanOp::Subtract,
    ];
    let shear = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.0, 1.0]];
    let sq = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let pil = [(0.75, 0.75), (1.25, 0.75), (1.25, 1.25), (0.75, 1.25)];
    for op in ops {
        // Transversal lanes: STRICT anti-correlation.
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
        assert_eq!(
            sense_census(op, &a, &b),
            SenseCensus {
                anti: 12,
                same: 0,
                unresolvable: 0
            },
            "two-brick {op:?}"
        );
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
        assert_eq!(
            sense_census(op, &a, &b),
            SenseCensus {
                anti: 8,
                same: 0,
                unresolvable: 0
            },
            "pocket {op:?}"
        );
        let a = tprism::<f64>(&sq, 0.0, 2.0, shear);
        let b = tprism::<f64>(&pil, 1.5, 2.5, shear);
        assert_eq!(
            sense_census(op, &a, &b),
            SenseCensus {
                anti: 8,
                same: 0,
                unresolvable: 0
            },
            "sheared {op:?}"
        );
        // Coplanar-cap lane: anti-correlation holds here too.
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
        let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
        assert_eq!(
            sense_census(op, &a, &b),
            SenseCensus {
                anti: 12,
                same: 0,
                unresolvable: 0
            },
            "fig151 {op:?}"
        );
    }
}

// =====================================================================
// A. bool_strut_order: the flagged weak spot. Derivation (review
// scratch): the convex-sector dot comparison orders germ angles
// correctly iff both angles from the anchor are <= pi. Crossing-minted
// vv sites have half-plane (width exactly pi) sectors, so the
// orientation IS forced there — the corpus was structurally confined
// to the forced class, not lucky. The unforced class needs a reflex
// (> 3pi/2) PRE-EXISTING corner reached by a tilted operand (below).
// =====================================================================

/// Fig 15.1 corner-overlap family at several overlap depths, plus
/// operand swap: every ∩ exact, spike chords nested at each cap
/// corner site.
#[test]
fn a_fig151_variants_exact() {
    for (off, expect) in [(1.0, 1.0), (1.5, 0.25), (0.5, 2.25)] {
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
        let b = brick::<f64>((off, off + 2.0), (off, off + 2.0), (0.0, 1.0));
        let r = run(topo::intersect_with, &a, &b);
        assert_eq!(vol(&body_of(&r).body), expect, "off {off}");
        let r = run(topo::intersect_with, &b, &a);
        assert_eq!(vol(&body_of(&r).body), expect, "off {off} swapped");
        // Union + subtract on the same family (with revert oracle).
        let r = run(union_with, &a, &b);
        assert_eq!(vol(&body_of(&r).body), 8.0 - expect, "off {off} union");
        sub_exact(&a, &b, 4.0 - expect);
    }
}

/// Three-brick coplanar corner pileup: (A meet B) meet C, all caps
/// coplanar, C overlapping the AB seam corner region. Chained spikes
/// at corner sites of a seam-born body.
#[test]
fn a_three_brick_corner_pileup() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
    let r = run(topo::intersect_with, &a, &b);
    let ab = match run(topo::intersect_with, &a, &b) {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!(),
    };
    assert_eq!(vol(&body_of(&r).body), 1.0);
    let c = brick::<f64>((1.5, 2.5), (1.5, 2.5), (0.0, 1.0));
    let r2 = run(topo::intersect_with, &ab, &c);
    assert_eq!(vol(&body_of(&r2).body), 0.25, "pileup meet");
    let r3 = run(union_with, &ab, &c);
    assert_eq!(vol(&body_of(&r3).body), 1.0 + 1.0 - 0.25, "pileup union");
}

/// MULTIPLE spikes at ONE corner site: C is a diamond prism whose
/// CORNER VERTEX coincides with the seam-born corner (1,1) of
/// AB = [1,2]^2 x [0,1], caps coplanar — a vertex-vertex site with two
/// polygon wedges. Exact oracle: the triangle {1<=x<=2, 1<=y<=x}
/// gives volume 1/2.
#[test]
fn a_multi_spike_shared_corner_vertex() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
    let ab = match run(topo::intersect_with, &a, &b) {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!(),
    };
    let c = prism_z::<f64>(&[(1.0, 1.0), (2.0, 0.0), (3.0, 1.0), (2.0, 2.0)], 0.0, 1.0).body;
    // REVIEW VERDICT: for z-parallel prisms a shared corner VERTEX
    // forces AB's vertical corner edge to lie IN C's vertical side
    // plane (any vertical plane through the vertex contains it) — the
    // documented boundary-on-boundary class. Refusal is typed,
    // deterministic, operand-preserving, with the on-edge count 4;
    // multi-spike vv sites AWAY from pre-existing corners are the
    // half-plane-sector class where the dot order is forced (see the
    // fig151 variants above). If this ever succeeds it must be exact.
    match topo::intersect_with(&ab, &c, &flush_declarations(&ab, &c), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
            assert_eq!(vol(&bb.body), 0.5, "corner-vertex meet EXACT");
        }
        Ok(BooleanResult::Empty) => panic!("nonempty overlap"),
        Err(_) => {
            let e = assert_typed_refusal(topo::intersect_with, &ab, &c);
            assert!(e.contains("UnpairedLooseEnds { count: 4 }"), "got {e}");
        }
    }
}

/// The REFLEX-SECTOR attack on `bool_strut_order` (the derivation's
/// unforced class): A has a 315-degree reflex corner at the origin
/// (material = everything but the 45-degree wedge between +x and
/// (1,1)); B is a z-sheared brick whose TILTED bottom cap passes
/// exactly through A's reflex top-cap corner vertex (0,0,1), its germ
/// line through the vertex at an angle > 90 degrees from one sector
/// bound — the window where the convex-sector dot comparison
/// misorders. Both germ-line orientations probed (whichever bound the
/// code anchors on, one variant lands in the window). Oracle:
/// vol(A meet B) = 13/24 (rational, non-dyadic — asserted to 1e-12;
/// a mis-nested chord would be off by O(0.1)).
#[test]
fn a_reflex_315_corner_tilted_cap() {
    let reflex = [
        (0.0, 0.0),
        (2.0, 2.0),
        (-2.0, 2.0),
        (-2.0, -2.0),
        (2.0, -2.0),
        (2.0, 0.0),
    ];
    let sq = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    // B cap plane z = 1 + (x + 2y)/4 resp. z = 1 + (2x - y)/4.
    let shears = [
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.25, 0.5, 1.0]],
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, -0.25, 1.0]],
    ];
    for (k, m) in shears.into_iter().enumerate() {
        let a = prism_z::<f64>(&reflex, 0.0, 1.0).body;
        assert_eq!(vol(&a), 14.0);
        let b = tprism::<f64>(&sq, 1.0, 3.0, m);
        assert_eq!(vol(&b), 8.0);
        match topo::intersect_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
            Ok(BooleanResult::Body(bb)) => {
                assert_eq!(validate_closed(&bb.body), Ok(()), "variant {k}");
                let v = vol(&bb.body);
                assert!(
                    (v - 13.0 / 24.0).abs() < 1e-12,
                    "variant {k}: SILENT WRONG VOLUME {v} (want 13/24)"
                );
                let r = run(subtract_with, &a, &b);
                let vs = vol(&body_of(&r).body);
                assert!(
                    (vs - (14.0 - 13.0 / 24.0)).abs() < 1e-12,
                    "variant {k}: sub {vs}"
                );
            }
            Ok(BooleanResult::Empty) => panic!("variant {k}: nonempty overlap"),
            Err(_) => {
                // Typed refusal is acceptable (honest envelope);
                // record its shape. REVIEW FINDING: both variants
                // refuse SeamOrientation — the zip witness catches the
                // reflex-sector misordering the derivation predicts;
                // never a wrong body. Undocumented envelope gap
                // (reflex-corner vertex under a tilted cap).
                let e = assert_typed_refusal(topo::intersect_with, &a, &b);
                assert!(e.contains("SeamOrientation"), "variant {k}: {e}");
            }
        }
    }
    // CONTROL: the same tilted cap crossing a PLAIN brick's top face
    // with the germ line through face INTERIOR (no reflex corner, no
    // vertex contact) must succeed with the same 13/24 volume — this
    // isolates the refusal to the reflex-corner vertex class.
    let a = brick::<f64>((-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0));
    let b = tprism::<f64>(&sq, 1.0, 3.0, shears[0]);
    let r = run(topo::intersect_with, &a, &b);
    let v = vol(&body_of(&r).body);
    assert!((v - 13.0 / 24.0).abs() < 1e-12, "control: {v} (want 13/24)");
    // Second control: a CONVEX (90-degree) corner vertex under the
    // same tilted cap — distinguishes "any vertex contact" from
    // "reflex sector". Exact dyadic oracle 3/8.
    let a = brick::<f64>((-2.0, 0.0), (-2.0, 0.0), (0.0, 1.0));
    let b = tprism::<f64>(&sq, 1.0, 3.0, shears[0]);
    match topo::intersect_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
            assert_eq!(vol(&bb.body), 0.375, "convex-vertex control EXACT");
        }
        Ok(BooleanResult::Empty) => panic!("convex control: nonempty"),
        Err(_) => {
            let e = assert_typed_refusal(topo::intersect_with, &a, &b);
            println!("convex-vertex control refuses: {e}");
        }
    }
}

/// Non-axis-aligned operand pairs with EXACT dyadic oracles: the
/// pocket fixture pushed through scaled rotations (45 deg about z as
/// the integer map (x-y, x+y), det 2), a z-shear (tilted caps, det 1),
/// a composed skew-axis map (det 4), and a REFLECTED (handedness-
/// flipped, det -1, reversed profile) shear. Volume scales by |det|.
#[test]
fn b_transformed_pocket_exact() {
    let sq = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let pil = [(0.75, 0.75), (1.25, 0.75), (1.25, 1.25), (0.75, 1.25)];
    let sq_r: Vec<_> = sq.iter().rev().copied().collect();
    let pil_r: Vec<_> = pil.iter().rev().copied().collect();
    let rz45 = [[1.0, -1.0, 0.0], [1.0, 1.0, 0.0], [0.0, 0.0, 1.0]]; // det 2
    let shear = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.0, 1.0]]; // det 1
    let skew = [[1.0, -1.0, 1.0], [1.0, 1.0, -1.0], [0.0, 1.0, 1.0]]; // det 4
    let refl = [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.0, 1.0]]; // det -1
    for (name, m, det) in [
        ("rz45-scaled", rz45, 2.0),
        ("z-shear", shear, 1.0),
        ("skew-axis", skew, 4.0),
    ] {
        let a = tprism::<f64>(&sq, 0.0, 2.0, m);
        let b = tprism::<f64>(&pil, 1.5, 2.5, m);
        assert_eq!(vol(&a), 8.0 * det, "{name}: operand oracle");
        let out = sub_exact(&a, &b, (8.0 - 0.125) * det);
        assert!(vol(&out) > 0.0, "{name}");
        let r = run(union_with, &a, &b);
        assert_eq!(vol(&body_of(&r).body), (8.0 + 0.125) * det, "{name}: union");
        let r = run(topo::intersect_with, &a, &b);
        assert_eq!(vol(&body_of(&r).body), 0.125 * det, "{name}: intersect");
    }
    // Reflected placement (det < 0, reversed profiles keep outward).
    let a = tprism::<f64>(&sq_r, 0.0, 2.0, refl);
    let b = tprism::<f64>(&pil_r, 1.5, 2.5, refl);
    assert_eq!(vol(&a), 8.0, "reflected operand oracle");
    sub_exact(&a, &b, 7.875);
    let r = run(union_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 8.125, "reflected union");
}

/// A 45-degree diamond pillar THROUGH an axis-aligned brick (mixed
/// alignment, double-ring tunnel lane): exact volumes for all ops,
/// operand swap included.
#[test]
fn b_diamond_through_brick_exact() {
    let a = brick::<f64>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
    let b = prism_z::<f64>(&[(2.0, 1.0), (3.0, 2.0), (2.0, 3.0), (1.0, 2.0)], -1.0, 2.0).body;
    // diamond area 2, inside height 1 -> meet = 2.
    let out = sub_exact(&a, &b, 16.0 - 2.0);
    assert_eq!(vol(&out), 14.0);
    let r = run(topo::intersect_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 2.0, "diamond core");
    let r = run(union_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 16.0 + 6.0 - 2.0, "union");
    // Operand swap: same geometry from B's side.
    let r = run(topo::intersect_with, &b, &a);
    assert_eq!(vol(&body_of(&r).body), 2.0, "swapped intersect");
    let swapped = sub_exact(&b, &a, 4.0);
    assert_eq!(vol(&swapped), 4.0);
}

// =====================================================================
// C. R1 closure depth.
// =====================================================================

/// Pocket spanning most of the top face (walls 0.1 thick): the ring
/// seam runs close to (never touching) all four face edges.
#[test]
fn c_pocket_spanning_most_of_face() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.125, 1.875), (0.125, 1.875), (1.5, 2.5));
    // pocket 1.75 x 1.75 x 0.5
    sub_exact(&a, &b, 8.0 - 1.75 * 1.75 * 0.5);
}

/// Pocket close to one edge (0.0625 clearance), strongly asymmetric.
#[test]
fn c_pocket_near_edge() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0625, 0.5625), (0.0625, 0.5625), (1.75, 2.25));
    sub_exact(&a, &b, 8.0 - 0.5 * 0.5 * 0.25);
}

/// Simultaneous pockets on ALL SIX faces of one brick in one op chain
/// (sequential subtracts on the running result), exact volume after
/// every step. The deepest R1-closure probe: every orientation's ring
/// lane fires against a body already carrying earlier seams.
#[test]
fn c_six_face_pocket_chain() {
    let pillars: [((f64, f64), (f64, f64), (f64, f64)); 6] = [
        ((0.75, 1.25), (0.75, 1.25), (1.5, 2.5)),  // +z
        ((0.75, 1.25), (0.75, 1.25), (-0.5, 0.5)), // -z
        ((1.5, 2.5), (0.75, 1.25), (0.75, 1.25)),  // +x
        ((-0.5, 0.5), (0.75, 1.25), (0.75, 1.25)), // -x
        ((0.75, 1.25), (1.5, 2.5), (0.75, 1.25)),  // +y
        ((0.75, 1.25), (-0.5, 0.5), (0.75, 1.25)), // -y
    ];
    let mut acc = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let mut expect = 8.0;
    for (i, (x, y, z)) in pillars.into_iter().enumerate() {
        let b = brick::<f64>(x, y, z);
        expect -= 0.5 * 0.5 * 0.5;
        acc = sub_exact(&acc, &b, expect);
        assert!(vol(&acc) == expect, "step {i}");
    }
    // 8 - 6 * 0.125 = 7.25, all six pockets present.
    assert_eq!(vol(&acc), 7.25);
}

/// The BOSS (union) orientation matrix mirroring the pocket one: the
/// identical pillar boss raised on each of the six faces, exact
/// volume + area each time.
#[test]
fn c_boss_orientation_matrix() {
    let pillars: [(&str, (f64, f64), (f64, f64), (f64, f64)); 6] = [
        ("+z", (0.75, 1.25), (0.75, 1.25), (1.5, 2.5)),
        ("-z", (0.75, 1.25), (0.75, 1.25), (-0.5, 0.5)),
        ("+x", (1.5, 2.5), (0.75, 1.25), (0.75, 1.25)),
        ("-x", (-0.5, 0.5), (0.75, 1.25), (0.75, 1.25)),
        ("+y", (0.75, 1.25), (1.5, 2.5), (0.75, 1.25)),
        ("-y", (0.75, 1.25), (-0.5, 0.5), (0.75, 1.25)),
    ];
    for (name, x, y, z) in pillars {
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<f64>(x, y, z);
        let r = run(union_with, &a, &b);
        let out = body_of(&r);
        assert_eq!(out.kind, BooleanResultKind::Seamed, "{name}");
        let m = mass_properties(&out.body, Tol::witness()).unwrap();
        assert_eq!(m.volume, 8.0 + 0.5 * 0.5 * 0.5, "{name}: boss volume");
        assert_eq!(m.surface_area, 24.0 + 4.0 * 0.25, "{name}: boss area");
    }
}

/// Pocket AND boss on the SAME face (two ops): the second ring seam
/// lands on a face already carrying a pocket seam.
#[test]
fn c_pocket_and_boss_same_face() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let pocket = brick::<f64>((0.25, 0.75), (0.25, 0.75), (1.5, 2.5));
    let boss = brick::<f64>((1.25, 1.75), (1.25, 1.75), (1.5, 2.5));
    let carved = sub_exact(&a, &pocket, 8.0 - 0.125);
    let r = run(union_with, &carved, &boss);
    let out = body_of(&r);
    assert_eq!(out.kind, BooleanResultKind::Seamed);
    // boss adds only its part above z=2: 0.5*0.5*0.5.
    assert_eq!(vol(&out.body), 8.0 - 0.125 + 0.125);
}

// =====================================================================
// E. R2 generalization: 6/8 collinear sites, mixed lanes, U-family.
// =====================================================================

/// SIX collinear sites on one germ line (three prongs cut by one
/// brick): base + three floating tips, exact volume.
#[test]
fn e_six_collinear_sites() {
    let w = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (5.0, 0.0),
            (5.0, 3.0),
            (4.0, 3.0),
            (4.0, 1.0),
            (3.0, 1.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    assert_eq!(vol(&w), 11.0);
    let b = brick::<f64>((-1.0, 6.0), (2.0, 2.5), (-0.5, 1.5));
    let r = run(subtract_with, &w, &b);
    let out = body_of(&r);
    assert_eq!(out.body.shells().count(), 4, "base + three tips");
    assert_eq!(vol(&out.body), 11.0 - 1.5);
}

/// EIGHT collinear sites (four prongs).
#[test]
fn e_eight_collinear_sites() {
    let comb = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (7.0, 0.0),
            (7.0, 3.0),
            (6.0, 3.0),
            (6.0, 1.0),
            (5.0, 1.0),
            (5.0, 3.0),
            (4.0, 3.0),
            (4.0, 1.0),
            (3.0, 1.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    assert_eq!(vol(&comb), 15.0);
    let b = brick::<f64>((-1.0, 8.0), (2.0, 2.5), (-0.5, 1.5));
    let r = run(subtract_with, &comb, &b);
    let out = body_of(&r);
    assert_eq!(out.body.shells().count(), 5, "base + four tips");
    assert_eq!(vol(&out.body), 15.0 - 2.0);
}

/// Collinear prong sites MIXED with transversal base crossings in one
/// op: the cutter dips into the base too (z-interior), so cap-plane
/// collinear runs and ordinary side-face crossings share the seam.
#[test]
fn e_collinear_mixed_with_transversal() {
    let w = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (5.0, 0.0),
            (5.0, 3.0),
            (4.0, 3.0),
            (4.0, 1.0),
            (3.0, 1.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = brick::<f64>((-1.0, 6.0), (0.5, 2.5), (0.25, 1.5));
    // REVIEW FINDING (E-2), CLOSED by the fix pass: the refusal was a
    // typed poison escalation (Join(RingHoming(Escalated {
    // point_in_loop_boundary, margin Invalid }))) from the ring
    // re-homing probe — a verbatim record of the diagnostic as
    // observed, so the name is the one that shipped then; the
    // degeneracy question it names now reports as
    // `point_in_loop_segment` —
    // `point_in_loop`'s boundary pre-pass divided by
    // a ZERO-LENGTH segment length (null scaffolding is legal in
    // mid-join loops), NaN-poisoning the foot distance. Degenerate
    // segments now measure the point distance exactly and the mixed
    // lane lands the exact oracle:
    // removed = prongs 3 * (1 * 1.5 * 0.75) + base 5 * 0.5 * 0.75.
    let r = subtract_with(&w, &b, &flush_declarations(&w, &b), Tol::witness()).unwrap();
    let BooleanResult::Body(bb) = r else {
        panic!("nonempty");
    };
    assert_eq!(validate_closed(&bb.body), Ok(()));
    assert_eq!(vol(&bb.body), 11.0 - 3.375 - 1.875, "mixed lane EXACT");
    // Neighbor variant: the same channel cut crossing the prongs
    // ENTIRELY (no collinear stub on the prong line).
    let b2 = brick::<f64>((-1.0, 6.0), (0.5, 3.5), (0.25, 1.5));
    match subtract_with(&w, &b2, &flush_declarations(&w, &b2), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
            assert_eq!(vol(&bb.body), 11.0 - 4.5 - 1.875, "full-cross EXACT");
        }
        Ok(BooleanResult::Empty) => panic!("nonempty"),
        Err(_) => {
            let e = assert_typed_refusal(subtract_with, &w, &b2);
            println!("full-cross variant refuses: {e}");
        }
    }
}

/// The U-slab family under ALL ops (not just subtract), with the
/// revert oracle on the subtract lane.
#[test]
fn e_uslab_all_ops() {
    let u = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    assert_eq!(vol(&u), 7.0);
    let b = brick::<f64>((-1.0, 4.0), (2.0, 2.5), (-0.5, 1.5));
    sub_exact(&u, &b, 6.0);
    let r = run(topo::intersect_with, &u, &b);
    let out = body_of(&r);
    assert_eq!(vol(&out.body), 1.0, "two prong bites");
    assert_eq!(out.body.shells().count(), 2, "disjoint bites");
    let r = run(union_with, &u, &b);
    assert_eq!(vol(&body_of(&r).body), 7.0 + 5.0 - 1.0, "union");
}

// =====================================================================
// F. Ring-lane role-order robustness (originally the residual_side_in
// probe; issue #93 replaced it with the intrinsic ring_run_ccw winding
// rule — these remain black-box attacks on the ring lane).
// =====================================================================

/// Probe vertices ON the other operand's boundary: an inscribed
/// diamond pillar whose four side planes pass exactly through ALL
/// four corners of A's top face (every non-seam outer-loop vertex of
/// the ring face lands OnBoundary). Must be exact or a typed refusal
/// — never a misclassified ring side.
#[test]
fn f_residual_probe_all_corners_on_boundary() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = prism_z::<f64>(
        &[(-1.0, 1.0), (1.0, -1.0), (3.0, 1.0), (1.0, 3.0)],
        1.0,
        3.0,
    )
    .body;
    match subtract_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => {
            assert_eq!(validate_closed(&bb.body), Ok(()));
            assert_eq!(vol(&bb.body), 4.0, "inscribed cutter EXACT");
        }
        Ok(BooleanResult::Empty) => panic!("nonempty"),
        Err(_) => {
            let e = assert_typed_refusal(subtract_with, &a, &b);
            println!("inscribed-diamond refuses: {e}");
        }
    }
}

/// Deeply nested ring seams: a second pocket cut into the FLOOR of a
/// first pocket (footprint strictly inside), the ring face itself
/// seam-born. Exact volumes at each step.
#[test]
fn f_nested_pocket_in_pocket_floor() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let p1 = brick::<f64>((0.5, 1.5), (0.5, 1.5), (1.5, 2.5));
    let carved = sub_exact(&a, &p1, 8.0 - 0.5);
    let p2 = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.25, 1.75));
    sub_exact(&carved, &p2, 8.0 - 0.5 - 0.0625);
}

/// One B producing BOTH a closed ring seam AND a transversal
/// edge-crossing seam on the SAME face of A: a U extruded along x
/// (axis permutation of tprism), one prong interior in y (ring), the
/// other crossing the y = 2 edge (open crossing).
#[test]
fn f_ring_plus_transversal_same_face() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    // (u, v) profile = (y, z) after the cyclic map (u,v,w) -> (w,u,v).
    let perm = [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let prof = [
        (0.5, 1.5),
        (1.0, 1.5),
        (1.0, 2.5),
        (1.75, 2.5),
        (1.75, 1.5),
        (2.25, 1.5),
        (2.25, 3.0),
        (0.5, 3.0),
    ];
    let b = tprism::<f64>(&prof, 0.5, 1.5, perm);
    // prong1 (ring): 1 x 0.5 x 0.5; prong2 (crossing): 1 x 0.25 x 0.5.
    sub_exact(&a, &b, 8.0 - 0.25 - 0.125);
}

/// TWO closed ring seams on the same face from one B (both prongs
/// interior).
#[test]
fn f_two_rings_same_face_from_one_operand() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let perm = [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    let prof = [
        (0.5, 1.5),
        (1.0, 1.5),
        (1.0, 2.5),
        (1.4, 2.5),
        (1.4, 1.5),
        (1.9, 1.5),
        (1.9, 3.0),
        (0.5, 3.0),
    ];
    let b = tprism::<f64>(&prof, 0.5, 1.5, perm);
    sub_exact(&a, &b, 8.0 - 0.25 - 0.25);
}

// =====================================================================
// G. Boundary-on-boundary refusal honesty.
// =====================================================================

/// Corner-flush {4} and stacked-full {8} — the PR 5.5 pinned
/// refusals — FLIPPED to exact successes under M5 S1's declared-REST
/// zip (the boundary-on-boundary class (iii) lane). The sharpness
/// claim survives inverted: the exact contact set takes the REST
/// lane, the epsilon-perturbed neighbors (shift 1/16, far beyond any
/// band) take the transversal chord lane, and BOTH produce exact
/// oracles — while the UNDECLARED exact contacts still refuse typed,
/// deterministic, operand-preserving at the coincidence door (the
/// boundary between the lanes is declared intent + the exact contact
/// set, never a mask).
#[test]
fn g_boundary_on_boundary_refusals_sharp() {
    // Corner-flush pillar: 2 contact-square edges on A's face edges.
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 0.5), (0.0, 0.5), (2.0, 3.0));
    let undeclared_refusal = |a: &Body<f64>, b: &Body<f64>| {
        let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
        let e1 = topo::union(a, b, Tol::witness()).map(|_| ()).unwrap_err();
        let e2 = topo::union(a, b, Tol::witness()).map(|_| ()).unwrap_err();
        assert_eq!(format!("{a:?}"), a0, "operand A untouched by refusal");
        assert_eq!(format!("{b:?}"), b0, "operand B untouched by refusal");
        assert_eq!(format!("{e1:?}"), format!("{e2:?}"), "deterministic");
        assert!(
            format!("{e1:?}").contains("UndeclaredCoincidence"),
            "{e1:?}"
        );
    };
    undeclared_refusal(&a, &b);
    let r = run(union_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 8.0 + 0.25, "corner-flush glued");
    // Stacked-full: B's bottom rim = A's top rim, all four on-edge.
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 3.0));
    undeclared_refusal(&a, &b);
    let r = run(union_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 8.0 + 4.0, "stacked-full glued");
    // Perturbed corner-flush (pulled 1/16 inside): exact union.
    let b = brick::<f64>((0.0625, 0.5625), (0.0625, 0.5625), (2.0, 3.0));
    let r = run(union_with, &a, &b);
    assert_eq!(vol(&body_of(&r).body), 8.0 + 0.25, "perturbed corner-flush");
    // Perturbed stack (shrunk 1/16 per side): exact union.
    let b = brick::<f64>((0.0625, 1.9375), (0.0625, 1.9375), (2.0, 3.0));
    let r = run(union_with, &a, &b);
    assert_eq!(
        vol(&body_of(&r).body),
        8.0 + 1.875 * 1.875,
        "perturbed stack"
    );
}

/// The documented refusal REASON checked against the data: on the
/// stacked-full fixture, dump the null-edge records — every germ at
/// the four shared-rim vv corner sites must have its germ line ALONG
/// an existing top-face edge direction (axis-aligned horizontal), the
/// class the docs say has no facing chord partner.
#[test]
fn g_stacked_full_on_edge_germ_dump() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 3.0));
    let red = topo::boolean_reduce_declared(
        topo::BooleanOp::Union,
        &a,
        &b,
        &flush_declarations(&a, &b),
        Tol::witness(),
    )
    .unwrap();
    assert_eq!(red.contacts.vv.len(), 4, "four shared rim corners");
    assert!(!red.null_edges.is_empty(), "seam scaffolding minted");
    let mut horizontal_axis = 0;
    let mut other = 0;
    for r in &red.null_edges {
        for g in &r.germs {
            let d = g.dir;
            let axis = (d.x.abs() == 1.0 && d.y == 0.0 && d.z == 0.0)
                || (d.y.abs() == 1.0 && d.x == 0.0 && d.z == 0.0);
            if axis {
                horizontal_axis += 1;
            } else {
                other += 1;
            }
        }
    }
    // ALL germ lines lie along the shared rim's edge directions: the
    // on-edge class, exactly as documented.
    assert_eq!(
        other, 0,
        "every germ on-edge (got {horizontal_axis} on-edge)"
    );
    assert!(horizontal_axis > 0);
}

// =====================================================================
// D. THE DIE: 21 single-ring pip pockets across all six faces.
// =====================================================================

/// Pip pillar bricks for a standard die on the [0,2]^3 cube: face
/// normal axis + pip centers on a {0.5, 1.0, 1.5} grid; pips 0.25
/// square, 0.125 deep. Opposite faces sum to 7.
fn die_pips() -> Vec<Body<f64>> {
    let g = [0.5, 1.0, 1.5];
    let layouts: [(usize, bool, Vec<(f64, f64)>); 6] = [
        (2, true, vec![(g[1], g[1])]), // +z: 1
        (
            2,
            false,
            vec![
                (g[0], g[0]),
                (g[1], g[1]),
                (g[2], g[2]),
                (g[0], g[2]),
                (g[2], g[0]),
                (g[0], g[1]),
            ],
        ), // -z: 6
        (0, true, vec![(g[0], g[0]), (g[2], g[2])]), // +x: 2
        (
            0,
            false,
            vec![
                (g[0], g[0]),
                (g[2], g[2]),
                (g[0], g[2]),
                (g[2], g[0]),
                (g[1], g[1]),
            ],
        ), // -x: 5
        (1, true, vec![(g[0], g[0]), (g[1], g[1]), (g[2], g[2])]), // +y: 3
        (
            1,
            false,
            vec![(g[0], g[0]), (g[2], g[2]), (g[0], g[2]), (g[2], g[0])],
        ), // -y: 4
    ];
    let mut pips = Vec::new();
    for (axis, positive, centers) in layouts {
        for (u, v) in centers {
            let (ur, vr) = ((u - 0.125, u + 0.125), (v - 0.125, v + 0.125));
            let nr = if positive {
                (1.875, 2.5)
            } else {
                (-0.5, 0.125)
            };
            let (x, y, z) = match axis {
                0 => (nr, ur, vr),
                1 => (ur, nr, vr),
                _ => (ur, vr, nr),
            };
            pips.push(brick::<f64>(x, y, z));
        }
    }
    assert_eq!(pips.len(), 21);
    pips
}

// =====================================================================
// Interval lane: the new fixture families decide identically.
// =====================================================================

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn interval_new_fixtures() {
        // Diamond through brick (mixed alignment).
        let a = brick::<Interval>((0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
        let b =
            prism_z::<Interval>(&[(2.0, 1.0), (3.0, 2.0), (2.0, 3.0), (1.0, 2.0)], -1.0, 2.0).body;
        let r = run(subtract_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        // Sheared (tilted-plane) pocket.
        let shear = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.5, 0.0, 1.0]];
        let sq = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
        let pil = [(0.75, 0.75), (1.25, 0.75), (1.25, 1.25), (0.75, 1.25)];
        let a = tprism::<Interval>(&sq, 0.0, 2.0, shear);
        let b = tprism::<Interval>(&pil, 1.5, 2.5, shear);
        let r = run(subtract_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        // Six collinear sites.
        let w = prism_z::<Interval>(
            &[
                (0.0, 0.0),
                (5.0, 0.0),
                (5.0, 3.0),
                (4.0, 3.0),
                (4.0, 1.0),
                (3.0, 1.0),
                (3.0, 3.0),
                (2.0, 3.0),
                (2.0, 1.0),
                (1.0, 1.0),
                (1.0, 3.0),
                (0.0, 3.0),
            ],
            0.0,
            1.0,
        )
        .body;
        let cutter = brick::<Interval>((-1.0, 6.0), (2.0, 2.5), (-0.5, 1.5));
        let r = run(subtract_with, &w, &cutter);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        // A -z boss union (orientation matrix representative) and one
        // die pip.
        let a = brick::<Interval>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<Interval>((0.75, 1.25), (0.75, 1.25), (-0.5, 0.5));
        let r = run(union_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        let pip = brick::<Interval>((0.875, 1.125), (0.875, 1.125), (1.875, 2.5));
        let r = run(subtract_with, &a, &pip);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
    }
}

/// THE DIE, end to end in topo: 21 sequential pip subtracts, exact
/// volume after each op, revert oracle and D9 replay throughout,
/// tiers 1+2 on every intermediate.
#[test]
fn d_die_21_pips_exact() {
    let mut acc = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let mut expect = 8.0;
    let pip_vol = 0.25 * 0.25 * 0.125;
    for (i, pip) in die_pips().iter().enumerate() {
        expect -= pip_vol;
        acc = sub_exact(&acc, pip, expect);
        assert!(vol(&acc) == expect, "pip {i}");
    }
    assert_eq!(vol(&acc), 8.0 - 21.0 * pip_vol); // 7.8359375
    assert_eq!(
        mass_properties(&acc, Tol::witness()).unwrap().surface_area,
        24.0 + 21.0 * 4.0 * 0.25 * 0.125
    );
}
