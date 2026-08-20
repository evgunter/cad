//! Orientation oracles for the swept and lofted corpus: the wall-facing
//! probe, and the LEVEL-SET machinery it decides against.
//!
//! A wall's outward normal is `sense_sign · (S_u × S_v)`. Checking it
//! means asking, of a point just off the wall, which side the material
//! is on — and the answer has to come from somewhere that never reads a
//! `sense`, a winding or a normal, or the check is circular. Everything
//! here reads POSITIONS off the shipped charts and nothing else.
//!
//! # Level sets, and the two ways to index them
//!
//! A body skinned from one profile through a stack of placements has
//! planar cross-sections: at chart parameter `v` every wall's iso-curve
//! is `Σ_j B_j(v) · A_j` applied to the profile's control points, and
//! since the `B_j` are a partition of unity that sum is one AFFINE map
//! applied to a planar section. So the level ring at `v` is planar for
//! any placements at all, straight path or turning — [`level_ring`]
//! asserts it rather than resting on the derivation.
//!
//! What differs between bodies is how a query point is matched to its
//! level. Where the stack does not turn much, the level height above a
//! plane oriented against a fixed axis falls monotonically and one
//! bisection finds the level; that is the loft corpus's index, and its
//! preconditions are its own. Where the stack turns through a full
//! revolution both halves fail — see [`LevelIndex`], which indexes by
//! continuity instead and enumerates every level a point lies on.

#![allow(dead_code)]

use geom::{NurbsSurface, Surface};
use geom_core::{Point3, Vec3};
use sweep::Lofted;
use topo::boolean::SolidContainment;
use topo::{Body, FaceKey};

// ---------------------------------------------------------------------
// Reading the shipped charts
// ---------------------------------------------------------------------

/// A wall's skinned chart and the knot parameters that FRACTIONAL chart
/// coordinates name on it. Fractions rather than knot values, so walls
/// with different spans are sampled on the same level set of `v`.
pub fn chart_at(
    body: &Body<f64>,
    face: FaceKey,
    su: f64,
    sv: f64,
) -> (&NurbsSurface<f64>, f64, f64) {
    let f = body.get_face(face).expect("wall face resolves");
    let Some(Surface::Nurbs(s)) = body.get_surface(f.surface) else {
        panic!("a skinned wall carries a NURBS chart");
    };
    let (u0, u1) = s.knots_u().domain();
    let (v0, v1) = s.knots_v().domain();
    (s, u0 + (u1 - u0) * su, v0 + (v1 - v0) * sv)
}

/// One wall's POSITION at fractional chart coordinates.
///
/// Every oracle in this module reads this and never [`wall_outward_at`],
/// which is what makes their independence from the datum under test
/// structural rather than conventional: they have no access to a
/// `sense`, a winding or a normal to be circular with. It also keeps
/// `normalize()` — poison on a degenerate jet — out of a path that only
/// ever wants `S(u, v)`.
pub fn wall_point_at(body: &Body<f64>, face: FaceKey, su: f64, sv: f64) -> Point3<f64> {
    let (s, u, v) = chart_at(body, face, su, sv);
    s.eval(u, v)
}

/// One wall's point and the OUTWARD normal it claims: the shipped
/// surface's `S_u × S_v` with the stored `sense` folded in.
pub fn wall_outward_at(
    body: &Body<f64>,
    face: FaceKey,
    su: f64,
    sv: f64,
) -> (Point3<f64>, Vec3<f64>) {
    let f = body.get_face(face).expect("wall face resolves");
    let (s, u, v) = chart_at(body, face, su, sv);
    let jet = s.ders(u, v);
    (
        s.eval(u, v),
        jet.du.cross(jet.dv).normalize() * f.sense_sign::<f64>(),
    )
}

/// One wall's mid-chart point and outward normal.
pub fn wall_outward(body: &Body<f64>, face: FaceKey) -> (Point3<f64>, Vec3<f64>) {
    wall_outward_at(body, face, 0.5, 0.5)
}

/// The chord from the bottom level to the top one — the stacking
/// direction, where the stack has one.
pub fn stack_axis(lofted: &Lofted<f64>) -> Vec3<f64> {
    let first = lofted.side_faces[0][0];
    wall_point_at(&lofted.body, first, 0.5, 1.0) - wall_point_at(&lofted.body, first, 0.5, 0.0)
}

// ---------------------------------------------------------------------
// The probe
// ---------------------------------------------------------------------

/// A MATERIAL-SIDE oracle: where a query point lies relative to the
/// solid, decided without reading any face's `sense`.
pub type Oracle<'a> = dyn Fn(Point3<f64>) -> SolidContainment + 'a;

/// The chart grid the turning fixtures are probed on: three columns
/// across `u` at five levels along `v`. Mid-chart alone cannot see a
/// twist — it is one level set of a chart that has many. The columns
/// avoid `u = 0` and `u = 1`, where a probe would sit on a profile
/// CORNER and its inward step would land on the neighbouring wall.
pub fn along_v() -> Vec<(f64, f64)> {
    let vs = [0.05, 0.25, 0.5, 0.75, 0.95];
    [0.25, 0.5, 0.75]
        .into_iter()
        .flat_map(|u| vs.map(|v| (u, v)))
        .collect()
}

/// Step `delta` off a wall both ways along its claimed outward normal
/// and ask the oracle. Returns `(inward_side, outward_side)` — a
/// flipped `sense` swaps them, which is what makes the pair two-sided.
pub fn probe_sides(
    oracle: &Oracle<'_>,
    p: Point3<f64>,
    outward: Vec3<f64>,
    delta: f64,
) -> (SolidContainment, SolidContainment) {
    (oracle(p - outward * delta), oracle(p + outward * delta))
}

/// The orientation claim, wall by wall and sample by sample. The
/// ORIENTATION assertion runs first and is what a wrong bit trips; the
/// `sense = true` value pin follows it, so a production flip reports
/// the material-side failure rather than a stored-value mismatch.
///
/// Returns the number of (wall, sample) probes run.
pub fn assert_walls_face_out(
    lofted: &Lofted<f64>,
    oracle: &Oracle<'_>,
    samples: &[(f64, f64)],
    delta: f64,
    expect: usize,
) -> usize {
    let mut n = 0;
    let mut probes = 0;
    for (li, walls) in lofted.side_faces.iter().enumerate() {
        for (si, &fk) in walls.iter().enumerate() {
            for &(su, sv) in samples {
                let (p, outward) = wall_outward_at(&lofted.body, fk, su, sv);
                let (inward_side, outward_side) = probe_sides(oracle, p, outward, delta);
                assert_eq!(
                    (inward_side, outward_side),
                    (SolidContainment::In, SolidContainment::Out),
                    "loop {li} segment {si} at chart ({su}, {sv}) = {p:?}: the wall \
                     claims {outward:?} is outward, so the oracle must read material \
                     against it and void along it"
                );
                probes += 1;
            }
            assert!(
                lofted.body.get_face(fk).unwrap().sense,
                "loop {li} segment {si}: the honest orientation above is the one \
                 the skinned chart encodes as sense = true"
            );
            n += 1;
        }
    }
    assert_eq!(n, expect, "every wall of the fixture was probed");
    probes
}

// ---------------------------------------------------------------------
// Level sets
// ---------------------------------------------------------------------

/// Samples per wall on a level-set polyline. The worst-curved wall in
/// this corpus is the holed plate's hole (radius 1, half a turn per
/// wall): 64 chords put the polyline ≈ 3.3e-4 off it, and every
/// fixture's actual chord error is MEASURED against a fixed bound in
/// [`level_ring`] rather than argued at the call site.
pub const LEVEL_SAMPLES: usize = 64;

/// The plane of the level ring at `v`-fraction `t`, normal UNORIENTED:
/// a point on it and a unit normal whose sign follows the ring's
/// traversal. Read off one mid-`u` sample per wall of the first loop —
/// the ring is planar ([`level_ring`] asserts it on the full sample),
/// so the wall midpoints fix its plane.
///
/// Newell over the whole ring, rather than three picked points, so no
/// one near-collinear sample can decide the normal. Hand-rolled rather
/// than `geom_brep::newell_plane`: that door certifies planarity
/// against the running ε, and an oracle whose validity moves with ε is
/// the trap #619 fell into. The planarity precondition here is a fixed
/// geometric bound.
pub fn level_ring_plane(lofted: &Lofted<f64>, t: f64) -> (Point3<f64>, Vec3<f64>) {
    let walls = &lofted.side_faces[0];
    let ring: Vec<Point3<f64>> = walls
        .iter()
        .map(|&fk| wall_point_at(&lofted.body, fk, 0.5, t))
        .collect();
    let mut n = Vec3::new(0.0, 0.0, 0.0);
    for i in 0..ring.len() {
        let (a, b) = (ring[i], ring[(i + 1) % ring.len()]);
        n = n + Vec3::new(
            (a.y - b.y) * (a.z + b.z),
            (a.z - b.z) * (a.x + b.x),
            (a.x - b.x) * (a.y + b.y),
        );
    }
    (ring[0], n.normalize())
}

/// The body's own level set at `v`-fraction `t`, against a plane the
/// caller's index has already oriented: one closed polyline per loop,
/// walls in traversal order, sampled off the shipped charts.
///
/// Two PRECONDITIONS, asserted rather than assumed, because the ring is
/// worthless without them:
///
/// - the level set is **planar**, or the polyline is not a
///   cross-section of anything (the module header derives why it is,
///   for any placements; this measures it);
/// - the polyline's **chord error** stays far under the probe steps,
///   or parity answers about the polyline stop being answers about the
///   solid. Estimated per wall from consecutive triples: on a circle
///   of radius `R` sampled at angular step `θ` the middle sample sits
///   `R(1 − cos θ)` off its neighbours' chord, four times the chord's
///   own sagitta `R(1 − cos θ/2)`. Wall JUNCTIONS are excluded — a
///   profile corner is a real corner, not sampling error.
pub fn level_ring(
    lofted: &Lofted<f64>,
    plane: (Point3<f64>, Vec3<f64>),
    t: f64,
) -> Vec<Vec<Point3<f64>>> {
    let (origin, n) = plane;
    let (mut off_plane, mut chord_err) = (0.0_f64, 0.0_f64);
    let mut loops = Vec::with_capacity(lofted.side_faces.len());
    for walls in &lofted.side_faces {
        let mut poly = Vec::with_capacity(walls.len() * LEVEL_SAMPLES);
        for &fk in walls {
            let run = poly.len();
            for i in 0..LEVEL_SAMPLES {
                #[allow(clippy::cast_precision_loss)]
                let su = i as f64 / LEVEL_SAMPLES as f64;
                let p = wall_point_at(&lofted.body, fk, su, t);
                off_plane = off_plane.max((p - origin).dot(n).abs());
                poly.push(p);
            }
            for i in run + 1..poly.len() - 1 {
                let mid = poly[i - 1] + (poly[i + 1] - poly[i - 1]) * 0.5;
                chord_err = chord_err.max((poly[i] - mid).norm() * 0.25);
            }
        }
        loops.push(poly);
    }
    assert!(
        off_plane < 1e-9,
        "the level set at v-fraction {t} must be planar for its polyline to be \
         a cross-section: off-plane by {off_plane}"
    );
    assert!(
        chord_err < 1e-3,
        "the level polyline at v-fraction {t} must track its iso-curves far \
         inside the probe steps (smallest 0.02): chord error {chord_err}"
    );
    loops
}

/// Crossing parity against a level set, in the level plane's own
/// frame: the material side of a closed planar curve, which is defined
/// without any orientation at all — a hole loop needs no special case,
/// its crossings simply cancel the plate's.
///
/// **A DECLARED COPY** (report §S17; this is the outermost of the
/// ray-parity implementations it tracks). Reuse is blocked in every
/// direction: `topo::splitting::containment::point_in_loop` is public
/// but takes `(&Body, LoopKey)` and walks the B-rep, so it cannot
/// consume a sampled polyline; `chart_region::point_in_polygon`,
/// `profile::validate::point_in_loop` and the shared walk the first
/// two now share (`topo::ray_parity::ray_verdict`, `pub(crate)`) are
/// private to their crates; and any `Decide`-certified door — which
/// the shared walk is — would tie this oracle's validity to the
/// running ε, which is the fragility #619 was faulted for.
pub fn level_set_contains(
    loops: &[Vec<Point3<f64>>],
    plane: (Point3<f64>, Vec3<f64>),
    q: Point3<f64>,
) -> bool {
    let (origin, n) = plane;
    let (e1, e2) = n.orthonormal_basis();
    let uv = |p: Point3<f64>| ((p - origin).dot(e1), (p - origin).dot(e2));
    let (qx, qy) = uv(q);
    let mut crossings = 0usize;
    for poly in loops {
        for i in 0..poly.len() {
            let (a, b) = (uv(poly[i]), uv(poly[(i + 1) % poly.len()]));
            if (a.1 > qy) != (b.1 > qy) && a.0 + (qy - a.1) / (b.1 - a.1) * (b.0 - a.0) > qx {
                crossings += 1;
            }
        }
    }
    crossings % 2 == 1
}

// ---------------------------------------------------------------------
// The index for a stack that TURNS
// ---------------------------------------------------------------------

/// Level planes sampled along `v`. Fine enough that a full revolution
/// of frame roll advances the ring normal by ~0.7° per step, which is
/// what makes the continuity chain below unambiguous; the ROOT
/// resolution it also has to buy is not argued from this number but
/// measured, by re-counting on the half-density subsample.
pub const LEVELS: usize = 512;

/// How closely consecutive level normals must agree for the continuity
/// chain to mean anything. This is a LOCAL condition and it is the
/// whole point: it says nothing about where the stack ends up.
const CONTINUITY_COS: f64 = 0.9;

/// A body's level planes, indexed by CONTINUITY along the stack — the
/// oracle for a swept body whose path turns through a full revolution.
///
/// # Why the loft corpus's index cannot be widened to reach here
///
/// That index orients every level plane against one fixed chord
/// ([`stack_axis`]) and then finds a query point's level by bisecting a
/// height it requires to be monotone. On a helix BOTH halves fail, and
/// they fail for different reasons — which is why this is a different
/// oracle and not a looser bound.
///
/// **There is no fixed axis to orient against.** The level planes are
/// the path's normal planes, so their normals sweep a cone of half
/// angle `atan(R/k)` about the helix axis while the chord from the
/// first section to the last is nearly the axis itself. For the corpus
/// helix (`R = 1`, `k = pitch/2π = 0.0637`) the cosine is `k/√(R²+k²) =
/// 0.0635` at EVERY level of a whole-turn sweep, and `0.011` at both
/// ends of a half turn. A guard at `0.1` refuses; lowering the guard
/// does not help, because a near-perpendicular Newell normal cannot be
/// oriented against that chord reliably at all.
///
/// **The level is not unique.** For the exact helix `c(a) = (R cos a,
/// R sin a, k a)` and a query point `q = (R, 0, z₀)` on the starting
/// section's radius, the height against the normal plane at `a` is
///
/// ```text
/// h(a) · √(R² + k²) = −R² sin a − k² a + k z₀,
/// ```
///
/// so `dh/da ∝ −R² cos a − k²`, which is POSITIVE for `a` in roughly
/// the second and third quadrants. The height stops being monotone
/// after about a quarter turn and a query point genuinely lies on
/// several level planes at once — 2 of them on the whole-turn fixture,
/// between 3 and 9 on the two-turn one. Bisecting would return
/// whichever root it landed on, and every spurious root satisfies
/// `h ≈ 0`, so no post-condition could tell them apart.
///
/// # What replaces it
///
/// Orient by CONTINUITY from `t = 0` — each level normal is flipped to
/// agree with its predecessor, and the agreement is asserted — so no
/// global direction is needed at all. Then, instead of assuming one
/// root, ENUMERATE them: `q` is in the body iff `q` lies inside the
/// level ring of some level, and `q` can only lie inside a ring whose
/// plane holds `q`, so the roots of `h` are exactly the candidate
/// levels. Test the ring at each; the body contains `q` iff one of them
/// claims it.
///
/// This is exactly the loft index where that one is valid — a monotone
/// height has one root — and it is defined where that one is not.
///
/// # Preconditions, all asserted
///
/// - consecutive level normals agree within [`CONTINUITY_COS`], or the
///   chain that orients them is arbitrary (in [`LevelIndex::build`]);
/// - the root count does not change when the sample density is halved,
///   or the enumeration is missing roots and the answer is a scan
///   artefact (in [`LevelIndex::contains`]);
/// - at most one level's ring claims the point, or the body overlaps
///   itself there and containment is not a function of position (in
///   [`LevelIndex::contains`]);
/// - each ring is planar and tracks its iso-curves (in [`level_ring`]).
///
/// Every one of them fires rather than lies.
pub struct LevelIndex<'a> {
    lofted: &'a Lofted<f64>,
    /// `(point, unit normal)` at `t = i / LEVELS`, normals oriented by
    /// continuity from `t = 0`.
    planes: Vec<(Point3<f64>, Vec3<f64>)>,
}

impl<'a> LevelIndex<'a> {
    /// Sample and continuity-orient the level planes.
    pub fn build(lofted: &'a Lofted<f64>) -> Self {
        let mut planes: Vec<(Point3<f64>, Vec3<f64>)> = Vec::with_capacity(LEVELS + 1);
        for i in 0..=LEVELS {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / LEVELS as f64;
            let (p, n) = level_ring_plane(lofted, t);
            let n = match planes.last() {
                None => n,
                Some(&(_, prev)) => {
                    let n = if n.dot(prev) < 0.0 { n * -1.0 } else { n };
                    assert!(
                        n.dot(prev) > CONTINUITY_COS,
                        "level planes {} and {i} of {LEVELS} turn by more than the \
                         continuity chain can follow: cos = {} — the sampling is too \
                         coarse for this stack, so which way each plane points is a \
                         guess",
                        i - 1,
                        n.dot(prev)
                    );
                    n
                }
            };
            planes.push((p, n));
        }
        Self { lofted, planes }
    }

    /// The continuity-oriented level planes, `t = i / LEVELS`.
    pub fn planes(&self) -> &[(Point3<f64>, Vec3<f64>)] {
        &self.planes
    }

    /// The level plane at an arbitrary `t`, oriented to agree with
    /// `reference` — used inside a bracket, where the two are one
    /// sample apart and the choice is not close.
    fn plane_at(&self, t: f64, reference: Vec3<f64>) -> (Point3<f64>, Vec3<f64>) {
        let (p, n) = level_ring_plane(self.lofted, t);
        let n = if n.dot(reference) < 0.0 { n * -1.0 } else { n };
        assert!(
            n.dot(reference) > CONTINUITY_COS,
            "the level plane at v-fraction {t} must agree with the bracketing \
             sample it is oriented from: cos = {}",
            n.dot(reference)
        );
        (p, n)
    }

    /// Is `q` inside the stacked solid? Every level whose plane holds
    /// `q`, tested for ring containment.
    pub fn contains(&self, q: Point3<f64>) -> SolidContainment {
        let h: Vec<f64> = self.planes.iter().map(|&(p, n)| (q - p).dot(n)).collect();
        let crossings = |step: usize| {
            (0..LEVELS / step)
                .filter(|i| (h[i * step] > 0.0) != (h[(i + 1) * step] > 0.0))
                .count()
        };
        let (fine, coarse) = (crossings(1), crossings(2));
        assert_eq!(
            fine, coarse,
            "the level-height roots for {q:?} must be resolved by the sampling: \
             {fine} at {LEVELS} levels but {coarse} at half that — the enumeration \
             below would be a scan artefact, not the body"
        );

        let mut claims = 0usize;
        for i in 0..LEVELS {
            if (h[i] > 0.0) == (h[i + 1] > 0.0) {
                continue;
            }
            #[allow(clippy::cast_precision_loss)]
            let (mut lo, mut hi) = (i as f64 / LEVELS as f64, (i + 1) as f64 / LEVELS as f64);
            let above = h[i] > 0.0;
            let reference = self.planes[i].1;
            for _ in 0..40 {
                let mid = (lo + hi) * 0.5;
                let (p, n) = self.plane_at(mid, reference);
                if ((q - p).dot(n) > 0.0) == above {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            let t = (lo + hi) * 0.5;
            let plane = self.plane_at(t, reference);
            if level_set_contains(&level_ring(self.lofted, plane, t), plane, q) {
                claims += 1;
            }
        }
        assert!(
            claims <= 1,
            "{claims} level rings claim {q:?} — the body overlaps itself there and \
             containment is not a function of position, so no verdict here is one \
             about the solid"
        );
        if claims == 1 {
            SolidContainment::In
        } else {
            SolidContainment::Out
        }
    }
}
