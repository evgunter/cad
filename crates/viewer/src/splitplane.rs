//! **The split tool's plane, read against its target** — what the panel
//! seeds the plane with, and the outline the viewport draws of it while
//! the tool is open.
//!
//! The plane's numbers are in the coordinates of the space the target
//! is built in ([`crate::session::SessionOp::AddSplit`], read off by the
//! session's door). The viewport draws it where the person aimed: under
//! the copy their pick was made through ([`crate::seats::Seats::through`]),
//! whose drawn map carries the target's body into the picture.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Evaluation, Framed, Operand, ProfileProgram, in_space};
use pncad::geom_core::{Affine3, Point3, Vec3};

use crate::marks;

/// **The target's body as the landed evaluation holds it**: the body
/// `target` reads (a split's half at its port), `None` where it has no
/// value.
pub fn source<'a>(
    doc: &Doc<ProfileProgram>,
    evaluation: &'a Evaluation<f64>,
    target: &Operand,
) -> Option<&'a Framed<f64>> {
    let var = crate::world::read_var(doc, target)?;
    evaluation.read_body(var, 0).ok()?.ok()
}

/// **The target's corners' extent** in the coordinates of the space it
/// is built in: the least and greatest of its vertices on each axis.
/// `None` for a body with no vertex.
pub fn extent(source: &Framed<f64>) -> Option<(Point3<f64>, Point3<f64>)> {
    let into_space = in_space(source);
    let mut points = source
        .body()
        .vertex_points()
        .map(|(_, p)| match into_space {
            Some(map) => map.transform_point(p),
            None => p,
        });
    let first = points.next()?;
    Some(points.fold((first, first), |(lo, hi), p| {
        (
            Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z)),
            Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z)),
        )
    }))
}

/// **The default plane's point**: the centre of the target's extent, so
/// the plane the panel opens with passes through the body rather than
/// along a face of it.
pub fn centre(source: &Framed<f64>) -> Option<[f64; 3]> {
    let (lo, hi) = extent(source)?;
    Some([
        0.5 * (lo.x + hi.x),
        0.5 * (lo.y + hi.y),
        0.5 * (lo.z + hi.z),
    ])
}

/// **The map from the target's space into the picture**, for a target
/// drawn through a copy whose body is drawn under `drawn` (its body's
/// coordinates into the picture's): `drawn` after the target's own map
/// into its space, undone. `None` where that map has no inverse.
pub fn space_to_picture(source: &Framed<f64>, drawn: Affine3<f64>) -> Option<Affine3<f64>> {
    match in_space(source) {
        None => Some(drawn),
        Some(map) => Some(drawn * map.inverse().ok()?.to_affine()),
    }
}

/// **What the viewport draws of the split plane**: a square on the
/// plane, centred where the plane meets the line through the target's
/// centre along the normal, spanning the target, and a stroke from its
/// centre along the normal toward the half the split calls above. In
/// the picture's coordinates.
#[derive(Clone, Copy, Debug)]
pub struct PlanePreview {
    /// The square's corners, in order round it.
    pub corners: [Point3<f64>; 4],
    /// The square's centre.
    pub centre: Point3<f64>,
    /// Where the normal stroke ends.
    pub tip: Point3<f64>,
}

/// How far the square reaches past the target's extent, as a fraction
/// of that extent's diagonal on each side of its centre.
const REACH: f64 = 0.6;

/// [`PlanePreview`] of the plane through `origin` with normal `normal`
/// (the panel's numbers, in the target's space), against the target's
/// extent, mapped into the picture by `to_picture`. `None` for a normal
/// with no direction or a target with no extent.
pub fn preview(
    source: &Framed<f64>,
    to_picture: Affine3<f64>,
    origin: [f64; 3],
    normal: [f64; 3],
) -> Option<PlanePreview> {
    let (lo, hi) = extent(source)?;
    let n = Vec3::new(normal[0], normal[1], normal[2]);
    let length = n.norm();
    if !(length.is_finite() && length > 0.0) {
        return None;
    }
    let n = n * (1.0 / length);
    let origin = Point3::new(origin[0], origin[1], origin[2]);
    let middle = lo + (hi - lo) * 0.5;
    let on_plane = middle - n * (middle - origin).dot(n);
    let diagonal = (hi - lo).norm();
    // A flat target still spans a square on any plane through it.
    let half = REACH * diagonal.max(f64::MIN_POSITIVE);
    // Any direction off the normal, then the one across both.
    let off = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = n.cross(off).normalize() * half;
    let v = n.cross(u);
    let map = |p: Point3<f64>| to_picture.transform_point(p);
    Some(PlanePreview {
        corners: [
            map(on_plane + u + v),
            map(on_plane - u + v),
            map(on_plane - u - v),
            map(on_plane + u - v),
        ],
        centre: map(on_plane),
        tip: map(on_plane + n * (0.5 * half)),
    })
}

/// **What the picture draws a split plane against**: the split tool's
/// seats, the landed pair, the display view and the pick index, as the
/// session and the app answer them.
pub struct Drawing<'a> {
    /// The open split tool's seats.
    pub seats: &'a crate::seats::Seats,
    /// The landed document and its evaluation.
    pub landed: (&'a Doc<ProfileProgram>, &'a Evaluation<f64>),
    /// The display view the picture is drawn under.
    pub display: &'a crate::display::DisplayView,
    /// The pick index built for the landed run.
    pub index: &'a crate::pickindex::PickIndex,
}

/// **The open split tool's plane, where the picture draws its target**:
/// under the drawn part its target's viewport pick hit
/// ([`crate::seats::Seats::through`], [`crate::pickindex::PickIndex::drawn_map`]),
/// against the landed evaluation, for the panel's numbers `origin` and
/// `normal`. What the viewport draws ([`push_split_plane`]). `None` with
/// no target held, a tree pick, a part the picture does not draw, or a
/// normal with no direction.
pub fn drawn(drawing: &Drawing<'_>, origin: [f64; 3], normal: [f64; 3]) -> Option<PlanePreview> {
    let target = drawing.seats.read(0)?;
    let part = drawing.seats.through(0)?;
    let (doc, evaluation) = drawing.landed;
    let source = source(doc, evaluation, &target)?;
    let drawn = drawing.index.drawn_map(drawing.display, part)?;
    preview(source, space_to_picture(source, drawn)?, origin, normal)
}

/// **The split plane, appended to a lane**: the square's outline, its
/// two diagonals, and the normal stroke. It goes in the marks lane, so a
/// fit does not frame it.
pub fn push_split_plane(lane: &mut marks::LegLane, preview: &PlanePreview) {
    let [a, b, c, d] = preview.corners;
    for (from, to) in [(a, b), (b, c), (c, d), (d, a), (a, c), (b, d)] {
        lane.leg(from, to);
    }
    lane.leg(preview.centre, preview.tip);
}
