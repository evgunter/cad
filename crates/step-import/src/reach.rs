//! **The region a file's faces occupy, before anything is built**
//! (`[one-way.places]`): the one instrument the interpretation layer's
//! readings over faces take their extent from ([`crate::one_record`],
//! [`crate::snap`]).
//!
//! A face's region is read off its edge carriers' covers over their spans
//! ([`geom::Curve3::cover_over`]) by its surface's own reading of a face
//! bounded so ([`geom::Surface::face_region`]): a bound, never narrower
//! than the face. A carrier with no cover leaves the faces with no region,
//! and the reading that asked is not taken.

use geom::cover::{Cover, ParamWindow, Site};
use geom_core::{Lever, Point3, Vec3};

use crate::entities::SolidSpec;

/// The region the faces occupy, as one cover.
pub(crate) struct Reach(Cover<f64>);

/// Why faces have no region ([`Reach::read`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum NoRegion {
    /// No face was named.
    NoFace,
    /// This `EDGE_CURVE` bounds one of the faces and its carrier has no
    /// cover over its interval, or the solid holds no such edge.
    Edge(u64),
    /// This `ADVANCED_FACE` has no bound, or its surface reads no region
    /// of its boundary.
    Face(u64),
}

/// The boundary edges of faces `faces` of `solid`, each use once per
/// loop it stands in; `Err` names an edge the solid does not hold.
fn boundary<'a>(
    solid: &'a SolidSpec,
    faces: &[usize],
) -> Result<Vec<&'a crate::entities::EdgeSpec>, NoRegion> {
    faces
        .iter()
        .flat_map(|&fi| &solid.faces[fi].loops)
        .flat_map(|lp| &lp.uses)
        .map(|u| solid.edges.get(&u.edge).ok_or(NoRegion::Edge(u.edge)))
        .collect()
}

/// **How far faces `faces` of `solid` run, from both sides**: the lever
/// of a reading consumed over the faces, for a verdict that acts on its
/// definite sign (`below`) and for one that acts on its Zero (`above`).
///
/// From below, the faces' boundary edges' own extent
/// ([`geom_brep::edges_extent_inf`]): every point it is read between is
/// a point of a boundary, so the measure never over-states the faces'
/// boundaries, the unsafe direction for a lever whose definite sign
/// builds. `topo::wedge_reach` reads a built edge's two faces the same
/// way, off their vertices. From above, their region's diameter
/// ([`Reach::lever`]), which holds a face's bulge past its boundary.
///
/// # Errors
///
/// [`NoRegion`], naming the entity whose region does not read.
pub(crate) fn faces_run(
    solid: &SolidSpec,
    faces: &[usize],
) -> Result<geom_brep::LeverBounds<f64>, NoRegion> {
    let above = Reach::read(solid, faces.iter().copied())?.lever();
    let below = geom_brep::edges_extent_inf(
        boundary(solid, faces)?
            .into_iter()
            .map(|spec| (&spec.carrier, (spec.t0, spec.t1))),
    );
    Ok(geom_brep::LeverBounds::Between { below, above })
}

impl Reach {
    /// The region faces `faces` of `solid` occupy (module docs). `None`
    /// where a boundary carrier has no cover, or there is no face.
    pub(crate) fn of(solid: &SolidSpec, faces: impl IntoIterator<Item = usize>) -> Option<Self> {
        Self::read(solid, faces).ok()
    }

    /// [`Self::of`], saying which entity's region does not read.
    ///
    /// # Errors
    ///
    /// [`NoRegion`].
    pub(crate) fn read(
        solid: &SolidSpec,
        faces: impl IntoIterator<Item = usize>,
    ) -> Result<Self, NoRegion> {
        let mut all: Option<Cover<f64>> = None;
        for fi in faces {
            let face = &solid.faces[fi];
            let mut boundary: Option<Cover<f64>> = None;
            for lp in &face.loops {
                for u in &lp.uses {
                    let unread = NoRegion::Edge(u.edge);
                    let spec = solid.edges.get(&u.edge).ok_or(unread)?;
                    let piece = spec
                        .carrier
                        .cover_over(ParamWindow::new(spec.t0, spec.t1))
                        .map_err(|_| unread)?;
                    boundary = Some(match boundary {
                        Some(c) => c.hull(piece),
                        None => piece,
                    });
                }
            }
            let region = boundary
                .and_then(|b| solid.surface_of(face).face_region(b).ok())
                .ok_or(NoRegion::Face(face.id))?;
            all = Some(match all {
                Some(c) => c.hull(region),
                None => region,
            });
        }
        all.map(Self).ok_or(NoRegion::NoFace)
    }

    /// The farthest a point of the region lies from `at`.
    pub(crate) fn from(&self, at: Point3<f64>) -> f64 {
        self.0.distance_range(&Site::Point(at)).1
    }

    /// The least and greatest `q · dir` over the region's points `q`.
    pub(crate) fn along(&self, dir: Vec3<f64>) -> (f64, f64) {
        self.0.range_along(dir)
    }

    /// The two points of `line` (its direction unit) at the ends of the
    /// region's extent along it.
    pub(crate) fn ends_on(&self, line: geom::Line<f64>) -> [Point3<f64>; 2] {
        let (lo, hi) = self.along(line.dir.get());
        let at = |s: f64| line.at(s - line.dir.get().dot(line.origin - Point3::origin()));
        [at(lo), at(hi)]
    }

    /// **How far apart two axes stand where the region is**: the farthest
    /// `on` (its direction unit) lies from `from` at the two ends of the
    /// region's extent along `on`. A point's distance from a line is
    /// convex along another line, so the ends bound it between them.
    pub(crate) fn axes_apart(&self, from: geom::Line<f64>, on: geom::Line<f64>) -> f64 {
        self.ends_on(on)
            .into_iter()
            .map(|q| from.distance(q))
            .fold(0.0, f64::max)
    }

    /// The middle of the region's box in the world axes.
    pub(crate) fn centre(&self) -> Point3<f64> {
        let mid = |dir: Vec3<f64>| {
            let (lo, hi) = self.along(dir);
            geom::mid_param(lo, hi)
        };
        Point3::new(
            mid(Vec3::new(1.0, 0.0, 0.0)),
            mid(Vec3::new(0.0, 1.0, 0.0)),
            mid(Vec3::new(0.0, 0.0, 1.0)),
        )
    }

    /// The region's diameter as a lever, from above
    /// ([`geom_brep::cover_diameter`]): an angle between two directions
    /// moves a point of the region by at most the angle times it.
    pub(crate) fn lever(&self) -> Lever<f64> {
        geom_brep::cover_diameter(&self.0)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **A face that bulges past its boundary is read to its bulge.** A
    /// cap of the unit sphere bounded by its circle at `z = 0.9` reaches
    /// the pole at `z = 1`, which its boundary's hull does not. Its run
    /// ([`faces_run`]) is the rim's width from below and no less than
    /// the sphere's from above: red against one bound returned for both.
    #[test]
    fn a_sphere_cap_is_read_to_its_pole() {
        let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
        let x = geom_core::Vec3::new(1.0, 0.0, 0.0);
        let rim = crate::entities::EdgeSpec {
            start: 1,
            end: 1,
            carrier: geom::Curve3::circle(Point3::new(0.0, 0.0, 0.9), z, 0.19_f64.sqrt(), x),
            t0: 0.0,
            t1: std::f64::consts::TAU,
            reversed: false,
            curve: None,
        };
        let solid = SolidSpec {
            id: 1,
            faces: vec![crate::entities::FaceSpec {
                id: 2,
                record: 900,
                sense: true,
                loops: vec![crate::entities::LoopSpec {
                    outer: true,
                    uses: vec![crate::entities::EdgeUse {
                        edge: 10,
                        forward: true,
                    }],
                }],
                band: false,
            }],
            edges: [(10, rim)].into_iter().collect(),
            vertices: [(1, Point3::new(0.19_f64.sqrt(), 0.0, 0.9))]
                .into_iter()
                .collect(),
            surfaces: crate::entities::records_of([(
                900,
                geom::Surface::Sphere {
                    frame: geom::Frame::new(Point3::origin(), z, x),
                    radius: 1.0,
                },
            )]),
            band_seams: std::collections::BTreeSet::new(),
        };
        let reach = Reach::of(&solid, [0]).expect("the cap has a region");
        let (_, top) = reach.along(z);
        assert!(top >= 1.0 - 1e-12, "the pole is in the cap's region: {top}");
        let run = faces_run(&solid, &[0]).unwrap();
        let rim = 2.0 * 0.19_f64.sqrt();
        let (below, above) = (run.below().length(), run.above().length());
        assert!(
            0.9 * rim <= below && below <= rim * (1.0 + 1e-12),
            "from below, the rim's own {rim} m across: {below}"
        );
        assert!(
            above >= 2.0,
            "from above, the sphere's 2 m: the rim bounds its larger cap too: {above}"
        );
    }
}
