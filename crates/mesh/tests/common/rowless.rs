//! **Pipe walls the chart lane meshes with no lattice node**, and the
//! count every such face holds: the bodies `rowless_cylinders`,
//! `d9_mesh_goldens` and `reach_probes` read.

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{brick, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

/// A disc of radius `r` about `(0, 0, z0)` extruded `depth` along `z`:
/// its wall is two half-turn faces between two rims of two half-circle
/// edges each, the seam lines at azimuths 0 and π.
pub fn rod(r: f64, z0: f64, depth: f64) -> Body<f64> {
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol()).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let profile = Profile::new(plane, vec![lp.into()])
        .validate(tol())
        .unwrap();
    let side = ExtrudeSide::Along;
    extrude(&profile, Extrusion::Distance { depth, side }, tol())
        .unwrap()
        .body
        .into_body()
}

/// `raw` moved by `pose`, finished.
pub fn posed(what: &str, raw: Body<f64>, pose: Affine3<f64>) -> AtRestBody<f64> {
    finished(
        what,
        topo::transform_rigid(&raw, &geom_core::test_support::rigid(&pose, tol())).unwrap(),
        tol(),
    )
}

/// `a ∖ b`, finished.
pub fn less(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    let cut = topo::subtract(a, b, tol()).unwrap();
    finished("the cut", Body::clone(&cut.body().unwrap().body), tol())
}

/// `a ∩ b`, finished.
pub fn meet(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    let cut = topo::intersect(a, b, tol()).unwrap();
    finished("the cut", Body::clone(&cut.body().unwrap().body), tol())
}

/// A bar `x × (−w, w) × z` turned `turn` about the `z` axis.
pub fn bar(x: (f64, f64), w: f64, z: (f64, f64), turn: f64) -> AtRestBody<f64> {
    posed(
        "the bar",
        brick(x, (-w, w), z, tol()),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
    )
}

/// The pipe of radius `r` over `z ∈ (z0, z0 + h)`, finished.
pub fn pipe(r: f64, z0: f64, h: f64) -> AtRestBody<f64> {
    finished("the pipe", rod(r, z0, h), tol())
}

/// The pipe of radius 1 over `z ∈ (−2, 2)` notched or holed by bars,
/// each named: a notch down from the top rim across the chart's seam
/// and one beside it, a narrow deep one and a wide shallow one, two
/// notches, a comb of six notches on one rim, a bar through (a notch
/// across the seam and a hole), and bars through of half-width `0.6`
/// and `0.9` of the radius.
pub fn walls() -> Vec<(&'static str, Body<f64>)> {
    let (r, z0, h) = (1.0, -2.0, 4.0);
    let pipe = pipe(r, z0, h);
    let top = z0 + h;
    let notch = |w: f64, depth: f64, turn: f64| {
        bar(
            (0.0, 1.1 * r),
            w * r,
            (top - depth * h, top + 0.5 * h),
            turn,
        )
    };
    let through = |w: f64| {
        bar(
            (-1.1 * r, 1.1 * r),
            w * r,
            (z0 + 0.4 * h, z0 + 0.6 * h),
            0.0,
        )
    };
    let mut comb = pipe.clone();
    for k in 0..6 {
        comb = less(&comb, &notch(0.1, 0.25, 0.4 + f64::from(k) * 0.9));
    }
    vec![
        (
            "a notch across the seam",
            less(&pipe, &notch(0.3, 0.5, 0.0)),
        ),
        (
            "a notch beside the seam",
            less(&pipe, &notch(0.3, 0.5, 1.0)),
        ),
        ("a narrow deep notch", less(&pipe, &notch(0.1, 0.875, 1.0))),
        ("a wide shallow notch", less(&pipe, &notch(0.6, 0.125, 1.0))),
        (
            "two notches",
            less(&less(&pipe, &notch(0.3, 0.5, 1.0)), &notch(0.3, 0.5, 2.5)),
        ),
        ("a comb of six notches", comb),
        ("a bar through", less(&pipe, &through(0.3))),
        ("a bar through, half-width 0.6", less(&pipe, &through(0.6))),
        ("a bar through, half-width 0.9", less(&pipe, &through(0.9))),
    ]
    .into_iter()
    .map(|(what, b)| (what, Body::clone(&b)))
    .collect()
}

/// The pipe of radius 1 over `z ∈ (−2, 2)` below a plane through its
/// middle tilted `tilt` about `y`.
pub fn tilted(tilt: f64) -> Body<f64> {
    let (r, z0, h) = (1.0, -2.0, 4.0);
    let below = posed(
        "the half space",
        brick(
            (-3.0 * r, 3.0 * r),
            (-3.0 * r, 3.0 * r),
            (z0 - h, z0 + 0.5 * h),
            tol(),
        ),
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, z0 + 0.5 * h),
            Vec3::new(0.0, 1.0, 0.0),
            tilt,
        ),
    );
    Body::clone(&meet(&pipe(r, z0, h), &below))
}

/// **Two small holes in a pipe wall, `gap` columns of free wall between
/// them**: the pipe of radius `r` over `z ∈ (−h/2, h/2)` less two thin
/// radial stubs at mid-height, `0.016·r` wide, cut one after the other,
/// centred half a column off a rim chord point (9.5 columns round). A
/// column is the rim's chord step on a chart-lane wall at `delta`: read
/// off the unit pipe with one such hole meshed at `delta / r`, so the
/// lane's own target sets it and the reading is the same at every scale.
/// The width is a fixed share of the radius, not of the column, so each
/// cut's decisions sit clear of the band at every ε the suite runs: a
/// hole `0.04` columns wide on the unit pipe at δ 10⁻³ pierces at a
/// margin of `1.58·10⁻⁶`, inside the band at ε 10⁻⁶, and the millimetre
/// pipe's probe hole at `−1.00·10⁻⁶`.
pub fn two_holes(r: f64, h: f64, gap: f64, delta: f64) -> Body<f64> {
    let d = 0.02 * h;
    let (hu, du) = (h / r, d / r);
    let probe = less(
        &pipe(1.0, -0.5 * hu, hu),
        &bar((0.5, 1.1), 0.01, (-du, du), 1.0),
    );
    let rims = mesh::tessellate(&probe, delta / r, tol()).unwrap();
    let n = rims
        .boundaries
        .iter()
        .map(|b| b.points.len())
        .max()
        .unwrap()
        - 1;
    #[allow(clippy::cast_precision_loss)]
    let column = core::f64::consts::PI / n as f64;
    let w = 0.008 * r;
    let stub = |turn: f64| bar((0.5 * r, 1.1 * r), w, (-d, d), turn);
    let (c, half) = (9.5 * column, 0.5 * gap * column + w / r);
    let one = less(&pipe(r, -0.5 * h, h), &stub(c - half));
    Body::clone(&less(&one, &stub(c + half)))
}

/// **What a face triangulated on its boundary points and `p` inserted
/// points holds** ([`count`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Count {
    /// The patch's triangles.
    pub triangles: usize,
    /// The boundary points the face's loops walk: each traversal of an
    /// edge its chord points but the last, so a seam the loop walks
    /// twice counts twice, as the trim polygon holds it. A pinch, or a
    /// vertex a loop meets again on one branch, is one vertex of the
    /// triangulation counted twice here, and [`Count::euler`] does not
    /// cover it; no body of the rows has one.
    pub n: usize,
    /// The face's inner loops.
    pub h: usize,
    /// The patch's points on no edge of the face: the points the lane
    /// inserted.
    pub p: usize,
}

impl Count {
    /// `n − 2 + 2h + 2p`: a constrained triangulation of a polygon of
    /// `n` vertices with `h` holes and `p` points inside has that many
    /// triangles (Euler's formula for the planar graph it makes: `V =
    /// n + p`, every inner face a triangle, `n` boundary edges, `h + 1`
    /// boundary cycles).
    pub fn euler(self) -> usize {
        self.n + 2 * self.h + 2 * self.p - 2
    }
}

/// Face `fk`'s [`Count`] in `mesh`.
pub fn count(body: &Body<f64>, mesh: &mesh::Mesh, fk: topo::FaceKey) -> Count {
    let mut on_edges = std::collections::HashSet::new();
    let (mut n, mut loops) = (0, 0);
    for (lk, _, _) in body.face(fk).unwrap().loops() {
        loops += 1;
        let (_, half_edges) =
            topo::props::loop_edges(body.cycle_loop(lk).expect("a cycle loop")).unwrap();
        for hek in half_edges {
            let edge = body.get_half_edge(hek).unwrap().edge;
            let line = mesh.boundaries.iter().find(|b| b.edge == edge).unwrap();
            n += line.points.len() - 1;
            on_edges.extend(line.points.iter().copied());
        }
    }
    let patch = mesh.patches.iter().find(|p| p.face == fk).unwrap();
    let inserted: std::collections::HashSet<u32> = patch
        .triangles
        .iter()
        .flatten()
        .copied()
        .filter(|i| !on_edges.contains(i))
        .collect();
    Count {
        triangles: patch.triangles.len(),
        n,
        h: loops - 1,
        p: inserted.len(),
    }
}
