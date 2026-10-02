//! The cutaway section (#91 C4): the tour's first `pncad::topo::split` — the
//! project box (itself a 15-op boolean result) split by a TILTED
//! plane, both halves validated independently, then translated apart
//! along the section normal by rigid transforms (re-minted witnesses,
//! #84) and rendered as a machinist's section pair.
//!
//! The plane passes through the two bored bosses at `x = 2.375`,
//! between floor and boss top, so each half's section through a boss
//! is ONE face with that boss's bore as its ring: an annulus, the hole
//! the ellipse a tilted plane cuts from a round bore. [`build`] asserts
//! the halves' section faces and rings; [`sectioned_beside`] reads the
//! same section back through `topo::plane_section` and checks its
//! regions against the closed-form areas.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p3, v3};
use pncad::geom_core::{Affine3, Point2, Vec3};
use pncad::topo::splitting::{PlaneSide, SplitPart, SplitPlane, plane_section, split};

use crate::SceneBody;
use crate::projectbox::{BORE_AXES, BORE_R};
use crate::scalar::{Scalar, split_plane};
use pncad::geom_core::Tol;

/// The section plane's normal, unnormalized: tilted about both
/// horizontal axes, no axis alignment.
const NORMAL: (f64, f64, f64) = (0.75, 0.1875, 1.0);

/// A point of the section plane: midway between the two bosses at
/// `x = 2.375`, at `z = 0.53`, so the plane meets both their bores
/// between the floor (`z = 0.25`) and the boss tops (`z = 0.875`) and
/// passes above the box over the other two.
const THROUGH: (f64, f64, f64) = (2.375, 1.0, 0.53);

/// The ring count of each section face a half keeps, ascending. Five
/// faces through the walls and floor, unringed: the open top and the
/// vent slots cut the walls' section into four bands and the piece
/// that turns the far corner through the floor. Two faces through the
/// bored bosses, islands in the cavity, each ringed by its bore.
const SECTION_RINGS: [usize; 7] = [0, 0, 0, 0, 0, 1, 1];

/// What `build` measured, for the narration.
pub(crate) struct SectionNumbers {
    pub v_above: f64,
    pub v_below: f64,
    pub v_box: f64,
    /// `|v_above + v_below − v_box|`.
    pub gap: f64,
    /// The section's area, from the halves' surface areas: the two
    /// halves' faces are the box's faces plus the section twice.
    pub area: f64,
    /// The half-width of `area`'s certified bracket: the bores' walls
    /// are measured by quadrature.
    pub area_pad: f64,
}

/// The section plane.
fn section_plane<S: Scalar>(tol: Tol) -> SplitPlane<S> {
    split_plane(
        p3(THROUGH.0, THROUGH.1, THROUGH.2),
        v3(NORMAL.0, NORMAL.1, NORMAL.2),
        tol,
    )
}

/// `cos φ`, φ the plane's tilt from the horizontal: a vertical prism of
/// cross-section `A` meets the plane in an area `A / cos φ`.
fn cos_tilt() -> f64 {
    NORMAL.2 / (NORMAL.0 * NORMAL.0 + NORMAL.1 * NORMAL.1 + NORMAL.2 * NORMAL.2).sqrt()
}

/// The split + explode, generic (the Probe sweep runs the same ops):
/// returns the two moved halves and the [`SectionNumbers`].
pub(crate) fn build<S: Scalar>(
    boxbody: &pncad::topo::Body<S>,
    tol: Tol,
) -> ((pncad::topo::Body<S>, pncad::topo::Body<S>), SectionNumbers) {
    let plane = section_plane::<S>(tol);
    let res = split(boxbody, &plane, tol).expect("split of the boolean-result box");
    let (SplitPart::Body(above), SplitPart::Body(below)) = (&res.above, &res.below) else {
        panic!("the section plane crosses the box: both sides must be bodies");
    };

    for (side, half, which) in [
        ("above", above, PlaneSide::Above),
        ("below", below, PlaneSide::Below),
    ] {
        let mut rings: Vec<usize> = res
            .naming
            .sections
            .iter()
            .filter(|(_, s)| *s == which)
            .map(|&(k, _)| {
                half.get_face(k)
                    .expect("a half keeps its own section faces")
                    .rings
                    .len()
            })
            .collect();
        rings.sort_unstable();
        assert_eq!(
            rings, SECTION_RINGS,
            "{side}: a section face per piece of the cut, each bored boss's face ringed by its bore"
        );
    }

    // The bores' walls are measured by quadrature, so each reading is a
    // certified bracket: midpoint and half-width.
    let props = |b: &pncad::topo::Body<S>, what: &str| {
        let p = pncad::topo::mass_properties(b, tol).expect(what);
        (
            (p.volume.f(), p.volume_pad),
            (p.surface_area.f(), p.area_pad),
        )
    };
    let ((v_box, vpad_box), (s_box, apad_box)) = props(boxbody, "box props");
    let ((v_above, vpad_above), (s_above, apad_above)) = props(above, "above props");
    let ((v_below, vpad_below), (s_below, apad_below)) = props(below, "below props");
    let gap = (v_above + v_below - v_box).abs();
    let vpad = vpad_above + vpad_below + vpad_box;
    assert!(
        gap <= vpad + 1e-12,
        "split halves must partition the volume (gap {gap:.3e}, brackets ± {vpad:.1e})"
    );
    let area = (s_above + s_below - s_box) / 2.0;
    let area_pad = (apad_above + apad_below + apad_box) / 2.0;

    // Pull the halves apart 0.75 along the section normal: rigid
    // transforms re-mint every moved witness (#84).
    let n = plane.normal.get() * S::from_f64(0.75);
    let moved_above = pncad::topo::transform_rigid(above, &Affine3::translation(n), tol)
        .expect("translate above half");
    let moved_below = pncad::topo::transform_rigid(below, &Affine3::translation(-n), tol)
        .expect("translate below half");
    (
        (moved_above, moved_below),
        SectionNumbers {
            v_above,
            v_below,
            v_box,
            gap,
            area,
            area_pad,
        },
    )
}

/// Twice the signed area of a polygon given by its corners.
fn twice_area(uv: &[Point2<f64>]) -> f64 {
    (0..uv.len())
        .map(|i| {
            let (a, b) = (uv[i], uv[(i + 1) % uv.len()]);
            a.x * b.y - b.x * a.y
        })
        .sum()
}

/// What [`read_section`] measured, for the narration.
struct SectionReading {
    regions: usize,
    holes: usize,
    /// The regions' area: the outlines' polygon areas less the holes'
    /// closed-form ellipses.
    area: f64,
}

/// `plane_section` of the box, checked against the closed forms and
/// against the area the split's halves enclose (`split_area`).
///
/// Every outline here is a polygon, so its area is its corners'; a
/// bore's section is two arcs between two corners, so a hole's area is
/// the closed form `π r² / cos φ`. The region a bored boss's section
/// makes is the boss's square section, `0.375² / cos φ`, around one
/// such hole.
fn read_section(
    boxbody: &pncad::topo::Body<f64>,
    (split_area, split_pad): (f64, f64),
    tol: Tol,
) -> SectionReading {
    let section = plane_section(boxbody, &section_plane::<f64>(tol), tol)
        .expect("plane_section of the box through its bores");
    let ellipse = PI * BORE_R * BORE_R / cos_tilt();
    let boss = 0.375 * 0.375 / cos_tilt();
    let on_a_bore = |p: &pncad::geom_core::Point3<f64>| {
        BORE_AXES
            .iter()
            .any(|&(cx, cy)| ((p.x - cx).hypot(p.y - cy) - BORE_R).abs() < 1e-9)
    };
    let mut area = 0.0;
    let mut holes = 0;
    for region in &section.regions {
        let outline = twice_area(&region.outline.uv) / 2.0;
        assert!(outline > 0.0, "an outline winds counter-clockwise");
        for hole in &region.holes {
            assert!(
                hole.points.iter().all(on_a_bore),
                "a hole's corners lie on a bore: {:?}",
                hole.points
            );
        }
        if !region.holes.is_empty() {
            assert_eq!(region.holes.len(), 1, "a boss's region holds its one bore");
            assert!(
                (outline - boss).abs() < 1e-9,
                "a bored boss's region is its square's section: {outline} against {boss}"
            );
        }
        holes += region.holes.len();
        area += outline - ellipse * region.holes.len() as f64;
    }
    assert_eq!(
        section.regions.len(),
        SECTION_RINGS.len(),
        "a region per section face a half keeps"
    );
    assert_eq!(holes, 2, "the plane crosses two bores");
    assert!(
        (area - split_area).abs() <= split_pad + 1e-12,
        "the section's regions enclose {area}, the split's halves {split_area} ± {split_pad:.1e}"
    );
    SectionReading {
        regions: section.regions.len(),
        holes,
        area,
    }
}

/// How far along +x the sectioned pair sits from the whole box. The
/// enclosure is 3 long and `build` has already pushed the below half
/// 0.445 the other way along the section normal, so 5 leaves about
/// 1.55 of clear air at the shared camera.
pub(crate) const SECTION_GAP: f64 = 5.0;

/// The two halves, placed beside the whole box for the shared cell.
///
/// The halves travel because they are already a FRAMING — `build` has
/// pulled them apart along the section normal — so placing them beside
/// the whole box is the same act again, where moving the box would put
/// the part somewhere its own narration does not say it is.
pub(crate) fn sectioned_beside(
    boxbody: &pncad::topo::Body<f64>,
    tol: Tol,
) -> (Vec<SceneBody>, String) {
    let ((moved_above, moved_below), n) = build(boxbody, tol);
    let reading = read_section(boxbody, (n.area, n.area_pad), tol);
    let note = format!(
        "first `topo::split` in the tour, ON a 15-op boolean result; section plane \
         normal (0.75, 0.1875, 1) — tilted, no axis alignment — through two bored \
         bosses; each half keeps {faces} section faces, the two through the bosses \
         annuli ringed by their bores; `plane_section` reads {regions} regions with \
         {holes} holes, area {area:.9} (outline polygons less pi r^2 / cos phi per \
         bore) against {split:.9} ± {pad:.1e} from the halves' surface areas; halves partition \
         the volume within their certified brackets ({v_above:.6} + {v_below:.6} = {v_box:.6}, gap \
         {gap:.1e}); halves then moved apart by rigid transforms (edge witnesses \
         re-minted, #84) and revalidated",
        faces = SECTION_RINGS.len(),
        regions = reading.regions,
        holes = reading.holes,
        area = reading.area,
        split = n.area,
        pad = n.area_pad,
        v_above = n.v_above,
        v_below = n.v_below,
        v_box = n.v_box,
        gap = n.gap,
    );
    let aside = Affine3::translation(Vec3::new(SECTION_GAP, 0.0, 0.0));
    let above = pncad::topo::transform_rigid(&moved_above, &aside, tol)
        .expect("place the above half beside the box");
    let below = pncad::topo::transform_rigid(&moved_below, &aside, tol)
        .expect("place the below half beside the box");
    (
        vec![
            // The plane frees the two bored bosses' tops: this half is
            // one solid of three outer shells, and the STEP writer's
            // shell classifier reads no circle edge.
            SceneBody::plain("cutaway_above", [0.40, 0.60, 0.72], above).step_at_frontier(
                |e| {
                    matches!(
                        e,
                        pncad::step_export::StepExportError::CurvedShellClassification { .. }
                    )
                },
                "the writer classifies curved shells now \
                 (work/export/step-export-reclassifies-shell-roles-with-its-own-planar-flux.md): \
                 this half's three shells are all outer, so it exports; drop this pin",
            ),
            SceneBody::plain("cutaway_below", [0.78, 0.60, 0.35], below),
        ],
        note,
    )
}
