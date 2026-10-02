//! **JOIN-1's mechanisms, each held by a row that goes red when it is
//! taken away** (the dual review's minor 1). Measured on the fix-pass
//! head, one mutation at a time:
//!
//! - **locus matching** — the chord join pairs a segment's two ends by
//!   the cells their germs lie in (`partners` compares `a_locus` /
//!   `b_locus`). Comparing the germs' recorded faces instead turns
//!   [`matching_reads_the_germs_loci`] red, and nothing else here.
//! - **the structural skip** — on the boolean lanes a chord is not
//!   minted when the edge between the two halves IS the segment's
//!   locus edge (`SegmentEdge::Is`). Never skipping turns
//!   [`the_skip_takes_the_locus_edge_for_the_segment`] red, and it
//!   stays green under the other two mutations.
//! - **the fold direction** — an on-bound joins the In run unless both
//!   neighbours read Out (`sectors::fold_on_bound`). Folding mixed
//!   bounds Out instead turns two `review_m3_pr55` rows
//!   (`g_stacked_full_on_edge_germ_dump`,
//!   `g_boundary_on_boundary_refusals_sharp`) and
//!   `verbs_1031b_arcwind::the_boolean_on_the_cup_reaches_the_join`
//!   red, which the other two mutations leave green; it also turns
//!   [`matching_reads_the_germs_loci`] red. (Re-measured at fix pass 2.)
//!
//! Each row asserts the body that builds: tiers 2 and 3′, the at-rest
//! certificate, the closed-form volume, and that it is a legal boolean
//! operand.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{collar_at, peg_at, volume, wall_decls};
use geom_core::Tol;
use topo::{BooleanError, BooleanResult};

fn tol() -> Tol {
    Tol::witness()
}

fn assert_sound(what: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let r = r.unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let bb = r.body().unwrap_or_else(|| panic!("{what}: empty"));
    topo::validate_closed(&bb.body).unwrap_or_else(|e| panic!("{what}: tier 2: {e:?}"));
    topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
        .unwrap_or_else(|e| panic!("{what}: tier 3′: {e:?}"));
    topo::validate_geometric_certificate(&bb.body, tol())
        .unwrap_or_else(|e| panic!("{what}: certificate: {e:?}"));
    let v = topo::mass_properties(&bb.body, tol()).unwrap().volume;
    assert!((v - want).abs() < 1e-9, "{what}: volume {v} against {want}");
    // Every boolean output is a legal boolean operand (DESIGN).
    let far = sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (0.0, 1.0), tol());
    topo::union(&bb.body, &far, tol())
        .unwrap_or_else(|e| panic!("{what}: the result is no legal operand: {e:?}"));
}

/// The walls declared `Rest` AND every flush pair of two planar faces
/// (the peg's caps in the collar's cap planes, abutting across the rim,
/// same sense). Left undeclared, that continuation would hand a declared
/// union to the REST zip instead of the chord join these rows are about
/// (`BooleanReduction::continuation`); declared, the merge stage glues
/// it and the result is a legal operand.
fn walls_and_caps(c: &topo::Body<f64>, p: &topo::Body<f64>) -> topo::BooleanDeclarations {
    let mut decls = wall_decls(c, p);
    let planar = |body: &topo::Body<f64>, f: topo::FaceKey| {
        body.get_face(f)
            .and_then(|fd| body.get_surface(fd.surface))
            .is_some_and(|s| matches!(s, geom::Surface::Plane { .. }))
    };
    for f in topo::flush::find_flush_candidates(c, p, tol()).expect("the detector decides") {
        let (fa, fb) = f.pair;
        if planar(c, fa) && planar(p, fb) {
            decls
                .coincident_faces
                .push(topo::FacePairDeclaration::new(fa, fb, f.class));
        }
    }
    decls
}

/// **Locus matching.** A peg in a collar's bore, flush with the
/// collar's bottom and proud above its top, every wall pair and the
/// flush bottom caps declared (`walls_and_caps`). The peg's bottom rim
/// arcs lie on the bore's: each section segment there is an edge of
/// both solids, and the loci name the same two edges at both of its
/// ends. The union is the collar with the
/// peg's proud part: additive.
///
/// The declared-REST zip takes over a declared union the join refuses,
/// and builds this one too, so the volume alone cannot tell the lanes
/// apart; the merge door's record can. The zip keeps the bore wall for
/// the door, which records the declared cylinder pair it has no arm
/// for; the join discards the bore wall with its surface, so the door
/// is handed no pair. The row asserts the join's answer.
#[test]
fn matching_reads_the_germs_loci() {
    let (c, p) = (collar_at(0.0), peg_at(0.0, 1.0, 1.5));
    let want = volume(&c) + volume(&p);
    let r = topo::union_with(&c, &p, &walls_and_caps(&c, &p), tol());
    if let Ok(BooleanResult::Body(bb)) = &r {
        let declared = bb
            .naming
            .merge_skipped
            .iter()
            .filter(|s| {
                matches!(
                    s.reason,
                    topo::MergeCoplanarError::DeclaredCarrierUnsupported { .. }
                )
            })
            .count();
        assert_eq!(
            declared, 0,
            "the chord join builds the union, not the zip that takes over its refusal"
        );
    }
    assert_sound("peg ∪ collar", r, want);
}

/// **The structural skip.** A peg exactly the collar's height, flush
/// at both ends, every wall pair and both cap pairs declared: the union is the
/// unbored collar. Both rims are edges of both solids. Each solid
/// mints ONE copy of a rim arc, in the face its fold leaves Out, and
/// keeps the rim itself as the other copy: the chord on the far side
/// of the rim is skipped because the edge between the two halves there
/// IS the segment's locus edge.
#[test]
fn the_skip_takes_the_locus_edge_for_the_segment() {
    let (c, p) = (collar_at(0.0), peg_at(0.0, 1.0, 1.0));
    let want = volume(&c) + volume(&p);
    assert_sound(
        "flush peg ∪ collar",
        topo::union_with(&c, &p, &walls_and_caps(&c, &p), tol()),
        want,
    );
}

/// **The OnEdge incidence check reads the whole site** (the dual
/// review's minor 2). The reflex-corner probe's `eLeft` profile, sheared
/// `sx = −0.5` and `−0.25` with `sy = 0`: its bottom edge through the
/// reflex corner meets that corner's site, which holds several null
/// edges, and an `OnEdge` germ's edge hangs on a different copy there
/// than the null half facing it. The check used to read only that half's
/// two ends and refused `JoinDesync { "an OnEdge germ's edge is not
/// incident to its site" }` under every op; it reads the site the null
/// edges tie together now. The poses then stop where they stopped
/// before JOIN-1, in the reflex corner's own refusals
/// (`work/join/reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap`):
/// `JoinDesync { "every chord arc separates a loose scaffolding pair" }`
/// under every op, as on main. The row pins that outcome exactly.
#[test]
fn the_incidence_check_reads_the_whole_site() {
    use topo::test_support::{
        FaceGeometry, describe_as_intersections, flush_declarations, prism_ops, prism_z,
    };
    // The 315° reflex prism (CCW): material everywhere but the 45°
    // wedge between +x and (1, 1).
    let a_prof = [
        (0.0, 0.0),
        (2.0, 2.0),
        (-2.0, 2.0),
        (-2.0, -2.0),
        (2.0, -2.0),
        (2.0, 0.0),
    ];
    let a = prism_z::<f64>(&a_prof, 0.0, 1.0, tol()).body;
    let prof = [(0.0, -0.5), (1.0, -0.5), (1.0, 0.5), (0.0, 0.5)];
    for sx in [-0.5, -0.25] {
        let mut b = topo::Body::<f64>::new();
        prism_ops(
            &mut b,
            &prof,
            (1.0, 3.0),
            |x, y, z| geom_core::Point3::new(x, y, z + sx * x),
            FaceGeometry::Certified,
            tol(),
        );
        describe_as_intersections(&mut b, tol());
        let d = flush_declarations(&a, &b, tol());
        for (op, r) in [
            ("∩", topo::intersect_with(&a, &b, &d, tol())),
            ("∪", topo::union_with(&a, &b, &d, tol())),
            ("∖", topo::subtract_with(&a, &b, &d, tol())),
        ] {
            // Exactly where main stopped these poses: the reflex corner's
            // own refusal, measured, not this unit's incidence check.
            assert!(
                matches!(
                    r,
                    Err(BooleanError::JoinDesync {
                        what: "every chord arc separates a loose scaffolding pair"
                    })
                ),
                "sx = {sx} {op}: {r:?}"
            );
        }
    }
}

/// **The north-star crosslap** (`demos/tour/src/crosslap.rs`;
/// `test_north_star.py`'s `TestCrosslapGlued`): two notched beams mated.
/// With the mate's `SameOpposite` pairs alone declared, the beams' tops
/// and bottoms flush across the notches are an undeclared continuation,
/// refused at the reduction. With every finding declared (the mate and
/// the continuations) the union builds at the scene's oracle, 14 faces,
/// sound and a legal operand.
#[test]
fn the_declared_crosslap_glues() {
    use sweep::test_support::brick;
    use topo::flush::{declare_all, find_flush_candidates};
    let sub = |a: &topo::Body<f64>, b: &topo::Body<f64>| match topo::subtract(a, b, tol()).unwrap()
    {
        BooleanResult::Body(bb) => bb.body,
        BooleanResult::Empty => panic!("a notched beam"),
    };
    let a = sub(
        &brick((0.0, 4.0), (1.75, 2.25), (0.0, 0.5), tol()),
        &brick((1.75, 2.25), (1.5, 2.5), (0.25, 0.75), tol()),
    );
    let b = sub(
        &brick((1.75, 2.25), (0.0, 4.0), (0.0, 0.5), tol()),
        &brick((1.5, 2.5), (1.75, 2.25), (-0.25, 0.25), tol()),
    );
    let found = find_flush_candidates(&a, &b, tol()).unwrap();
    let mate: Vec<_> = found
        .iter()
        .filter(|f| f.evidence.relation == topo::PlaneRelation::SameOpposite)
        .cloned()
        .collect();
    let r = topo::union_with(&a, &b, &declare_all(&mate), tol());
    assert!(
        matches!(
            r,
            Err(BooleanError::UndeclaredCoincidence {
                relation: topo::PlaneRelation::SameOriented,
                ..
            })
        ),
        "the mate alone leaves the continuations undeclared: {r:?}"
    );
    let r = topo::union_with(&a, &b, &declare_all(&found), tol());
    let faces = r
        .as_ref()
        .ok()
        .and_then(|r| r.body())
        .map(|bb| bb.body.faces().count());
    assert_eq!(faces, Some(14), "every flush pair declared: {r:?}");
    assert_sound("crosslap ∪", r, 1.875);
}
