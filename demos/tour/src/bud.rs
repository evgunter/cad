//! **The calochortus bud, rounded** — the curved-support fillet family
//! (VERBS-ARMS-2's ten coaxial arms) on one body, from an outside
//! consumer's seat.
//!
//! The fairy lantern scene next door renders the PLANT. This scene
//! renders the one part of it the fillet lane was blocked on: the bud
//! as a bored solid of revolution — a sphere zone on a flat base
//! annulus, a conical pucker, a lip disk, and a bore up the axis —
//! whose latitude rims are CURVED support pairs the kernel refused
//! until ARMS-2 shipped the coaxial half of #319.
//!
//! Three of those rims are filleted here, each through a DIFFERENT arm
//! of the same closed-form family:
//!
//! | rim | support pair | arm |
//! |---|---|---|
//! | the mouth | sphere × cone | the acceptance case of #319's coaxial half |
//! | the lip | cone × plane (⊥) | the same sheet reduction, one support flat |
//! | the bore's base | cylinder × plane (⊥) | the third coaxial arm |
//!
//! Every one of them mints a TORUS band through the ring-free annulus
//! surgery, which is what "coaxial" buys: a support pair sharing an
//! axis of revolution confines the rolling ball's centre to the
//! meridian half-plane, where each support cuts a line or a circle and
//! the centre is the crossing of the two offset traces.
//!
//! # Why this panel's PROOF is not its pixels
//!
//! At montage scale the fillets barely move the silhouette — a 50 mm
//! roll on a 1 m sphere and two 30 mm rolls on a 150 mm lip are a few
//! pixels — and that is expected rather than a defect: a constant-radius
//! fillet is a local surgery, and a panel that only showed a changed
//! outline would be showing a fillet too large to be typical. So the
//! scene's evidence is stated in numbers that a picture cannot fake:
//!
//! - **the census delta**, exactly three times the annulus band's own
//!   `(+1 vertex, +2 edges, +1 face)` — two feet minted and the rim
//!   vertex retired, two seam children and two trim circles minted
//!   against the rim and the host seam's rim-side piece, two strips
//!   minted and one merged away;
//! - **the band faces exist and are tori**, each storing the radius the
//!   caller asked for;
//! - **the mouth band's torus is the CLOSED FORM**, re-derived in this
//!   file from the two defining equations rather than read back from
//!   the arm that produced it;
//! - **mass properties fall against the unfilleted twin**, and the
//!   scene builds that twin explicitly so the comparison is between two
//!   bodies rather than between a body and a remembered number.
//!
//! The camera is nevertheless picked to give the mouth rim its best
//! honest chance — high enough to look down into the pucker, where the
//! roll is seen across rather than edge-on.
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **All three rims roll in ONE call — including the two that
//!    SHARE the pucker cone.** This scene originally met the opposite
//!    walking in: the obvious spelling refused `UnsupportedChain`,
//!    because an annulus band consumes its supports' seam meridians
//!    and a later rim's plan named a seam the first carve had split.
//!    #935 serves it now — the carve re-reads each later rim's
//!    seam-piece identities against the partially-carved body, every
//!    decision still made in the plan against the source — so the
//!    convenient spelling IS the door's grain. The scene pins the
//!    success the way it used to pin the refusal: the one-call body's
//!    volume equals the sequential composition's (the mouth first,
//!    then lip + bore base on its result) to one summation ulp —
//!    measured: the faces are the same closed forms under different
//!    arena keys, so only the integrator's summation order differs —
//!    so the widening is a widening and not a divergence.
//!
//!    The radius is per REQUEST, not per edge: a bud wanting a
//!    different roll at its lip would be a further call, exactly as
//!    the die composes its blank's radius with its pip rims'. This
//!    scene wants one radius, so one call says everything.
//! 2. **The rims are selected BY DESCRIPTION and the description is
//!    ambiguous at the bore.** `(Cylinder, Plane)` names the bore's
//!    base rim and its top rim both — the same arm at the other end —
//!    so a consumer who wants only the base adds an axial station. The
//!    kernel-side selector that says this in the ratified vocabulary
//!    (`select_where` + `GeomPred::AdjacentKinds`) is DOCUMENT-LAYER
//!    ONLY, and a body built by calling `revolve` directly has none:
//!    this scene scans `body.edges()` through two back-pointers by
//!    hand, exactly as `klein::corner_edges` does. That is a gap in
//!    reach rather than a refusal, and it is already on
//!    `docs/KERNEL-VERBS.md`'s register; the scene is one more consumer
//!    of it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, validated};
use pncad::geom::{Curve3, Surface};
use pncad::geom_brep::SurfaceKind;
use pncad::geom_core::{Point2, Tol, Vec2};
use pncad::prelude::{Open, Start, fillet_edges};
use pncad::profile::{ArcSweep, Center, ProfileLoop, SketchPlane};
use pncad::sweep::{Revolution, RevolveAxis, revolve};
use pncad::topo::{Body, EdgeKey};

use crate::{SceneBody, Stop, View};

/// The bore's radius: the bud is ANNULAR, which is what makes the full
/// revolve mint one wall per meridian segment and every latitude rim a
/// CLOSED edge.
const BORE: f64 = 0.2;
/// The sphere zone's radius — the unit sphere, centred on the axis at
/// the origin, so the revolve mints a `Surface::Sphere`.
const GLOBE: f64 = 1.0;
/// Where the sphere zone ends and the pucker begins: the 3-4-5 point
/// of the unit circle, so the mouth's coordinates are exact in binary.
const MOUTH: (f64, f64) = (0.8, 0.6);
/// The lip disk's inner corner — where the pucker cone stops.
const LIP_R: f64 = 0.35;
/// The lip's axial station.
const TOP: f64 = 0.75;

/// The scene's chord budget — the die's, one verb over, and about the
/// same fraction of this body's size that the hollow ring spends on
/// its own. Well below the roll radius, so the bands mesh as bands
/// rather than collapsing to a facet, which is what makes proof 5
/// below a statement about the mesh.
const DELTA: f64 = 5e-3;

/// The roll, one radius for all three rims (`verbs_arms2_bud.rs::R`),
/// which the door takes per REQUEST rather than per edge. It fits every
/// wall it touches with room to spare: the lip disk is `LIP_R - BORE` =
/// 0.15 m wide and the sphere zone spans `asin(0.6)` of arc.
const ROLL: f64 = 0.05;

/// The bud's **meridian**, said through the paths lattice: the base
/// annulus, the sphere's own arc about the globe centre, the conical
/// pucker, the lip disk, and the bore closing back on the start.
///
/// Authored centre-first (`Center`), so the belly rides an exact
/// circle centred ON the revolve axis and the wall it sweeps is a
/// sphere rather than a fitted stand-in — which is what the
/// sphere×cone arm needs to exist at all.
///
/// **Not shared with `bodies::bud_rim`, deliberately.** That body is
/// this same bud, probe-lane only (no stop, no cell), and it exists so
/// the `fillet3_*` family records margins in the K corpus. Its own
/// meridian is authored VIA a decimal midpoint rather than about a
/// centre, and its bytes are what that corpus is pinned to — re-spelling
/// it to share this function would move the K baseline for a
/// refactoring's sake. The two stay separate until something else asks
/// for the shape.
fn meridian(tol: Tol) -> ProfileLoop<f64> {
    Open.at(Point2::new(BORE, 0.0))
        .line_to(Point2::new(GLOBE, 0.0), tol)
        .expect("the base annulus")
        .arc_to(
            Center {
                c: Point2::new(0.0, 0.0),
                winding: ArcSweep::Ccw,
                p: Point2::new(MOUTH.0, MOUTH.1),
            },
            tol,
        )
        .expect("the belly rides the globe")
        .line_to(Point2::new(LIP_R, TOP), tol)
        .expect("the conical pucker")
        .line_to(Point2::new(BORE, TOP), tol)
        .expect("the lip disk")
        .line_to(Start, tol)
        .expect("the bore closes the meridian")
        .into()
}

/// The bud, in one `revolve` call.
fn bud(tol: Tol) -> Body<f64> {
    let profile = validated(SketchPlane::xy(), vec![meridian(tol)], tol)
        .expect("the bud's meridian validates");
    revolve(
        &profile,
        RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol,
    )
    .expect("the meridian fully revolves")
    .body
}

/// Every edge of `body` whose two incident faces carry the surface
/// kinds `a` and `b`, in either order — the selection said BY
/// DESCRIPTION, by hand, because a directly revolved body has no
/// selector (finding 2; `klein::corner_edges` is the same scan).
fn rims_between(body: &Body<f64>, a: SurfaceKind, b: SurfaceKind) -> Vec<EdgeKey> {
    let kind_at = |he| {
        let l = body.get_half_edge(he)?.parent_loop;
        let f = body.get_loop(l)?.face;
        body.get_surface(body.get_face(f)?.surface)
            .map(SurfaceKind::of)
    };
    body.edges()
        .filter(|(_, e)| {
            let (ka, kb) = (kind_at(e.he_plus), kind_at(e.he_minus));
            (ka, kb) == (Some(a), Some(b)) || (ka, kb) == (Some(b), Some(a))
        })
        .map(|(k, _)| k)
        .collect()
}

/// The one rim between `a` and `b` when the description names exactly
/// one.
fn rim_between(body: &Body<f64>, a: SurfaceKind, b: SurfaceKind, what: &str) -> EdgeKey {
    let hits = rims_between(body, a, b);
    assert_eq!(hits.len(), 1, "{what}: the description names one rim");
    hits[0]
}

/// The axial station of a closed circular rim.
fn rim_station(body: &Body<f64>, e: EdgeKey) -> f64 {
    let c = body
        .get_curve_geom(body.get_edge(e).expect("the edge").curve)
        .expect("the carrier")
        .certified()
        .expect("a revolved rim carries a certified carrier");
    match *c.carrier() {
        Curve3::Circle { center, .. } => center.y,
        ref other => panic!("a latitude rim is a circle, got {other:?}"),
    }
}

/// The mouth band's ball centre in the meridian, from the two defining
/// equations — **not** from the arm that produced the band. The sphere
/// keeps the ball at `|c| = 1 − r`; the pucker keeps it at
/// `(c − m)·n̂ = −r` for the cone's own outward unit `n̂`; the branch
/// that collapses onto the mouth as `r → 0` is the one the fillet
/// takes.
fn mouth_ball_centre() -> (f64, f64) {
    let (mx, my) = MOUTH;
    // The pucker falls `LIP_R - mx` of radius over `TOP - my` of axis,
    // so its outward normal in the meridian is that leg turned a
    // quarter turn and normalized.
    let (dx, dy) = (LIP_R - mx, TOP - my);
    let len = (dx * dx + dy * dy).sqrt();
    let (nx, ny) = (dy / len, -dx / len);
    // The offset line: the pucker pushed inward by r, parameterized
    // along its own direction.
    let (tx, ty) = (-ny, nx);
    let (ox, oy) = (mx - nx * ROLL, my - ny * ROLL);
    // |o + t·λ| = 1 − r  →  λ² + 2λ(o·t) + |o|² − (1−r)² = 0.
    let b = ox * tx + oy * ty;
    let c = ox * ox + oy * oy - (GLOBE - ROLL) * (GLOBE - ROLL);
    let disc = (b * b - c).sqrt();
    let (l1, l2) = (-b + disc, -b - disc);
    let near = |l: f64| (ox + tx * l - mx).powi(2) + (oy + ty * l - my).powi(2);
    let l = if near(l1) <= near(l2) { l1 } else { l2 };
    (ox + tx * l, oy + ty * l)
}

/// The `(major, minor)` radii of a band face's torus.
fn band_torus(body: &Body<f64>, face: pncad::topo::FaceKey) -> (f64, f64) {
    match body.get_surface(body.get_face(face).expect("the band face").surface) {
        Some(Surface::Torus {
            major_radius,
            minor_radius,
            ..
        }) => (*major_radius, *minor_radius),
        other => panic!("a coaxial band is a torus, got {other:?}"),
    }
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    // The unfilleted twin, kept alive: every claim below is against
    // THIS body rather than against a remembered number.
    let sharp = bud(tol);
    assert_eq!(
        (
            sharp.vertices().count(),
            sharp.edges().count(),
            sharp.faces().count(),
        ),
        (5, 10, 5),
        "the bud is five walls, five latitude rims and five meridian seams"
    );

    // The three rims, said BY DESCRIPTION. Two of them the description
    // names uniquely; the bore's does not (finding 2), so the base is
    // separated from the top by its axial station.
    let mouth = rim_between(&sharp, SurfaceKind::Sphere, SurfaceKind::Cone, "the mouth");
    let lip = rim_between(&sharp, SurfaceKind::Cone, SurfaceKind::Plane, "the lip");
    let bore_rims = rims_between(&sharp, SurfaceKind::Cylinder, SurfaceKind::Plane);
    assert_eq!(
        bore_rims.len(),
        2,
        "the bore meets a plane at BOTH ends — the same arm twice, which the \
         description alone cannot separate"
    );
    let bore_base = *bore_rims
        .iter()
        .min_by(|a, b| {
            rim_station(&sharp, **a)
                .partial_cmp(&rim_station(&sharp, **b))
                .expect("finite stations")
        })
        .expect("two bore rims");
    assert!(
        rim_station(&sharp, bore_base).abs() < 1e-12,
        "the bore's BASE rim sits on the base annulus at y = 0"
    );

    // ---- finding 1, executed: the convenient spelling BUILDS ----
    //
    // All three rims at once is what a consumer writes first, and the
    // mouth and the lip share the pucker cone. #935's seam refresh
    // serves exactly this, so the natural spelling is the one the
    // scene ships.
    let rolled = fillet_edges(&sharp, &[mouth, lip, bore_base], ROLL, tol).unwrap_or_else(|e| {
        panic!(
            "all three rims roll in ONE call — the shared pucker cone is served by \
                 the #935 seam refresh; got {e:?}"
        )
    });
    println!("   [budfillet] all three rims in ONE call — three bands, one request");

    // ---- finding 1, cross-checked: the one call IS the sequential
    // composition, to the bit ----
    //
    // The recourse the old refusal named stays true and stays equal:
    // the mouth first, then the two rims that share nothing on its
    // result. A widened door that DIVERGED from it would be a wrong
    // door wearing a convenience.
    let first = fillet_edges(&sharp, &[mouth], ROLL, tol)
        .unwrap_or_else(|e| panic!("the bud's sphere-cone mouth rim rolls, got {e:?}"));
    let lip2 = rim_between(
        &first.body,
        SurfaceKind::Cone,
        SurfaceKind::Plane,
        "the lip, re-selected",
    );
    let bore2 = rims_between(&first.body, SurfaceKind::Cylinder, SurfaceKind::Plane);
    let base2 = *bore2
        .iter()
        .min_by(|a, b| {
            rim_station(&first.body, **a)
                .partial_cmp(&rim_station(&first.body, **b))
                .expect("finite stations")
        })
        .expect("two bore rims");
    let sequential = fillet_edges(&first.body, &[lip2, base2], ROLL, tol).unwrap_or_else(|e| {
        panic!(
            "the lip and the bore's base share no support face, so they roll \
                 TOGETHER on the mouth's result; got {e:?}"
        )
    });
    let one_call_volume = pncad::topo::mass_properties(&rolled.body, tol)
        .expect("the one-call bud's props")
        .volume;
    let sequential_volume = pncad::topo::mass_properties(&sequential.body, tol)
        .expect("the sequential bud's props")
        .volume;
    // MEASURED at one ulp, and the ulp is the integrator's, not the
    // carve's: the two paths mint the same closed-form faces under
    // different arena keys, so `mass_properties` sums the same
    // per-face contributions in a different order. (The kernel's own
    // rows measure the zone and lantern pairs bit-equal —
    // `sweep/tests/blend_tworims.rs`; this body is where the
    // summation-order ulp shows up.)
    assert!(
        one_call_volume > 0.0
            && (one_call_volume - sequential_volume).abs()
                <= 2.0 * f64::EPSILON * one_call_volume.abs(),
        "one call and the sequential composition agree to a summation ulp: \
         {one_call_volume:.17e} vs {sequential_volume:.17e}"
    );

    // ---- proof 1: the census delta, three times the band's own ----
    let (v, e, f) = (
        rolled.body.vertices().count(),
        rolled.body.edges().count(),
        rolled.body.faces().count(),
    );
    assert_eq!(
        (v, e, f),
        (8, 16, 8),
        "three annulus bands, each (+1 vertex, +2 edges, +1 face) over the sharp bud"
    );

    // ---- proof 2: the band faces EXIST, and each is a torus of the
    // requested tube radius. A silhouette that did not move cannot say
    // this; three new revolution walls can only be there or not.
    assert_eq!(rolled.band_faces.len(), 3, "one band per rim, one call");
    assert_eq!(first.band_faces.len(), 1, "the cross-check's mouth band");
    assert_eq!(
        sequential.band_faces.len(),
        2,
        "the cross-check's lip and bore-base bands"
    );
    let bands: Vec<pncad::topo::FaceKey> = rolled
        .body
        .faces()
        .filter(|(_, face)| {
            matches!(
                rolled.body.get_surface(face.surface),
                Some(Surface::Torus { .. })
            )
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(bands.len(), 3, "one torus band per rim");
    let mut majors: Vec<f64> = Vec::new();
    for face in &bands {
        let (major, minor) = band_torus(&rolled.body, *face);
        assert!(
            (minor - ROLL).abs() < 1e-12,
            "every band's tube is the requested {ROLL}, got {minor}"
        );
        assert!(
            rolled
                .body
                .get_face(*face)
                .expect("the band face")
                .rings
                .is_empty(),
            "a curved band is ring-free: one cycle, two closed trim circles and a slit"
        );
        majors.push(major);
    }
    majors.sort_by(|a, b| a.partial_cmp(b).expect("finite"));

    // ---- proof 3: the mouth band is the CLOSED FORM, re-derived here
    // from the two defining equations rather than read back from the
    // arm that produced it.
    let (cx, cy) = mouth_ball_centre();
    let mouth_face = *bands
        .iter()
        .max_by(|a, b| {
            band_torus(&rolled.body, **a)
                .0
                .partial_cmp(&band_torus(&rolled.body, **b).0)
                .expect("finite")
        })
        .expect("three bands");
    let (mouth_major, _) = band_torus(&rolled.body, mouth_face);
    assert!(
        (mouth_major - cx).abs() < 1e-9,
        "the mouth band's spine radius is the ball centre's own radial coordinate \
         {cx}, got {mouth_major}"
    );
    let Some(&Surface::Torus { center, .. }) = rolled.body.get_surface(
        rolled
            .body
            .get_face(mouth_face)
            .expect("the mouth band")
            .surface,
    ) else {
        panic!("the mouth band is a torus")
    };
    assert!(
        center.x.abs() < 1e-12 && center.z.abs() < 1e-12 && (center.y - cy).abs() < 1e-9,
        "the mouth band's spine circle is centred at (0, {cy}, 0), got {center:?}"
    );

    // ---- proof 4: mass properties move, against the twin ----
    let sharp_props = pncad::topo::mass_properties(&sharp, tol).expect("the sharp bud's props");
    let rolled_props =
        pncad::topo::mass_properties(&rolled.body, tol).expect("the rolled bud's props");
    let dv = sharp_props.volume - rolled_props.volume;
    let da = sharp_props.surface_area - rolled_props.surface_area;
    assert!(
        dv > 0.0,
        "a convex rim's roll REMOVES material: ΔV = {dv} must be positive"
    );
    // The scale the removal has to be on: each band replaces a sharp
    // corner by a quarter-ish tube, so the missing meridian region is
    // bounded by the corner square r² and the whole roll by Pappus over
    // the three rims' own radii. A shift far outside this bracket is
    // not a fillet, whatever the census says.
    let rim_radii = [MOUTH.0, LIP_R, BORE];
    let pappus_cap: f64 = rim_radii.iter().map(|r| 2.0 * PI * r * ROLL * ROLL).sum();
    assert!(
        dv < pappus_cap,
        "ΔV = {dv} exceeds the corner-square bound {pappus_cap} — that is not a roll"
    );

    // ---- proof 5: the mesh actually carries the bands ----
    let mesh = pncad::mesh::tessellate(&rolled.body, DELTA, tol).expect("the bud tessellates");
    let band_tris: usize = bands
        .iter()
        .map(|k| {
            mesh.patches
                .iter()
                .find(|p| p.face == *k)
                .expect("every band face is meshed")
                .triangles
                .len()
        })
        .sum();
    assert!(
        band_tris > 0,
        "the bands are meshed, not degenerate at the scene's own δ"
    );

    vec![Stop {
        name: "budfillet",
        caption: "THE FILLETED BUD (three curved-support arms, one body)".to_string(),
        // Montage cell RETIRED by the montage-v3 curation (Evan,
        // 2026-08-30) on this module's OWN stated grounds: "at montage
        // scale the fillets barely move the silhouette ... so the
        // scene's evidence is stated in numbers that a picture cannot
        // fake". The sheet's rule is that a scene is `montage: false`
        // when it is a PROOF rather than a part, and this one says so
        // about itself. WHAT LEAVES THE SHEET WITH IT, stated rather
        // than glossed: of the three coaxial arms, only cylinder x
        // plane still has a cell — the teapot's lid rolls its knob's
        // top rim through that one. Sphere x cone (the acceptance case
        // of #319's coaxial half) and cone x plane have none. The lid
        // is the natural host for both, being a bored revolve whose
        // latitude rims are already closed, and a conical flange would
        // give it them — outstanding work on the teapot, not something
        // this retirement can claim. Standalone render and every number
        // here stay.
        montage: false,
        story: "the calochortus bud as a bored solid of revolution — sphere zone, \
                conical pucker, lip disk, bore — with its mouth (sphere x cone), its \
                lip (cone x plane) and its bore's base (cylinder x plane) rolled. \
                Three different arms of the coaxial curved-support family, on one body",
        ops: "revolve(meridian, +y axis, Revolution::Full), then fillet_edges ONCE, \
              all three rims in one request: each coaxial pair confines the rolling \
              ball's centre to the meridian half-plane, where the two offset traces \
              cross, so every band is an exact TORUS through the ring-free annulus \
              surgery. The mouth and the lip share the pucker cone, which one call \
              serves by re-reading the later rim's seam-piece identities between \
              carves (#935)",
        delta: DELTA,
        note: Some(format!(
            "{v} vertices, {e} edges, {f} faces — the sharp bud's 5/10/5 plus exactly \
             three times the annulus band's own (+1, +2, +1). All three rims roll in \
             ONE call, the mouth and the lip sharing the pucker cone included: the \
             carve re-reads the later rim's seam-piece identities against the \
             partially-carved body (#935), and the scene cross-checks the one-call \
             body against the sequential composition — volumes equal to one \
             summation ulp ({one_call_volume:.9} m³). AT MONTAGE SCALE THE \
             ROLLS BARELY MOVE THE SILHOUETTE, and that is expected: a constant-radius \
             fillet is a local surgery, so this panel's proof is numeric rather than \
             pictorial. Three band faces exist, each a ring-free torus wall storing the \
             requested tube radius {ROLL} and spine radii {majors:?}; the mouth band's \
             spine is at ({cx:.9}, {cy:.9}) in the meridian, re-derived IN THIS FILE \
             from |c| = 1 - r and (c - m)·n = -r rather than read back from the arm \
             that minted it. Against the unfilleted twin the volume falls by \
             {dv:.9} m³ ({:.4}%) and the area by {da:.9} m² ({:.4}%) — material \
             removed, as a convex rim's roll must. The three bands carry {band_tris} \
             triangles of this scene's mesh at δ = {DELTA}. Every rim was selected BY \
             DESCRIPTION (the pair of adjacent surface kinds); the bore's cylinder-plane \
             description names BOTH its ends, so the base is picked out by its axial \
             station — and the ratified selector that would say this in the document \
             vocabulary is document-layer only, so the scan here is by hand",
            100.0 * dv / sharp_props.volume,
            100.0 * da / sharp_props.surface_area
        )),
        // High enough to look down into the pucker: the mouth's roll is
        // then seen across rather than edge-on, which is the most an
        // honest camera can do for a 50 mm band on a 1 m sphere.
        view: View {
            elev: 34.0,
            azim: -42.0,
            up: 'y',
        },
        bodies: vec![SceneBody::plain(
            "budfillet",
            [0.86, 0.78, 0.52],
            rolled.body,
        )],
    }]
}
