//! **The plane × NURBS lane's boundary pass**: the wall's knot
//! rectangle decided against the plane before any march (C3).
//!
//! The march meets the wall's domain boundary only as a box test on
//! marched states, so on its own it cannot tell a plane through a
//! corner from one flush with a side or from a branch shorter than its
//! step. This pass decides the four sides and the four corners once, at
//! ε, before seeding:
//!
//! - **A side** is plane × one boundary curve
//!   ([`crate::boundary_section`]). Either it lies within the band of
//!   the plane, and the pass reports a [`SsiBoundaryContact::Side`]
//!   region, or it meets the plane at isolated roots, each decided
//!   transversal along the side.
//! - **A corner** within the band of the plane is classified by the
//!   plane distance's two inward partials over a corner cell
//!   ([`NurbsBoxes::deriv_box`]), the cell walking the tube ladder.
//!   Both one-signed and of one sign: the locus leaves the domain
//!   there, and the corner is a [`SsiBoundaryContact::Corner`] region,
//!   or, where the corner's own distance has that sign too, nothing at
//!   all. Of opposite signs: a branch starts at the corner. A partial
//!   along a side that is not one-signed at any rung is the locus
//!   tangent to that side, a graze, which refuses naming the side.
//!
//! In the band the pass picks no side: a region asserts no topology,
//! only that within its cell the solution set is at most one arc lying
//! within `reach` of the corner or side. Whether a vertex lies on a face
//! stays its consumer's decision. Outside the domain the exact empty
//! answer stands. Every root the pass keeps becomes a branch end, settled
//! onto both surfaces, and the crossings are the only ends an open
//! branch on this lane has.

use geom::NurbsSurface;
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{div_down, div_up, max_bound};
use geom_core::k_stats::decide;
use geom_core::{Band, Interval, Margin, Point3, Sign, Vec3};

use super::enclose::{ChartSpeeds, NurbsBoxes};
use super::exhaust::UvRect;
use super::march::MarchTol;
use super::section::{
    BandVerdict, BoundarySection, SectionRoot, boundary_roots, boundary_section, settle_root,
};
use super::system::{LocalSystem, ParametricPairR4};
use super::{ChartAxis, SsiError, TraceDecision};
use crate::dihedral::decide_reported;
use crate::nurbs_iso::{boundary_iso_u, boundary_iso_v};
use crate::recourse::Refused;

/// Which end of a parameter's domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartEnd {
    /// The domain's low end.
    Low,
    /// The domain's high end.
    High,
}

/// A side of a NURBS wall's knot rectangle: the iso-line where `fixed`
/// sits at its `end`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartSide {
    /// The parameter held fixed along the side.
    pub fixed: ChartAxis,
    /// Which end of its domain it is held at.
    pub end: ChartEnd,
}

impl core::fmt::Display for ChartSide {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let axis = match self.fixed {
            ChartAxis::U => "u",
            ChartAxis::V => "v",
        };
        let end = match self.end {
            ChartEnd::Low => "low",
            ChartEnd::High => "high",
        };
        write!(f, "the wall's {axis} = {end} side")
    }
}

/// A corner of a NURBS wall's knot rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartCorner {
    /// Which end of the `u` domain.
    pub u: ChartEnd,
    /// Which end of the `v` domain.
    pub v: ChartEnd,
}

impl core::fmt::Display for ChartCorner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let word = |e| match e {
            ChartEnd::Low => "low",
            ChartEnd::High => "high",
        };
        write!(
            f,
            "the wall's (u {}, v {}) corner",
            word(self.u),
            word(self.v)
        )
    }
}

/// Where a branch meets the wall's knot rectangle: a side and the
/// parameter along it (`v` on a `u` side, `u` on a `v` side).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundaryPoint {
    /// The side.
    pub side: ChartSide,
    /// The parameter along the side.
    pub t: f64,
}

/// **A certified region of the wall's boundary the plane meets in the
/// band** — not a contact the kernel decided: within the region the
/// solution set is at most one arc, lying within `reach` of the corner
/// or side. Whether that makes a vertex or an edge lie on a face is the
/// consumer's decision; outside the band no region is reported.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SsiBoundaryContact {
    /// A side of the wall lies within the band of the plane.
    Side {
        /// The side.
        side: ChartSide,
        /// How far from the side the locus may lie, in metres.
        reach: f64,
    },
    /// The plane passes within the band of a corner, and the locus
    /// leaves the domain there.
    Corner {
        /// The corner.
        corner: ChartCorner,
        /// How far from the corner the locus may lie, in metres.
        reach: f64,
    },
}

impl core::fmt::Display for SsiBoundaryContact {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Side { side, reach } => write!(
                f,
                "a region along {side}, which lies within the tolerance of the plane: the \
                 intersection there is at most one arc within {reach:e} m of the side"
            ),
            Self::Corner { corner, reach } => write!(
                f,
                "a region at {corner}, which lies within the tolerance of the plane: the \
                 intersection there is at most one arc within {reach:e} m of the corner"
            ),
        }
    }
}

/// A branch end the boundary pass certified: where on the boundary, and
/// the state settled onto both surfaces there.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Crossing {
    /// The boundary point.
    pub at: BoundaryPoint,
    /// The ℝ⁴ state `(plane u, plane v, wall u, wall v)`, settled.
    pub state: [f64; 4],
}

/// What the boundary pass decided.
#[derive(Clone, Debug)]
pub(crate) struct BoundaryPass {
    /// The regions it reports.
    pub contacts: Vec<SsiBoundaryContact>,
    /// The regions' certified cells, which the accounting banks.
    pub regions: Vec<UvRect>,
    /// The branch ends, in side order and ascending along each side.
    pub crossings: Vec<Crossing>,
}

/// The plane, as the pass reads it.
#[derive(Clone, Copy)]
pub(crate) struct PassPlane {
    /// A point on it.
    pub origin: Point3<f64>,
    /// Its unit normal.
    pub normal: Vec3<f64>,
    /// Its chart's `u` axis.
    pub u_ref: Vec3<f64>,
}

/// The four sides, in the pass's fixed order (D9).
const SIDES: [ChartSide; 4] = [
    ChartSide {
        fixed: ChartAxis::U,
        end: ChartEnd::Low,
    },
    ChartSide {
        fixed: ChartAxis::U,
        end: ChartEnd::High,
    },
    ChartSide {
        fixed: ChartAxis::V,
        end: ChartEnd::Low,
    },
    ChartSide {
        fixed: ChartAxis::V,
        end: ChartEnd::High,
    },
];

/// The four corners, in the pass's fixed order (D9).
const CORNERS: [ChartCorner; 4] = [
    ChartCorner {
        u: ChartEnd::Low,
        v: ChartEnd::Low,
    },
    ChartCorner {
        u: ChartEnd::High,
        v: ChartEnd::Low,
    },
    ChartCorner {
        u: ChartEnd::Low,
        v: ChartEnd::High,
    },
    ChartCorner {
        u: ChartEnd::High,
        v: ChartEnd::High,
    },
];

/// The value of a domain at an end.
fn at(domain: (f64, f64), end: ChartEnd) -> f64 {
    match end {
        ChartEnd::Low => domain.0,
        ChartEnd::High => domain.1,
    }
}

/// The inward sign of a domain end: `+1` at the low end, `−1` at the
/// high.
fn inward(end: ChartEnd) -> f64 {
    match end {
        ChartEnd::Low => 1.0,
        ChartEnd::High => -1.0,
    }
}

/// Everything the pass reads, minted once.
pub(crate) struct Pass<'a> {
    /// The wall.
    pub wall: &'a NurbsSurface<f64>,
    /// Its chart speeds.
    pub speeds: ChartSpeeds,
    /// The plane.
    pub plane: PassPlane,
    /// The ℝ⁴ system the crossings are settled on.
    pub sys: &'a ParametricPairR4<'a>,
    /// The march tolerance a settled state meets.
    pub tol: MarchTol,
    /// The section floor, in metres.
    pub floor: f64,
    /// The lever arm and the tube ladder's scale, in metres.
    pub extent: f64,
    /// The run band.
    pub band: Band,
}

/// A side's decided section, with what the corners need from it.
struct SideSection {
    side: ChartSide,
    section: BoundarySection,
}

/// What a corner within the band of the plane is.
enum CornerClass {
    /// The locus leaves the domain at the corner: a region of `reach`
    /// over `cell`.
    Contact { cell: UvRect, reach: f64 },
    /// The plane misses the corner cell (the exact empty answer).
    Empty { cell: UvRect },
    /// A branch starts at the corner.
    Start { cell: UvRect },
}

impl Pass<'_> {
    /// The wall's `(u, v)` domain.
    fn domain(&self) -> ((f64, f64), (f64, f64)) {
        (self.wall.knots_u().domain(), self.wall.knots_v().domain())
    }

    /// The side's curve: a row of a clamped wall's net, verbatim. Every
    /// `KnotVector` is clamped by construction, so no extraction beyond
    /// the copy is needed.
    fn curve(&self, side: ChartSide) -> Result<geom::NurbsCurve3<f64>, SsiError> {
        let high = side.end == ChartEnd::High;
        let row = match side.fixed {
            ChartAxis::U => boundary_iso_u(self.wall, high),
            ChartAxis::V => boundary_iso_v(self.wall, high),
        };
        row.map_err(|_| SsiError::UnsupportedCertificate {
            what: "a NURBS wall's boundary row is not valid spline structure",
        })
    }

    /// The certified speed along a side: the chart speed of the
    /// parameter that varies along it.
    fn along_speed(&self, side: ChartSide) -> geom_core::SupSpeed<f64> {
        match side.fixed {
            ChartAxis::U => self.speeds.v,
            ChartAxis::V => self.speeds.u,
        }
    }

    /// The plane distance's partials over a chart rectangle.
    fn partials(&self, r: UvRect) -> (Interval, Interval) {
        let boxes = NurbsBoxes::new(self.wall);
        let n = [
            self.plane.normal.x,
            self.plane.normal.y,
            self.plane.normal.z,
        ]
        .map(Interval::point);
        let dot = |b: super::enclose::Box3| n[0] * b.x + n[1] * b.y + n[2] * b.z;
        (
            dot(boxes.deriv_box(r.u.0, r.u.1, r.v.0, r.v.1, true)),
            dot(boxes.deriv_box(r.u.0, r.u.1, r.v.0, r.v.1, false)),
        )
    }

    /// The plane distance of a corner: a clamped wall's corner is its
    /// net's corner control point, so this encloses it exactly.
    fn corner_distance(&self, c: ChartCorner) -> Interval {
        let (nu, nv) = (
            self.wall.knots_u().control_count(),
            self.wall.knots_v().control_count(),
        );
        let iu = match c.u {
            ChartEnd::Low => 0,
            ChartEnd::High => nu - 1,
        };
        let iv = match c.v {
            ChartEnd::Low => 0,
            ChartEnd::High => nv - 1,
        };
        let Some(&p) = self.wall.control().get(iu * nv + iv) else {
            return Interval::refused();
        };
        let n = self.plane.normal;
        let o = self.plane.origin;
        Interval::point(n.x) * (Interval::point(p.x) - Interval::point(o.x))
            + Interval::point(n.y) * (Interval::point(p.y) - Interval::point(o.y))
            + Interval::point(n.z) * (Interval::point(p.z) - Interval::point(o.z))
    }

    /// The ladder's rungs: the tube ladder's radii, widest first, each
    /// crossed into chart pads per axis.
    fn rungs(&self) -> Vec<(f64, f64)> {
        super::certify::tube_ladder(self.extent, self.band)
            .map(|r| self.speeds.pad(r))
            .collect()
    }

    /// A strip along `side`, `pad` deep into the domain.
    fn strip(&self, side: ChartSide, pad: (f64, f64)) -> UvRect {
        let ((u0, u1), (v0, v1)) = self.domain();
        let cut = |d: (f64, f64), end: ChartEnd, w: f64| match end {
            ChartEnd::Low => (d.0, (d.0 + w).min(d.1)),
            ChartEnd::High => ((d.1 - w).max(d.0), d.1),
        };
        match side.fixed {
            ChartAxis::U => UvRect {
                u: cut((u0, u1), side.end, pad.0),
                v: (v0, v1),
            },
            ChartAxis::V => UvRect {
                u: (u0, u1),
                v: cut((v0, v1), side.end, pad.1),
            },
        }
    }

    /// A cell at `corner`, `pad` deep along each axis.
    fn corner_cell(&self, corner: ChartCorner, pad: (f64, f64)) -> UvRect {
        let ((u0, u1), (v0, v1)) = self.domain();
        let cut = |d: (f64, f64), end: ChartEnd, w: f64| match end {
            ChartEnd::Low => (d.0, (d.0 + w).min(d.1)),
            ChartEnd::High => ((d.1 - w).max(d.0), d.1),
        };
        UvRect {
            u: cut((u0, u1), corner.u, pad.0),
            v: cut((v0, v1), corner.v, pad.1),
        }
    }

    /// **A side within the band of the plane**: the strip along it where
    /// the inward partial is one-signed, the widest rung that has one,
    /// and the region's reach. The strip's partial is the surfaces'
    /// transversality across the side, so a strip that never clears
    /// the band is the surfaces tangent along the wall's edge: C7's
    /// regime, refused by the transversality decision.
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryTangent`] naming the side. `Ok(None)` where
    /// the slope across the side straddles zero over every rung's strip:
    /// the plane does not lie along the side there, or the surfaces are
    /// tangent along it, which the caller tells apart.
    fn side_region(&self, side: ChartSide, sup: f64) -> Result<Option<(UvRect, f64)>, SsiError> {
        let speed = match side.fixed {
            ChartAxis::U => self.speeds.u,
            ChartAxis::V => self.speeds.v,
        };
        let mut chosen: Option<(UvRect, f64)> = None;
        for pad in self.rungs() {
            let strip = self.strip(side, pad);
            let (pu, pv) = self.partials(strip);
            let across = match side.fixed {
                ChartAxis::U => pu,
                ChartAxis::V => pv,
            };
            let inf = super::enclose::zero_free_lower_bound(across);
            chosen = Some((strip, inf));
            if inf > 0.0 {
                break;
            }
        }
        let Some((strip, inf)) = chosen else {
            return Err(SsiError::TubeLadderEmpty {
                extent: self.extent,
                floor: super::certify::SSI_TUBE_RADIUS * self.band.zero(),
            });
        };
        if inf <= 0.0 {
            return Ok(None);
        }
        // The sine of the angle between the wall and the plane across the
        // side, levered as the march's transversality is.
        let sine = div_down(inf, speed.get());
        let margin = Margin::levered(sine, self.extent);
        let verdict = match decide_reported("ssi_boundary_strip", margin, self.band) {
            Ok(decided) => match Refused::of(decided, self.band) {
                None => {
                    let reach = div_up(sup, sine) + self.band.zero();
                    return Ok(Some((strip, reach)));
                }
                Some(r) => BandVerdict::Refused(r),
            },
            Err(cause) => BandVerdict::Undecided(cause),
        };
        Err(SsiError::BoundaryTangent { side, verdict })
    }

    /// The surfaces tangent along `side`: its slope across the side
    /// decided on the zero it reads at every rung.
    fn tangent(&self, side: ChartSide) -> SsiError {
        let margin = Margin::levered(0.0, self.extent);
        let verdict = match decide_reported("ssi_boundary_strip", margin, self.band) {
            Ok(decided) => match Refused::of(decided, self.band) {
                Some(r) => BandVerdict::Refused(r),
                None => unreachable!(
                    "a zero slope across {side} decided positive: the strip decision passes \
                     only a margin clear of the band"
                ),
            },
            Err(cause) => BandVerdict::Undecided(cause),
        };
        SsiError::BoundaryTangent { side, verdict }
    }

    /// **A corner within the band of the plane**, classified by the
    /// plane distance's inward partials over the widest corner cell of
    /// the ladder where both are one-signed (module docs).
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryGraze`] naming the side along which the
    /// partial is never one-signed.
    fn corner_class(&self, corner: ChartCorner, phi: Interval) -> Result<CornerClass, SsiError> {
        let mut last: Option<(UvRect, Interval, Interval)> = None;
        for pad in self.rungs() {
            let cell = self.corner_cell(corner, pad);
            let (pu, pv) = self.partials(cell);
            last = Some((cell, pu, pv));
            if one_signed(pu) && one_signed(pv) {
                break;
            }
        }
        let Some((cell, pu, pv)) = last else {
            return Err(SsiError::TubeLadderEmpty {
                extent: self.extent,
                floor: super::certify::SSI_TUBE_RADIUS * self.band.zero(),
            });
        };
        if !one_signed(pu) || !one_signed(pv) {
            // A partial along a side that never clears zero: the locus is
            // tangent to that side at the corner. `φ_u` runs along the
            // `v` side, `φ_v` along the `u` side.
            let side = if one_signed(pu) {
                ChartSide {
                    fixed: ChartAxis::U,
                    end: corner.u,
                }
            } else {
                ChartSide {
                    fixed: ChartAxis::V,
                    end: corner.v,
                }
            };
            let bracket = match side.fixed {
                ChartAxis::U => cell.v,
                ChartAxis::V => cell.u,
            };
            return Err(graze(side, bracket, self.extent, self.band));
        }
        // The inward partials' signs.
        let su = (pu.lo() > 0.0) == (inward(corner.u) > 0.0);
        let sv = (pv.lo() > 0.0) == (inward(corner.v) > 0.0);
        if su != sv {
            return Ok(CornerClass::Start { cell });
        }
        // Both inward partials carry φ away from the corner's value with
        // sign `su`: a corner distance of that sign keeps the cell clear.
        if one_signed(phi) && (phi.lo() > 0.0) == su {
            return Ok(CornerClass::Empty { cell });
        }
        let (speeds_u, speeds_v) = (self.speeds.u.get(), self.speeds.v.get());
        let inf_u = super::enclose::zero_free_lower_bound(pu);
        let inf_v = super::enclose::zero_free_lower_bound(pv);
        let mag = max_bound(phi.lo().abs(), phi.hi().abs());
        let reach = div_up(mag, div_down(inf_u, speeds_u))
            + div_up(mag, div_down(inf_v, speeds_v))
            + self.band.zero();
        Ok(CornerClass::Contact { cell, reach })
    }

    /// The state at a boundary point settled onto both surfaces, or
    /// [`SsiError::EndNotOnLocus`].
    fn settle(&self, side: ChartSide, root: SectionRoot) -> Result<Crossing, SsiError> {
        let ((u0, u1), (v0, v1)) = self.domain();
        let curve = self.curve(side)?;
        let tol = self.tol.settling();
        let refuse = || SsiError::EndNotOnLocus {
            side,
            bracket: root.bracket,
        };
        let t = settle_root(
            &curve,
            self.plane.origin,
            self.plane.normal,
            root.bracket,
            tol,
        )
        .ok_or_else(refuse)?;
        if t.is_nan() {
            return Err(SsiError::UnsupportedCertificate {
                what: super::exhaust::SIDE_ENCLOSURE_REFUSED,
            });
        }
        let (u, v) = match side.fixed {
            ChartAxis::U => (at((u0, u1), side.end), t),
            ChartAxis::V => (t, at((v0, v1), side.end)),
        };
        let q = self.wall.eval(u, v) - self.plane.origin;
        let v_ref = self.plane.normal.cross(self.plane.u_ref);
        let state = [q.dot(self.plane.u_ref), q.dot(v_ref), u, v];
        if !self.sys.residual(&state).iter().all(|r| r.abs() <= tol) {
            return Err(refuse());
        }
        Ok(Crossing {
            at: BoundaryPoint { side, t },
            state,
        })
    }

    /// **The pass** (module docs).
    ///
    /// # Errors
    ///
    /// [`SsiError::BoundaryGraze`], [`SsiError::BoundaryTangent`],
    /// [`SsiError::EndNotOnLocus`], and the section's refusals.
    pub(crate) fn run(&self) -> Result<BoundaryPass, SsiError> {
        let ((u0, u1), (v0, v1)) = self.domain();
        let mut contacts = Vec::new();
        let mut regions: Vec<UvRect> = Vec::new();
        let mut sections = Vec::new();
        for side in SIDES {
            let curve = self.curve(side)?;
            let section = boundary_section(
                &curve,
                self.plane.origin,
                self.plane.normal,
                self.along_speed(side),
                self.floor,
                self.extent,
                self.band,
            )
            .map_err(|e| e.on_side(side))?;
            let section = match section {
                BoundarySection::On { sup } => match self.side_region(side, sup)? {
                    Some((strip, reach)) => {
                        contacts.push(SsiBoundaryContact::Side { side, reach });
                        regions.push(strip);
                        BoundarySection::On { sup }
                    }
                    // No strip beside the side holds the wall's slope
                    // across it clear of zero. Either the plane crosses a
                    // side shorter than the band, whose crossings decide
                    // it, or the surfaces are tangent along the side.
                    None => match boundary_roots(
                        &curve,
                        self.plane.origin,
                        self.plane.normal,
                        self.along_speed(side),
                        self.floor,
                        self.extent,
                        self.band,
                    ) {
                        Ok(roots) => roots,
                        Err(_) => return Err(self.tangent(side)),
                    },
                },
                roots => roots,
            };
            sections.push(SideSection { side, section });
        }
        let on = |s: ChartSide| {
            sections
                .iter()
                .any(|x| x.side == s && matches!(x.section, BoundarySection::On { .. }))
        };
        // The corners: each end root of a side lands at one.
        let mut starts: Vec<UvRect> = Vec::new();
        let mut clear: Vec<UvRect> = Vec::new();
        for corner in CORNERS {
            let through = [
                ChartSide {
                    fixed: ChartAxis::U,
                    end: corner.u,
                },
                ChartSide {
                    fixed: ChartAxis::V,
                    end: corner.v,
                },
            ];
            if through.iter().any(|s| on(*s)) {
                // A side region covers the corner.
                continue;
            }
            let phi = self.corner_distance(corner);
            let margin = if phi.is_certified() {
                Margin::of(max_bound(phi.lo().abs(), phi.hi().abs()))
            } else {
                Margin::of(f64::NAN)
            };
            match decide("ssi_boundary_corner", margin, self.band) {
                Ok(Sign::Positive) => continue,
                Ok(Sign::Zero | Sign::Negative) => {}
                Err(cause) if cause.margin.is_invalid() => {
                    return Err(TraceDecision::BoundarySection.escalated(cause));
                }
                Err(_) => {}
            }
            match self.corner_class(corner, phi)? {
                CornerClass::Empty { cell } => clear.push(cell),
                CornerClass::Contact { cell, reach } => {
                    contacts.push(SsiBoundaryContact::Corner { corner, reach });
                    regions.push(cell);
                }
                CornerClass::Start { cell } => starts.push(cell),
            }
        }
        // The crossings: every root not inside a region; an end root at
        // a corner within the band only where a branch starts there, and
        // once per corner.
        let mut crossings: Vec<Crossing> = Vec::new();
        for SideSection { side, section } in &sections {
            let BoundarySection::Roots {
                interior,
                at_start,
                at_end,
            } = section
            else {
                continue;
            };
            let mut roots: Vec<SectionRoot> = Vec::new();
            for (end_root, end) in [(at_start, ChartEnd::Low), (at_end, ChartEnd::High)] {
                let Some(root) = end_root else { continue };
                let corner = match side.fixed {
                    ChartAxis::U => ChartCorner {
                        u: side.end,
                        v: end,
                    },
                    ChartAxis::V => ChartCorner {
                        u: end,
                        v: side.end,
                    },
                };
                let point = (at((u0, u1), corner.u), at((v0, v1), corner.v));
                let in_start = starts.iter().any(|c| holds(*c, point));
                // Inside a region the root is the region's; in a corner
                // cell the plane is certified clear of, it is none.
                if regions.iter().chain(&clear).any(|c| holds(*c, point)) {
                    continue;
                }
                if !in_start {
                    // A root within the floor of a corner the plane is
                    // decided clear of: an ordinary crossing.
                    super::section::decide_crossing(*root, self.extent, self.band)
                        .map_err(|e| e.on_side(*side))?;
                }
                roots.push(*root);
            }
            roots.extend(interior.iter().copied());
            for root in roots {
                let c = self.settle(*side, root)?;
                let point = (c.state[2], c.state[3]);
                if regions.iter().any(|r| holds(*r, point)) {
                    continue;
                }
                // One crossing per corner a branch starts at.
                let twin = starts.iter().any(|cell| {
                    holds(*cell, point)
                        && crossings
                            .iter()
                            .any(|o| holds(*cell, (o.state[2], o.state[3])))
                });
                if twin {
                    continue;
                }
                crossings.push(c);
            }
        }
        Ok(BoundaryPass {
            contacts,
            regions,
            crossings,
        })
    }
}

/// Whether a chart point lies in a rectangle.
fn holds(r: UvRect, (u, v): (f64, f64)) -> bool {
    u >= r.u.0 && u <= r.u.1 && v >= r.v.0 && v <= r.v.1
}

/// Whether an enclosure is certified and one-signed.
fn one_signed(i: Interval) -> bool {
    i.is_certified() && (i.lo() > 0.0 || i.hi() < 0.0)
}

/// The graze refusal at `side` where the slope along it is not
/// one-signed: the crossing decision on a zero slope, which refuses.
fn graze(side: ChartSide, bracket: (f64, f64), arm: f64, band: Band) -> SsiError {
    let root = SectionRoot {
        bracket,
        slope: 0.0,
        rising: None,
    };
    match super::section::decide_crossing(root, arm, band) {
        Err(e) => e.on_side(side),
        // A zero margin decides zero at every band, which refuses.
        Ok(()) => unreachable!(
            "a zero slope decided positive at {side}: the crossing decision passes only a \
             margin clear of the band"
        ),
    }
}
