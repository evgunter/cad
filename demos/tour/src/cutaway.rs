//! The cutaway section (#91 C4): the tour's first `pncad::topo::split` — the
//! project box (itself a 15-op boolean result) split by a TILTED
//! plane, both halves validated independently, then translated apart
//! along the section normal by rigid transforms (re-minted witnesses,
//! #84) and rendered as a machinist's section pair.
//!
//! The plane passes through the two bored bosses at `x = 2.375`,
//! between floor and boss top, so each half's section through a boss
//! is ONE face with that boss's bore as its ring: the ellipse a tilted
//! plane cuts from the boss around the one it cuts from the bore. [`build`] asserts
//! the halves' section faces and rings; [`sectioned_beside`] reads the
//! same section back through `topo::plane_section` and checks its
//! regions against the closed-form areas.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use pncad::authoring::{p3, v3};
use pncad::geom_core::{Affine3, Vec3};
use pncad::topo::splitting::{PlaneSide, SplitPart, SplitPlane, plane_section, split};

use crate::SceneBody;
use crate::projectbox::{BORE_R, BOSS_AXES, BOSS_R, BOSS_Z, FLOOR_TOP};
use crate::scalar::{Scalar, split_plane};
use pncad::geom_core::Tol;

/// The section plane's normal, unnormalized: tilted about both
/// horizontal axes, no axis alignment.
const NORMAL: (f64, f64, f64) = (0.75, 0.1875, 1.0);

/// A point of the section plane: midway between the two bosses at
/// `x = 2.375`, at `z = 0.53`, so the plane meets both bosses between
/// [`FLOOR_TOP`] and their tops and passes above the box over the
/// other two ([`crossed_bosses`]).
const THROUGH: (f64, f64, f64) = (2.375, 1.0, 0.53);

/// The plane's height over `(x, y)`.
fn plane_z(x: f64, y: f64) -> f64 {
    THROUGH.2 - (NORMAL.0 * (x - THROUGH.0) + NORMAL.1 * (y - THROUGH.1)) / NORMAL.2
}

/// How far the plane rises across a boss's radius: its slope times
/// [`BOSS_R`].
fn rise_over_a_boss() -> f64 {
    BOSS_R * NORMAL.0.hypot(NORMAL.1) / NORMAL.2
}

/// The bosses the plane crosses wholly between the floor top and the
/// boss top, so each one's section is an island in the cavity; and,
/// asserted, that there are two of them.
fn crossed_bosses() -> Vec<(f64, f64)> {
    let crossed: Vec<(f64, f64)> = BOSS_AXES
        .into_iter()
        .filter(|&(x, y)| {
            let z = plane_z(x, y);
            z - rise_over_a_boss() > FLOOR_TOP && z + rise_over_a_boss() < BOSS_Z.1
        })
        .collect();
    let missed = BOSS_AXES
        .iter()
        .all(|&(x, y)| crossed.contains(&(x, y)) || plane_z(x, y) - rise_over_a_boss() > BOSS_Z.1);
    assert!(
        crossed.len() == 2 && missed,
        "the plane crosses two bosses whole and passes over the others"
    );
    crossed
}

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
    // A bracket check is only as sharp as its brackets, so each is
    // capped at a hundredth of the smallest thing it exists to see, so
    // losing that thing still lands a hundred brackets out.
    // The partition would miss a lost piece, the least of which is a
    // boss top the cut frees: its annulus over its least height, the
    // boss top less the plane's highest point over the boss.
    let least_cap = crossed_bosses()
        .into_iter()
        .map(|(x, y)| {
            PI * (BOSS_R * BOSS_R - BORE_R * BORE_R)
                * (BOSS_Z.1 - plane_z(x, y) - rise_over_a_boss())
        })
        .fold(f64::INFINITY, f64::min);
    let gap = (v_above + v_below - v_box).abs();
    let vpad = vpad_above + vpad_below + vpad_box;
    assert!(
        vpad <= least_cap * 1e-2,
        "the volume brackets (± {vpad:.1e}) are too wide to see a freed boss top ({least_cap:.3e})"
    );
    assert!(
        gap <= vpad + 1e-12,
        "split halves must partition the volume (gap {gap:.3e}, brackets ± {vpad:.1e})"
    );
    let area = (s_above + s_below - s_box) / 2.0;
    let area_pad = (apad_above + apad_below + apad_box) / 2.0;
    // The section's area would miss a lost bore hole, πr² / cos φ.
    let hole = PI * BORE_R * BORE_R / cos_tilt();
    assert!(
        area_pad <= hole * 1e-2,
        "the area brackets (± {area_pad:.1e}) are too wide to see a bore hole ({hole:.3e})"
    );

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

/// What [`read_section`] measured, for the narration.
struct SectionReading {
    regions: usize,
    holes: usize,
    /// The regions' area, as `plane_section` reads it on their edges.
    area: f64,
}

/// `plane_section` of the box, checked against the closed forms and
/// against the area the split's halves enclose (`split_area`).
///
/// A wall or floor region is a polygon; a bored boss's region is the
/// boss's ellipse around the bore's, each two corners joined by two
/// arcs, and its area is checked against the closed form
/// `π (R² − r²) / cos φ`.
fn read_section(
    boxbody: &pncad::topo::Body<f64>,
    (split_area, split_pad): (f64, f64),
    tol: Tol,
) -> SectionReading {
    let section = plane_section(boxbody, &section_plane::<f64>(tol), tol)
        .expect("plane_section of the box through its bores");
    let annulus = PI * (BOSS_R * BOSS_R - BORE_R * BORE_R) / cos_tilt();
    let crossed = &crossed_bosses();
    let on_circle = |r: f64| {
        move |p: &pncad::geom_core::Point3<f64>| {
            crossed
                .iter()
                .any(|&(cx, cy)| ((p.x - cx).hypot(p.y - cy) - r).abs() < 1e-9)
        }
    };
    let mut area = 0.0;
    let mut holes = 0;
    for region in &section.regions {
        let corners = &region.outline.points;
        let enclosed = region.area();
        if region.holes.is_empty() {
            assert!(enclosed > 0.0, "a wall region winds counter-clockwise");
            assert!(
                !corners.iter().any(on_circle(BOSS_R)),
                "a wall region's corners lie off the bosses: {corners:?}"
            );
        } else {
            assert_eq!(region.holes.len(), 1, "a boss's region holds its one bore");
            let hole = &region.holes[0].points;
            assert!(
                corners.iter().all(on_circle(BOSS_R)) && hole.iter().all(on_circle(BORE_R)),
                "a bored boss's region is its boss around its bore: {corners:?}, {hole:?}"
            );
            assert!(
                (enclosed - annulus).abs() <= 1e-12,
                "a bored boss's region encloses {enclosed}, its annulus {annulus}"
            );
        }
        area += enclosed;
        holes += region.holes.len();
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
         through {through:?}, normal {normal:?} — tilted, no axis alignment — through \
         two bored bosses; each half keeps {faces} section faces, the two through the \
         bosses annuli ringed by their bores; `plane_section` reads {regions} regions \
         with {holes} holes, area {area:.9} (read on the regions' segments and arcs, each \
         bored boss's pi (R^2 - r^2) / cos phi) against {split:.9} \
         ± {pad:.1e} from the halves' surface areas; halves partition the volume \
         within their certified brackets ({v_above:.6} + {v_below:.6} = {v_box:.6}, \
         gap {gap:.1e}); halves then moved apart by rigid transforms (edge witnesses \
         re-minted, #84) and revalidated",
        through = THROUGH,
        normal = NORMAL,
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
            // shell classifier reads no circle edge (its first is a
            // bore's or a boss's arc).
            SceneBody::plain("cutaway_above", [0.40, 0.60, 0.72], above).step_at_frontier(
                |e| {
                    matches!(
                        e,
                        pncad::step_export::StepExportError::CurvedShellClassification { kind, .. }
                            if kind == "circle curve"
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
