//! The tour's sweep bodies, each built through the public
//! profile/sweep API. Retirements: the donut (#91 refresh) and the
//! pulley (#91 revision pass) both fold into the rope-groove sheave —
//! it carries plane + cylinder + cone + torus on one real part; the
//! plain wedge became the quarter-turn chute (same partial-revolve
//! capability, a real profile).
//!
//! Constructors are generic over [`Scalar`] (M4 PR 8b): the f64 tour
//! and the Probe K-telemetry sweep build the SAME geometry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::Vec2;
use pncad::prelude::{Open, Start, Via};
use pncad::profile::{ProfileLoop, SketchPlane};
use pncad::sweep::chamfer::chamfer_edges;
use pncad::sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};

use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::authoring::{p2, validated};
use pncad::geom_core::Tol;

fn axis_y<S: Scalar>() -> RevolveAxis<S> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(S::from_f64(0.0), S::from_f64(1.0)),
    }
}

/// A circle loop, algebra-authored (LIB-G1): `circle` is a one-step
/// complete-loop PROGRAM FORM, not a chain — it authors no seam, so the
/// conventional two-semicircle split is its private lowering and PQ4
/// (no mid-carrier seams for chains) is untouched. Lowers to exactly
/// the two-vertex bulge-1 loop this helper used to build by hand.
fn circle<S: Scalar>(cx: f64, cy: f64, r: f64, tol: Tol) -> ProfileLoop<S> {
    pncad::profile::circle(p2(cx, cy), S::from_f64(r), tol)
        .expect("circle radius is positive")
        .into()
}

/// L-bracket: polyline + one fillet arc at the inner corner, extruded.
pub fn bracket<S: Scalar>(tol: Tol) -> pncad::topo::Body<S> {
    // The inner corner (1, 1) carries an r = 0.5 tangent fillet,
    // authored through the CONSTRUCTIVE path (#101): `fillet` computes
    // the tangent points (1.5, 1)/(1, 1.5) and the arc bulge exactly
    // and declares both joints tangent by construction. History: the
    // pre-#100 demo hand-supplied a decimal-rounded via point (1.146),
    // whose carrier sat ~2.3e-6 clear of the adjacent lines — inside
    // the escalation band at CAD_TOLERANCE_EPS=1e-6 (#99); #100 fixed
    // the constant in place (1.5 − 0.5/√2, margin ~1e-16); #101
    // replaces the hand computation entirely and makes the intent
    // declared, verified data. Undeclared exact tangency is now
    // refused typed (UndeclaredTangency), so the constructor is the
    // demo's authoring path, not just a convenience. (The 1.146 datum
    // itself lives on as the large-K lint's litmus fixture —
    // tools/k-lint.)
    //
    // Algebra-authored at LIB-G1, which is where the two constructors
    // it needed arrived. LIB-U2 PR-2 measured why it could not move
    // then: the corner is never authored, so the PATHS spelling reaches
    // it through a director, and `.angle(PI)` carries sin(PI) = 1.22e-16
    // into the ray — 1 ulp on both trim vertices. `.toward(-1, 0)` fixes
    // the RAY instead of an angle, so the corner comes out exactly
    // (1, 1). The filleted side then has to END at its authored far
    // vertex (1, 3), which needed the far-end anchor `.to(p)` — before
    // it, the only spellings were a synthetic mid-side anchor plus a
    // measured length. The lowering is bit-identical to the raw chain
    // this replaces (pinned in path_differential).
    let lp = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(3.0, 0.0), tol)
        .expect("bracket base")
        .line_to(p2(3.0, 1.0), tol)
        .expect("bracket riser")
        .toward(S::from_f64(-1.0), S::from_f64(0.0), tol)
        .expect("west, exactly")
        .fillet(S::from_f64(0.5), tol) // r = 0.5 inner fillet
        .expect("bracket fillet fits")
        .toward(S::from_f64(0.0), S::from_f64(1.0), tol)
        .expect("north, exactly")
        .to(p2(1.0, 3.0), tol)
        .expect("the filleted side ends at its far vertex")
        .line_to(p2(0.0, 3.0), tol)
        .expect("bracket top")
        .line_to(Start, tol)
        .expect("bracket seam")
        .into();
    extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("profile validation"),
        Extrusion::Distance(S::from_f64(0.75)),
        tol,
    )
    .expect("extrude bracket")
    .body
}

/// Rectangular plate with two circular holes: a genus-2 extrusion.
pub fn plate<S: Scalar>(tol: Tol) -> pncad::topo::Body<S> {
    // Outer rectangle algebra-authored at LIB-U2 PR-2, the hole circles
    // at LIB-G1 (see `circle`) — per-loop wholesale, never mixed within
    // a loop.
    let outer =
        crate::paths::path_polygon(&[(-3.0, -1.5), (3.0, -1.5), (3.0, 1.5), (-3.0, 1.5)], tol);
    let holes = vec![circle(-1.5, 0.0, 0.7, tol), circle(1.5, 0.0, 0.7, tol)];
    let mut loops = vec![outer];
    loops.extend(holes);
    extrude(
        &validated(SketchPlane::xy(), loops, tol).expect("profile validation"),
        Extrusion::Distance(S::from_f64(0.6)),
        tol,
    )
    .expect("extrude plate")
    .body
}

/// Solid vase: an axis-touching profile — conical base, spherical
/// belly (the arc's carrier center sits ON the revolve axis, so the
/// swept surface is a sphere zone; an OFF-axis arc carrier sweeps a
/// ring torus — see the sheave, which showcases exactly that), conical
/// flared lip — fully revolved about the y axis.
pub fn vase<S: Scalar>(tol: Tol) -> pncad::topo::Body<S> {
    // Belly arc: circle of radius 1.3 centered at (0, 0.8) — on the
    // axis — from (1.2, 0.3) through (1.3, 0.8) to (0.5, 2.0).
    // Algebra-authored (LIB-G1): `Via` is the through-point binding
    // mode the belly wanted all along — all three points are authored
    // and stored verbatim, and the bulge is derived at lowering by the
    // same closed form the raw chain used, so nothing computed is
    // re-typed and the lowering is bit-identical.
    let lp = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(1.2, 0.0), tol)
        .expect("vase base")
        .line_to(p2(1.2, 0.3), tol)
        .expect("vase base wall")
        .arc_to(
            Via {
                q: p2(1.3, 0.8),
                p: p2(0.5, 2.0),
            },
            tol,
        )
        .expect("vase belly arc")
        .line_to(p2(0.9, 2.5), tol)
        .expect("vase lip flare")
        .line_to(p2(0.0, 2.5), tol)
        .expect("vase lip")
        .line_to(Start, tol)
        .expect("vase axis seam")
        .into();
    revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("profile validation"),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .expect("revolve vase")
    .body
}

/// Rope-groove sheave (the donut's AND the pulley's successor): full
/// revolve of a polyline + arc profile — stepped hub, recessed web,
/// rim with TAPERED (conical) shoulders flanking a SEMICIRCULAR rope
/// groove. The groove arc's carrier center sits OFF the revolve axis,
/// so its wall is a ring-torus zone (`WallKind::Torus` — only
/// horn/spindle tori refuse typed); the shoulders are cone zones. One
/// stop carries all four analytic surface kinds (plane, cylinder,
/// cone, torus), which is why the pulley (plane/cylinder/cone only)
/// retired into it at the #91 revision pass. Center bore → genus 1.
pub fn sheave<S: Scalar>(tol: Tol) -> (pncad::topo::Body<S>, String) {
    // Algebra-authored (LIB-G1): the groove is an arc bound `Via` its
    // own deepest point, between two chord-derived line sides.
    let lp = Open
        .at(p2(0.4, 0.0))
        .line_to(p2(0.9, 0.0), tol)
        .expect("sheave hub face")
        .line_to(p2(0.9, 0.25), tol)
        .expect("sheave hub step")
        .line_to(p2(1.6, 0.25), tol)
        .expect("sheave web")
        .line_to(p2(1.6, 0.0), tol)
        .expect("sheave web step")
        .line_to(p2(2.0, 0.0), tol)
        .expect("sheave rim face")
        .line_to(p2(2.1, 0.2), tol) // tapered shoulder: cone zone
        .expect("sheave lower shoulder")
        // groove: r = 0.3 semicircle
        .arc_to(
            Via {
                q: p2(1.8, 0.5),
                p: p2(2.1, 0.8),
            },
            tol,
        )
        .expect("sheave rope groove")
        .line_to(p2(2.0, 1.0), tol) // tapered shoulder: cone zone
        .expect("sheave upper shoulder")
        .line_to(p2(1.6, 1.0), tol)
        .expect("sheave rim back")
        .line_to(p2(1.6, 0.75), tol)
        .expect("sheave web step (back)")
        .line_to(p2(0.9, 0.75), tol)
        .expect("sheave web (back)")
        .line_to(p2(0.9, 1.0), tol)
        .expect("sheave hub step (back)")
        .line_to(p2(0.4, 1.0), tol)
        .expect("sheave hub face (back)")
        .line_to(Start, tol)
        .expect("sheave bore seam")
        .into();
    let body: pncad::topo::Body<S> = revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("profile validation"),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .expect("revolve sheave")
    .body;
    // Closed-form volume by Pappus (independent derivation, exact
    // rationals): hub + web + rim annuli + two cone shoulder wedges,
    // minus the revolved half-disc groove:
    //   V = 2*pi*(1997/1200) - (189/1000)*pi^2.
    let pi = core::f64::consts::PI;
    let oracle = 2.0 * (1997.0 / 1200.0) * pi - 0.189 * pi * pi;
    let v = pncad::topo::mass_properties(&body, tol)
        .expect("sheave mass properties")
        .volume
        .f();
    let rel = ((v - oracle) / oracle).abs();
    assert!(
        rel < 1e-12,
        "sheave volume {v} vs closed-form {oracle} (rel {rel:.3e})"
    );
    let kind_count = |pred: fn(&pncad::topo::Surface<S>) -> bool| {
        body.faces()
            .filter(|(_, f)| body.get_surface(f.surface).is_some_and(pred))
            .count()
    };
    assert_eq!(
        kind_count(|s| matches!(s, pncad::topo::Surface::Torus { .. })),
        1,
        "the groove must be a torus wall"
    );
    assert_eq!(
        kind_count(|s| matches!(s, pncad::topo::Surface::Cone { .. })),
        2,
        "the rim shoulders must be cone walls"
    );
    let note = format!(
        "surface census: 6 planes, 5 cylinders, 2 CONES (tapered rim shoulders), \
         1 TORUS (groove) — all four analytic wall kinds on one part (the pulley, \
         whose kinds were a subset, retired here); volume matches the closed-form \
         Pappus value 2pi*1997/1200 - 0.189pi^2 to {rel:.1e} relative"
    );
    (body, note)
}

/// Quarter-turn chute (the wedge's successor at the #91 revision
/// pass): a C-channel cross-section swept through a 270-degree
/// partial revolve — a curved trough with wedge caps at both ends,
/// annular rims, and four cylinder bands. A more interesting partial
/// revolve than the old plain rectangle, still boolean-free.
pub fn chute<S: Scalar>(tol: Tol) -> (pncad::topo::Body<S>, String) {
    // C-channel polygon: algebra-authored (LIB-U2 PR-2).
    let lp = crate::paths::path_polygon(
        &[
            (1.0, 0.0),
            (1.75, 0.0),
            (1.75, 0.625),
            (1.5625, 0.625),
            (1.5625, 0.1875),
            (1.1875, 0.1875),
            (1.1875, 0.625),
            (1.0, 0.625),
        ],
        tol,
    );
    let body: pncad::topo::Body<S> = revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("profile validation"),
        axis_y(),
        Revolution::Partial(S::from_f64(3.0 * core::f64::consts::FRAC_PI_2)),
        tol,
    )
    .expect("revolve chute")
    .body;
    // Pappus for the partial sweep (independent derivation, exact):
    // profile first moment 429/1024, angle 3pi/2 => V = (1287/2048)pi.
    let oracle = (1287.0 / 2048.0) * core::f64::consts::PI;
    let v = pncad::topo::mass_properties(&body, tol)
        .expect("chute mass properties")
        .volume
        .f();
    let rel = ((v - oracle) / oracle).abs();
    assert!(
        rel < 1e-12,
        "chute volume {v} vs closed-form {oracle} (rel {rel:.3e})"
    );
    let note = format!(
        "C-channel profile x 270 degrees about y; volume matches the closed-form \
         Pappus value (1287/2048)pi to {rel:.1e} relative"
    );
    (body, note)
}

// Narration fields arrive as one flat argument list on purpose — the
// call sites below read as a table of stops; a params struct would just
// re-spell `Stop` itself.
#[allow(clippy::too_many_arguments)]
fn stop(
    name: &'static str,
    story: &'static str,
    ops: &'static str,
    delta: f64,
    view: View,
    color: [f64; 3],
    body: pncad::topo::Body<f64>,
    note: Option<String>,
) -> Stop {
    Stop {
        name,
        caption: String::new(),
        montage: true,
        story,
        ops,
        delta,
        note,
        view,
        bodies: vec![SceneBody::plain(name, color, body)],
    }
}

/// **A machined spacer with every edge broken** — the chamfer verb
/// from an outside consumer's seat: extrude the pad, hand
/// [`chamfer_edges`] the body's twelve edges at one setback, render
/// what comes back.
///
/// The part is the reason chamfers exist in a shop: a rectangular
/// spacer whose edges are all "broken" so nothing on it can cut a
/// hand or a gasket. Twelve flat strips and eight flat corner
/// triangles, every face a plane — the exact analytic case, no fitted
/// band anywhere on the part.
///
/// **The friction this scene records** (demo-purpose rule): there is
/// no whole-body edge selector on the PLAIN body API, so "break every
/// edge" is spelled by enumerating the arena's own edge keys. The
/// document layer has the door (`Node::fillet`'s `all_edges`
/// materializer, which `diefillet` uses); the kernel-level verb does
/// not, because there is no `Node::chamfer` yet to reach it through.
/// A consumer wanting a chamfer in a RECIPE — with names, with a
/// rebuild — cannot have one today.
pub fn spacer<S: Scalar>(tol: Tol) -> (pncad::topo::Body<S>, String) {
    let (x, y, z) = (4.0, 2.4, 1.0);
    let setback = 0.15;
    let lp: ProfileLoop<S> = Open
        .at(p2(0.0, 0.0))
        .line_to(p2(x, 0.0), tol)
        .expect("spacer south")
        .line_to(p2(x, y), tol)
        .expect("spacer east")
        .line_to(p2(0.0, y), tol)
        .expect("spacer north")
        .line_to(Start, tol)
        .expect("spacer seam")
        .into();
    let pad = extrude(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("profile validation"),
        Extrusion::Distance(S::from_f64(z)),
        tol,
    )
    .expect("extrude spacer")
    .body;
    // "Every edge of it" — spelled the only way the plain-body door
    // allows (see the note above).
    let edges: Vec<pncad::topo::EdgeKey> = pad.edges().map(|(k, _)| k).collect();
    let t = tol.get();
    let band = pncad::geom_core::Band::new(t.eps, t.k * t.eps).expect("a band from the tolerance");
    let broken = chamfer_edges(&pad, &edges, S::from_f64(setback), band, tol)
        .expect("every edge of a rectangular pad breaks at 0.15");
    let note = format!(
        "chamfer_edges over the plain body API: {} strips + {} corner patches, every face a \
         plane. Friction recorded: (1) the plain-body door has no whole-body edge selector, \
         so `all twelve` is spelled by enumerating arena keys; (2) there is no \
         `Node::chamfer`, so the verb is unreachable from a recipe; (3) the call wants BOTH \
         a `Tol` and a `Band`, and the `Band` this scene passes is derived from that same \
         `Tol` — every caller in the tour writes the same three-line derivation, so the \
         second argument carries no information the first did not.",
        broken.blend_faces.len(),
        broken.corner_faces.len()
    );
    (broken.body, note)
}

/// A stop kept OUT of the montage sheet (standalone render, full
/// narration, corpus/latency roles untouched) — the curation lever, so
/// a retirement is one wrapped call site and not a reshaped `stop`
/// table.
fn off_sheet(mut s: Stop) -> Stop {
    s.montage = false;
    s
}

/// The sweep stops, in tour order.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let (sheave_body, sheave_note) = sheave::<f64>(tol);
    let (chute_body, chute_note) = chute::<f64>(tol);
    let (spacer_body, spacer_note) = spacer::<f64>(tol);
    vec![
        // Montage cell RETIRED by the M6 curation unit: `rocker` now
        // covers PROFILE fillets on the sheet, and far more
        // comprehensively (six corners, the whole line/arc taxonomy)
        // than the bracket's single inner blend, while `diefillet`
        // covers the rolling-ball kind. The bracket keeps its
        // standalone render and every non-sheet role.
        off_sheet(stop(
            "bracket",
            "L-bracket with a filleted inner corner (polyline + tangent arc profile)",
            "PATHS algebra (toward/fillet/far-end anchor) -> Profile::validate -> extrude(Distance)",
            1e-2,
            View {
                elev: 32.0,
                azim: -55.0,
                up: 'z',
            },
            [0.36, 0.56, 0.86],
            bracket(tol),
            None,
        )),
        stop(
            "spacer",
            "machined spacer with every edge broken (12 flat strips + 8 flat corner patches)",
            "extrude(Distance) -> chamfer_edges(all twelve edges, equal setback)",
            1e-2,
            View {
                elev: 30.0,
                azim: -50.0,
                up: 'z',
            },
            [0.62, 0.66, 0.72],
            spacer_body,
            Some(spacer_note),
        ),
        stop(
            "plate",
            "plate with two circular holes — genus 2 (each hole: 2 rings, wall band)",
            "polygon outer + two closed arc-carrier holes -> extrude(Distance)",
            1e-2,
            View {
                elev: 42.0,
                azim: -60.0,
                up: 'z',
            },
            [0.86, 0.51, 0.27],
            plate(tol),
            None,
        ),
        stop(
            "vase",
            "solid vase — axis-touching profile, spherical belly zone + conical lip",
            "PATHS algebra (line_to/arc_to Via) -> revolve(axis y, Full); sphere/cone/plane faces",
            2e-2,
            View {
                elev: 16.0,
                azim: -55.0,
                up: 'y',
            },
            [0.42, 0.72, 0.50],
            vase(tol),
            None,
        ),
        stop(
            "sheave",
            "rope-groove sheave — hub, web, TAPERED rim shoulders, semicircular groove: \
             plane + cylinder + cone + torus on one part",
            "PATHS algebra polyline + arc_to Via -> revolve(axis y, Full), genus 1",
            5e-2,
            View {
                elev: 26.0,
                azim: -55.0,
                up: 'y',
            },
            [0.78, 0.42, 0.72],
            sheave_body,
            Some(sheave_note),
        ),
        stop(
            "chute",
            "quarter-turn chute — C-channel profile swept 270 degrees; wedge caps, \
             curved trough",
            "8-gon C-channel -> revolve(axis y, Partial(3pi/2))",
            2e-2,
            View {
                elev: 35.0,
                azim: -40.0,
                up: 'y',
            },
            [0.44, 0.68, 0.78],
            chute_body,
            Some(chute_note),
        ),
    ]
}

// RE-HOMED (LIB-RETTAIL, Evan's ruling on #413): `finale_fail_loud` —
// the bowtie coda — left the tour. A broken-on-purpose scene is not a
// use case, and it was the last thing keeping a raw public authoring
// tier alive. Its fail-loud contract did not evaporate: it is
// `crates/profile/tests/rejections.rs::the_bowtie_authors_cleanly_and_
// refuses_at_validation`, which keeps the exact oracle (the chain
// AUTHORS — the lattice's junction checks are local and all four
// corners are sharp — and `Profile::validate` refuses it typed, never
// Ok) and pins the error variant the demo only printed.

/// **A bud's mouth rim, filleted** — the curved-support fillet verb
/// from an outside consumer's seat, and #319's own consumer shape: a
/// sphere zone meeting a conical pucker on a bored base, revolved
/// FULL, with a rolling ball run along the sphere×cone latitude circle
/// where they meet.
///
/// The pair is COAXIAL, so its blend is a torus and the arm is exact —
/// no fitted band anywhere on the part. It is a probe-only body: it
/// carries no stop and no montage cell, and it exists so the
/// curved-support half of the `fillet3_*` family (in particular
/// `fillet3_support_coaxiality`, which no plane-supported scene can
/// reach) records margins in the K corpus, exactly as `spacer` does for
/// the chamfer.
///
/// # Panics
///
/// If the profile does not validate, the revolve fails, the mouth rim
/// is not the one closed latitude circle of radius `0.8`, or the fillet
/// refuses — each of which would be a frontier that moved.
pub fn bud_rim<S: Scalar>(tol: Tol) -> pncad::topo::Body<S> {
    // The sphere zone rides the UNIT circle about the origin from its
    // equator to the 3-4-5 point (0.8, 0.6), where the pucker takes
    // over; the via point is that arc's own midpoint.
    let lp: ProfileLoop<S> = Open
        .at(p2(0.2, 0.0))
        .line_to(p2(1.0, 0.0), tol)
        .expect("bud base annulus")
        .arc_to(
            Via {
                q: p2(0.948_683_298_050_513_8, 0.316_227_766_016_837_9),
                p: p2(0.8, 0.6),
            },
            tol,
        )
        .expect("bud belly rides the unit sphere")
        .line_to(p2(0.35, 0.75), tol)
        .expect("bud conical pucker")
        .line_to(p2(0.2, 0.75), tol)
        .expect("bud lip disk")
        .line_to(Start, tol)
        .expect("bud bore")
        .into();
    let body = revolve(
        &validated(SketchPlane::xy(), vec![lp], tol).expect("bud profile validation"),
        axis_y(),
        Revolution::Full,
        tol,
    )
    .expect("revolve bud")
    .body;
    // The mouth: the one CLOSED latitude rim of radius 0.8. Selected by
    // the analytically known radius, the way every rim fixture is.
    let mouth: Vec<pncad::topo::EdgeKey> = body
        .edges()
        .filter(|(_, e)| {
            let closed =
                body.get_half_edge(e.he_plus).map(|h| h.start) == body.half_edge_end(e.he_plus);
            // The radius is read through `Bounds`, not compared as a
            // scalar: `Scalar` is the recording lane too, where a bare
            // `<` is not available and would not mean what it says.
            let r = body
                .get_curve_geom(e.curve)
                .and_then(|g| g.certified())
                .and_then(|c| match *c.carrier() {
                    pncad::geom::Curve3::Circle { radius, .. } => Some(radius),
                    _ => None,
                });
            closed && r.is_some_and(|r| (r - S::from_f64(0.8)).abs().hi() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(mouth.len(), 1, "the bud has one mouth rim of radius 0.8");
    pncad::sweep::fillet::fillet_edges(
        &body,
        &mouth,
        S::from_f64(0.05),
        pncad::geom_core::Band::linear(tol).expect("the run's band"),
        tol,
    )
    .expect("the sphere-cone mouth rim fillets")
    .body
}
