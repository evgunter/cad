//! **In-op exhaustiveness** — the never-silence obligation, mechanized
//! (M5 PR 7 spec §4, C3).
//!
//! Marching finds what it is seeded onto. The banked principle names
//! the classic silent disaster — a small loop nobody seeded and nobody
//! missed *loudly* — so found-ness here is never allowed to depend on
//! luck. The bounded domain is subdivided and **every** cell must
//! finish in one of three states:
//!
//! 1. **excluded** — a certified enclosure of `f₁` (or `f₂`, or the
//!    chart form `φ`) over the cell does not contain zero, so no
//!    solution can be in it;
//! 2. **accounted** — the cell lies inside a found branch's uniqueness
//!    tube, where limb 3 already proved there is exactly one arc;
//! 3. **refine** — split and recurse.
//!
//! At the named floor [`SSI_FLOOR`]·ε a cell that is still in state 3
//! ends the operation with the typed
//! `SsiExhaustivenessInconclusive` refusal. "Every branch found" is
//! then a theorem about enclosures, or it is a typed failure. It is
//! never silence.
//!
//! # The subdivision is also the seed generator
//!
//! The same recursion, run with an empty tube set and the coarser
//! [`SSI_SEED_FLOOR`]·extent floor, returns the cells that survived
//! exclusion. Their centers are the marcher's seeds. A branch that
//! touches no domain boundary — the interior small loop that boundary
//! seeding provably cannot reach — is found here or the operation
//! refuses; there is no third outcome.
//!
//! # Brute force, deliberately, for now
//!
//! Cells are enumerated by recursive bisection on the widest axis with
//! a fixed tie-break (D9). PR 8's BVH swaps in under its already-merged
//! differential suite when profiling asks for it; the plan explicitly
//! permits brute force here and nothing in the contract changes when
//! the pruning does.

use geom_core::{Point3, RingInterval, Vec3};
use geom_surfaces::{NurbsSurface, Surface};

use super::SsiError;
use super::enclose::{Box3, NurbsBoxes, implicit_enclosure};

/// The refinement floor, as a multiple of ε: a cell narrower than this
/// is not split again. Fixed and named (C3: "a named constant tied to
/// ε").
pub const SSI_FLOOR: f64 = 1.0;

/// The seed-generation floor, as a fraction of the caller's named
/// **extent** (not of ε — see `SsiDomain::seed_floor`): a seed only has
/// to land in Newton's basin, whose size is a property of the geometry.
pub const SSI_SEED_FLOOR: f64 = 1.0 / 64.0;

/// The cell budget. Exceeding it is a typed refusal, never a silent
/// truncation of the search.
pub const SSI_MAX_CELLS: usize = 200_000;

/// What the subdivision proved about the domain.
#[derive(Clone, Copy, Debug, Default)]
pub struct Exhaustiveness {
    /// Cells examined in total.
    pub examined: u32,
    /// Cells proved solution-free by enclosure.
    pub excluded: u32,
    /// Cells proved to lie inside a found branch's uniqueness tube.
    pub accounted: u32,
    /// Cells that were neither, and were split — the interior nodes of
    /// the subdivision tree. Reported so the receipt adds up:
    /// `examined == excluded + accounted + refined`, with every LEAF in
    /// one of the first two. That identity is the theorem.
    pub refined: u32,
    /// The deepest recursion reached.
    pub max_depth: u32,
    /// The floor used, in meters.
    pub floor: f64,
}

/// Subdivide a 3-D session-box slab against an analytic pair.
///
/// With `tubes` empty this is **seed generation**: the returned points
/// are the centers of the cells that survived exclusion. With `tubes`
/// populated it is **accounting**: any surviving cell at the floor is
/// the typed refusal.
///
/// Iterative (an explicit stack, so recursion depth is not a stack-
/// overflow path) and depth-first with the two halves pushed in a fixed
/// order — same cells, same order, every run (D9).
///
/// # Errors
///
/// [`SsiError::ExhaustivenessInconclusive`] at the floor,
/// [`SsiError::CellBudget`] if the enumeration exceeds
/// [`SSI_MAX_CELLS`], [`SsiError::UnsupportedCertificate`] when a
/// surface kind has no ring-computable enclosure.
pub(crate) fn sweep_r3(
    s1: &Surface<f64>,
    s2: &Surface<f64>,
    root: Box3,
    tubes: &[Box3],
    floor: f64,
) -> Result<(Exhaustiveness, Vec<Point3<f64>>), SsiError> {
    let mut stats = Exhaustiveness {
        floor,
        ..Exhaustiveness::default()
    };
    let mut out = Vec::new();
    let mut stack = vec![(root, 0u32)];
    while let Some((cell, depth)) = stack.pop() {
        if stats.examined as usize >= SSI_MAX_CELLS {
            return Err(SsiError::CellBudget {
                budget: SSI_MAX_CELLS,
            });
        }
        stats.examined += 1;
        stats.max_depth = stats.max_depth.max(depth);

        let e1 = implicit_enclosure(s1, cell);
        let e2 = implicit_enclosure(s2, cell);
        if e1.is_poison() || e2.is_poison() {
            return Err(SsiError::UnsupportedCertificate {
                what: "this surface kind has no ring-computable implicit \
                       enclosure, so its domain cannot be proved exhausted \
                       (per-arm retirement, C12.1)",
            });
        }
        // (i) exclusion.
        if excludes_zero(e1) || excludes_zero(e2) {
            stats.excluded += 1;
            continue;
        }
        // (ii) accounted: inside a found branch's uniqueness tube.
        if tubes.iter().any(|t| cell.contained_in(*t)) {
            stats.accounted += 1;
            continue;
        }
        // (iii) refine, unless we are at the floor.
        if cell.width() <= floor {
            if tubes.is_empty() {
                out.push(cell.center());
                continue;
            }
            return Err(SsiError::ExhaustivenessInconclusive {
                cell_width: cell.width(),
                floor,
                examined: stats.examined,
            });
        }
        stats.refined += 1;
        let (a, b) = cell.split();
        stack.push((b, depth + 1));
        stack.push((a, depth + 1));
    }
    Ok((stats, out))
}

fn excludes_zero(i: RingInterval) -> bool {
    !i.is_poison() && (i.lo() > 0.0 || i.hi() < 0.0)
}

/// A rectangle in a surface's parameter domain — the chart lane's cell.
#[derive(Clone, Copy, Debug)]
pub(crate) struct UvRect {
    /// `u` bounds.
    pub u: (f64, f64),
    /// `v` bounds.
    pub v: (f64, f64),
}

impl UvRect {
    fn width(self) -> f64 {
        (self.u.1 - self.u.0).max(self.v.1 - self.v.0)
    }

    fn center(self) -> (f64, f64) {
        (0.5 * (self.u.0 + self.u.1), 0.5 * (self.v.0 + self.v.1))
    }

    fn split(self) -> (Self, Self) {
        if (self.u.1 - self.u.0) >= (self.v.1 - self.v.0) {
            let m = 0.5 * (self.u.0 + self.u.1);
            (
                Self {
                    u: (self.u.0, m),
                    ..self
                },
                Self {
                    u: (m, self.u.1),
                    ..self
                },
            )
        } else {
            let m = 0.5 * (self.v.0 + self.v.1);
            (
                Self {
                    v: (self.v.0, m),
                    ..self
                },
                Self {
                    v: (m, self.v.1),
                    ..self
                },
            )
        }
    }

    fn contained_in(self, o: Self) -> bool {
        o.u.0 <= self.u.0 && self.u.1 <= o.u.1 && o.v.0 <= self.v.0 && self.v.1 <= o.v.1
    }
}

/// Subdivide a NURBS surface's parameter rectangle against a **plane**:
/// the locus is `φ(u,v) = n·(S(u,v) − p₀) = 0`, so exclusion is a
/// zero-free enclosure of `φ`, computed from the surface's certified
/// first-order box.
///
/// Same two modes as [`sweep_r3`]: empty `tubes` generates seeds,
/// populated `tubes` proves accounting.
///
/// # Errors
///
/// As [`sweep_r3`].
pub(crate) fn sweep_chart_plane(
    surface: &NurbsSurface<f64>,
    plane_origin: Point3<f64>,
    plane_normal: Vec3<f64>,
    root: UvRect,
    tubes: &[UvRect],
    floor_uv: f64,
) -> Result<(Exhaustiveness, Vec<(f64, f64)>), SsiError> {
    let boxes = NurbsBoxes::new(surface);
    let mut stats = Exhaustiveness {
        floor: floor_uv,
        ..Exhaustiveness::default()
    };
    let mut out = Vec::new();
    let mut stack = vec![(root, 0u32)];
    while let Some((cell, depth)) = stack.pop() {
        if stats.examined as usize >= SSI_MAX_CELLS {
            return Err(SsiError::CellBudget {
                budget: SSI_MAX_CELLS,
            });
        }
        stats.examined += 1;
        stats.max_depth = stats.max_depth.max(depth);

        let b = boxes.rect_box(cell.u.0, cell.u.1, cell.v.0, cell.v.1);
        let phi = RingInterval::point(plane_normal.x) * (b.x - RingInterval::point(plane_origin.x))
            + RingInterval::point(plane_normal.y) * (b.y - RingInterval::point(plane_origin.y))
            + RingInterval::point(plane_normal.z) * (b.z - RingInterval::point(plane_origin.z));
        if phi.is_poison() {
            return Err(SsiError::UnsupportedCertificate {
                what: "the NURBS control-net enclosure poisoned (a malformed \
                       net or a weight hull touching zero)",
            });
        }
        if excludes_zero(phi) {
            stats.excluded += 1;
            continue;
        }
        if tubes.iter().any(|t| cell.contained_in(*t)) {
            stats.accounted += 1;
            continue;
        }
        if cell.width() <= floor_uv {
            if tubes.is_empty() {
                out.push(cell.center());
                continue;
            }
            return Err(SsiError::ExhaustivenessInconclusive {
                cell_width: cell.width(),
                floor: floor_uv,
                examined: stats.examined,
            });
        }
        stats.refined += 1;
        let (a, c) = cell.split();
        stack.push((c, depth + 1));
        stack.push((a, depth + 1));
    }
    Ok((stats, out))
}
