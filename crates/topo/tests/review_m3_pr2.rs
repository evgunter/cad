//! Adversarial review suite for M3 PR 2 (split part 1: reduction +
//! neighborhood classification). Independent derivations — fixtures and
//! expected censuses derived from the geometry by the reviewer, not
//! copied from the shipped tests.
//!
//! Naming: R1.. tags match the review report's findings/witnesses.
//!
//! **ITS PROBE-GATED CODE IS NOT EXECUTED BY CI**, and the rest of this file
//! IS executed: only the K-telemetry block inside
//! `r5_crossing_vertex_on_is_declared_not_measured` sits behind the feature,
//! and no CI row passes it, while every test here runs on every merge. The
//! probe suites CI runs are rostered in `scripts/gates/probe-suite-census.sh`
//! (`RUN_FLOOR`) and run by `scripts/k_probe_sweep.sh`; this file is on
//! neither list, so claim (c) below is evidence for a reader rather than a
//! gate. By hand:
//! `cargo test -p topo --features probe --test all -- review_m3_pr2::`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, PlaneSide, SplitPlane, SplitReduceError, VertexKey, split_reduce};

fn plane_y<T: geom_core::Decide>(y: f64, ny: f64) -> SplitPlane<T> {
    SplitPlane {
        origin: Point3::new(T::from_f64(0.0), T::from_f64(y), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(0.0), T::from_f64(ny), T::from_f64(0.0)),
    }
}

fn point_of(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn vertex_at(body: &Body<f64>, x: f64, y: f64, z: f64) -> VertexKey {
    let hits: Vec<_> = body
        .vertices()
        .filter(|(_, v)| {
            let p = *body.get_point(v.point).unwrap();
            p.x == x && p.y == y && p.z == z
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(hits.len(), 1, "expected one vertex at ({x},{y},{z})");
    hits[0]
}

/// Does any edge of `body` join vertices `a` and `b` (either order)?
fn joins(body: &Body<f64>, a: VertexKey, b: VertexKey) -> bool {
    body.edges().any(|(_, e)| {
        let s1 = body.get_half_edge(e.he_plus).unwrap().start;
        let s2 = body.get_half_edge(e.he_minus).unwrap().start;
        (s1 == a && s2 == b) || (s1 == b && s2 == a)
    })
}

/// R1a — independent tangent-edge fixture (reviewer's own dims: block
/// [0,10]x[0,4], V-notch tip at (6,2), split plane y = 2). The AOA
/// adjudication's load-bearing consequence, derived independently:
/// Above's material at the tip is two wedges whose face fans are
/// DISJOINT (slantL+capL vs slantR+capR share no face), and a half-edge
/// vertex admits exactly one cyclic orbit — so one shared copy cannot
/// host both wedges. AOA→BELOW must therefore mint TWO copies, and each
/// copy's orbit must be exactly {its slant half-edge, its null half}.
#[test]
fn r1a_tangent_tip_two_disjoint_copies_with_two_edge_orbits() {
    let profile = [
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (8.0, 4.0),
        (6.0, 2.0), // V tip ON y = 2
        (4.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(2.0, 1.0), Tol::witness()).unwrap();
    for z in [0.0, 1.0] {
        let tip = vertex_at(&fx.body, 6.0, 2.0, z);
        let recs: Vec<_> = red
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        assert_eq!(recs.len(), 2, "two ABOVE runs at the tip");
        assert!(recs.iter().all(|r| !r.dangling));
        let copies = [recs[0].attr.above_end, recs[1].attr.above_end];
        assert_ne!(copies[0], copies[1], "distinct copies (book AOA→BELOW)");
        // Each copy's orbit: exactly one real (slant) half-edge plus its
        // null half — a single wedge fan per copy, never both.
        for &c in &copies {
            let anchor = red.body.get_vertex(c).unwrap().emanating.unwrap();
            let orbit = red.body.vertex_orbit(anchor).unwrap();
            assert_eq!(orbit.len(), 2, "copy orbit = {{slant, null}}");
            // Exactly one orbit member ends Above (the slant); the other
            // ends ON (the null edge back to the old vertex).
            let mut ends: Vec<_> = orbit
                .iter()
                .map(|&he| red.sides[red.body.half_edge_end(he).unwrap()])
                .collect();
            ends.sort_by_key(|s| matches!(s, PlaneSide::On));
            assert_eq!(ends, vec![PlaneSide::Above, PlaneSide::On]);
        }
        // The tip edge itself stays on the old (Below-side) vertices.
        let tip_other = vertex_at(&fx.body, 6.0, 2.0, 1.0 - z);
        assert!(joins(&red.body, tip, tip_other), "tip edge kept below");
    }
}

/// R1b — split-plane orientation equivariance: the argument that
/// actually FORCES BOB→ABOVE once AOA→BELOW is established. Splitting
/// the touching-wedge body by (o, +n) and by (o, −n) must assign the
/// tangent tip edge to the same PHYSICAL side (the physically-above
/// piece), because the −n run reads the same physical configuration as
/// AOA. Book's table (AOA→B, BOB→A) is swap-symmetric and passes; a
/// mixed table (e.g. book AOA + TOG BOB) would fail this test.
/// Reviewer's own wedge: block [0,10]x[0,4], Λ tip from below at (3,2).
#[test]
fn r1b_orientation_equivariance_pins_bob_from_aoa() {
    let profile = [
        (0.0, 0.0),
        (2.0, 0.0),
        (3.0, 2.0), // Λ tip ON y = 2, material above
        (4.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let (tip_b, tip_t) = (
        vertex_at(&fx.body, 3.0, 2.0, 0.0),
        vertex_at(&fx.body, 3.0, 2.0, 1.0),
    );

    // +n (Above = +y): BOB→ABOVE moves the tip edge to Above copies —
    // physically the above side. One dangling null (wide cap bisector).
    let red_pos = split_reduce(&fx.body, &plane_y(2.0, 1.0), Tol::witness()).unwrap();
    assert!(
        !joins(&red_pos.body, tip_b, tip_t),
        "+n: tip edge left the old vertices (BOB→ABOVE)"
    );
    for tip in [tip_b, tip_t] {
        let recs: Vec<_> = red_pos
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        assert_eq!(recs.len(), 2);
        assert_eq!(recs.iter().filter(|r| r.dangling).count(), 1);
    }

    // −n (frame-Above = physical below): the tip context reads AOA;
    // AOA→BELOW keeps the tip edge on the OLD vertices — which are the
    // frame-below = physically-ABOVE side. Same physical assignment.
    let red_neg = split_reduce(&fx.body, &plane_y(2.0, -1.0), Tol::witness()).unwrap();
    assert!(
        joins(&red_neg.body, tip_b, tip_t),
        "−n: tip edge stays on old vertices (AOA→BELOW) = physically above"
    );
    for tip in [tip_b, tip_t] {
        let recs: Vec<_> = red_neg
            .null_edges
            .iter()
            .filter(|r| r.at_vertex == tip)
            .collect();
        // Two frame-Above (physically below) wedge runs ⇒ two copies,
        // none dangling: the physically-below rim edges get the copies.
        assert_eq!(recs.len(), 2);
        assert!(recs.iter().all(|r| !r.dangling));
        let copies: Vec<_> = recs.iter().map(|r| r.attr.above_end).collect();
        assert_ne!(copies[0], copies[1]);
    }
}

/// R2 — the tangency residue, executed: a triangular prism touching the
/// plane from above along its apex edge only (no below material
/// anywhere). Documents EXACTLY what PR 3 receives: one wrapped ABOVE
/// run per apex vertex ⇒ one non-dangling null edge; the old vertex is
/// left holding only the bare tangent edge plus the null half — the
/// degenerate Below piece the adjudication record promises PR 3 must
/// detect and refuse. NOTE the `dangling` flag is FALSE here: PR 3 has
/// no PR 2-level flag for this residue, only the reconstructable fact
/// that the below side of these ON vertices holds no real material.
#[test]
fn r2_one_sided_tangency_residue_documented() {
    let profile = [(3.0, 4.0), (6.0, 1.0), (9.0, 4.0)]; // CCW, apex down
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0), Tol::witness()).unwrap();
    assert_eq!(red.on_vertices.len(), 2); // apex bottom + top
    assert_eq!(red.null_edges.len(), 2); // one per apex vertex
    assert!(red.null_edges.iter().all(|r| !r.dangling));
    for z in [0.0, 1.0] {
        let apex = vertex_at(&fx.body, 6.0, 1.0, z);
        let anchor = red.body.get_vertex(apex).unwrap().emanating.unwrap();
        let orbit = red.body.vertex_orbit(anchor).unwrap();
        // Old vertex keeps: the tangent (apex) edge + the null half.
        assert_eq!(orbit.len(), 2, "below residue = bare edge + null");
        let ends: Vec<_> = orbit
            .iter()
            .map(|&he| red.sides[red.body.half_edge_end(he).unwrap()])
            .collect();
        assert!(ends.iter().all(|&s| s == PlaneSide::On));
    }
}

/// R3 — rule (a) neighbor propagation on an all-ON neighborhood: a
/// collinear ON run (6,1)-(4,1)-(2,1) puts a vertex whose EVERY orbit
/// entry starts ON (two coplanar-sector edges + the strut). Rule (a)
/// must reclassify all three Below via the two flanking coplanar side
/// faces (material below ⇒ outward +y ⇒ BELOW), the straight (exactly
/// 180°) cap corner must take the convex-subdivision duplicate without
/// escalating, and NO ConsecutiveOnSectors false-refusal may fire.
/// Census derived independently: ON = 6 structural + 4 crossings; null
/// edges = 4 crossings + 2 notch corners × 2 rims = 8; the mid vertex
/// (4,1) mints ZERO null edges (its whole neighborhood is Below).
#[test]
fn r3_collinear_on_run_all_on_neighborhood() {
    let profile = [
        (0.0, 0.0),
        (8.0, 0.0),
        (8.0, 2.0),
        (6.0, 1.0),
        (4.0, 1.0), // mid vertex of the collinear ON run
        (2.0, 1.0),
        (0.0, 2.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0), Tol::witness()).unwrap();
    assert_eq!(red.on_vertices.len(), 10);
    assert_eq!(red.null_edges.len(), 8);
    for z in [0.0, 1.0] {
        let mid = vertex_at(&fx.body, 4.0, 1.0, z);
        assert_eq!(
            red.null_edges.iter().filter(|r| r.at_vertex == mid).count(),
            0,
            "all-Below neighborhood mints nothing"
        );
        for (x, expect) in [(6.0, 1), (2.0, 1)] {
            let v = vertex_at(&fx.body, x, 1.0, z);
            assert_eq!(
                red.null_edges.iter().filter(|r| r.at_vertex == v).count(),
                expect
            );
        }
    }
}

/// R4 — straight-band soundness probe: an ON vertex whose cap-face
/// corner is EXACTLY 180° (collinear diagonal boundary through the
/// plane) exercises the sin≈0/cosine-disambiguation path and the
/// claimed no-escalation-cliff. Independent truth: the above material
/// at (4,1) is ONE contiguous wedge (side-face above part + cap above
/// part), so exactly ONE null edge may be minted here — a duplicate
/// landing cyclically non-adjacent to its material would split the
/// wedge into two runs and falsify the convex-subdivision claim.
#[test]
fn r4_straight_cap_corner_single_wedge_single_null_edge() {
    let profile = [
        (0.0, 0.0),
        (3.0, 0.0),
        (4.0, 1.0), // ON; neighbors (3,0) and (5,2) are collinear
        (5.0, 2.0),
        (8.0, 2.0),
        (8.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0), Tol::witness()).unwrap();
    // Crossings: x=0 wall rims at y=1 (2 of them: z=0, z=1) — plus the
    // two structural ON vertices at (4,1).
    assert_eq!(red.on_vertices.len(), 4);
    for z in [0.0, 1.0] {
        let v = vertex_at(&fx.body, 4.0, 1.0, z);
        let recs: Vec<_> = red.null_edges.iter().filter(|r| r.at_vertex == v).collect();
        assert_eq!(recs.len(), 1, "one contiguous above wedge ⇒ one run");
        assert!(!recs[0].dangling);
    }
    // Total: 2 crossings + 2 straight-corner vertices.
    assert_eq!(red.null_edges.len(), 4);
}

/// R5 — "crossing vertices are ON by construction": what the declared
/// coincidence actually is, measured. Diagonal crossing edges at 1e5
/// coordinate scale against a diagonal plane: the interpolated point is
/// genuinely OFF the plane (extended-precision residual ≈ 5e-14 here —
/// nonzero, so ON is a declaration, not a measurement), while the f64
/// margin predicate itself often computes exactly 0 for its own
/// construction (correlated rounding). The test (a) witnesses the
/// nonzero true residual, (b) checks it stays below the coincidence
/// threshold at the current ε row (declared and measured agree at sane
/// scales), (c) proves via K-telemetry that the reduction never
/// re-measures a constructed vertex, and (d) checks a consumer
/// re-sweeping the reduced body reproduces ON.
///
/// (c) is the only Probe-dependent claim in this file, and it is a
/// block INSIDE this test rather than a test of its own — so it carries
/// its own `#[cfg(feature = "probe")]` instead of the file being split.
/// (a), (b) and (d) run in the default build.
#[test]
fn r5_crossing_vertex_on_is_declared_not_measured() {
    // Plane x + 3y = 400000, normal (1,3,0)/√10; crossing edges hit it
    // at non-dyadic parameters.
    let profile = [
        (99004.0, 98986.0),
        (101007.0, 99000.0),
        (101011.0, 101017.0),
        (98993.0, 101003.0),
    ];
    let l = 10.0f64.sqrt();
    let plane = SplitPlane {
        origin: Point3::new(100000.0, 100000.0, 0.0),
        normal: Vec3::new(1.0 / l, 3.0 / l, 0.0),
    };
    let fx = prism::<f64>(&profile, 1.0);
    let band = geom_core::Band::linear(Tol::witness()).unwrap();

    let red = match split_reduce(&fx.body, &plane, Tol::witness()) {
        Ok(red) => red,
        // At the strictest ε row the certified split_edge lane REFUSES
        // this construction outright: the child-curve re-certification
        // residual at 1e5-scale coordinates exceeds a 1e-12 band
        // (ResidualExceeded, typed). That is the fail-loud backstop for
        // exactly the off-plane-construction concern this test probes —
        // pin it and stop here for such rows.
        // (Fix pass MINOR-2: the refusal now carries the crossing site
        // — edge + straddling endpoints — with the typed error nested.)
        Err(SplitReduceError::CrossingInsertion {
            source: topo::EulerOpError::Certification { .. },
            ..
        }) => {
            assert!(
                band.zero() < 1e-10,
                "certification refusal expected only at strict ε rows"
            );
            return;
        }
        Err(other) => panic!("unexpected refusal: {other:?}"),
    };

    // (c) Telemetry (the Probe recording scalar): split_vertex_side ran
    // exactly once per OPERAND vertex — constructed crossing vertices
    // and null-edge copies are cached, never re-measured through the
    // predicate.
    //
    // `Probe` is behind the `probe` feature (it is a `Real`
    // instantiation, so everything generic monomorphizes at it), and
    // (a)/(b)/(d) below are f64 claims that must keep running in the
    // default build — so the gate is on this block rather than on the
    // whole test or the whole file.
    #[cfg(feature = "probe")]
    {
        use geom_core::k_stats::{Probe, start_recording, take_samples};
        let n_operand_vertices = fx.body.vertices().count();
        let fx_p = prism::<Probe>(&profile, 1.0);
        let plane_p = SplitPlane {
            origin: Point3::new(Probe(100000.0), Probe(100000.0), Probe(0.0)),
            normal: Vec3::new(Probe(1.0 / l), Probe(3.0 / l), Probe(0.0)),
        };
        start_recording();
        let red_p = split_reduce(&fx_p.body, &plane_p).unwrap();
        let samples = take_samples();
        assert_eq!(red_p.on_vertices.len(), 4);
        let sweeps = samples
            .iter()
            .filter(|s| s.predicate == "split_vertex_side")
            .count();
        assert_eq!(sweeps, n_operand_vertices, "no re-measurement anywhere");
    }

    assert_eq!(red.on_vertices.len(), 4); // 2 crossing segments × 2 rims
    // (a)+(b): extended-precision residual (two_prod/two_sum) of each
    // stored crossing point.
    let two_sum = |a: f64, b: f64| {
        let s = a + b;
        let bp = s - a;
        (s, (a - (s - bp)) + (b - bp))
    };
    let two_prod = |a: f64, b: f64| {
        let p = a * b;
        (p, a.mul_add(b, -p))
    };
    let mut max_residual: f64 = 0.0;
    for &v in &red.on_vertices {
        assert_eq!(red.sides[v], PlaneSide::On, "declared ON");
        let p = point_of(&red.body, v);
        let (h1, e1) = two_prod(p.x - plane.origin.x, plane.normal.x);
        let (h2, e2) = two_prod(p.y - plane.origin.y, plane.normal.y);
        let (s, es) = two_sum(h1, h2);
        let residual = s + (es + e1 + e2);
        max_residual = max_residual.max(residual.abs());
    }
    assert!(
        max_residual > 0.0,
        "all constructed points exactly on-plane — the declared-ON \
         question would be vacuous here"
    );
    assert!(
        max_residual < band.zero(),
        "constructed-point residual {max_residual:e} exceeds the \
         coincidence threshold {:e}: declared ON is inconsistent with \
         the band at this ε row",
        band.zero()
    );
    // (d) A consumer CANNOT re-sweep the reduced body through the
    // public gate: it now carries null scaffolding, and the operand
    // gate refuses it typed (ScaffoldingOperand). The declared-ON cache
    // in `red.sides` is therefore the only currency downstream — which
    // is exactly the declared-coincidence design, pinned here.
    match topo::vertex_sides(&red.body, &plane, Tol::witness()) {
        Err(SplitReduceError::ScaffoldingOperand { .. }) => {}
        other => panic!("expected ScaffoldingOperand refusal, got {other:?}"),
    }
}

/// R6 — F6 sweep honesty at the current ε row: in-band on BOTH sides of
/// the plane escalates typed; a definitely-off vertex (past the
/// escalation threshold) is never conscripted into ON and produces a
/// clean, null-edge-free reduction when nothing crosses.
#[test]
fn r6_band_honesty_both_sides_and_no_conscription() {
    let eps = geom_core::Tol::witness().get().eps;
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    // In-band below the plane (the shipped teeth only test above).
    let profile = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0 - 3.0 * eps),
        (0.0, 1.0 - 3.0 * eps),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    match split_reduce(&fx.body, &plane_y(1.0, 1.0), Tol::witness()) {
        Err(SplitReduceError::SliverVertex { vertex, diag }) => {
            assert!(diag.predicate.is_some());
            let p = point_of(&fx.body, vertex);
            assert_eq!(p.y, 1.0 - 3.0 * eps);
        }
        other => panic!("expected SliverVertex, got {other:?}"),
    }
    // Definitely off (2× the escalation threshold): clean Below, no ON
    // set, no surgery — never conscripted.
    let off = 2.0 * band.escalate();
    let profile = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0 - off), (0.0, 1.0 - off)];
    let fx = prism::<f64>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y(1.0, 1.0), Tol::witness()).unwrap();
    assert!(red.on_vertices.is_empty());
    assert!(red.null_edges.is_empty());
    assert!(red.sides.iter().all(|(_, &s)| s == PlaneSide::Below));
}

/// R7 — enters_material re-derived independently of the shipped docs:
/// on a solid's face, a direction pointing into the material half-space
/// (against the outward normal) must classify Enters; the rule-(a)
/// composition (dir = +n_SP against a coplanar face) must then send
/// sectors to the side OPPOSITE the face normal's agreement with n_SP.
/// Checked on oblique, non-axis-aligned vectors — a sign flip anywhere
/// in the chain flips these.
#[test]
fn r7_enters_material_oblique_independent() {
    use geom_brep::{EntersMaterial, OutwardNormal, enters_material};
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    // Chart normal along (1,-2,2)/3. On a `sense == true` face that IS
    // the outward normal; on its `sense == false` twin the outward
    // normal is its negation — the two faces are the same chart read
    // through the two sense signs, which is the only way to mint one.
    let chart = Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let n = OutwardNormal::from_chart(chart, true);
    let n_rev = OutwardNormal::from_chart(chart, false);
    // A direction with negative component along n: into material.
    let into = Vec3::new(-1.0, 1.0, 0.5);
    assert!(into.dot(chart) < 0.0, "fixture sanity");
    assert_eq!(
        enters_material(into, n, 2.0, band).unwrap(),
        EntersMaterial::Enters
    );
    assert_eq!(
        enters_material(-into, n, 2.0, band).unwrap(),
        EntersMaterial::Exits
    );
    // The rule-(a) reading: a coplanar face whose outward normal AGREES
    // with n_SP has its material below the plane ⇒ going up (+n_SP)
    // exits ⇒ sector reclassifies BELOW; disagreement ⇒ ABOVE. Both
    // face senses through the same primitive:
    let n_sp = chart; // coplanar: the split plane's normal, no sense
    assert_eq!(
        enters_material(n_sp, n, 1.0, band).unwrap(),
        EntersMaterial::Exits, // agree ⇒ Exits ⇒ rule (a) BELOW
    );
    assert_eq!(
        enters_material(n_sp, n_rev, 1.0, band).unwrap(),
        EntersMaterial::Enters, // oppose ⇒ Enters ⇒ rule (a) ABOVE
    );
}

/// R8 — D9 determinism, byte-level: two reductions of the same operand
/// produce Debug-identical outputs (sides in arena order, ON set,
/// records) — no hash-order or address-dependent iteration anywhere.
#[test]
fn r8_determinism_byte_identical_replay() {
    let profile = [
        (0.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (8.0, 4.0),
        (6.0, 2.0),
        (4.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let dump = |red: &topo::SplitReduction<f64>| {
        let sides: Vec<_> = red.sides.iter().map(|(k, v)| (k, *v)).collect();
        format!("{sides:?}|{:?}|{:?}", red.on_vertices, red.null_edges)
    };
    let r1 = split_reduce(&fx.body, &plane_y(2.0, 1.0), Tol::witness()).unwrap();
    let r2 = split_reduce(&fx.body, &plane_y(2.0, 1.0), Tol::witness()).unwrap();
    assert_eq!(dump(&r1), dump(&r2));
}

/// R9 — interval lane: (a) the ±n equivariance pair from R1b replayed
/// at `T = Interval` with the same censuses; (b) a NON-dyadic crossing
/// (interpolated parameter 1/3 — a genuine multi-ulp enclosure after
/// interval interpolation) still reduces, its constructed vertex
/// declared ON; (c) an in-band vertex escalates typed under interval
/// exactly like f64 (straddling enclosure ⇒ SliverVertex).
#[cfg(feature = "interval")]
#[test]
fn r9_interval_lane_equivariance_and_nondyadic_crossing() {
    use geom_core::Interval;
    // (a) The R1b wedge at Interval, both plane senses.
    let wedge = [
        (0.0, 0.0),
        (2.0, 0.0),
        (3.0, 2.0),
        (4.0, 0.0),
        (10.0, 0.0),
        (10.0, 4.0),
        (0.0, 4.0),
    ];
    let fx = prism::<Interval>(&wedge, 1.0);
    for (ny, dangling_expected) in [(1.0, 2), (-1.0, 0)] {
        let red = split_reduce(&fx.body, &plane_y::<Interval>(2.0, ny)).unwrap();
        // 2 tips + 4 crossings (x=0/x=10 walls at y=2, both rims).
        assert_eq!(red.on_vertices.len(), 6);
        // Tips mint 2 each; crossings 1 each.
        assert_eq!(red.null_edges.len(), 8, "ny = {ny}");
        let dangling = red.null_edges.iter().filter(|r| r.dangling).count();
        assert_eq!(dangling, dangling_expected, "ny = {ny}");
    }
    // (b) Crossing at s = 1/3 on a diagonal edge: (9,0) → (10,3) meets
    // y = 1 at x = 28/3 — not dyadic; the interval interpolation yields
    // a non-singleton enclosure for the constructed point.
    let profile = [(0.0, 0.0), (9.0, 0.0), (10.0, 3.0), (0.0, 3.0)];
    let fx = prism::<Interval>(&profile, 1.0);
    let red = split_reduce(&fx.body, &plane_y::<Interval>(1.0, 1.0)).unwrap();
    assert_eq!(red.on_vertices.len(), 4); // 2 diagonal + 2 wall crossings
    for &v in &red.on_vertices {
        assert_eq!(red.sides[v], PlaneSide::On);
    }
    assert_eq!(red.null_edges.len(), 4);
    // (c) In-band vertex: typed escalation, no snap, same as f64.
    let eps = geom_core::Tol::witness().get().eps;
    let profile = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0 + 3.0 * eps),
        (0.0, 1.0 + 3.0 * eps),
    ];
    let fx = prism::<Interval>(&profile, 1.0);
    match split_reduce(&fx.body, &plane_y::<Interval>(1.0, 1.0)) {
        Err(SplitReduceError::SliverVertex { .. }) => {}
        other => panic!("expected SliverVertex under interval, got {other:?}"),
    }
}
