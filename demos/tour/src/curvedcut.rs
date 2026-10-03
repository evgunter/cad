//! The tilted cut — the tour's curved-surface stop.
//!
//! A cylinder whose top cap is engraved with the word CUT — three
//! glyphs of lines and circular arcs, each extruded as a tool and
//! subtracted as a blind pocket — then split by a plane tilted [`PHI`]
//! through its mid-height. The section edges carry an exact
//! `Curve3::Ellipse` (a = r/cos φ, b = r) described as the wall×plane
//! `Intersection`, with a rounding-scale certificate residual because
//! the carrier is zero-residual by construction (D4 ¶2), not fitted.
//! The halves' volumes are certified quadrature enclosures, their
//! conic-trimmed walls tessellate through the pcurve-driven trimmed
//! lane, and the module is generic over [`Scalar`] so the K-probe
//! sweep rebuilds it at the recording scalar (`crate::probe`).
//!
//! **Oracle.** Each pocket removes exactly its glyph's planar area ×
//! [`DEPTH`], the area closed-form because a glyph is lines and arcs;
//! the upper half's certified bracket contains πr²H/2 less the three
//! pockets, the lower half's πr²H/2.
//!
//! **Walls.** The lettering belongs on the elliptical section face (an
//! oval nameplate), the natural order is cut first and engrave after,
//! and the natural C has one arc per side. [`walls`] attempts each every
//! run, the section face on both halves: the U now cuts the upper
//! half's section face at its closed-form volume (checked there); the
//! C on the lower half's, the cap after the cut and the one-arc C still
//! refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p2, p3, polygon, v3, validated};
use pncad::geom_core::{OrthoFrame, Tol};
use pncad::prelude::{Open, Start};
use pncad::profile::{ArcSweep, Center, ConstructedLoop, Profile, SketchPlane, ValidatedProfile};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::splitting::{SplitPart, split};
use pncad::topo::{Body, BooleanError, BooleanResult, Curve3, EdgeDescription, PointInSolidError};

use crate::booleans::try_subtract;
use crate::scalar::{Scalar, sketch_frame, split_plane};
use crate::{SceneBody, Stop, View};

/// The cylinder's radius (m).
const R: f64 = 1.0;
/// The cylinder's height (m).
const H: f64 = 2.5;
/// The section plane's tilt from the cylinder's cross-section (rad).
const PHI: f64 = 0.3;
/// How deep each glyph is engraved (m).
const DEPTH: f64 = 0.05;

/// The disc profile: two half-circle arcs (bulge 1) of radius [`R`] —
/// extrudes to a cylinder whose two wall faces share ONE cylinder
/// surface.
fn disc<S: Scalar>(tol: Tol) -> ValidatedProfile<S> {
    let lp = pncad::profile::circle(p2::<S>(0.0, 0.0), S::from_f64(R), tol)
        .expect("disc radius is positive")
        .into();
    Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .expect("the disc profile validates")
}

/// One glyph of the lettering: its outline in sketch coordinates and
/// its area in closed form.
struct Glyph<S: Scalar> {
    name: &'static str,
    outline: ConstructedLoop<S>,
    area: f64,
}

/// "CUT", 0.5 m tall and 1.5 m wide, centred on the sketch origin.
fn lettering<S: Scalar>(tol: Tol) -> [Glyph<S>; 3] {
    [glyph_c(tol), glyph_u(tol), glyph_t(tol)]
}

/// The C's centre and its outer and inner radii.
const C_ARCS: (f64, f64, f64) = (-0.5, 0.25, 0.15);
/// Half the C's opening, about +x from its centre (rad).
const C_GAP: f64 = PI / 4.0;

/// How the C's two sides are drawn.
#[derive(Clone, Copy)]
enum Sides {
    /// One arc per side: the spelling a user writes first.
    OneArc,
    /// Each side as two arcs meeting tangent at the C's leftmost point.
    SplitAtApex,
}

/// The C's outline: the annular sector about [`C_ARCS`]'s centre
/// between its two radii, open over 2·[`C_GAP`] facing +x, with its
/// sides drawn per `sides`.
fn c_outline<S: Scalar>(sides: Sides, tol: Tol) -> ConstructedLoop<S> {
    let (cx, ro, ri) = C_ARCS;
    let at = |r: f64, a: f64| p2::<S>(cx + r * a.cos(), r * a.sin());
    let about = |winding, p| Center {
        c: p2(cx, 0.0),
        winding,
        p,
    };
    let (start, outer_end) = (at(ro, C_GAP), at(ro, -C_GAP));
    let (inner_start, inner_end) = (at(ri, -C_GAP), at(ri, C_GAP));
    match sides {
        Sides::OneArc => Open
            .at(start)
            .arc_to(about(ArcSweep::Ccw, outer_end), tol)
            .expect("the C's outer arc")
            .line_to(inner_start, tol)
            .expect("its lower terminal")
            .arc_to(about(ArcSweep::Cw, inner_end), tol)
            .expect("the C's inner arc")
            .line_to(Start, tol)
            .expect("its upper terminal closes the C")
            .into(),
        Sides::SplitAtApex => Open
            .at(start)
            .arc_to(about(ArcSweep::Ccw, p2(cx - ro, 0.0)), tol)
            .expect("the C's outer arc, to its leftmost point")
            .tangent()
            .tangent_arc_to(outer_end, tol)
            .expect("and on round to its lower end")
            .line_to(inner_start, tol)
            .expect("its lower terminal")
            .arc_to(about(ArcSweep::Cw, p2(cx - ri, 0.0)), tol)
            .expect("the C's inner arc, to its leftmost point")
            .tangent()
            .tangent_arc_to(inner_end, tol)
            .expect("and on round to its upper end")
            .line_to(Start, tol)
            .expect("its upper terminal closes the C")
            .into(),
    }
}

/// C: the annular sector of [`c_outline`], sweeping θ = 2π − 2·[`C_GAP`]
/// and enclosing (θ/2)(r_o² − r_i²).
///
/// Its sides are split at the apex: drawn as one arc each, the C cuts
/// no pocket (`work/zip/an-engraved-annular-sector-refuses-seam-orientation.md`,
/// wall 4 in [`walls`]).
fn glyph_c<S: Scalar>(tol: Tol) -> Glyph<S> {
    let (_, ro, ri) = C_ARCS;
    let sweep = 2.0 * PI - 2.0 * C_GAP;
    Glyph {
        name: "C",
        outline: c_outline(Sides::SplitAtApex, tol),
        area: sweep / 2.0 * (ro * ro - ri * ri),
    }
}

/// U: two 0.1 × 0.3 stems over x ∈ ±[0.1, 0.2], joined below
/// y = −0.05 by the half annulus about (0, −0.05) between radii 0.2
/// and 0.1, the stems running tangent into it: 2 · 0.03 +
/// (π/2)(0.2² − 0.1²).
fn glyph_u<S: Scalar>(tol: Tol) -> Glyph<S> {
    let outline = Open
        .at(p2::<S>(-0.2, 0.25))
        .line_to(p2(-0.2, -0.05), tol)
        .expect("the U's outer left flank")
        .tangent()
        .tangent_arc_to(p2(0.2, -0.05), tol)
        .expect("the outer bowl")
        .continue_to(p2(0.2, 0.25), tol)
        .expect("the outer right flank")
        .line_to(p2(0.1, 0.25), tol)
        .expect("the right serif")
        .line_to(p2(0.1, -0.05), tol)
        .expect("the inner right flank")
        .tangent()
        .tangent_arc_to(p2(-0.1, -0.05), tol)
        .expect("the inner bowl")
        .continue_to(p2(-0.1, 0.25), tol)
        .expect("the inner left flank")
        .line_to(Start, tol)
        .expect("the left serif closes the U")
        .into();
    Glyph {
        name: "U",
        outline,
        area: 0.06 + 0.5 * PI * (0.2 * 0.2 - 0.1 * 0.1),
    }
}

/// T: a 0.4 × 0.1 bar on a 0.1 × 0.4 stem, lines only: 0.08.
fn glyph_t<S: Scalar>(tol: Tol) -> Glyph<S> {
    let outline = polygon(
        &[
            (0.35, 0.25),
            (0.35, 0.15),
            (0.5, 0.15),
            (0.5, -0.25),
            (0.6, -0.25),
            (0.6, 0.15),
            (0.75, 0.15),
            (0.75, 0.25),
        ],
        tol,
    )
    .expect("the T's outline");
    Glyph {
        name: "T",
        outline,
        area: 0.08,
    }
}

/// A glyph's tool: its outline on `plane`, which lies [`DEPTH`] inside
/// the face being engraved, extruded 2·[`DEPTH`] along the plane's
/// normal so the tool straddles that face.
fn tool<S: Scalar>(plane: SketchPlane<S>, outline: ConstructedLoop<S>, tol: Tol) -> Body<S> {
    let profile = validated(plane, vec![outline], tol).expect("a glyph validates");
    extrude(&profile, Extrusion::Distance(S::from_f64(2.0 * DEPTH)), tol)
        .expect("extrude a glyph")
        .body
}

/// The xy sketch plane at height `z`.
fn level<S: Scalar>(z: f64) -> SketchPlane<S> {
    SketchPlane::from_frame(OrthoFrame::axes_xy(p3(0.0, 0.0, z)))
}

/// The scene's bodies.
pub struct Cut<S: Scalar> {
    /// The cylinder, then the cylinder after each glyph's pocket, in
    /// [`lettering`] order.
    pub stages: Vec<Body<S>>,
    /// The half on the section normal's side: it carries the
    /// engraved cap.
    pub above: Body<S>,
    /// The half against the section normal, unengraved.
    pub below: Body<S>,
}

/// `body` split by the plane tilted [`PHI`] through mid-height, as
/// (above, below).
fn tilted_cut<S: Scalar>(body: &Body<S>, tol: Tol) -> (Body<S>, Body<S>) {
    let plane = split_plane(p3(0.0, 0.0, H / 2.0), v3(PHI.sin(), 0.0, PHI.cos()), tol);
    let result = split(body, &plane, tol).expect("the tilted cut splits the cylinder");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (result.above, result.below) else {
        panic!("the section plane crosses the wall: both sides must be bodies");
    };
    (above, below)
}

/// Engraves the cap, then cuts the cylinder by the tilted plane
/// through mid-height.
pub fn build<S: Scalar>(tol: Tol) -> Cut<S> {
    let cylinder = extrude(&disc::<S>(tol), Extrusion::Distance(S::from_f64(H)), tol)
        .expect("extrude cylinder")
        .body;
    let mut stages = vec![cylinder];
    for g in lettering::<S>(tol) {
        let last = stages.last().expect("the cylinder is the first stage");
        let pocketed = match try_subtract(last, &tool(level(H - DEPTH), g.outline, tol), tol) {
            Ok(BooleanResult::Body(b)) => b.body,
            other => panic!(
                "engraving the {} into the cap: {:?}",
                g.name,
                other.map(|_| ())
            ),
        };
        stages.push(pocketed);
    }
    let (above, below) = tilted_cut(stages.last().expect("three pockets were cut"), tol);
    Cut {
        above,
        below,
        stages,
    }
}

/// Narration for one half: how many exact ellipse arcs it carries,
/// their semi-axes against the closed form (a = r/cos φ, b = r), the
/// worst certificate residual, and the certified volume bracket
/// against `exact`.
fn section_narration(label: &str, body: &Body<f64>, exact: f64, tol: Tol) -> String {
    let mut arcs = 0usize;
    let mut worst = 0.0f64;
    for (_, edge) in body.edges() {
        let Some(curve) = body.get_curve_geom(edge.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let Curve3::Ellipse { major, minor, .. } = *curve.carrier() else {
            continue;
        };
        assert!(
            (major - R / PHI.cos()).abs() < 1e-12 && (minor - R).abs() < 1e-12,
            "{label}: section semi-axes must be the closed form \
             (a = r/cos phi, b = r), got a = {major}, b = {minor}"
        );
        assert!(
            matches!(curve.description(), EdgeDescription::Intersection { .. }),
            "{label}: a section edge must be described as the wall x plane intersection"
        );
        worst = worst.max(curve.certificate().max_residual);
        arcs += 1;
    }
    assert!(arcs > 0, "{label}: the tilted cut must mint ellipse arcs");

    let m = pncad::topo::mass_properties(body, tol).expect("the quadrature lane computes");
    let (v, pad) = (m.volume, m.volume_pad);
    assert!(
        v - pad <= exact && exact <= v + pad,
        "{label}: certified bracket [{}, {}] must contain the closed form {exact}",
        v - pad,
        v + pad
    );
    format!(
        "{arcs} exact Ellipse arc(s): a = r/cos {PHI} = {:.6} m, b = r = {R} m, \
         described as the wall x plane intersection; worst certified residual \
         {worst:.2e} m; certified volume enclosure {v:.9} ± {pad:.2e} m^3 \
         BRACKETS the closed form {exact:.9}",
        R / PHI.cos()
    )
}

/// The pockets against their closed forms: what each subtraction
/// removed from the closed-form cylinder volume is its glyph's area ×
/// [`DEPTH`]. Returns the pockets' closed-form total and the narration.
fn pocket_narration(stages: &[Body<f64>], tol: Tol) -> (f64, String) {
    let volume = |b: &Body<f64>| {
        let m = pncad::topo::mass_properties(b, tol).expect("the engraved cylinder measures");
        assert_eq!(
            m.volume_pad, 0.0,
            "the engraved cylinder is lines, circles and planes: its volume is closed-form"
        );
        m.volume
    };
    let glyphs = lettering::<f64>(tol);
    assert_eq!(
        stages.len(),
        glyphs.len() + 1,
        "one stage per glyph after the bare cylinder"
    );
    let mut total = 0.0;
    let mut lines = Vec::new();
    for (g, pair) in glyphs.iter().zip(stages.windows(2)) {
        let removed = volume(&pair[0]) - volume(&pair[1]);
        let exact = g.area * DEPTH;
        let err = (removed - exact).abs();
        // Both volumes are closed forms near 7.85 m^3: their difference
        // is good to a few ulps of that, about 1e-15.
        assert!(
            err < 1e-14,
            "the {} pocket removed {removed} m^3, not its area x depth {exact} m^3",
            g.name
        );
        lines.push(format!(
            "{}: area {:.9} m^2 x {DEPTH} m = {exact:.9} m^3, removed {removed:.9} \
             (|diff| {err:.1e})",
            g.name, g.area
        ));
        total += exact;
    }
    (total, lines.join("; "))
}

/// The lettering where it was wanted, attempted live.
fn walls(cut: &Cut<f64>, tol: Tol) {
    // The section face's own frame: u along the ellipse's major axis,
    // v along its minor, so u × v is the section normal. The sketch
    // sits DEPTH below the section, so the tool straddles it and sinks
    // DEPTH into either half.
    let n = (PHI.sin(), 0.0, PHI.cos());
    let section = || {
        sketch_frame(
            p3(-DEPTH * n.0, 0.0, H / 2.0 - DEPTH * n.2),
            v3(PHI.cos(), 0.0, -PHI.sin()),
            v3(0.0, 1.0, 0.0),
            tol,
        )
    };
    // The section face's `Ellipse` rim is crossed or cleared exactly
    // since REACH's conic rung (PR 3805); the C on the lower half's face
    // then stops at the containment probe
    // (`work/contact/at-infinity-probe-measures-in-closed-form-only.md`).
    let below = &cut.below;
    let c = tool(section(), glyph_c::<f64>(tol).outline, tol);
    crate::walls::wall(
        "tilted cut",
        1,
        "engrave the C into the lower half's elliptical section face",
        try_subtract(below, &c, tol),
        |e| {
            matches!(
                e,
                BooleanError::Containment(PointInSolidError::VolumeUncertified)
            )
        },
        "move the lettering onto the section face (the oval nameplate) and retire \
         this probe",
    );
    // Before the conic rung every arc-bearing glyph (C, U, a disc)
    // refused at the rim on either half's section face, at every pose
    // tried (offsets (0, 0), (0.3, 0.2) and (−0.2, −0.3) in the face's
    // frame, depths 0.02, 0.05 and 0.2); a lines-only glyph depended on
    // the pose and the half (on the lower half a square and the T
    // refused `Containment(VolumeUncertified)` at all nine poses, on the
    // upper half a square cut at 8 of 9). Re-measured with the rung only
    // at the walls' own pose.
    // The U on the upper half's section face BUILDS since the conic
    // rung (PR 3805), and is held to the scene's own oracle: its pocket
    // removes the glyph's area × DEPTH from the half, inside the
    // certified bracket, at tier 3. The scene still engraves the cap.
    let above = &cut.above;
    let glyph = glyph_u::<f64>(tol);
    let removed = glyph.area * DEPTH;
    let u = tool(section(), glyph.outline, tol);
    let engraved = try_subtract(above, &u, tol)
        .ok()
        .and_then(|r| r.body().map(|b| b.body.clone()))
        .expect("the U engraves the upper half's section face");
    pncad::topo::validate_geometric(&engraved, tol).expect("the engraved half is tier-3 valid");
    let before = pncad::topo::mass_properties(above, tol).expect("the half measures");
    let after = pncad::topo::mass_properties(&engraved, tol).expect("the engraved half measures");
    assert!(
        (before.volume - after.volume - removed).abs() <= before.volume_pad + after.volume_pad,
        "the U pocket removed {} m^3, not its area x depth {removed} m^3",
        before.volume - after.volume
    );
    // The order the scene would adopt: cut first, then engrave the
    // upper half's top cap. Every glyph tried refuses the same way on
    // either half's round cap after the cut — C, U, T, a square and a
    // disc, at depths 0.02, 0.05 and 0.2.
    let (bare_above, _) = tilted_cut(&cut.stages[0], tol);
    let c_cap = tool(level(H - DEPTH), glyph_c::<f64>(tol).outline, tol);
    crate::walls::wall(
        "tilted cut",
        3,
        "engrave the C into the upper half's round cap, after the cut",
        try_subtract(&bare_above, &c_cap, tol),
        |e| {
            matches!(
                e,
                BooleanError::Containment(PointInSolidError::VolumeUncertified)
            )
        },
        "engrave the cap after the cut, the natural order, and retire this probe",
    );
    let c_whole = tool(level(H - DEPTH), c_outline(Sides::OneArc, tol), tol);
    crate::walls::wall(
        "tilted cut",
        4,
        "engrave the C drawn with one arc per side into the cylinder's cap",
        try_subtract(&cut.stages[0], &c_whole, tol),
        |e| matches!(e, BooleanError::SeamOrientation { .. }),
        "draw glyph_c with Sides::OneArc and retire this probe",
    );
}

/// The rendering stop.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let cut = build::<f64>(tol);
    walls(&cut, tol);
    let half = PI * R * R * H / 2.0;
    let (pockets, narration_pockets) = pocket_narration(&cut.stages, tol);
    let narration_above = section_narration("tiltedcut_above", &cut.above, half - pockets, tol);
    let narration_below = section_narration("tiltedcut_below", &cut.below, half, tol);
    let Cut { above, below, .. } = cut;
    vec![Stop {
        name: "tiltedcut",
        caption: "tilted cut (engraved cap, exact ellipse section)".to_string(),
        montage: true,
        story: "a cylinder with CUT engraved in its cap, cut at an angle: each pocket \
                is its glyph's area times its depth, the section edges carry an EXACT \
                ellipse, and each half's volume is a certified quadrature enclosure",
        ops: "extrude(disc); per glyph: extrude(lines + arcs) -> subtract (blind \
              pocket); topo::split(tilted plane); exact Curve3::Ellipse section \
              carriers; pcurve trim loops + certified quadrature",
        // The glyphs' inner arcs (radius 0.1 to 0.15) want 2e-3, and
        // both halves take it so they render alike: one body's delta
        // covers every face of it
        // (`work/tess/a-body-meshes-every-face-at-its-smallest-features-delta.md`).
        delta: 2e-3,
        note: Some(format!(
            "cutting a cylinder at an angle produces an ellipse, and this kernel \
             stores one: exact semi-axes, zero residual by construction. The cap \
             is engraved first, three blind pockets whose volumes are closed-form; \
             the cut halves are measured by certified volume enclosures and drawn \
             as watertight trimmed walls.\n   \
             [pockets] {narration_pockets}\n   \
             [tiltedcut_above] pi*r^2*H/2 less the pockets; section: {narration_above}\n   \
             [tiltedcut_below] pi*r^2*H/2; section: {narration_below}"
        )),
        view: View {
            elev: 30.0,
            azim: -60.0,
            up: 'z',
        },
        bodies: vec![
            SceneBody::plain("tiltedcut_above", [0.62, 0.44, 0.80], above),
            SceneBody::plain("tiltedcut_below", [0.40, 0.62, 0.80], below),
        ],
    }]
}
