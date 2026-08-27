//! **The tube door with a WALL** — `pncad::sweep::tube_along_arc_hollow`,
//! in its two window policies, side by side.
//!
//! `tube` next door renders the SOLID torus door: intent parameters in,
//! stored bit-exactly, over a window `[t0, t1]`. This module renders the
//! same door's hollow sibling on the same spine, the same outer radius
//! and the same window, plus one number the solid door has no seat for —
//! a `wall` thickness. Two scenes, because the door's two window
//! policies produce two genuinely different SOLIDS and the difference is
//! what the pair is for:
//!
//! - **the windowed hollow elbow** — an open elbow of annular
//!   cross-section. Its ends are open, so nothing is enclosed: ONE
//!   shell, no cavity, and the bore is visible in an OPAQUE render at a
//!   camera that looks into a cap. This is the panel that shows a wall
//!   without any translucency trick.
//! - **the full-period hollow torus** — the same tube closed on itself.
//!   The inner wall stops being a visible surface and becomes a
//!   CAVITY: two shells in one solid, the inner one inserted through
//!   the shared void-insertion door by the revolve's own holed path.
//!   A cavity cannot be read from an opaque render at any camera, so
//!   this one is drawn see-through at 45, exactly as the hollow ring
//!   and the bottle's loop tubes are.
//!
//! # What each panel PINS that the other cannot
//!
//! The elbow pins the door's storage contract on a body whose inner
//! wall is a wall: both outer half-walls store the caller's
//! `minor_radius` bit for bit (the solid door's contract, unchanged by
//! growing a bore), and both inner half-walls store
//! `minor_radius - wall` — ONE IEEE subtraction of the caller's own two
//! numbers, which a consumer recovers by writing the same subtraction.
//! It also pins the OUTER walls' triangle counts against the solid
//! tube's: same surface, same window, same δ, so `tube_along_arc`'s
//! outer half-walls and this elbow's must mesh identically, and a
//! sizing change that moved one and not the other would be a fork.
//!
//! The torus pins the cavity: `Revolved::cavities` names it,
//! `classify_shells` gives it the `Void` role, and its signed volume is
//! the bore's own closed form negated — the capacity of the pipe, asked
//! for directly rather than inferred from the difference of two
//! numbers.
//!
//! # The standing gate the torus scene declares
//!
//! **The full-period hollow tube cannot leave as STEP.** It is a
//! multi-shell CURVED solid, and the writer's outward/void shell
//! classifier has closed forms for planar faces only, so it refuses
//! `CurvedShellClassification` — the known standing gate of
//! OFFSET-DESIGN O6's demo-gates list, which the hollow ring hits at
//! export and which `docs/KERNEL-VERBS.md` says this shape "joins".
//! Until this scene, that sentence was an EXPECTATION: nothing in the
//! tree ran a hollow tube through the STEP writer. The scene declares
//! the frontier at the body (`SceneBody::step_at_frontier`), which runs
//! the export on every pass and fails the tour if the refusal ever
//! changes variant or stops — the same self-retiring shape as klein's
//! wall 6, on a different door's shape.
//!
//! # Findings this scene records (the demo-purpose rule)
//!
//! 1. **A wall costs the caller one argument and costs the body its
//!    shell count — but only at the full period.** The two window
//!    policies are one door and one `wall` number, and the caller does
//!    not choose between "a shell" and "an open pipe": the WINDOW
//!    chooses. A consumer who wants a bore they can see into asks for
//!    an arc; a consumer who wants a closed toroidal cavity asks for
//!    the full period and gets a second shell they did not name. Both
//!    are right and neither is announced in the signature — the
//!    `Revolved::cavities` list is where the body says which one
//!    happened, and both scenes read it rather than assuming.
//! 2. **The hollow door is the solid door plus a subtraction, and the
//!    elbow's mesh proves it face for face.** The outer walls are not
//!    "the same kind of surface" as the solid tube's; they are the same
//!    surface, and the assertion below is on triangle counts, which is
//!    the strongest statement the tour can make without reaching into
//!    the sizing lane.
//! 3. **`step_at_frontier` is the whole STEP story for BOTH hollow
//!    curved bodies now, and they retire together.** The hollow ring's
//!    pin, klein's wall 6, and this one all name one gate. The retire
//!    note below says so, so a lane that widens the classifier finds
//!    all three from any one of them.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::geom::Surface;
use pncad::geom_core::{Point3, Tol, Vec3};
use pncad::sweep::{TubeWindow, tube_along_arc, tube_along_arc_hollow};
use pncad::topo::Body;

use crate::{SceneBody, Stop, View};

// The spine, the window, the outer radius and the chord budget are
// IMPORTED from `tube`, which owns them. This panel's headline
// assertion is that the two doors' outer walls mesh face for face, and
// that is a statement about the DOORS only if the two scenes cannot
// differ in the fixture — a second copy here would demote it to a
// claim that two constant tables agree. (They are
// `verbs_tubewall.rs`'s constants too, R for R and window for window.)
use crate::tube::{DELTA as DELTA_ELBOW, MINOR as OUTER, R, T0, T1};

/// The wall thickness — the one number the solid door has no seat for
/// (`verbs_tubewall.rs::WALL`).
const WALL: f64 = 0.125;
/// The bore's radius, as the caller recovers it: ONE IEEE subtraction.
const INNER: f64 = OUTER - WALL;
/// The full period sweeps 2π of spine where the window sweeps 1.5 rad
/// — 4.2× the arc at the same chord budget — so this panel spends a
/// coarser one. It is a montage panel, not a measurement of the
/// schedule; the elbow is where the schedule is compared.
const DELTA_TORUS: f64 = 2e-2;

/// The hollow door at this scene's constants, for either window.
fn hollow(window: TubeWindow<f64>, tol: Tol) -> pncad::sweep::Revolved<f64> {
    tube_along_arc_hollow::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        window,
        OUTER,
        WALL,
        tol,
    )
    .expect("the hollow tube builds")
}

/// Every torus wall's stored minor radius, as the body holds it.
fn stored_minors(body: &Body<f64>) -> Vec<f64> {
    body.faces()
        .filter_map(|(_, face)| match body.get_surface(face.surface) {
            Some(Surface::Torus { minor_radius, .. }) => Some(*minor_radius),
            _ => None,
        })
        .collect()
}

/// **The door's storage contract, executed on the scene body.** The
/// outer wall carries the caller's number; the inner carries the
/// caller's own subtraction. `==` on the bits, not a tolerance — that
/// is what the door exists for.
fn assert_intent_stored_bit_exact(body: &Body<f64>, what: &str) {
    let mut got: Vec<u64> = stored_minors(body).iter().map(|r| r.to_bits()).collect();
    got.sort_unstable();
    let mut want = vec![
        INNER.to_bits(),
        INNER.to_bits(),
        OUTER.to_bits(),
        OUTER.to_bits(),
    ];
    want.sort_unstable();
    assert_eq!(
        got, want,
        "{what}: two half-walls per circle, each storing the authored outer radius \
         {OUTER} or the caller's own subtraction {OUTER} - {WALL} = {INNER}, BIT for \
         bit — if this drifts, the profile -> bulge -> radius reconstruction the door \
         retired has come back through the wall"
    );
}

/// The triangle count of every torus face of `body` whose stored minor
/// radius is `r`, read off a mesh of that body — one tessellation per
/// body, since the scene compares two bodies' walls and not two
/// budgets.
fn wall_triangles(body: &Body<f64>, mesh: &pncad::mesh::Mesh, r: f64) -> Vec<usize> {
    let mut counts: Vec<usize> = body
        .faces()
        .filter(|(_, face)| {
            matches!(
                body.get_surface(face.surface),
                Some(Surface::Torus { minor_radius, .. }) if minor_radius.to_bits() == r.to_bits()
            )
        })
        .map(|(k, _)| {
            mesh.patches
                .iter()
                .find(|p| p.face == k)
                .expect("every face is meshed")
                .triangles
                .len()
        })
        .collect();
    // Sorted, so what the comparison below means is "the same multiset
    // of wall meshes" rather than "the same faces in the same iteration
    // order". Face order is the slotmap's business, and a reorder would
    // false-red an assertion that is about the SCHEDULE.
    counts.sort_unstable();
    counts
}

pub fn stops(tol: Tol) -> Vec<Stop> {
    let mut out = Vec::new();

    // ---- the windowed hollow elbow: a wall you can look down ----
    let elbow = hollow(TubeWindow::Arc { t0: T0, t1: T1 }, tol);
    assert_eq!(
        elbow.body.shells().count(),
        1,
        "an open elbow encloses nothing: one shell"
    );
    assert!(
        elbow.cavities.is_empty(),
        "a window has open ends, so it has no cavity"
    );
    assert_eq!(
        elbow.body.faces().count(),
        6,
        "two half-walls per circle + two annular caps"
    );
    assert_intent_stored_bit_exact(&elbow.body, "the elbow");

    // Pappus on the ANNULUS: section area A = π(rₒ² − rᵢ²), centroid on
    // the spine at distance R, swept θ. The walls are the curve form
    // (θ·R·2πr each), the caps flat annuli.
    let theta = T1 - T0;
    let area = PI * (OUTER * OUTER - INNER * INNER);
    let v_elbow = theta * R * area;
    let a_elbow = theta * R * 2.0 * PI * (OUTER + INNER) + 2.0 * area;
    let props = pncad::topo::mass_properties(&elbow.body, tol).expect("mass properties");
    assert!(
        ((props.volume - v_elbow) / v_elbow).abs() < 1e-12,
        "elbow V = {} vs the Pappus form {v_elbow}",
        props.volume
    );
    assert!(
        ((props.surface_area - a_elbow) / a_elbow).abs() < 1e-12,
        "elbow A = {} vs the closed form {a_elbow}",
        props.surface_area
    );
    assert_eq!(props.volume_pad, 0.0, "closed forms need no pad");

    // The elbow is hollow as a NUMBER: the solid tube of the same
    // window over the same spine is heavier by exactly the bore, which
    // is the same Pappus form on the inner disc.
    let solid = tube_along_arc::<f64>(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::unit_y(),
        Vec3::unit_x(),
        R,
        TubeWindow::Arc { t0: T0, t1: T1 },
        OUTER,
        tol,
    )
    .expect("the solid elbow builds")
    .body;
    let solid_props = pncad::topo::mass_properties(&solid, tol).expect("mass properties");
    let bore = theta * R * PI * INNER * INNER;
    assert!(
        ((solid_props.volume - props.volume - bore) / bore).abs() < 1e-12,
        "the bore is {}, closed form {bore}",
        solid_props.volume - props.volume
    );

    // Finding 2, executed: the outer walls are the SOLID door's own
    // surface over the same window at the same δ, so they mesh
    // identically. A sizing change that moved one and not the other
    // would be a semantic fork the census could not see.
    let elbow_mesh = pncad::mesh::tessellate(&elbow.body, DELTA_ELBOW, tol)
        .expect("the hollow elbow tessellates");
    let solid_mesh =
        pncad::mesh::tessellate(&solid, DELTA_ELBOW, tol).expect("the solid elbow tessellates");
    let outer_here = wall_triangles(&elbow.body, &elbow_mesh, OUTER);
    let outer_solid = wall_triangles(&solid, &solid_mesh, OUTER);
    assert_eq!(
        outer_here, outer_solid,
        "the hollow door's OUTER walls are the solid door's, face for face at δ = \
         {DELTA_ELBOW} — growing a bore must not resize the wall outside it"
    );
    let inner_here = wall_triangles(&elbow.body, &elbow_mesh, INNER);

    let (ve, ee, fe) = (
        elbow.body.vertices().count(),
        elbow.body.edges().count(),
        elbow.body.faces().count(),
    );
    // The census, pinned ABSOLUTELY: each annular cap splits both of
    // its circles at the two seam meridians (2 outer + 2 inner
    // vertices, 4 arcs), and the four longitudinal seams run cap to
    // cap. A face appearing or vanishing fails here rather than only
    // shifting the tessellation baseline's per-scene total.
    assert_eq!((ve, ee, fe), (8, 12, 6), "elbow census");

    out.push(Stop {
        name: "hollowelbow",
        caption: "THE WINDOWED HOLLOW ELBOW (a wall, and an open bore)".to_string(),
        montage: true,
        story: "the tube door's hollow sibling over `tube`'s own wedge — same spine, \
                same outer radius, same window, plus a wall. The ends are OPEN, so the \
                annular section is on screen and the bore reads without any \
                transparency",
        ops: "sweep::tube_along_arc_hollow(origin, +y, +x, R = 2, Arc{t0 = 0.25, \
              t1 = 1.75}, r = 0.5, wall = 0.125): the annular section is a second \
              directly constructed traversal at r - wall, handed to the SAME revolve \
              machinery as a hole loop — no second construction, no fork",
        delta: DELTA_ELBOW,
        note: Some(format!(
            "{ve} vertices, {ee} edges, {fe} faces in ONE shell and no cavity — a \
             window has open ends, so nothing is enclosed and `Revolved::cavities` is \
             empty. Hollow as a number: V = {:.9} m³ = θ·R·π(rₒ²−rᵢ²) by Pappus on the \
             ANNULUS, and the SOLID tube over the same window is heavier by exactly the \
             bore, {bore:.9} m³; A = {:.9} m² counts the inner wall and both annular \
             caps. Both at zero enclosure pad. The door's storage contract survives the \
             wall: both outer half-walls store minor_radius == {OUTER} bit for bit, and \
             both inner half-walls store minor_radius - wall == {INNER}, ONE IEEE \
             subtraction of the caller's own two numbers (asserted with == on the bits). \
             At δ = {DELTA_ELBOW} the outer walls mesh at {outer_here:?} triangles — \
             EQUAL, face for face, to the solid `tube_along_arc` panel's, since they are \
             the same surface over the same window — and the inner walls at \
             {inner_here:?}",
            props.volume, props.surface_area
        )),
        // `tube`'s camera: the ring lies in the world XZ plane (its
        // axis is +y), and 18 degrees off +y is where the window reads
        // as an angle AND the near cap is shaded rather than flat. On
        // this body that cap is an ANNULUS, which is the panel's whole
        // subject.
        view: View {
            elev: 28.0,
            azim: 72.0,
            up: 'z',
        },
        bodies: vec![SceneBody::plain(
            "hollowelbow",
            [0.72, 0.55, 0.38],
            elbow.body,
        )],
    });

    // ---- the full period: the inner wall becomes a cavity ----
    let torus = hollow(TubeWindow::Full, tol);
    assert_eq!(
        torus.body.shells().count(),
        2,
        "outer boundary + the cavity, in ONE solid"
    );
    assert_eq!(torus.cavities.len(), 1, "one cavity per hole loop");
    assert_ne!(
        torus.cavities[0], torus.shell,
        "the cavity is not the outer"
    );
    assert_eq!(
        torus
            .body
            .get_shell(torus.cavities[0])
            .expect("the cavity shell")
            .solid,
        torus.solid,
        "the cavity belongs to the tube's own solid"
    );
    assert_intent_stored_bit_exact(&torus.body, "the torus shell");

    // The torus closed forms — outer minus bore, and an area that
    // counts the inner wall.
    let props_t = pncad::topo::mass_properties(&torus.body, tol).expect("mass properties");
    let v_solid_t = 2.0 * PI * PI * R * OUTER * OUTER;
    let v_want = 2.0 * PI * PI * R * (OUTER * OUTER - INNER * INNER);
    let a_want = 4.0 * PI * PI * R * (OUTER + INNER);
    assert!(
        ((props_t.volume - v_want) / v_want).abs() < 1e-12,
        "torus V = {} vs the closed form {v_want}",
        props_t.volume
    );
    assert!(
        ((props_t.surface_area - a_want) / a_want).abs() < 1e-12,
        "torus A = {} vs the closed form {a_want}",
        props_t.surface_area
    );
    assert_eq!(props_t.volume_pad, 0.0, "closed forms need no pad");

    // The cavity's own capacity, asked for directly (the hollow ring's
    // finding 2, on this door's shape): the `Void`-role shell IS the
    // named cavity, and its signed volume is the bore's closed form
    // negated by the orientation convention.
    let classes = pncad::topo::classify_shells(&torus.body, tol).expect("per-shell classification");
    let voids: Vec<_> = classes
        .iter()
        .filter(|c| c.role == pncad::topo::ShellRole::Void)
        .collect();
    assert_eq!(voids.len(), 1, "one cavity, one Void shell");
    assert_eq!(
        voids[0].shell, torus.cavities[0],
        "the Void shell is the named cavity"
    );
    let v_bore_t = 2.0 * PI * PI * R * INNER * INNER;
    assert!(
        ((voids[0].volume + v_bore_t) / v_bore_t).abs() < 1e-12,
        "the cavity's capacity is {} vs the closed form -{v_bore_t}",
        voids[0].volume
    );

    let (vt, et, ft) = (
        torus.body.vertices().count(),
        torus.body.edges().count(),
        torus.body.faces().count(),
    );
    // Absolute, and the same numbers the one-call hollow ring carries:
    // per shell, 2 half-tube walls, 2 seam meridians, 2 full-period
    // rims and 2 vertices, twice over.
    assert_eq!((vt, et, ft), (4, 8, 4), "torus-shell census");

    out.push(Stop {
        name: "hollowtorus",
        caption: "THE FULL-PERIOD HOLLOW TUBE (the bore becomes a cavity)".to_string(),
        montage: true,
        story: "the SAME door, the same spine and the same wall, closed on itself. At \
                the full period the inner wall stops being a visible surface and \
                becomes a toroidal CAVITY — a second shell the caller never named",
        ops: "sweep::tube_along_arc_hollow(origin, +y, +x, R = 2, TubeWindow::Full, \
              r = 0.5, wall = 0.125): the inner traversal closes and enters as a \
              REVERSED cavity shell through the shared void-insertion door, by the \
              revolve's own holed-profile path — the hollow ring's route, reached \
              from the parameter door",
        delta: DELTA_TORUS,
        note: Some(format!(
            "{vt} vertices, {et} edges, {ft} faces over TWO shells in one solid — the \
             hollow ring's census exactly, reached through the INTENT-parameter door \
             instead of through a holed profile. V = {:.9} m³ against 2π²R(rₒ²−rᵢ²), \
             where the solid tube of the same outer radius would be {v_solid_t:.9}; \
             A = {:.9} m² = 4π²R(rₒ+rᵢ). The cavity is asked for directly rather than \
             inferred: `classify_shells` gives it the Void role and its signed volume is \
             {:.9} m³, the bore's own capacity negated by the orientation convention. \
             Drawn see-through at 45 because a cavity cannot be read from an opaque \
             render at any camera — the elbow panel next door is the same wall with the \
             ends OPEN, which is why that one needs no transparency. Its remaining wall \
             is STEP: a multi-shell CURVED solid refuses CurvedShellClassification, \
             which docs/KERNEL-VERBS.md predicted for this shape and this scene now \
             PROBES on every pass. δ = {DELTA_TORUS} rather than the elbow's \
             {DELTA_ELBOW}: the full period sweeps 2π of spine where the window sweeps \
             {theta}, so the same chord budget would cost 4.2x the mesh",
            props_t.volume, props_t.surface_area, voids[0].volume
        )),
        // The hollow ring's camera, for the same reason and on the same
        // kind of body: the tube axis is +y, and this elevation shows
        // the bore's silhouette through the wall.
        view: View {
            elev: 24.0,
            azim: -55.0,
            up: 'y',
        },
        bodies: vec![
            SceneBody::plain("hollowtorus", [0.52, 0.60, 0.78], torus.body)
                .transparent(45)
                .step_at_frontier(
                    |e| {
                        matches!(
                            e,
                            pncad::step_export::StepExportError::CurvedShellClassification { .. }
                        )
                    },
                    "the writer's outward/void classifier has grown a curved arm. Say so \
                     in klein's findings entry 7 and in docs/KERNEL-VERBS.md's hollow-ring \
                     STEP row, and retire ALL THREE probes of this one gate together: \
                     klein's WALL 6, the `ring` scene's `step_at_frontier`, and this one \
                     — the ring is the profile door's shape, this is the parameter \
                     door's, and a widened classifier retires both",
                ),
        ],
    });

    out
}
