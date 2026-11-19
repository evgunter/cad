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

impl Reach {
    /// The region faces `faces` of `solid` occupy (module docs). `None`
    /// where a boundary carrier has no cover, or there is no face.
    pub(crate) fn of(solid: &SolidSpec, faces: impl IntoIterator<Item = usize>) -> Option<Self> {
        let mut all: Option<Cover<f64>> = None;
        for fi in faces {
            let face = &solid.faces[fi];
            let mut boundary: Option<Cover<f64>> = None;
            for lp in &face.loops {
                for u in &lp.uses {
                    let spec = solid.edges.get(&u.edge)?;
                    let piece = spec
                        .carrier
                        .cover_over(ParamWindow::new(spec.t0, spec.t1))
                        .ok()?;
                    boundary = Some(match boundary {
                        Some(c) => c.hull(piece),
                        None => piece,
                    });
                }
            }
            let region = face.surface.face_region(boundary?).ok()?;
            all = Some(match all {
                Some(c) => c.hull(region),
                None => region,
            });
        }
        all.map(Self)
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
        let (lo, hi) = self.along(line.dir);
        let at = |s: f64| line.at(s - line.dir.dot(line.origin - Point3::origin()));
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

    /// The region's diameter as a lever: an angle between two directions
    /// moves a point of the region by at most the angle times it.
    pub(crate) fn lever(&self) -> Lever<f64> {
        let corner = |pick: fn((f64, f64)) -> f64| {
            Point3::new(
                pick(self.along(Vec3::new(1.0, 0.0, 0.0))),
                pick(self.along(Vec3::new(0.0, 1.0, 0.0))),
                pick(self.along(Vec3::new(0.0, 0.0, 1.0))),
            )
        };
        geom_brep::points_diameter(&[corner(|(lo, _)| lo), corner(|(_, hi)| hi)])
    }
}
