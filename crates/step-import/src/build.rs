//! **The build** (`[import]`, D7 step 2): every edge's description chosen
//! at the file's band, and every cache built from it by the kernel's own
//! constructions.
//!
//! Interpretation hands this pass a shell whose surfaces are descriptions
//! and whose edges are still the file's statements: a printed carrier, its
//! derived interval, and printed vertex points ([`SolidSpec`], the
//! evidence). The pass answers the interpreted model ([`Shell`]), which
//! holds no value the file printed as a cache:
//!
//! 1. **Selection.** Each edge's description is read off its two faces'
//!    surfaces at the file band ([`FileBand`]), the printed carrier only
//!    choosing the component: a spline wall's boundary column or row it
//!    repeats, the section of two distinct surfaces, their tangency, a
//!    periodic face's wrap edge, or a curve one surface holds.
//! 2. **Carriers.** Each description's carrier is constructed at the run's
//!    ε: the section nearest the printed carrier ([`topo::section_near`],
//!    the pair section for the pairs it serves), the tangent locus
//!    ([`geom_brep::tangent_locus`], [`geom_brep::coaxial_tangent_circle`]),
//!    the wall's own iso ([`geom_brep::boundary_iso_u`]), the surface's own
//!    seam generator. The printed carrier is then read on the built one
//!    over its trim ([`description_gap`]): certified by locus where an arm
//!    of [`crate::on_built`] serves the pair (its tightness depends on
//!    the pair, so a promoted carrier may certify tighter than the spline
//!    it was promoted from), at its ends and middle where none does. A
//!    printed carrier the file band reads off the built one
//!    refuses as the file's (D7: data no description satisfies within
//!    ε_in); a description the kernel cannot construct refuses
//!    not-yet-supported, naming the lane.
//! 3. **Vertices.** Each vertex is the root of a surface around it along
//!    a built carrier through it that the surface does not hold
//!    ([`topo::root_along`]), else the foot of the printed point on its
//!    one carrier; the move from the printed point is read at the file
//!    band and reported where it passes rounding.
//! 4. **Parameters.** Each edge's interval is its built ends read on its
//!    built carrier by the carrier's own inverse.
//!
//! Certification at ε is the adoption's ([`crate::adopt`]), through the
//! doors a native build uses.

use std::collections::BTreeMap;

use geom::{Curve3, Surface};
use geom_brep::Pcurve;
use geom_core::k_stats::decide;
use geom_core::{Margin, Point3, Sign, Tol, Vec3};

use crate::entities::{EdgeSpec, FaceSpec, SolidSpec, SurfaceRecord};
use crate::error::{BuiltEntity, StepImportError};
use crate::interpret::{FileBand, Read};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Whether a printed carrier lies on the description built for it: how
/// far it stands off the built carrier over its trim ([`description_gap`]),
/// read at the file's band.
const STEP_EDGE_ON_DESCRIPTION: &str = "step_edge_on_description";

/// Whether a printed vertex point lies where the build put its vertex,
/// read at the file's band.
pub(crate) const STEP_VERTEX_BUILT: &str = "step_vertex_built";

/// The cosine between a built carrier and the printed one at the printed
/// middle: its sign is the built carrier's sense.
pub(crate) const STEP_BUILT_SENSE: &str = "step_built_sense";

/// One of an edge's two uses: the forward use's face (`Plus`, the
/// half-edge `he_plus` assembly realizes) or the reverse use's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    /// The forward use's face.
    Plus,
    /// The reverse use's face.
    Minus,
}

/// **An edge's description as selection chose it**, keyed by side rather
/// than by surface key: adoption names the keys.
#[derive(Clone, Debug)]
pub(crate) enum Description {
    /// The transverse section of the two surfaces, the component through
    /// the built carrier's middle.
    Intersection,
    /// The tangential contact of the two surfaces.
    Tangent,
    /// A spline wall's boundary column or row: its chart image on the
    /// built carrier's parameter.
    Iso {
        /// The wall.
        wall: Side,
        /// The image.
        image: Pcurve<f64>,
    },
    /// A periodic face's wrap edge: its surface's own seam generator.
    Wrap,
    /// A curve one chart holds: in the plane of a plane × spline wall pair
    /// (the native rim's description), or inside the one surface two
    /// faces share.
    Chart {
        /// The chart.
        chart: Side,
    },
}

/// **One edge of the interpreted model**: its ends, its description, and
/// the caches built from it.
#[derive(Clone, Debug)]
pub(crate) struct Edge {
    /// The start vertex (the plus half's start).
    pub(crate) start: u64,
    /// The end vertex.
    pub(crate) end: u64,
    /// The description.
    pub(crate) description: Description,
    /// The built carrier, running start → end in increasing parameter.
    pub(crate) carrier: Curve3<f64>,
    /// The parameter at `start`.
    pub(crate) t0: f64,
    /// The parameter at `end`.
    pub(crate) t1: f64,
}

/// **One shell of the interpreted model** (module docs): the faces as
/// interpretation left them (each a kernel surface interpreted from its
/// record, and its loops by entity id), every edge's description and
/// built caches, and every built vertex point. No edge carrier, interval
/// or vertex holds a value the file printed; a vertex no edge ends at is
/// no vertex of the model and is left out.
#[derive(Debug)]
pub(crate) struct Shell<'a> {
    /// The solid record's entity id; for a void, its shell's.
    pub(crate) id: u64,
    /// The faces, in `CLOSED_SHELL` order.
    pub(crate) faces: &'a [FaceSpec],
    /// The surface records the faces wear.
    pub(crate) surfaces: &'a BTreeMap<u64, SurfaceRecord>,
    /// Every edge, keyed by `EDGE_CURVE` id.
    pub(crate) edges: BTreeMap<u64, Edge>,
    /// Every vertex's built point, keyed by `VERTEX_POINT` id.
    pub(crate) vertices: BTreeMap<u64, Point3<f64>>,
    /// What the edges on each printed curve entity hold
    /// ([`CurveHolders`]), which decides its `CurvePromoted` record.
    pub(crate) holders: CurveHolders,
}

/// **What the edges on each printed curve hold**, keyed by curve entity,
/// over the edges whose printed carrier is analytic (a promotion's, or a
/// `LINE` or `CIRCLE` the file states): `spline` where such an edge's
/// built carrier is a spline the body holds instead (a wall's column,
/// [`column_match`]), `analytic` where it is ANY analytic carrier —
/// the printed one, or one the build constructed in its place (a
/// section's line, a re-anchored line). A promotion is reported where an
/// edge carries it (`crate::promotion_carried`).
#[derive(Clone, Debug, Default)]
pub(crate) struct CurveHolders {
    /// Curves an edge carries as a spline the body holds.
    pub(crate) spline: std::collections::BTreeSet<u64>,
    /// Curves an edge carries as their analytic curve.
    pub(crate) analytic: std::collections::BTreeSet<u64>,
}

impl CurveHolders {
    /// Both sets of `other` added to this one's.
    pub(crate) fn extend(&mut self, other: CurveHolders) {
        self.spline.extend(other.spline);
        self.analytic.extend(other.analytic);
    }
}

/// An edge mid-build: its description and carrier, before its ends are
/// built.
struct Chosen {
    description: Description,
    /// The built carrier; `None` while it waits on its ends
    /// ([`Deferred`]).
    carrier: Option<Curve3<f64>>,
    /// What builds the carrier from the edge's built ends, for an edge
    /// whose description is fixed by them.
    deferred: Option<Deferred>,
    /// Whether the carrier was built from both surfaces (a section, a
    /// tangency, a wall's iso, a seam): a vertex is rooted along such a
    /// carrier first.
    intrinsic: bool,
    /// For a wall's iso on a straight carrier: the wall parameter `a + b·t`
    /// at carrier parameter `t`, and whether the image is a column.
    iso: Option<IsoMap>,
    /// A straight edge whose carrier is the segment between its built
    /// ends.
    between_ends: bool,
}

impl Chosen {
    /// An edge whose description builds its carrier now.
    fn built(description: Description, carrier: Curve3<f64>) -> Self {
        Self {
            description,
            carrier: Some(carrier),
            deferred: None,
            intrinsic: true,
            iso: None,
            between_ends: false,
        }
    }
}

/// **A carrier its built ends fix**: a curve one surface holds, built once
/// the vertices it runs between are.
#[derive(Clone, Copy, Debug)]
enum Deferred {
    /// The surface's own generator through a built end: a periodic face's
    /// seam, or a meridian, parallel, ruling or tube circle between two
    /// faces of one record.
    Generator,
    /// The segment between the built ends, inside one plane.
    Segment,
}

/// A wall iso's image as a map of the carrier's parameter.
#[derive(Clone, Copy, Debug)]
struct IsoMap {
    /// The fixed chart coordinate: `u` of a column, `v` of a row.
    fixed: f64,
    /// Column (`u` fixed) or row (`v` fixed).
    column: bool,
    /// The running chart coordinate at carrier parameter `t`.
    running: Running,
}

/// **The running chart coordinate of a wall iso at carrier parameter
/// `t`.**
#[derive(Clone, Copy, Debug)]
enum Running {
    /// The carrier is the iso on its own parameter: `t`.
    Own,
    /// A straight carrier on its arc length: `from` at `t = 0`, `to` at
    /// `t = len`, each end exactly.
    Chord {
        /// The coordinate at `t = 0`.
        from: f64,
        /// The coordinate at `t = len`.
        to: f64,
        /// The chord's length.
        len: f64,
    },
    /// The iso of the wall reversed along it, on `[lo, hi]`: `lo + hi − t`.
    Mirror {
        /// The domain's start.
        lo: f64,
        /// The domain's end.
        hi: f64,
    },
}

/// **Builds one shell** (module docs). `moves` receives a record for each
/// vertex the build moved off its printed point, and for each edge whose
/// built carrier is not the printed one.
///
/// # Errors
///
/// An interpretation refusal where the file's statements are no
/// description at its band ([`StepImportError::Interpretation`]),
/// [`StepImportError::NotYetSupported`] where the kernel constructs none,
/// and [`StepImportError::BuildEscalated`] where the run's band does not
/// decide a construction.
pub(crate) fn build<'a>(
    solid: &'a SolidSpec,
    tol: Tol,
    file: FileBand,
    moves: &mut Vec<StructureNormalization>,
) -> Result<Shell<'a>, StepImportError> {
    let sides = edge_faces(solid)?;
    let mut chosen = BTreeMap::new();
    for (&id, spec) in &solid.edges {
        let (plus, minus) = sides[&id];
        chosen.insert(id, choose(solid, id, spec, (plus, minus), tol, file)?);
    }
    let vertices = build_vertices(solid, &sides, &mut chosen, tol, file, moves)?;
    let mut edges = BTreeMap::new();
    let mut holders = CurveHolders::default();
    for (
        id,
        Chosen {
            description,
            carrier,
            iso,
            between_ends,
            ..
        },
    ) in chosen
    {
        let spec = &solid.edges[&id];
        let (p_start, p_end) = (vertices[&spec.start], vertices[&spec.end]);
        let Some(carrier) = carrier else {
            return Err(StepImportError::Topology {
                id,
                what: "internal: an edge whose carrier waits on ends the build did not reach",
            });
        };
        let carrier = crate::signed_zero::plus_zero_curve(rebuilt_in_surface(
            between_ends,
            carrier,
            (p_start, p_end),
            file,
        ));
        let whole = matches!(description, Description::Iso { .. });
        let (carrier, t0, t1) = params(
            id,
            carrier,
            (p_start, p_end),
            spec.start == spec.end,
            whole,
            (tol, file),
        )?;
        let description = match (description, iso) {
            (Description::Iso { wall, .. }, Some(map)) => Description::Iso {
                wall,
                image: map.image(t0, t1),
            },
            (other, _) => other,
        };
        if let Some(curve) = spec.curve
            && !matches!(spec.carrier, Curve3::Nurbs(_))
        {
            if matches!(carrier, Curve3::Nurbs(_)) {
                holders.spline.insert(curve);
            } else {
                holders.analytic.insert(curve);
            }
        }
        // Reported where the built carrier is not the printed one, by
        // value; the move is measured only then.
        if carrier_sig(&carrier) != carrier_sig(&spec.carrier) {
            moves.push(StructureNormalization {
                face: id,
                kind: NormalizationKind::CarrierBuilt {
                    moved_by: held_gap(&carrier, spec),
                },
                file_census: FaceCensus::EMPTY,
                kernel_census: FaceCensus::EMPTY,
            });
        }
        edges.insert(
            id,
            Edge {
                start: spec.start,
                end: spec.end,
                description,
                carrier,
                t0,
                t1,
            },
        );
    }
    Ok(Shell {
        id: solid.id,
        faces: &solid.faces,
        surfaces: &solid.surfaces,
        edges,
        vertices,
        holders,
    })
}

impl IsoMap {
    fn image(self, t0: f64, t1: f64) -> Pcurve<f64> {
        // Held to the iso's own ends where the map lands one unit of the
        // last place past one: its arithmetic rounds once, so its image
        // may step one place past the end, and that step is the end. A
        // coordinate farther out is left as it lands, and certification
        // reads it.
        let held = |x: f64, a: f64, b: f64| {
            let (lo, hi) = (a.min(b), a.max(b));
            if x < lo && x >= lo.next_down() {
                lo
            } else if x > hi && x <= hi.next_up() {
                hi
            } else {
                x
            }
        };
        let at = |t: f64| match self.running {
            Running::Own => t,
            Running::Chord { from, to, len } => held(from + (to - from) * (t / len), from, to),
            Running::Mirror { lo, hi } => held(lo + hi - t, lo, hi),
        };
        if self.column {
            Pcurve::iso_column(self.fixed, (at(t0), at(t1)), (t0, t1))
        } else {
            Pcurve::iso_row(self.fixed, (at(t0), at(t1)), (t0, t1))
        }
    }
}

/// Each edge's two faces, by index: the forward use's, then the reverse
/// use's.
fn edge_faces(solid: &SolidSpec) -> Result<BTreeMap<u64, (usize, usize)>, StepImportError> {
    let mut found: BTreeMap<u64, (Option<usize>, Option<usize>)> = BTreeMap::new();
    for (fi, face) in solid.faces.iter().enumerate() {
        for l in &face.loops {
            for u in &l.uses {
                let slot = found.entry(u.edge).or_default();
                if u.forward {
                    slot.0 = Some(fi);
                } else {
                    slot.1 = Some(fi);
                }
            }
        }
    }
    solid
        .edges
        .keys()
        .map(|&id| match found.get(&id) {
            Some(&(Some(p), Some(m))) => Ok((id, (p, m))),
            _ => Err(StepImportError::Topology {
                id,
                what: "internal: an edge without one forward and one reverse use at the build",
            }),
        })
        .collect()
}

/// A kernel reading at the file's band that escalated, as the file's
/// ambiguity at edge `id` ([`crate::interpret::Refusal::in_band`]), or as
/// the build's escalation where its margin is no point value.
fn ambiguous(id: u64, source: &geom_core::Indeterminate, file: FileBand) -> StepImportError {
    match crate::interpret::Refusal::in_band(STEP_EDGE_ON_DESCRIPTION, source, file.run_eps()) {
        Some(refusal) => file.refused(id, refusal),
        None => escalated(file, id, *source),
    }
}

/// The not-yet-supported refusal of edge `id`, read at the import door of
/// a file declaring `eps_in`.
fn unbuilt(file: FileBand, id: u64, what: String, lane: &'static str) -> StepImportError {
    not_yet(file, (BuiltEntity::Edge, id), what, lane)
}

/// [`unbuilt`] for an entity of either kind.
fn not_yet(
    file: FileBand,
    (entity, id): (BuiltEntity, u64),
    what: String,
    lane: &'static str,
) -> StepImportError {
    StepImportError::NotYetSupported {
        entity,
        id,
        what,
        lane,
        file: geom_core::FileCoincidence::new(file.eps_in()),
    }
}

/// The build's escalation at the run's band, for edge `id`.
fn escalated(file: FileBand, id: u64, source: geom_core::Indeterminate) -> StepImportError {
    escalated_at(file, (BuiltEntity::Edge, id), source)
}

/// [`escalated`] for an entity of either kind.
fn escalated_at(
    file: FileBand,
    (entity, id): (BuiltEntity, u64),
    source: geom_core::Indeterminate,
) -> StepImportError {
    StepImportError::BuildEscalated {
        entity,
        id,
        source,
        file: geom_core::FileCoincidence::new(file.eps_in()),
    }
}

/// The pair `(a, b)` as the refusals name it: `its {a} and its {b}`.
fn pair_names(a: &Surface<f64>, b: &Surface<f64>) -> String {
    format!("its {} and its {}", a.kind().name(), b.kind().name())
}

/// The lane an edge on a marched section loop across the loop's start
/// waits on.
const MARCHED_LANE: &str = "an edge's interval read across a marched section loop's start \
                            (`work/importgate/a-marched-section-edge-across-its-loops-start-refuses-at-import.md`)";

/// The lane an edge between two records of one surface that
/// `crate::one_record` does not join waits on.
const ONE_LOCUS_LANE: &str = "one record of a surface stated twice that one_record does not join \
     (`work/importgate/an-edge-between-two-records-of-one-surface-one-record-does-not-join-refuses.md`)";

/// The lane a pair with no section construction waits on.
const SECTION_LANE: &str = "a section construction for this pair \
                            (`work/importgate/an-imported-edge-on-a-pair-the-c5-table-does-not-build-refuses.md`)";

/// The lane a vertex only an unsupported root fixes waits on.
const ROOT_LANE: &str = "a root of a spline wall along a carrier, or of any surface along a \
                         spline carrier other than a plane's \
                         (`work/importgate/a-vertex-only-a-spline-wall-fixes-refuses-not-yet-supported.md`)";

/// The lane a curve inside one surface, no generator of it, waits on.
const ONE_SURFACE_LANE: &str = "a construction of a curve inside one surface \
                                (`work/importgate/an-imported-edge-on-a-pair-the-c5-table-does-not-build-refuses.md`)";

/// The lane a tangency with no closed-form locus waits on.
const TANGENT_LANE: &str = "a C7 tangent-locus construction for this pose \
                            (`work/importgate/an-imported-tangency-with-no-closed-form-locus-refuses.md`)";

/// **Selection and the carrier's construction** for one edge (module
/// docs, steps 1 and 2). Which description an edge is comes from its two
/// faces' surfaces, read at the file's band; the printed carrier only
/// picks the component.
fn choose(
    solid: &SolidSpec,
    id: u64,
    spec: &EdgeSpec,
    (plus, minus): (usize, usize),
    tol: Tol,
    file: FileBand,
) -> Result<Chosen, StepImportError> {
    let (sa, sb) = (solid.surface(plus), solid.surface(minus));
    let p_start = solid.vertices[&spec.start];
    let p_end = solid.vertices[&spec.end];
    let one_record = solid.faces[plus].record == solid.faces[minus].record;
    if let Some(chosen) = iso_of_wall(id, spec, (sa, sb), (p_start, p_end), one_record, file)? {
        return Ok(chosen);
    }
    if one_record {
        return one_surface(id, sa, plus == minus, file);
    }
    let eps_band = geom_core::Band::linear(tol).map_err(|_| StepImportError::Topology {
        id,
        what: "the run's tolerance makes no band to build at",
    })?;
    let span = (spec.t0, spec.t1);
    let region =
        geom_brep::Reach::of_span(&spec.carrier, span).map_err(|_| StepImportError::Topology {
            id,
            what: "an edge whose printed carrier has no region over its interval",
        })?;
    // The pose readings are levered at how far the edge's two faces run,
    // from below for a reading that builds on its definite sign and from
    // above for one that builds on its Zero ([`crate::reach::faces_run`]).
    let run = crate::reach::faces_run(solid, &[plus, minus]).map_err(|unread| match unread {
        crate::reach::NoRegion::Edge(edge) => StepImportError::Topology {
            id: edge,
            what: "an edge whose printed carrier has no region over its interval",
        },
        crate::reach::NoRegion::Face(face) => StepImportError::Topology {
            id: face,
            what: "a face whose surface reads no region of its boundary",
        },
        crate::reach::NoRegion::NoFace => {
            unreachable!("an edge's two faces were named")
        }
    })?;
    // A cone's pose readings are levered from its apex (the station of a
    // coaxial cylinder's circles is a distance from it): no diameter is a
    // reach from there, so each bound takes in the edge's reach from the
    // apex on its own side ([`geom_brep::edge_reach_bounds`]). Only the
    // half from below is consumed: no reader of `above` has a cone arm.
    let apexes = [sa, sb].into_iter().filter_map(|s| match *s {
        Surface::Cone { frame, .. } => Some(frame.origin()),
        _ => None,
    });
    let extent = apexes.fold(run, |extent, apex| {
        let reach =
            geom_brep::edge_reach_bounds(&spec.carrier, span, apex, region.lever_from(apex));
        geom_brep::LeverBounds::Between {
            below: extent.below().max(reach.below()),
            above: extent.above().max(reach.above()),
        }
    });
    let spline = |s: &Surface<f64>| s.spline_chart().is_some();
    if spline(sa) || spline(sb) {
        return spline_pair(
            id,
            spec,
            (sa, sb),
            (p_start, p_end),
            &region,
            extent,
            (file, eps_band),
        );
    }
    analytic_pair(id, spec, (sa, sb), &region, extent, (file, eps_band))
}

/// [`topo::section_near`] of the pair at `band`, through the printed
/// carrier: the section's component nearest it, or the verdict that
/// builds none.
fn read_section(
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_core::Lever<f64>,
    band: geom_core::Band,
) -> Result<Result<Curve3<f64>, topo::SectionVerdict>, geom_core::Indeterminate> {
    // Two planes meet in one line, which the section anchors at the plane
    // across it through its seed's middle: seeded at the world origin
    // along the printed direction, the anchor is the line's own point
    // nearest the origin, which the two planes alone fix, so the line's
    // construction reads no digit the file printed but its sense.
    let canonical;
    let seed = if plane_normals(sa, sb).is_some() {
        // A printed line's own direction, which its import adopted
        // (`unit_direction`); another carrier's tangent squared, its sense
        // all the seed reads.
        let dir = match &spec.carrier {
            Curve3::Line(line) => line.dir,
            other => geom_core::UnitVec3::squared(other.deriv(geom::mid_param(spec.t0, spec.t1))),
        };
        canonical = Curve3::Line(geom::Line {
            origin: Point3::origin(),
            dir,
        });
        (&canonical, (0.0, 0.0))
    } else {
        (&spec.carrier, (spec.t0, spec.t1))
    };
    topo::section_near(
        Some(topo::SectionLane::f64()),
        (sa, sb),
        seed,
        region,
        extent,
        band,
    )
}

/// A spline wall in the pair: the plane × wall or wall × wall section,
/// selected at the file's band and built at the run's, as an analytic
/// pair's is ([`analytic_pair`]). A cap's rim that is the wall's own
/// boundary row is that row ([`iso_of_wall`], taken before this).
fn spline_pair(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    ends: (Point3<f64>, Point3<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_brep::LeverBounds<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    use topo::SectionVerdict as V;
    let plane = matches!(sa, Surface::Plane { .. }) || matches!(sb, Surface::Plane { .. });
    let both_spline = sa.spline_chart().is_some() && sb.spline_chart().is_some();
    if !plane && !both_spline {
        return Err(unbuilt(
            file,
            id,
            format!(
                "an edge between {}, whose section the kernel builds only against a plane or a \
                 second spline wall",
                pair_names(sa, sb)
            ),
            SECTION_LANE,
        ));
    }
    // **A cap's rational rim is the wall's own row**: a rational
    // boundary row the cap plane holds is the native body's chart image of
    // its rim (a rational wall mints nothing there), built exactly from the
    // wall; a polynomial one is the plane × wall section, which the kernel
    // builds as the same exact row, and the native rim's class.
    if let Some(rim) = cap_row(id, spec, (sa, sb), ends, file, true)? {
        return Ok(rim);
    }
    // The section builds on its pose readings' definite signs.
    let read = |band| read_section(spec, (sa, sb), region, extent.below(), band);
    // An escalation at the file's band is the file's ambiguity: its
    // statements do not decide the reading at their own precision.
    match read(file.band()).map_err(|source| ambiguous(id, &source, file))? {
        Ok(_) => {}
        Err(V::Tangent) => {
            return Err(unbuilt(
                file,
                id,
                format!(
                    "a tangency of {}, read at the file's band",
                    pair_names(sa, sb)
                ),
                TANGENT_LANE,
            ));
        }
        Err(verdict) => {
            if let Some(rim) = cap_row(id, spec, (sa, sb), ends, file, false)? {
                return Ok(rim);
            }
            return Err(unbuilt(
                file,
                id,
                format!(
                    "the section of {} near it, read at the file's band: {verdict}",
                    pair_names(sa, sb)
                ),
                SECTION_LANE,
            ));
        }
    }
    let carrier = match read(eps_band) {
        Ok(Ok(carrier)) => carrier,
        Ok(Err(verdict)) => {
            return Err(unbuilt(
                file,
                id,
                format!(
                    "the section of {} builds at the file's band and not at the run's: \
                     {verdict}",
                    pair_names(sa, sb)
                ),
                crate::snap::UNCATALOGUED_LANE,
            ));
        }
        Err(source) => return Err(escalated(file, id, source)),
    };
    // A straight section is the line it is, where its spline reads as the
    // straight map ([`crate::recognize_curve::straight_map`], at the
    // file's band). No committed row reaches this read with a re-timed
    // straight section: measured, a re-timed straight boundary row in a
    // cap plane — the case where a section IS a wall's row — refuses
    // first at the plane × wall section's limb 2
    // (`work/exch/a-retimed-straight-row-in-a-cap-plane-refuses-at-the-section.md`),
    // and the map reading is the gate here, not a row.
    let carrier = match &carrier {
        Curve3::Nurbs(n) => {
            crate::recognize_curve::straight_map(n, file).map_or(carrier.clone(), |(line, _)| line)
        }
        _ => carrier,
    };
    on_description(id, spec, &carrier, file)?;
    // A spline section the recognition does not certify straight at the
    // band (a marched branch's fit) is the segment between its built ends
    // where the section holds that chord ([`rebuilt_in_surface`]).
    let between_ends = matches!(carrier, Curve3::Nurbs(_));
    Ok(Chosen {
        between_ends,
        ..Chosen::built(Description::Intersection, carrier)
    })
}

/// Whether two planes an edge lies on cross: the sine of their normals'
/// angle levered at the faces' run from below, read at the file's band.
pub(crate) const STEP_PLANES_CROSS: &str = "step_planes_cross";

/// Whether a wall's boundary row or column lies in a cap plane: the
/// farthest of its controls from the plane, read at the file's band.
pub(crate) const STEP_ISO_IN_PLANE: &str = "step_iso_in_plane";

/// Two distinct analytic surfaces: the section at the file band chooses
/// crossing or tangency and the component, and the run builds it.
fn analytic_pair(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_brep::LeverBounds<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    use topo::SectionVerdict as V;
    // The section builds on its pose readings' definite signs.
    let read = |band| read_section(spec, (sa, sb), region, extent.below(), band);
    // An escalation at the file's band is the file's ambiguity: its
    // statements do not decide the reading at their own precision.
    let at_file = read(file.band()).map_err(|source| ambiguous(id, &source, file))?;
    match at_file {
        // The file band builds the section; a run that builds another
        // verdict (a tangency, a pose it does not read coaxial) reads apart
        // a relation the file states that the snap catalogue
        // (`crate::snap`), run before this, did not make exact.
        Ok(_) => match read(eps_band) {
            Ok(Ok(carrier)) => {
                on_description(id, spec, &carrier, file)?;
                Ok(Chosen::built(Description::Intersection, carrier))
            }
            Ok(Err(verdict)) => Err(unbuilt(
                file,
                id,
                format!(
                    "the section of {} builds at the file's band and not at the run's: \
                     {verdict}",
                    pair_names(sa, sb)
                ),
                crate::snap::UNCATALOGUED_LANE,
            )),
            Err(source) => Err(escalated(file, id, source)),
        },
        Err(V::Tangent) => tangent(id, spec, (sa, sb), region, extent, (file, eps_band)),
        // Two planes the file's band reads parallel meet in no line: the
        // file's statements are no description there.
        Err(V::Unsupported { .. }) if let Some(normals) = plane_normals(sa, sb) => {
            let crossing = normals.0.cross(normals.1).norm() * extent.below().length();
            Err(file.refused(id, file.touching(STEP_PLANES_CROSS, crossing)))
        }
        Err(V::Unsupported { .. }) => {
            served_pair(id, spec, (sa, sb), region, extent, (file, eps_band))
        }
        Err(verdict) => Err(unbuilt(
            file,
            id,
            format!(
                "the section of {} near it, read at the file's band: {verdict}",
                pair_names(sa, sb)
            ),
            SECTION_LANE,
        )),
    }
}

/// The two normals of a pair of planes; `None` for any other pair.
fn plane_normals(a: &Surface<f64>, b: &Surface<f64>) -> Option<(Vec3<f64>, Vec3<f64>)> {
    match (a, b) {
        (
            Surface::Plane {
                frame: plane_frame_2,
                ..
            },
            Surface::Plane {
                frame: plane_frame, ..
            },
        ) => {
            let n1 = plane_frame_2.axis();
            let n2 = plane_frame.axis();
            Some((n1, n2))
        }
        _ => None,
    }
}

/// A pair the offset's closed forms do not serve: the pair section's
/// classification at the file band, and the run's carried component
/// through the printed middle. The edge is the pair's tangency only where
/// a tangency decided the arm and the printed middle lies on no crossing
/// component.
fn served_pair(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_brep::LeverBounds<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    let ball = Some(region.ball());
    let classified =
        |band, at_file: bool| -> Result<Option<geom_brep::PairSection<f64>>, StepImportError> {
            match geom_brep::pair_section(sa, sb, ball, Some(geom_brep::SsiLane::marched()), band) {
                Ok(geom_brep::SectionRead::Built(section)) => Ok(Some(section)),
                Ok(geom_brep::SectionRead::Elsewhere(_)) => Ok(None),
                Err(geom_brep::SectionError::Escalated(source)) if at_file => {
                    Err(ambiguous(id, &source, file))
                }
                Err(geom_brep::SectionError::Escalated(source)) => Err(escalated(file, id, source)),
                Err(geom_brep::SectionError::CoincidentSurfaces) => Err(unbuilt(
                    file,
                    id,
                    format!(
                        "an edge between two records of one {}, a curve inside one surface the \
                     two records state twice",
                        sa.kind().name()
                    ),
                    ONE_LOCUS_LANE,
                )),
                Err(e) => Err(unbuilt(
                    file,
                    id,
                    format!("the section of {}: {e}", pair_names(sa, sb)),
                    SECTION_LANE,
                )),
            }
        };
    let Some(at_file) = classified(file.band(), true)? else {
        return Err(unbuilt(
            file,
            id,
            format!(
                "an edge between {}, a pair whose section the kernel does not build",
                pair_names(sa, sb)
            ),
            SECTION_LANE,
        ));
    };
    let tangency = at_file
        .decided()
        .iter()
        .any(|z| matches!(z.decision, geom_brep::SectionDecision::Tangency(_)));
    let Some(at_eps) = classified(eps_band, false)? else {
        return Err(unbuilt(
            file,
            id,
            format!(
                "the section of {} builds at the file's band and not at the run's",
                pair_names(sa, sb)
            ),
            crate::snap::UNCATALOGUED_LANE,
        ));
    };
    let mid = spec.carrier.mid_point(spec.t0, spec.t1);
    let carried = at_eps.carry(Some(geom_brep::SsiLane::marched()), eps_band);
    let on_crossing =
        carried
            .as_ref()
            .ok()
            .and_then(|carried| match carried.site(mid, file.band()) {
                Ok(geom_brep::SiteReading::OnArc(foot)) => {
                    Some(carried.components()[foot.component].carrier.clone())
                }
                Ok(geom_brep::SiteReading::AtNode(_)) | Err(_) => None,
            });
    let Some(component) = on_crossing else {
        if tangency {
            return tangent(id, spec, (sa, sb), region, extent, (file, eps_band));
        }
        return Err(match carried {
            Err(e) => unbuilt(
                file,
                id,
                format!(
                    "the section of {} carries no component: {e}",
                    pair_names(sa, sb)
                ),
                SECTION_LANE,
            ),
            Ok(carried) => {
                // The printed middle on no component at the file's band:
                // the file's statements are no description there.
                let nearest = carried
                    .components()
                    .iter()
                    .map(|c| gap_to(&c.carrier, mid))
                    .fold(f64::INFINITY, f64::min);
                file.agree_at(id, STEP_EDGE_ON_DESCRIPTION, nearest)?;
                unbuilt(
                    file,
                    id,
                    format!(
                        "the printed carrier's middle names no one component of the section of \
                         {}",
                        pair_names(sa, sb)
                    ),
                    SECTION_LANE,
                )
            }
        });
    };
    // A marched loop is periodic; its edge is an arc of the loop's whole
    // turn, read on the clamped spline of that turn.
    let carrier = match component {
        Curve3::Periodic(p) => Curve3::Nurbs(std::sync::Arc::new(p.whole_loop().clone())),
        other => other,
    };
    let carrier = running_with(id, carrier, spec, file)?;
    on_description(id, spec, &carrier, file)?;
    Ok(Chosen::built(Description::Intersection, carrier))
}

/// A tangency read at the file band: the tangent locus at the run, a
/// ruling of a plane and a cylinder or of two parallel cylinders, or the
/// parallel of a coaxial pair. A pair the run reads apart where the file
/// reads it tangent is one the snap catalogue (`crate::snap`), run before
/// this, has no construction for.
fn tangent(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_brep::LeverBounds<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    let locus = if geom_brep::tangent_locus_kinds(sa.kind(), sb.kind()) {
        match geom_brep::tangent_locus(sa, sb, region.ball(), eps_band) {
            Ok(geom_brep::TangentLocus::Line { line, .. }) => Curve3::Line(line),
            Err(geom_brep::TangentLocusError::NotTangent { .. }) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!(
                        "{} read tangent at the file's band and apart at the run's",
                        pair_names(sa, sb)
                    ),
                    crate::snap::UNCATALOGUED_LANE,
                ));
            }
            Err(geom_brep::TangentLocusError::Escalated(source)) => {
                return Err(escalated(file, id, source));
            }
            Err(geom_brep::TangentLocusError::Unsupported { what }) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!("the tangency of {}: {what}", pair_names(sa, sb)),
                    TANGENT_LANE,
                ));
            }
        }
    } else {
        // The one row here that reads the extent is the torus's axis fork
        // (`pt_axis_in_plane`), which takes its Zero for the meridian
        // pose: from above. A plane the file read tangent along a parallel
        // stands normal to that axis, where the row is Positive at any
        // extent past the band, so no import's outcome turns on the bound.
        let coaxial = |band| match geom_brep::coaxial_tangent_circle(sa, sb, extent.above(), band) {
            Ok(circle) => Ok(circle),
            Err(geom_brep::SectionError::Escalated(source)) => Err(Err(source)),
            Err(e) => Err(Ok(e)),
        };
        match coaxial(eps_band) {
            Ok(Some(circle)) => circle,
            Ok(None) => {
                let at_file = match coaxial(file.band()) {
                    Ok(circle) => circle.is_some(),
                    Err(Err(source)) => return Err(ambiguous(id, &source, file)),
                    Err(Ok(_)) => false,
                };
                let (what, lane) = if at_file {
                    (
                        format!(
                            "{} read coaxial and tangent at the file's band and not at the run's",
                            pair_names(sa, sb)
                        ),
                        crate::snap::UNCATALOGUED_LANE,
                    )
                } else {
                    (
                        format!(
                            "a tangency of {} with no closed-form locus",
                            pair_names(sa, sb)
                        ),
                        TANGENT_LANE,
                    )
                };
                return Err(unbuilt(file, id, what, lane));
            }
            Err(Err(source)) => return Err(escalated(file, id, source)),
            Err(Ok(e)) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!("the tangency of {}: {e}", pair_names(sa, sb)),
                    TANGENT_LANE,
                ));
            }
        }
    };
    let carrier = running_with(id, locus, spec, file)?;
    on_description(id, spec, &carrier, file)?;
    Ok(Chosen::built(Description::Tangent, carrier))
}

/// **One surface record on both sides** (a closed spline patch's seam is
/// taken by [`iso_of_wall`]): a curve the surface holds, which its built
/// ends fix. A periodic surface's is its own generator through them (a
/// face's wrap edge, or a meridian, parallel, ruling or tube circle
/// between two faces of the record); a plane's is the segment between
/// them. Its carrier waits on its ends ([`Deferred`]).
fn one_surface(
    id: u64,
    surface: &Surface<f64>,
    one_face: bool,
    file: FileBand,
) -> Result<Chosen, StepImportError> {
    let description = if one_face {
        Description::Wrap
    } else {
        Description::Chart { chart: Side::Plus }
    };
    let deferred = match surface {
        Surface::Plane { .. } => Deferred::Segment,
        Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => Deferred::Generator,
        _ => {
            return Err(unbuilt(
                file,
                id,
                format!(
                    "a curve inside one {} two of its faces share, which is no generator of \
                     the surface",
                    surface.kind().name()
                ),
                ONE_SURFACE_LANE,
            ));
        }
    };
    Ok(Chosen {
        description,
        carrier: None,
        deferred: Some(deferred),
        intrinsic: matches!(deferred, Deferred::Generator),
        iso: None,
        between_ends: false,
    })
}

/// **A line direction the import built from a file's geometry, as the
/// kernel's witness**: its bits where it is unit to rounding
/// ([`geom_core::UnitVec3::stated`]), and otherwise a typed refusal
/// naming `id`, as other bad file data refuses: the file boundary never
/// hands the kernel a direction to poison.
fn unit_direction(
    id: u64,
    dir: geom_core::Vec3<f64>,
) -> Result<geom_core::UnitVec3<f64>, StepImportError> {
    geom_core::UnitVec3::stated(dir).ok_or(StepImportError::MalformedRecord {
        id,
        expected: "a line direction unit to rounding",
    })
}

/// **A deferred carrier built through its edge's built ends**: the
/// segment between them, or the surface's generator through one of them
/// that the printed carrier's middle lies on at the file's band (exactly
/// one: a torus has two through a point, a sphere a meridian and a
/// parallel). Where no built end names a generator (both stand on the
/// axis), the generator through the printed middle's own foot on the
/// surface: the ends leave the azimuth free, and the printed value chooses
/// among equally valid models.
fn deferred_carrier(
    id: u64,
    spec: &EdgeSpec,
    surface: &Surface<f64>,
    deferred: Deferred,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Result<Curve3<f64>, StepImportError> {
    if matches!(deferred, Deferred::Segment) {
        let Ok((dir, _)) = (p_end - p_start).checked_direction() else {
            return Err(StepImportError::Topology {
                id,
                what: "an edge inside one plane whose built ends coincide",
            });
        };
        // The plane's curve between two points is the segment; the printed
        // carrier only has to lie on it.
        let segment = Curve3::line(p_start, unit_direction(id, dir)?);
        if file.read(STEP_EDGE_ON_DESCRIPTION, held_gap(&segment, spec)) != Read::Zero {
            return Err(unbuilt(
                file,
                id,
                "a curve inside one plane two of its faces share that is not the segment \
                 between its built ends"
                    .to_owned(),
                ONE_SURFACE_LANE,
            ));
        }
        return Ok(segment);
    }
    let mid = spec.carrier.mid_point(spec.t0, spec.t1);
    // The first of the built ends, then the printed middle, whose
    // generators hold the printed edge: a point a rounding off the axis
    // names an azimuth no generator of the edge runs at.
    let on: Vec<Curve3<f64>> = [p_start, p_end, mid]
        .into_iter()
        .map(|p| {
            generators_through(surface, p)
                .into_iter()
                .filter(|g| file.read(STEP_EDGE_ON_DESCRIPTION, held_gap(g, spec)) == Read::Zero)
                .collect::<Vec<_>>()
        })
        .find(|held| !held.is_empty())
        .unwrap_or_default();
    let generator = match <[Curve3<f64>; 1]>::try_from(on) {
        Ok([g]) => g,
        Err(on) if on.is_empty() => {
            return Err(unbuilt(
                file,
                id,
                format!(
                    "a curve inside one {} that no generator of it through its ends holds at \
                     the file's band",
                    surface.kind().name()
                ),
                ONE_SURFACE_LANE,
            ));
        }
        // Two generators the printed middle lies on: it stands where they
        // cross, and the file's band does not say which one the edge runs
        // along.
        Err(on) => {
            let gap = on.iter().map(|g| held_gap(g, spec)).fold(0.0_f64, f64::max);
            return Err(file.refused(id, file.touching(STEP_EDGE_ON_DESCRIPTION, gap)));
        }
    };
    let carrier = running_with(id, generator, spec, file)?;
    on_description(id, spec, &carrier, file)?;
    Ok(carrier)
}

/// **A periodic surface's own generators through `p`**: the ruling of a
/// cylinder or a cone, the meridian and the parallel of a sphere, and a
/// torus's tube circle and parallel, each the surface's iso through the
/// azimuth (and, on a sphere or a torus, the station or tube angle) of
/// `p`. None where `p` stands on the axis, where no azimuth is read, or on
/// a surface with no periodic chart coordinate.
fn generators_through(surface: &Surface<f64>, p: Point3<f64>) -> Vec<Curve3<f64>> {
    let Some(frame) = surface.frame() else {
        return Vec::new();
    };
    let axis = frame.square().axis;
    let off = p - frame.origin();
    let along = off.dot(axis);
    let across = off - axis * along;
    let Ok((radial, rho)) = across.checked_direction() else {
        return Vec::new();
    };
    match *surface {
        Surface::Cylinder { frame, radius, .. } => {
            let origin = frame.origin();
            // A direction not unit to rounding has no generator to offer.
            geom_core::UnitVec3::stated(axis)
                .map(|axis| Curve3::line(origin + radial * radius, axis))
                .into_iter()
                .collect()
        }
        Surface::Cone {
            frame, half_angle, ..
        } => {
            let apex = frame.origin();
            // The ruling from the apex to `p`: along `+axis` on the nappe
            // that opens that way, against it on the mirror nappe (the
            // kernel's cone is the complete double-nappe locus).
            let (s, c) = half_angle.sin_cos();
            let c = if along < 0.0 { -c } else { c };
            geom_core::UnitVec3::stated(axis * c + radial * s)
                .map(|ruling| Curve3::line(apex, ruling))
                .into_iter()
                .collect()
        }
        Surface::Sphere {
            frame: carrier_frame,
            radius,
            ..
        } => {
            let center = carrier_frame.origin();
            // The station's direction in the meridian half-plane, read as
            // the unit vector it is.
            let Ok((dir, _)) = Vec3::new(rho, along, 0.0).checked_direction() else {
                return Vec::new();
            };
            vec![
                Curve3::circle(center, radial.cross(axis), radius, radial),
                Curve3::circle(
                    center + axis * (radius * dir.y),
                    axis,
                    radius * dir.x,
                    frame.square().u_ref,
                ),
            ]
        }
        Surface::Torus {
            frame: carrier_frame,
            major_radius,
            minor_radius,
            ..
        } => {
            let center = carrier_frame.origin();
            // The tube's direction at `p` in the meridian half-plane,
            // (cos v, sin v) read as the unit vector it is, not an angle.
            let Ok((tube, _)) = Vec3::new(rho - major_radius, along, 0.0).checked_direction()
            else {
                return Vec::new();
            };
            let (cv, sv) = (tube.x, tube.y);
            vec![
                Curve3::circle(
                    center + radial * major_radius,
                    radial.cross(axis),
                    minor_radius,
                    radial,
                ),
                Curve3::circle(
                    center + axis * (minor_radius * sv),
                    axis,
                    major_radius + minor_radius * cv,
                    frame.square().u_ref,
                ),
            ]
        }
        Surface::Plane { .. }
        | Surface::Spindle { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => Vec::new(),
    }
}

/// **A built carrier turned to run with the printed one**: its sense is
/// the printed carrier's at the printed middle, read where the built one
/// passes nearest. A built carrier crossing the printed one there is not
/// its component, and refuses as the file's.
fn running_with(
    id: u64,
    built: Curve3<f64>,
    spec: &EdgeSpec,
    file: FileBand,
) -> Result<Curve3<f64>, StepImportError> {
    let half = geom::mid_param(spec.t0, spec.t1);
    let (mid, tangent) = spec.carrier.ders1(half);
    let Some(at) = param_at(&built, mid) else {
        return Err(unbuilt(
            file,
            id,
            "a built spline carrier the printed carrier's middle does not project onto, so \
             its sense is not read"
                .to_owned(),
            "the spline foot's projection (`geom::NurbsCurve3::project`)",
        ));
    };
    let along = built.deriv(at);
    let cosine = along.dot(tangent) / (along.norm() * tangent.norm());
    match decide(STEP_BUILT_SENSE, Margin::of(cosine), file.band()) {
        Ok(Sign::Positive) => Ok(built),
        Ok(Sign::Negative) => built.reversed().ok_or(StepImportError::Topology {
            id,
            what: "internal: a built carrier that does not reverse",
        }),
        Ok(Sign::Zero) | Err(_) => Err(file.refused(id, file.ambiguous(STEP_BUILT_SENSE, cosine))),
    }
}

/// **The parameter of a built spline's end `p`**, projected from `seed`.
/// An end the run's band reads at a domain end is that end: a projection
/// lands a hair inside it, and an interval a hair short of an iso's own
/// domain is no iso of the wall.
fn spline_end_param(
    id: u64,
    n: &geom::NurbsCurve3<f64>,
    p: Point3<f64>,
    seed: f64,
    band: geom_core::Band,
) -> Result<f64, StepImportError> {
    let (lo, hi) = n.domain();
    for end in [lo, hi] {
        let gap = n.eval(end).distance(p);
        if decide(STEP_AT_DOMAIN_END, Margin::of(gap), band).is_ok_and(|s| s.is_zero()) {
            return Ok(end);
        }
    }
    n.project_from_seed(p, seed)
        .map(|f| f.t)
        .map_err(|_| StepImportError::Topology {
            id,
            what: "a built spline carrier whose end does not project onto it",
        })
}

/// Whether a built spline carrier's end stands at one of its domain's
/// ends, read at the run's band.
pub(crate) const STEP_AT_DOMAIN_END: &str = "step_at_domain_end";

/// The parameter of `p`'s foot on `c`: the closed-form inverse of an
/// analytic carrier, the projection of a spline one ([`spline_foot`]).
fn param_at(c: &Curve3<f64>, p: Point3<f64>) -> Option<f64> {
    match c {
        Curve3::Nurbs(n) => spline_foot(n, p).map(|(t, _)| t),
        _ => c.param_near(p, 0.0),
    }
}

/// `p`'s distance from `c`, at its foot.
fn gap_to(c: &Curve3<f64>, p: Point3<f64>) -> f64 {
    match c {
        Curve3::Nurbs(n) => spline_foot(n, p).map_or(f64::INFINITY, |(_, d)| d),
        _ => c
            .param_near(p, 0.0)
            .map_or(f64::INFINITY, |t| c.eval(t).distance(p)),
    }
}

/// **`p`'s foot on a spline carrier**, its parameter and distance: the
/// projection ([`geom::NurbsCurve3::project`]).
fn spline_foot(n: &geom::NurbsCurve3<f64>, p: Point3<f64>) -> Option<(f64, f64)> {
    n.project(p).ok().map(|f| (f.t, f.distance))
}

/// The farthest of the printed carrier's ends, quarters and middle from
/// `built`: how far the printed edge stands off a candidate.
fn held_gap(built: &Curve3<f64>, spec: &EdgeSpec) -> f64 {
    [0.0, 0.25, 0.5, 0.75, 1.0]
        .into_iter()
        .map(|f| spec.carrier.eval(spec.t0 + (spec.t1 - spec.t0) * f))
        .map(|p| gap_to(built, p))
        .fold(0.0_f64, f64::max)
}

/// **D7 step 2's verification**: the printed carrier lies on the built one
/// within the file's band, else the file's statements are no description
/// there. The gap read is [`description_gap`].
fn on_description(
    id: u64,
    spec: &EdgeSpec,
    built: &Curve3<f64>,
    file: FileBand,
) -> Result<(), StepImportError> {
    file.agree_at(id, STEP_EDGE_ON_DESCRIPTION, description_gap(spec, built))
}

/// **How far the printed carrier stands off `built` over its trim**: the
/// whole-trim certificate ([`crate::on_built::printed_on`]) where an arm
/// serves the pair, a non-finite one included, which reads as a miss in
/// band; else, where no arm serves it, the farthest of the printed
/// carrier's ends and middle from it, a sampled read
/// (`work/exch/the-printed-on-built-certificate-has-no-arm-for-a-built-spline.md`).
fn description_gap(spec: &EdgeSpec, built: &Curve3<f64>) -> f64 {
    if let Some(bound) = crate::on_built::printed_on(spec, built) {
        return bound;
    }
    let c = &spec.carrier;
    [
        c.eval(spec.t0),
        c.mid_point(spec.t0, spec.t1),
        c.eval(spec.t1),
    ]
    .into_iter()
    .map(|p| gap_to(built, p))
    .fold(0.0_f64, f64::max)
}

/// **A spline wall's own boundary column or row the printed carrier
/// lies along** (the iso rung): matched by locus ([`column_match`]), and
/// carried on the column itself, or on the straight line its ends span
/// where the column reads as that straight map; the image is on the
/// carrier's own parameter.
fn iso_of_wall(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    one_record: bool,
    file: FileBand,
) -> Result<Option<Chosen>, StepImportError> {
    let walls: Vec<(Side, &geom::NurbsSurface<f64>)> = [(Side::Plus, sa), (Side::Minus, sb)]
        .into_iter()
        .filter_map(|(side, s)| match s {
            Surface::Nurbs(p) if !p.is_placeholder() => Some((side, p.as_ref())),
            _ => None,
        })
        .collect();
    if walls.is_empty() {
        return Ok(None);
    }
    let structure = |source| StepImportError::WallColumnStructure { id, source };
    // A column the printed carrier repeats, beside any surface; a row
    // only between two walls, or on one: beside an analytic face a row is
    // the pair's section, or a cap's rim.
    let both = walls.len() == 2 || one_record;
    let chosen = |wall, carrier, map: IsoMap| Chosen {
        iso: Some(map),
        ..Chosen::built(
            Description::Iso {
                wall,
                image: map.image(0.0, 1.0),
            },
            carrier,
        )
    };
    for column in [true, false] {
        for end in [false, true] {
            for &(wall, net) in &walls {
                let iso = if column {
                    geom_brep::boundary_iso_u(net, end)
                } else {
                    geom_brep::boundary_iso_v(net, end)
                }
                .map_err(structure)?;
                let ((d0, d1), (r0, r1)) = if column {
                    (net.knot_values_u().domain(), net.knot_values_v().domain())
                } else {
                    (net.knot_values_v().domain(), net.knot_values_u().domain())
                };
                let fixed = if end { d1 } else { d0 };
                let own = IsoMap {
                    fixed,
                    column,
                    running: Running::Own,
                };
                // A spline row is matched between two walls only: beside an
                // analytic face the row is a section.
                if !(column || both) {
                    continue;
                }
                match column_match(spec, &iso, (p_start, p_end), file) {
                    Some(ColumnMatch::Column) => {
                        if let Some((carrier, running)) =
                            column_carrier(id, net, (column, end), iso, spec, file)?
                        {
                            let map = IsoMap { running, ..own };
                            return Ok(Some(chosen(
                                wall,
                                Curve3::Nurbs(std::sync::Arc::new(carrier)),
                                map,
                            )));
                        }
                    }
                    Some(ColumnMatch::Line {
                        origin,
                        chord,
                        forward,
                    }) => {
                        let (dir, len) = chord.direction_and_length();
                        let (from, to) = if forward { (r0, r1) } else { (r1, r0) };
                        let map = IsoMap {
                            fixed,
                            column,
                            running: Running::Chord { from, to, len },
                        };
                        let line = Curve3::line(origin, unit_direction(id, dir)?);
                        return Ok(Some(chosen(wall, line, map)));
                    }
                    None => {}
                }
            }
        }
    }
    Ok(None)
}

/// **A cap's rim that is the wall's own boundary row** (`rational_only`:
/// a rational row, before the section; otherwise any row the plane × wall
/// section does not build, such as one whose weights do not factor):
/// beside a plane, the boundary row
/// or column the plane holds (every control in it at the file's band, so
/// the rational iso lies in it) and the printed edge lies on, read at the
/// file's band by the segment gap where the row is straight as a locus
/// ([`segment_gap`]) and by [`description_gap`] otherwise. The wall's exact iso is the carrier, on its
/// own parameter, or on the reversed wall's where the edge runs against
/// it.
fn cap_row(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    file: FileBand,
    rational_only: bool,
) -> Result<Option<Chosen>, StepImportError> {
    let walls: Vec<(Side, &geom::NurbsSurface<f64>)> = [(Side::Plus, sa), (Side::Minus, sb)]
        .into_iter()
        .filter_map(|(side, s)| match s {
            Surface::Nurbs(p) if !p.is_placeholder() => Some((side, p.as_ref())),
            _ => None,
        })
        .collect();
    let structure = |source| StepImportError::WallColumnStructure { id, source };
    let chosen = |wall, carrier, map: IsoMap| Chosen {
        iso: Some(map),
        ..Chosen::built(
            Description::Iso {
                wall,
                image: map.image(0.0, 1.0),
            },
            carrier,
        )
    };
    for &(wall, net) in &walls {
        let cap = match (wall, sa, sb) {
            (
                Side::Plus,
                _,
                Surface::Plane {
                    frame: plane_frame, ..
                },
            )
            | (
                Side::Minus,
                Surface::Plane {
                    frame: plane_frame, ..
                },
                _,
            ) => {
                let (origin, normal) = (plane_frame.origin(), plane_frame.axis());
                (origin, normal)
            }
            _ => continue,
        };
        let Ok((normal, _)) = cap.1.checked_direction() else {
            continue;
        };
        for column in [true, false] {
            for end in [false, true] {
                let iso = if column {
                    geom_brep::boundary_iso_u(net, end)
                } else {
                    geom_brep::boundary_iso_v(net, end)
                }
                .map_err(structure)?;
                if rational_only && iso.constant_weight().is_some() {
                    continue;
                }
                let off_plane = iso
                    .control()
                    .iter()
                    .map(|q| (*q - cap.0).dot(normal).abs())
                    .fold(0.0_f64, f64::max);
                if file.read(STEP_ISO_IN_PLANE, off_plane) != Read::Zero {
                    continue;
                }
                let (fixed_domain, running_domain) = if column {
                    (net.knot_values_u().domain(), net.knot_values_v().domain())
                } else {
                    (net.knot_values_v().domain(), net.knot_values_u().domain())
                };
                let fixed = if end { fixed_domain.1 } else { fixed_domain.0 };
                // A straight rim (the iso straight at the band, its ends the
                // edge's) is the line its ends span, on the chord's arc
                // length, as a straight column is: read off the iso, not
                // the printed carrier's type, which only has to lie on it.
                // The chord's affine image holds only where the iso reads as
                // the straight MAP (INV-C5, [`line_on_column`]); a re-timed
                // straight row is carried on itself, below. A polynomial row
                // reaches this arm only where the plane × wall section does
                // not build; measured, a re-timed straight polynomial row in a
                // cap plane takes that section, which refuses it
                // (`work/exch/a-retimed-straight-row-in-a-cap-plane-refuses-at-the-section.md`),
                // so no committed file reaches the arm with one.
                let straight = line_on_column(&iso, (p_start, p_end), file);
                if let Some(LineOnColumn::Chord {
                    origin,
                    chord,
                    forward,
                }) = straight
                {
                    let (dir, len) = chord.direction_and_length();
                    let line = Curve3::line(origin, unit_direction(id, dir)?);
                    if file.read(STEP_EDGE_ON_DESCRIPTION, description_gap(spec, &line))
                        != Read::Zero
                    {
                        continue;
                    }
                    let (r0, r1) = running_domain;
                    let (from, to) = if forward { (r0, r1) } else { (r1, r0) };
                    let map = IsoMap {
                        fixed,
                        column,
                        running: Running::Chord { from, to, len },
                    };
                    return Ok(Some(chosen(wall, line, map)));
                }
                // A row straight as a locus is certified as the segment it
                // covers; any other is read by the one gap, against itself.
                let held = match straight {
                    Some(LineOnColumn::Locus) => segment_gap(spec, &iso, (p_start, p_end), file),
                    _ => None,
                }
                .unwrap_or_else(|| {
                    description_gap(spec, &Curve3::Nurbs(std::sync::Arc::new(iso.clone())))
                });
                if file.read(STEP_EDGE_ON_DESCRIPTION, held) != Read::Zero {
                    continue;
                }
                let Some((carrier, running)) =
                    column_carrier(id, net, (column, end), iso, spec, file)?
                else {
                    continue;
                };
                let map = IsoMap {
                    fixed,
                    column,
                    running,
                };
                return Ok(Some(chosen(
                    wall,
                    Curve3::Nurbs(std::sync::Arc::new(carrier)),
                    map,
                )));
            }
        }
    }
    Ok(None)
}

/// Whether the printed edge runs along a wall's iso at its middle: the
/// cosine of their tangents there, read at the file's band.
fn runs_along(iso: &geom::NurbsCurve3<f64>, spec: &EdgeSpec, file: FileBand) -> bool {
    let (mid, tangent) = spec.carrier.ders1(geom::mid_param(spec.t0, spec.t1));
    spline_foot(iso, mid).is_some_and(|(t, _)| {
        let along = iso.deriv(t);
        let cosine = along.dot(tangent) / (along.norm() * tangent.norm());
        decide(STEP_BUILT_SENSE, Margin::of(cosine), file.band())
            .is_ok_and(|s| matches!(s, Sign::Positive))
    })
}

/// Whether a wall's polynomial iso is straight: each control's distance
/// from its end-to-end segment, read at the file's band.
pub(crate) const STEP_ISO_STRAIGHT: &str = "step_iso_straight";

/// Whether a straight iso's ends are an edge's printed vertices, read at
/// the file's band.
pub(crate) const STEP_ISO_ENDS: &str = "step_iso_ends";

/// **A wall's boundary column or row as the edge's carrier**, on the
/// column's own parameter where the edge runs with it, or on the wall's
/// reversed column, whose parameter `t` is the wall's running coordinate
/// `lo + hi − t` ([`Running::Mirror`]; a knot vector that is its own
/// reflection), where the edge runs against it. The one home of that
/// choice for every rung that carries an edge on a column (the iso rung,
/// a cap's rim). `None` where the edge runs along neither.
fn column_carrier(
    id: u64,
    net: &geom::NurbsSurface<f64>,
    (column, end): (bool, bool),
    iso: geom::NurbsCurve3<f64>,
    spec: &EdgeSpec,
    file: FileBand,
) -> Result<Option<(geom::NurbsCurve3<f64>, Running)>, StepImportError> {
    if runs_along(&iso, spec, file) {
        return Ok(Some((iso, Running::Own)));
    }
    let reversed = if column {
        net.reversed_v()
    } else {
        net.reversed_u()
    };
    let Ok(reversed) = reversed else {
        return Ok(None);
    };
    let back = if column {
        geom_brep::boundary_iso_u(&reversed, end)
    } else {
        geom_brep::boundary_iso_v(&reversed, end)
    }
    .map_err(|source| StepImportError::WallColumnStructure { id, source })?;
    if !runs_along(&back, spec, file) {
        return Ok(None);
    }
    let running_domain = if column {
        net.knot_values_v().domain()
    } else {
        net.knot_values_u().domain()
    };
    let (lo, hi) = running_domain;
    Ok(Some((back, Running::Mirror { lo, hi })))
}

/// How the edge on a wall's column is carried ([`column_match`]).
enum ColumnMatch {
    /// The column itself ([`column_carrier`]).
    Column,
    /// The line from `origin` along `chord`, its chart image affine in
    /// the line's parameter (`forward` where the edge starts at the
    /// column's start).
    Line {
        origin: Point3<f64>,
        chord: Vec3<f64>,
        forward: bool,
    },
}

/// **The iso rung's one LOCUS match** of an edge to a wall's column, and
/// what the edge is then carried on. The column's own law chooses the
/// carrier, and the printed carrier, of whatever kind, has only to lie on
/// that carrier over its trim, certified ([`crate::on_built`]) and read
/// Zero at the file's band; the printed carrier's kind chooses nothing:
///
/// - a printed spline the column repeats bit for bit is the column;
/// - a column whose ends are the edge's vertices and that reads as the
///   straight MAP ([`line_on_column`], INV-C5) carries the edge on its
///   line, with the affine chart image that map certifies;
/// - one straight at the band but re-timed along it is the segment as a
///   locus, carried on itself ([`segment_gap`]);
/// - a curved one is certified against a circle: the printed one, or,
///   under a printed spline, the column's own as a witness, with the
///   spline certified on the arc the column covers
///   ([`circle_column_match`]). Any other printed carrier on a curved
///   column is withheld
///   (`work/exch/the-printed-on-built-certificate-has-no-arm-for-a-built-spline.md`).
///
/// The edge is carried on the column itself ([`column_carrier`]) but for
/// the straight map. `None` withholds the column.
fn column_match(
    spec: &EdgeSpec,
    iso: &geom::NurbsCurve3<f64>,
    ends: (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Option<ColumnMatch> {
    if let Curve3::Nurbs(n) = &spec.carrier
        && crate::on_built::bitwise_twin(n, iso)
    {
        return Some(ColumnMatch::Column);
    }
    let on = |gap: Option<f64>| {
        gap.is_some_and(|gap| file.read(STEP_EDGE_ON_DESCRIPTION, gap) == Read::Zero)
    };
    match line_on_column(iso, ends, file) {
        Some(LineOnColumn::Chord {
            origin,
            chord,
            forward,
        }) => {
            let line = Curve3::line(origin, geom_core::UnitVec3::chord(chord).0);
            on(crate::on_built::printed_on(spec, &line)).then_some(ColumnMatch::Line {
                origin,
                chord,
                forward,
            })
        }
        Some(LineOnColumn::Locus) => {
            on(segment_gap(spec, iso, ends, file)).then_some(ColumnMatch::Column)
        }
        None => match &spec.carrier {
            Curve3::Circle(circle) => {
                circle_column_match(iso, circle, ends, None, file).then_some(ColumnMatch::Column)
            }
            Curve3::Nurbs(printed) => {
                let witness = column_circle(iso, ends, file)?;
                circle_column_match(iso, &witness, ends, Some((printed, spec)), file)
                    .then_some(ColumnMatch::Column)
            }
            _ => None,
        },
    }
}

/// **How far the printed carrier stands off a straight column it would be
/// carried on as a locus** ([`LineOnColumn::Locus`]), a certified bound.
/// The column covers the segment between its ends within the farthest of
/// its controls from that segment, `δ`: each column point lies in its
/// controls' hull, and its projection on the segment's line runs
/// continuously from one end to the other, so every segment point has a
/// column point within `δ`. The printed carrier lies within its own bound
/// of that segment ([`crate::on_built::printed_on_segment`]), so within
/// the sum of the two of the column. `None` where the column's ends are
/// not the edge's vertices; a bound that certifies nothing is non-finite.
fn segment_gap(
    spec: &EdgeSpec,
    iso: &geom::NurbsCurve3<f64>,
    ends: (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Option<f64> {
    let control = iso.control();
    let (first, last) = (*control.first()?, *control.last()?);
    ends_order((first, last), ends, file)?;
    let spread = control
        .iter()
        .map(|&p| crate::on_built::point_segment_sup(p, (first, last)))
        .fold(0.0_f64, geom_core::interval::max_bound);
    let printed = crate::on_built::printed_on_segment(spec, (first, last));
    Some(geom_core::interval::add_up(printed, spread))
}

/// What a straight column is to the edge whose ends it spans
/// ([`line_on_column`]).
enum LineOnColumn {
    /// The straight MAP: the line from `origin` along `chord`, its chart
    /// image affine (`forward` where the edge starts at the column's
    /// start).
    Chord {
        origin: Point3<f64>,
        chord: Vec3<f64>,
        forward: bool,
    },
    /// The segment as a locus only: a column straight at the band but
    /// re-timed along it, carried on itself.
    Locus,
}

/// **A wall's column whose ends are the edge's printed vertex pair, read
/// as a line** — the one home of the line arm, for the iso rung and a
/// cap's rim. The ends read Zero against the vertices, either way round
/// ([`STEP_ISO_ENDS`]). Where the column reads as the straight MAP
/// ([`crate::recognize_curve::straight_map`]: its Greville hull, which
/// bounds the column's distance from the affine chord map everywhere and
/// so its straightness too) it is [`LineOnColumn::Chord`]; otherwise,
/// where every control lies on the end-to-end segment at the band
/// ([`STEP_ISO_STRAIGHT`], one constant weight), it is the segment as a
/// locus, [`LineOnColumn::Locus`]. `None` withholds it.
fn line_on_column(
    iso: &geom::NurbsCurve3<f64>,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Option<LineOnColumn> {
    let control = iso.control();
    let (first, last) = (*control.first()?, *control.last()?);
    let chord = last - first;
    let len2 = chord.dot(chord);
    if !(len2.is_finite() && len2 > 0.0) {
        return None;
    }
    let zero = |name, gap: f64| file.read(name, gap) == Read::Zero;
    let order = ends_order((first, last), (p_start, p_end), file)?;
    let (origin, chord, forward) = if order.forward {
        (first, chord, true)
    } else {
        (last, -chord, false)
    };
    if crate::recognize_curve::straight_map(iso, file).is_some() {
        return Some(LineOnColumn::Chord {
            origin,
            chord,
            forward,
        });
    }
    iso.constant_weight()?;
    let (line, length) = geom::Line::chord(first, last);
    control
        .iter()
        .all(|&p| {
            line.param(p).is_finite()
                && zero(STEP_ISO_STRAIGHT, line.distance_within(p, (0.0, length)))
        })
        .then_some(LineOnColumn::Locus)
}

/// **A wall's column whose locus and coverage are certified against a
/// circle**: the edge's printed circle (a promoted spline's, or a `CIRCLE`
/// the file states), or, for a printed spline, the column's own circle as
/// a witness ([`column_circle`]); its ends the edge's printed vertices
/// ([`ends_order`]). The column's distance from the ARC of that circle it
/// covers ([`crate::recognize_curve::arc_residual`], the promotion's own
/// certificate) reads Zero at the file's band
/// ([`STEP_ISO_CIRCLE`]), and it turns about the circle's axis in the
/// edge's sense, or against it where its ends are the vertices the
/// other way round (then it is about the axis reversed, and
/// [`column_carrier`] carries the edge on the reversed column); a closed
/// column on a self-loop is read both ways.
///
/// A printed spline (`printed`) must also cover the same arc: its own
/// arc residual against the circle, turning from the edge's start in the
/// circle's sense over its whole domain, summed with the column's, reads
/// Zero ([`STEP_EDGE_ON_DESCRIPTION`]). The column covers the arc within
/// its residual (its azimuth runs continuously over the arc), so every
/// printed point lies within the sum of the column.
///
/// **What carrying the edge on the column moves.** A printed circle lies
/// on the arc the column covers within the column's residual; a promoted
/// spline lies within its promotion residual of that circle besides, and
/// a printed spline read here within the sum above. The edge's
/// `CarrierBuilt` reports the move measured at five samples
/// ([`held_gap`]), not either bound.
fn circle_column_match(
    iso: &geom::NurbsCurve3<f64>,
    circle: &geom::Circle<f64>,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    printed: Option<(&geom::NurbsCurve3<f64>, &EdgeSpec)>,
    file: FileBand,
) -> bool {
    let (Some(&first), Some(&last)) = (iso.control().first(), iso.control().last()) else {
        return false;
    };
    let Some(order) = ends_order((first, last), (p_start, p_end), file) else {
        return false;
    };
    let closed = file.read(
        crate::interpret::STEP_RECOGNIZED_CLOSURE,
        first.distance(last),
    ) == Read::Zero;
    // A closed column's ends read both ways round: it runs with the edge
    // about the circle's axis or against it about the reversed one.
    [
        (order.forward, circle.frame().axis()),
        (order.reversed, -circle.frame().axis()),
    ]
    .into_iter()
    .filter(|&(holds, _)| holds)
    .any(|(_, axis)| {
        let arc = |curve: &geom::NurbsCurve3<f64>, axis: Vec3<f64>| {
            crate::recognize_curve::arc_residual(
                curve,
                (
                    circle.frame().origin(),
                    axis,
                    circle.frame().u_ref(),
                    circle.radius(),
                ),
                closed,
            )
            .map(|(residual, _)| residual)
        };
        let Some(column) = arc(iso, axis).filter(|&r| file.read(STEP_ISO_CIRCLE, r) == Read::Zero)
        else {
            return false;
        };
        // A printed spline turns from the edge's start in the circle's
        // own sense, over its whole domain.
        printed.is_none_or(|(spline, spec)| {
            let (lo, hi) = spline.domain();
            spec.t0 <= lo
                && spec.t1 >= hi
                && arc(spline, circle.frame().axis()).is_some_and(|own| {
                    let gap = geom_core::interval::add_up(own, column);
                    file.read(STEP_EDGE_ON_DESCRIPTION, gap) == Read::Zero
                })
        })
    })
}

/// **The circle a curved column is certified against where the printed
/// carrier is no circle**: the curve recognizer's estimate of the column
/// ([`crate::recognize_curve::try_circle`]), a witness frame only, wound
/// in the edge's sense (its axis reversed where the column runs from the
/// edge's end). Both the column and the printed spline are then certified
/// against it ([`circle_column_match`]). `None` where the column reads as
/// no circle at the band.
fn column_circle(
    iso: &geom::NurbsCurve3<f64>,
    ends: (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Option<geom::Circle<f64>> {
    let (&first, &last) = (iso.control().first()?, iso.control().last()?);
    let order = ends_order((first, last), ends, file)?;
    let Ok(Some((Curve3::Circle(circle), _, _))) = crate::recognize_curve::try_circle(iso, file)
    else {
        return None;
    };
    if order.forward {
        return Some(circle);
    }
    match Curve3::circle(
        circle.frame().origin(),
        -circle.frame().axis(),
        circle.radius(),
        circle.frame().u_ref(),
    ) {
        Curve3::Circle(reversed) => Some(reversed),
        _ => None,
    }
}

/// Which way round a wall's column's ends `(first, last)` are the edge's
/// printed vertices, each end read Zero at the file's band
/// ([`STEP_ISO_ENDS`]): `forward` where the column starts at the edge's
/// start, `reversed` where it starts at its end — both for a closed
/// column on a self-loop. The one reading of a column's ends for every
/// arm of the iso rung. `None` where neither holds.
fn ends_order(
    (first, last): (Point3<f64>, Point3<f64>),
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Option<EndsOrder> {
    let on = |a: Point3<f64>, b: Point3<f64>| {
        file.read(STEP_ISO_ENDS, a.distance(p_start)) == Read::Zero
            && file.read(STEP_ISO_ENDS, b.distance(p_end)) == Read::Zero
    };
    let order = EndsOrder {
        forward: on(first, last),
        reversed: on(last, first),
    };
    (order.forward || order.reversed).then_some(order)
}

/// [`ends_order`]'s answer.
#[derive(Clone, Copy)]
struct EndsOrder {
    /// The column starts at the edge's start.
    forward: bool,
    /// The column starts at the edge's end.
    reversed: bool,
}

/// Whether a wall's column lies on the arc of an edge's printed circle (a
/// promoted spline's, or a stated `CIRCLE`) that it covers: the column's
/// [`crate::recognize_curve::arc_residual`] against that circle's frame,
/// in metres, read at the file's band.
pub(crate) const STEP_ISO_CIRCLE: &str = "step_iso_circle";

/// **A carrier's exact structural signature**: variant tag and field
/// bits. Two carriers are the same
/// value exactly where their signatures are equal (the ledger's
/// `CarrierBuilt` test). A periodic cubic is its own variant read through
/// its debug form, which prints every datum round-trip exact; no import
/// path prints one.
fn carrier_sig(carrier: &Curve3<f64>) -> Vec<u64> {
    use geom::CurveData;
    let p = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    match carrier.data() {
        CurveData::Analytic(data) => {
            let tag: u64 = match carrier {
                Curve3::Line(_) => 0,
                Curve3::Circle(_) => 1,
                Curve3::Ellipse(_) => 2,
                Curve3::Spiric { .. } => 3,
                Curve3::Nurbs(_) | Curve3::Periodic(_) => {
                    unreachable!("a spline carrier's data is not analytic")
                }
            };
            // An ellipse stores its centre or its anchor, and the two
            // read as one value: an anchor's walk is three scalars
            // longer, its phase, and a signature is compared whole.
            core::iter::once(tag)
                .chain(
                    data.into_iter()
                        .flat_map(|(_, value)| value.scalars())
                        .map(f64::to_bits),
                )
                .collect()
        }
        CurveData::Nurbs(n) => {
            let knots = n.knot_values();
            let runs: Vec<(f64, usize)> = knots.runs().collect();
            let mut sig = vec![
                4,
                knots.degree() as u64,
                runs.len() as u64,
                n.control().len() as u64,
            ];
            sig.extend(runs.iter().flat_map(|&(k, m)| [k.to_bits(), m as u64]));
            sig.extend(n.control().iter().flat_map(p));
            sig.extend(n.weights().iter().map(|w| w.to_bits()));
            sig
        }
        CurveData::Periodic(n) => core::iter::once(5)
            .chain(format!("{n:?}").bytes().map(u64::from))
            .collect(),
    }
}

/// **Every vertex built** (module docs, step 3), and every deferred
/// carrier with it.
///
/// - **Fixed**: a vertex the description fixes is an end of a wall's whole
///   iso it stands at ([`iso_end`]), the root of a surface around it along
///   a built carrier through it that the surface does not hold
///   ([`rooted`]), or a sphere's pole or a cone's apex its generators cross
///   at ([`singular_point`]). A root the run's band does not decide refuses
///   [`StepImportError::BuildEscalated`]; a vertex every root asked of
///   answers unsupported refuses [`StepImportError::NotYetSupported`].
/// - **Free**: a vertex the description leaves free along one built
///   carrier (a seam's end on a rim, a split along one locus) is the foot
///   of its printed point on that kernel-built carrier: the printed value
///   choosing among equally valid models, not a cache. Certification reads
///   it at ε on every incident carrier.
/// - A deferred carrier is built through its built ends once they are
///   ([`deferred_carrier`]), and may fix the vertices it reaches.
///
/// The worklist roots every vertex it can, builds every deferred carrier
/// whose ends are built, and only then foots one free vertex, in id
/// order; a vertex no built carrier reaches refuses. Every move off the
/// printed point is read at the file's band and reported.
fn build_vertices(
    solid: &SolidSpec,
    sides: &BTreeMap<u64, (usize, usize)>,
    chosen: &mut BTreeMap<u64, Chosen>,
    tol: Tol,
    file: FileBand,
    moves: &mut Vec<StructureNormalization>,
) -> Result<BTreeMap<u64, Point3<f64>>, StepImportError> {
    let eps_band = geom_core::Band::linear(tol).map_err(|_| StepImportError::Topology {
        id: solid.id,
        what: "the run's tolerance makes no band to build at",
    })?;
    // Each vertex's edges and the faces around it.
    let mut around: BTreeMap<u64, (Vec<u64>, Vec<usize>)> = BTreeMap::new();
    for (&id, spec) in &solid.edges {
        let (plus, minus) = sides[&id];
        for v in [spec.start, spec.end] {
            let slot = around.entry(v).or_default();
            if !slot.0.contains(&id) {
                slot.0.push(id);
            }
            for f in [plus, minus] {
                if !slot.1.contains(&f) {
                    slot.1.push(f);
                }
            }
        }
    }
    // A vertex no edge ends at is no vertex of the model: assembly reaches
    // a vertex only through its edges, and a vertex loop's vertex is
    // never recorded (the whole sphere it bounds is built with its own
    // poles). It is left out, so no printed point stands in the model.
    let mut built: BTreeMap<u64, Point3<f64>> = BTreeMap::new();
    let place = |v: u64,
                 point: Point3<f64>,
                 built: &mut BTreeMap<u64, Point3<f64>>,
                 moves: &mut Vec<StructureNormalization>|
     -> Result<(), StepImportError> {
        let printed = solid.vertices[&v];
        let moved = point.distance(printed);
        file.agree_at(v, STEP_VERTEX_BUILT, moved)?;
        if moved > 0.0 {
            moves.push(StructureNormalization {
                face: v,
                kind: NormalizationKind::VertexBuilt { moved_by: moved },
                file_census: FaceCensus::EMPTY,
                kernel_census: FaceCensus::EMPTY,
            });
        }
        built.insert(v, crate::signed_zero::plus_zero_point(point));
        Ok(())
    };
    loop {
        let mut progress = false;
        for (&v, (edges, faces)) in &around {
            if built.contains_key(&v) {
                continue;
            }
            let printed = solid.vertices[&v];
            if let Some(point) = iso_end(chosen, edges, printed, file) {
                place(v, point, &mut built, moves)?;
                progress = true;
                continue;
            }
            let fixed = match rooted(solid, sides, chosen, (edges, faces), printed, eps_band) {
                Ok(Rooted::At(point)) => Some(point),
                Ok(Rooted::Unfixed) => singular_point(solid, faces, printed, file)?,
                Ok(Rooted::Unsupported(what)) => {
                    match singular_point(solid, faces, printed, file)? {
                        Some(point) => Some(point),
                        None => {
                            return Err(not_yet(
                                file,
                                (BuiltEntity::Vertex, v),
                                format!("a vertex its description fixes: {what}"),
                                ROOT_LANE,
                            ));
                        }
                    }
                }
                Err(source) => return Err(escalated_at(file, (BuiltEntity::Vertex, v), source)),
            };
            if let Some(point) = fixed {
                place(v, point, &mut built, moves)?;
                progress = true;
            }
        }
        for (&id, c) in chosen.iter_mut() {
            let (Some(deferred), None) = (c.deferred, &c.carrier) else {
                continue;
            };
            let spec = &solid.edges[&id];
            let ends = (built.get(&spec.start), built.get(&spec.end));
            let ready = match deferred {
                Deferred::Segment => matches!(ends, (Some(_), Some(_))),
                Deferred::Generator => !matches!(ends, (None, None)),
            };
            if !ready {
                continue;
            }
            // A generator through one built end: the other end is fixed
            // on it, or footed there, by the next pass.
            let (a, b) = match ends {
                (Some(&a), Some(&b)) => (a, b),
                (Some(&a), None) => (a, a),
                (None, Some(&b)) => (b, b),
                (None, None) => continue,
            };
            let surface = solid.surface(sides[&id].0);
            c.carrier = Some(deferred_carrier(id, spec, surface, deferred, (a, b), file)?);
            progress = true;
        }
        if progress {
            continue;
        }
        // Nothing fixed: foot the first free vertex on a built carrier
        // through it, intrinsic carriers first; or, where only carriers
        // waiting on it reach it (a whole torus's one vertex), on the one
        // surface around it.
        let unplaced = || around.iter().filter(|(v, _)| !built.contains_key(*v));
        let free = unplaced()
            .find_map(|(&v, (edges, _))| footed(chosen, edges, solid.vertices[&v]).map(|p| (v, p)))
            .or_else(|| {
                unplaced().find_map(|(&v, (_, faces))| {
                    one_surface_around(solid, faces)
                        .and_then(|surface| surface.foot(solid.vertices[&v]))
                        .map(|p| (v, p))
                })
            });
        match free {
            Some((v, point)) => place(v, point, &mut built, moves)?,
            None => break,
        }
    }
    if let Some((&v, _)) = around.iter().find(|(v, _)| !built.contains_key(*v)) {
        return Err(not_yet(
            file,
            (BuiltEntity::Vertex, v),
            "a vertex no built carrier reaches: every edge ending at it waits on it".to_owned(),
            ONE_SURFACE_LANE,
        ));
    }
    Ok(built)
}

/// The one surface record every face around a vertex lies on, if they
/// share one.
fn one_surface_around<'a>(solid: &'a SolidSpec, faces: &[usize]) -> Option<&'a Surface<f64>> {
    let (&first, rest) = faces.split_first()?;
    let record = solid.faces[first].record;
    rest.iter()
        .all(|&f| solid.faces[f].record == record)
        .then_some(solid.surface(first))
}

/// **A chart singularity the vertex's faces' generators cross at**: a
/// sphere's pole or a cone's apex, where the printed point stands at the
/// file's band and every face around the vertex is that one surface.
fn singular_point(
    solid: &SolidSpec,
    faces: &[usize],
    printed: Point3<f64>,
    file: FileBand,
) -> Result<Option<Point3<f64>>, StepImportError> {
    let Some(surface) = one_surface_around(solid, faces) else {
        return Ok(None);
    };
    let point = match *surface {
        Surface::Cone { frame, .. } => frame.origin(),
        Surface::Sphere {
            frame: carrier_frame,
            radius,
            ..
        } => {
            let center = carrier_frame.origin();
            let Some(frame) = surface.frame() else {
                return Ok(None);
            };
            let axis = frame.square().axis;
            let side = if (printed - center).dot(axis) >= 0.0 {
                radius
            } else {
                -radius
            };
            center + axis * side
        }
        _ => return Ok(None),
    };
    Ok((file.read(STEP_VERTEX_BUILT, point.distance(printed)) == Read::Zero).then_some(point))
}

/// **An end of a wall's iso a vertex stands at**: an edge whose
/// description is a spline wall's boundary column or row spans the whole
/// iso, so its ends are the iso's ends, fixed by the description (the
/// corner of a knot-split wall's piece among them, where a crease line
/// meets the wall's rim). The one end the printed point reads Zero
/// against at the file's band; none where both do (a closed iso) or
/// neither.
fn iso_end(
    chosen: &BTreeMap<u64, Chosen>,
    edges: &[u64],
    printed: Point3<f64>,
    file: FileBand,
) -> Option<Point3<f64>> {
    edges.iter().find_map(|e| {
        let c = &chosen[e];
        let (Description::Iso { .. }, Some(Curve3::Nurbs(iso))) = (&c.description, &c.carrier)
        else {
            return None;
        };
        domain_end_at(iso, printed, file)
    })
}

/// The one end of `iso`'s domain the printed point reads Zero against at
/// the file's band ([`iso_end`]).
fn domain_end_at(
    iso: &geom::NurbsCurve3<f64>,
    printed: Point3<f64>,
    file: FileBand,
) -> Option<Point3<f64>> {
    let (lo, hi) = iso.domain();
    let at = |t: f64| {
        let p = iso.eval(t);
        (file.read(STEP_VERTEX_BUILT, p.distance(printed)) == Read::Zero).then_some(p)
    };
    match (at(lo), at(hi)) {
        (Some(p), None) | (None, Some(p)) => Some(p),
        _ => None,
    }
}

/// What the fixing constructions around a vertex answered ([`rooted`]).
enum Rooted {
    /// A definite root: the vertex.
    At(Point3<f64>),
    /// No construction fixes the vertex here: none was asked, or one read
    /// a definite no-root or a decided graze. The vertex is free.
    Unfixed,
    /// Every construction asked answered that it does not solve this
    /// root, or read a sine that is no number: the vertex is fixed by a
    /// construction the kernel does not have.
    Unsupported(String),
}

/// The first root, in edge then face order, of a face's surface along a
/// built intrinsic carrier ending at the vertex that the surface does not
/// hold, the vertex's tangent edges first: any other carrier through a
/// vertex on a tangency grazes one of its two surfaces there, where a root
/// is square-root-conditioned on the tangency's gap. A reading the run's band does not decide is kept and the search
/// goes on, as a climb keeps what it passes over: the first definite root
/// answers, and the first escalation refuses only where none does.
///
/// **Every verdict ends.** A definite no-root and a decided graze say the
/// surface does not fix the vertex along that carrier. An unsupported
/// root, and a graze whose sine is no number (a spline surface has no
/// implicit form to read one from), say nothing: where every construction
/// asked answers so, the vertex is [`Rooted::Unsupported`], never footed.
fn rooted(
    solid: &SolidSpec,
    sides: &BTreeMap<u64, (usize, usize)>,
    chosen: &BTreeMap<u64, Chosen>,
    (edges, faces): (&[u64], &[usize]),
    printed: Point3<f64>,
    band: geom_core::Band,
) -> Result<Rooted, geom_core::Indeterminate> {
    let mut unsettled = None;
    let mut unsupported: Option<String> = None;
    let mut decided = false;
    let tangent_first = edges
        .iter()
        .filter(|e| matches!(chosen[e].description, Description::Tangent))
        .chain(
            edges
                .iter()
                .filter(|e| !matches!(chosen[e].description, Description::Tangent)),
        );
    for &e in tangent_first {
        let c = &chosen[&e];
        let Some(carrier) = c.carrier.as_ref().filter(|_| c.intrinsic) else {
            continue;
        };
        let (plus, minus) = sides[&e];
        let held = [solid.faces[plus].record, solid.faces[minus].record];
        let spec = &solid.edges[&e];
        let extent = geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1);
        for &f in faces {
            if held.contains(&solid.faces[f].record) {
                continue;
            }
            let surface = solid.surface(f);
            match root_on(surface, carrier, printed, extent, band) {
                Ok(Ok(point)) => return Ok(Rooted::At(point)),
                Ok(Err(topo::CornerVerdict::Unsupported { what })) => {
                    unsupported.get_or_insert_with(|| {
                        format!(
                            "face #{}'s surface along edge #{e}: {what}",
                            solid.faces[f].id
                        )
                    });
                }
                Ok(Err(topo::CornerVerdict::Graze { sine })) if !sine.is_finite() => {
                    unsupported.get_or_insert_with(|| {
                        format!(
                            "face #{}'s surface along edge #{e}: its root reads a sine that is \
                             no number",
                            solid.faces[f].id
                        )
                    });
                }
                Ok(Err(
                    topo::CornerVerdict::NoRoot
                    | topo::CornerVerdict::Graze { .. }
                    | topo::CornerVerdict::Ssi(_),
                )) => decided = true,
                Err(source) => {
                    unsettled.get_or_insert(source);
                }
            }
        }
    }
    if let Some(source) = unsettled {
        return Err(source);
    }
    Ok(match unsupported {
        Some(what) if !decided => Rooted::Unsupported(what),
        _ => Rooted::Unfixed,
    })
}

/// **The root of `surface` along `carrier` from `printed`, settled**
/// ([`settled`]): Newton seeded at its own root again, so building the
/// vertex from its own point gives it back. `None` where no root lies
/// along the carrier there.
fn root_on(
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    printed: Point3<f64>,
    extent: geom_core::Lever<f64>,
    band: geom_core::Band,
) -> Result<Result<Point3<f64>, topo::CornerVerdict<f64>>, geom_core::Indeterminate> {
    let mut verdict = None;
    let root = |p: Point3<f64>| -> Result<Option<Point3<f64>>, geom_core::Indeterminate> {
        let Some(seed) = param_at(carrier, p) else {
            return Ok(None);
        };
        Ok(
            match topo::root_along(surface, carrier, seed, extent, band)? {
                Ok(t) => Some(carrier.eval(t)),
                Err(v) => {
                    verdict.get_or_insert(v);
                    None
                }
            },
        )
    };
    Ok(match settled(printed, root)? {
        Some(point) => Ok(point),
        None => Err(verdict.unwrap_or(topo::CornerVerdict::NoRoot)),
    })
}

/// The printed point's foot on the first built carrier ending at the
/// vertex, intrinsic carriers first.
fn footed(
    chosen: &BTreeMap<u64, Chosen>,
    edges: &[u64],
    printed: Point3<f64>,
) -> Option<Point3<f64>> {
    let by_rank = edges
        .iter()
        .filter(|e| chosen[*e].intrinsic)
        .chain(edges.iter().filter(|e| !chosen[*e].intrinsic));
    by_rank
        .into_iter()
        .find_map(|e| settled_foot(chosen[e].carrier.as_ref()?, printed))
}

/// How many times [`settled`] re-reads before it reads the cycle it has
/// entered.
const SETTLE: usize = 8;

/// **A construction of a point from a point, settled**: `make` read again
/// on its own output until a point repeats, and the least point of the
/// cycle it entered (by its coordinates' total order). A foot or a root
/// computed from a seed lands within rounding of the locus, and computing
/// it again from that point may step back (a circle whose frame sits a
/// rounding off its plane alternates between two feet; Newton from either
/// of two seeds lands on the other): the settled point is the one point
/// every member of the cycle returns, so building it again from it gives
/// it back. The build's output is the next import's input, and its
/// export a fixed point.
fn settled(
    start: Point3<f64>,
    mut make: impl FnMut(Point3<f64>) -> Result<Option<Point3<f64>>, geom_core::Indeterminate>,
) -> Result<Option<Point3<f64>>, geom_core::Indeterminate> {
    let key = |p: &Point3<f64>| (p.x.to_bits(), p.y.to_bits(), p.z.to_bits());
    let Some(first) = make(start)? else {
        return Ok(None);
    };
    let mut seen = vec![first];
    for _ in 0..SETTLE {
        let Some(next) = make(seen[seen.len() - 1])? else {
            break;
        };
        if let Some(at) = seen.iter().position(|q| key(q) == key(&next)) {
            return Ok(seen[at..].iter().copied().min_by(|a, b| {
                a.x.total_cmp(&b.x)
                    .then(a.y.total_cmp(&b.y))
                    .then(a.z.total_cmp(&b.z))
            }));
        }
        seen.push(next);
    }
    Ok(seen.last().copied())
}

/// The foot of `printed` on `c`, settled ([`settled`]).
fn settled_foot(c: &Curve3<f64>, printed: Point3<f64>) -> Option<Point3<f64>> {
    let foot = |p: Point3<f64>| Ok(param_at(c, p).map(|t| c.eval(t)));
    settled(printed, foot).ok().flatten()
}

/// A section the band does not certify straight, rebuilt between its
/// built ends (`between_ends`): the segment between them, where the
/// section holds that chord at the file's band (its ends, quarters and
/// middle on the section). Every other carrier as built.
fn rebuilt_in_surface(
    between_ends: bool,
    carrier: Curve3<f64>,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    file: FileBand,
) -> Curve3<f64> {
    if !between_ends {
        return carrier;
    }
    let Ok((dir, len)) = (p_end - p_start).checked_direction() else {
        return carrier;
    };
    let off = [0.0, 0.25, 0.5, 0.75, 1.0]
        .into_iter()
        .map(|f| gap_to(&carrier, p_start + dir * (len * f)))
        .fold(0.0_f64, f64::max);
    // The printed carrier is kept where the segment does not hold, or its
    // direction is not unit to rounding.
    match geom_core::UnitVec3::stated(dir) {
        Some(dir) if file.read(STEP_EDGE_ON_DESCRIPTION, off) == Read::Zero => {
            Curve3::line(p_start, dir)
        }
        _ => carrier,
    }
}

/// **The built interval**: the built ends read on the built carrier by its
/// own inverse ([`crate::geometry::endpoint_params`] for an analytic
/// carrier, the projection for a spline). A spline carrier is cut
/// ([`geom::NurbsCurve3::split_at`]) at the edge's own built ends, so it
/// is the edge's span of the section and nothing past it. A wall's own
/// iso (`whole`) is not cut: its image is on its own parameter, which the
/// interval is read on.
fn params(
    id: u64,
    carrier: Curve3<f64>,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    self_loop: bool,
    whole: bool,
    (tol, file): (Tol, FileBand),
) -> Result<(Curve3<f64>, f64, f64), StepImportError> {
    let Curve3::Nurbs(n) = &carrier else {
        let (t0, t1) = crate::geometry::endpoint_params(id, &carrier, p_start, p_end, self_loop)?;
        return Ok((carrier, t0, t1));
    };
    let (lo, hi) = n.domain();
    if self_loop {
        return Ok((carrier, lo, hi));
    }
    let band = geom_core::Band::linear(tol).map_err(|_| StepImportError::Topology {
        id,
        what: "the run's tolerance makes no band to build at",
    })?;
    let at = |p: Point3<f64>, seed: f64| spline_end_param(id, n, p, seed, band);
    let (t0, t1) = (at(p_start, lo)?, at(p_end, hi)?);
    if t1 <= t0 {
        return Err(unbuilt(
            file,
            id,
            "a built spline carrier whose end reads at or before its start (an edge across \
             a marched loop's start)"
                .to_owned(),
            MARCHED_LANE,
        ));
    }
    if whole {
        return Ok((carrier, t0, t1));
    }
    let cut = |c: &geom::NurbsCurve3<f64>, u: f64, keep_left: bool| {
        c.split_at(u)
            .map(|(left, right)| if keep_left { left } else { right })
            .map_err(|_| StepImportError::Topology {
                id,
                what: "a built spline carrier that does not cut at its edge's end",
            })
    };
    let mut piece = n.as_ref().clone();
    if t1 < hi {
        piece = cut(&piece, t1, true)?;
    }
    if t0 > lo {
        piece = cut(&piece, t0, false)?;
    }
    Ok((Curve3::Nurbs(std::sync::Arc::new(piece)), t0, t1))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod unit_direction_rows {
    use super::{StepImportError, unit_direction};
    use geom_core::Vec3;

    /// **A built line direction is adopted as its bits, or refused typed,
    /// naming the entity**: the file boundary never hands the kernel a
    /// direction to poison. A unit direction keeps every bit; a chord, a
    /// zero vector and a NaN refuse as `MalformedRecord` on the entity.
    /// Red with the door squaring what it is handed.
    #[test]
    fn a_built_direction_is_adopted_or_refused_typed() {
        let d = Vec3::new(0.6, 0.8, 0.0);
        let got = unit_direction(7, d).unwrap().get();
        assert_eq!(
            [got.x.to_bits(), got.y.to_bits()],
            [d.x.to_bits(), d.y.to_bits()]
        );
        for v in [
            Vec3::new(3.0, 0.0, 4.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(f64::NAN, 0.0, 1.0),
        ] {
            assert!(
                matches!(
                    unit_direction(42, v),
                    Err(StepImportError::MalformedRecord { id: 42, .. })
                ),
                "{v:?} refuses typed on its entity"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    /// **A carrier's signature parts an ellipse held at a point as its
    /// centre from one held there on its arc**: one frame and one pair of
    /// semi-axes, two ellipses a semi-axis apart, two signatures. Red
    /// where the signature reads fewer scalars than the walk yields.
    #[test]
    fn a_carriers_signature_parts_a_centre_from_an_anchor() {
        let (at, axis, u) = (Point3::new(0.3, -1.2, 2.5), Vec3::unit_z(), Vec3::unit_x());
        let centred = Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(at, axis, u), 2.0, 0.75));
        let anchored = Curve3::Ellipse(geom::Ellipse::anchored(at, axis, u, 2.0, 0.75));
        assert!(centred.eval(1.0).distance(anchored.eval(1.0)) > 0.5);
        assert_ne!(carrier_sig(&centred), carrier_sig(&anchored));
        assert_eq!(carrier_sig(&anchored), carrier_sig(&anchored.clone()));
        assert_eq!(
            carrier_sig(&anchored).len(),
            carrier_sig(&centred).len() + 3
        );
    }

    /// **The line arm's selection table** ([`line_on_column`]): forward
    /// vertex order answers the column's start control and its chord,
    /// reversed answers its end control and the chord turned; a column
    /// that reads as the straight MAP is a chord, one straight but
    /// re-timed along its line is the segment as a locus only, and every
    /// structural miss withholds the column: a curved column, a varying
    /// weight vector, a vertex the file band reads apart from the
    /// column's end. Writer noise inside the band keeps it: a vertex
    /// record and a wall's control record are different writer objects.
    #[test]
    fn a_straight_column_matches_by_its_ends_and_reads_its_map() {
        let file = FileBand::test(1e-6);
        type Key = Option<Option<([f64; 3], [f64; 3], bool)>>;
        let key = |m: Option<LineOnColumn>| -> Key {
            m.map(|m| match m {
                LineOnColumn::Chord {
                    origin: o,
                    chord: c,
                    forward,
                } => Some(([o.x, o.y, o.z], [c.x, c.y, c.z], forward)),
                LineOnColumn::Locus => None,
            })
        };
        let chord = |o: Point3<f64>, c: Vec3<f64>, f: bool| -> Key {
            Some(Some(([o.x, o.y, o.z], [c.x, c.y, c.z], f)))
        };
        let (a, b) = (Point3::new(0.1, 0.2, 0.3), Point3::new(0.1, 0.2, 0.9));
        let column = |w: f64| {
            let knots = KnotVector::clamped(vec![2.0, 2.0, 5.0, 5.0], 1).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, b], vec![w, w]).unwrap()
        };
        assert_eq!(
            key(line_on_column(&column(1.0), (a, b), file)),
            chord(a, b - a, true),
            "the forward reading"
        );
        assert_eq!(
            key(line_on_column(&column(1.0), (b, a), file)),
            chord(b, a - b, false),
            "a reversed vertex order starts at the column's end"
        );
        assert_eq!(
            key(line_on_column(&column(0.5), (a, b), file)),
            chord(a, b - a, true),
            "a constant weight cancels: the polynomial ruling is the map"
        );
        let varying = geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![a, b],
            vec![1.0, 0.5],
        )
        .unwrap();
        assert!(
            line_on_column(&varying, (a, b), file).is_none(),
            "a varying weight withholds"
        );
        let ulp_off = Point3::new(0.1, 0.2, 0.9 + 1e-15);
        assert!(
            line_on_column(&column(1.0), (a, ulp_off), file).is_some(),
            "writer noise inside the band keeps the column"
        );
        let far_off = Point3::new(0.1, 0.2, 0.9 + 3e-6);
        assert!(
            line_on_column(&column(1.0), (a, far_off), file).is_none(),
            "a vertex the band does not read on the column's end withholds it"
        );
        let deg2 = |mid: f64| {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, Point3::new(0.1, 0.2, mid), b], vec![1.0; 3])
                .unwrap()
        };
        assert_eq!(
            key(line_on_column(&deg2(0.6), (a, b), file)),
            chord(a, b - a, true),
            "a straight degree-2 column at its Greville middle is the map"
        );
        assert_eq!(
            key(line_on_column(&deg2(0.4), (a, b), file)),
            Some(None),
            "a straight degree-2 column re-timed along its line is the locus only"
        );
        let curved = {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, Point3::new(0.4, 0.2, 0.6), b], vec![1.0; 3])
                .unwrap()
        };
        assert!(
            line_on_column(&curved, (a, b), file).is_none(),
            "a column bent past the band is not straight"
        );
    }

    /// **The circle arm matches a column against the PRINTED circle**
    /// ([`circle_column_match`]), never an estimate against an estimate:
    /// a metre-scale (R = 30 m) 90° arc of nine rational-quadratic spans,
    /// promoted, and the piece of it a C⁰-knot split cuts at its first
    /// joint, read at a 1e-12 m band — where two independently estimated
    /// frames of the arc and the piece differ by more than the band. The
    /// piece matches forward; its ends the other way round name the
    /// complement on the printed circle, and the piece run against the
    /// edge on the circle printed with its axis reversed; the piece moved
    /// off the circle by 1e-9 m does not match, and neither does the
    /// piece read against the vertices of a different stretch.
    #[test]
    fn a_column_matches_the_printed_circle_at_a_metre_scale_band() {
        let file = FileBand::test(1e-12);
        let radius = 30.0f64;
        let spans = 9usize;
        #[allow(clippy::cast_precision_loss)]
        let h = core::f64::consts::FRAC_PI_2 / spans as f64;
        let on = |t: f64| Point3::new(radius * t.cos(), radius * t.sin(), 0.0);
        let mut control = vec![on(0.0)];
        let mut weights = vec![1.0];
        let mut knots = vec![0.0; 3];
        for k in 0..spans {
            #[allow(clippy::cast_precision_loss)]
            let (a, b) = (h * k as f64, h * (k + 1) as f64);
            let c = (h / 2.0).cos();
            let m = (a + b) / 2.0;
            control.push(Point3::new(radius / c * m.cos(), radius / c * m.sin(), 0.0));
            weights.push(c);
            control.push(on(b));
            weights.push(1.0);
            if k + 1 < spans {
                #[allow(clippy::cast_precision_loss)]
                knots.extend([(k + 1) as f64; 2]);
            }
        }
        #[allow(clippy::cast_precision_loss)]
        knots.extend([spans as f64; 3]);
        let kv = KnotVector::clamped(knots, 2).unwrap();
        let lifted_arc = geom::NurbsCurve3::new(
            kv.clone(),
            control
                .iter()
                .map(|p| *p + Vec3::new(0.0, 0.0, 1e-9))
                .collect(),
            weights.clone(),
        )
        .unwrap();
        let arc = geom::NurbsCurve3::new(kv, control, weights).unwrap();
        let Ok(Some((Curve3::Circle(circle), _, _))) =
            crate::recognize_curve::try_circle(&arc, file)
        else {
            panic!("the arc promotes");
        };
        let (piece, _) = arc.split_at(1.0).unwrap();
        let kv_piece = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (p0, p1) = (piece.eval(0.0), piece.eval(1.0));
        assert!(
            circle_column_match(&piece, &circle, (p0, p1), None, file),
            "the split piece matches the printed circle"
        );
        // The same ends the other way round, on the circle as printed, name
        // the long way round from `p1` to `p0`: not the piece.
        assert!(
            !circle_column_match(&piece, &circle, (p1, p0), None, file),
            "the complement is not the piece"
        );
        // On the circle printed the other way round (its axis reversed),
        // the edge from `p1` to `p0` IS the piece, run against it.
        let reversed = circle.placed(geom::Frame::reversed);
        assert!(
            circle_column_match(&piece, &reversed, (p1, p0), None, file),
            "a column run against the edge matches the reversed circle"
        );
        let (lifted, _) = lifted_arc.split_at(1.0).unwrap();
        assert!(
            !circle_column_match(&lifted, &circle, (p0, p1), None, file),
            "a column off the circle past the band does not match"
        );
        // Bowed: the piece's ends ON the circle, its middle control pushed
        // 1e-9 m outward, so only the column's interior is off the circle —
        // the ends read Zero and the turning holds, and the deciding read
        // is the residual's.
        let mut bowed_net = piece.control().to_vec();
        let mid = bowed_net[1] - circle.frame().origin();
        bowed_net[1] = bowed_net[1] + mid * (1e-9 / mid.norm());
        let bowed =
            geom::NurbsCurve3::new(kv_piece.clone(), bowed_net, piece.weights().to_vec()).unwrap();
        assert!(
            !circle_column_match(&bowed, &circle, (p0, p1), None, file),
            "a column bowed off the circle between its ends does not match"
        );
        assert!(
            !circle_column_match(&piece, &circle, (on(4.0 * h), on(5.0 * h)), None, file),
            "nor one whose ends are not the edge's"
        );
    }

    /// **A vertex only unsupported roots fix refuses, never footed**
    /// (A4, m1): a vertex where a spline wall meets two cylinders, whose
    /// three fixing roots are the wall along the cylinders' carrier (a
    /// sine that is no number: the wall has no implicit form) and each
    /// cylinder along the other's spline section with the wall (a root
    /// the kernel solves for a plane only). Every construction asked says
    /// nothing, so the vertex refuses not-yet-supported, naming the root
    /// lane; it does not drop to the printed point's foot.
    #[test]
    fn a_vertex_only_unsupported_roots_fix_refuses_naming_the_root_lane() {
        let wall = {
            let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let control = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 0.0, 2.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 2.0),
            ];
            Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap(),
            ))
        };
        let cylinder = |origin: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>| Surface::Cylinder {
            frame: geom::Frame::new(origin, axis, u_ref),
            radius: 1.0,
        };
        let face = |id: u64| FaceSpec {
            id,
            record: id + 100,
            sense: true,
            loops: Vec::new(),
            band: false,
        };
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let stated = [
            (10, wall),
            (11, cylinder(Point3::new(0.5, 1.0, 0.0), z, x)),
            (12, cylinder(Point3::new(0.0, 1.0, 1.0), x, y)),
        ];
        let faces = stated.iter().map(|&(id, _)| face(id)).collect();
        let surfaces = crate::entities::records_of(stated.into_iter().map(|(id, s)| (id + 100, s)));
        let p = Point3::new(0.5, 0.0, 1.0);
        let segment = |a: Point3<f64>, b: Point3<f64>| {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            Curve3::Nurbs(std::sync::Arc::new(
                geom::NurbsCurve3::new(knots, vec![a, b], vec![1.0, 1.0]).unwrap(),
            ))
        };
        // Edge 1 on the cylinders, edges 2 and 3 on the wall and one
        // cylinder each, all from vertex 1 at `p`.
        let carriers = [
            (
                1,
                (1, 2),
                Curve3::line(p, geom::stated_direction(z)),
                (0.0, 1.0),
                Point3::new(0.5, 0.0, 2.0),
            ),
            (
                2,
                (0, 1),
                segment(p, Point3::new(0.5, 0.0, 2.0)),
                (0.0, 1.0),
                Point3::new(0.5, 0.0, 2.0),
            ),
            (
                3,
                (0, 2),
                segment(p, Point3::new(1.0, 0.0, 1.0)),
                (0.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
            ),
        ];
        let mut vertices = BTreeMap::from([(1, p)]);
        let mut edges = BTreeMap::new();
        let mut sides = BTreeMap::new();
        let mut chosen = BTreeMap::new();
        for (id, side, carrier, (t0, t1), far) in carriers {
            vertices.insert(id + 1, far);
            edges.insert(
                id,
                EdgeSpec {
                    start: 1,
                    end: id + 1,
                    carrier: carrier.clone(),
                    t0,
                    t1,
                    reversed: false,
                    curve: None,
                },
            );
            sides.insert(id, side);
            chosen.insert(id, Chosen::built(Description::Intersection, carrier));
        }
        let solid = SolidSpec {
            id: 1,
            faces,
            edges,
            vertices,
            surfaces,
            band_seams: std::collections::BTreeSet::new(),
        };
        let mut moves = Vec::new();
        let refused = build_vertices(
            &solid,
            &sides,
            &mut chosen,
            Tol::witness(),
            FileBand::test(1e-6),
            &mut moves,
        );
        match refused {
            Err(StepImportError::NotYetSupported {
                entity: BuiltEntity::Vertex,
                id: 1,
                lane,
                ..
            }) => assert_eq!(lane, ROOT_LANE, "the refusal names the root lane"),
            other => panic!("the vertex refuses not-yet-supported, never footed: {other:?}"),
        }
    }

    /// Two unit discs folded by `fold` along a chord `2w` long that each
    /// is cut back to: face 0 in the plane `z = 0`, face 1 in the plane
    /// turned `fold` about the chord, and edge 1, the chord. The faces'
    /// only vertices are the chord's two ends; each face runs a whole
    /// disc past them.
    fn folded_discs(fold: f64) -> SolidSpec {
        let w = 0.01_f64;
        let c0 = (1.0 - w * w).sqrt();
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let (low, high) = (Point3::new(c0, -w, 0.0), Point3::new(c0, w, 0.0));
        let phi = w.asin();
        let pi = core::f64::consts::PI;
        // The folded plane's normal, and its direction away from the chord.
        let n = Vec3::new(fold.sin(), 0.0, fold.cos());
        let away = Vec3::new(fold.cos(), 0.0, -fold.sin());
        let disc = |centre: Point3<f64>, axis: Vec3<f64>, u_ref: Vec3<f64>| {
            Curve3::Circle(geom::Circle::new(
                geom::Frame::new(centre, axis, u_ref),
                1.0,
            ))
        };
        let edges = [
            (
                1,
                (1, 2),
                Curve3::line(low, geom::stated_direction(y)),
                (0.0, 2.0 * w),
            ),
            (
                2,
                (2, 1),
                disc(Point3::origin(), z, x),
                (phi, 2.0 * pi - phi),
            ),
            (
                3,
                (1, 2),
                disc(Point3::new(c0, 0.0, 0.0) + away * c0, n, away),
                (phi - pi, pi - phi),
            ),
        ]
        .into_iter()
        .map(|(id, (start, end), carrier, (t0, t1))| {
            (
                id,
                EdgeSpec {
                    start,
                    end,
                    carrier,
                    t0,
                    t1,
                    reversed: false,
                    curve: None,
                },
            )
        })
        .collect();
        let face = |id: u64, arc: u64| FaceSpec {
            id,
            record: id,
            sense: true,
            loops: vec![crate::entities::LoopSpec {
                outer: true,
                uses: [1, arc]
                    .into_iter()
                    .map(|edge| crate::entities::EdgeUse {
                        edge,
                        forward: true,
                    })
                    .collect(),
            }],
            band: false,
        };
        SolidSpec {
            id: 1,
            faces: vec![face(10, 2), face(11, 3)],
            edges,
            vertices: BTreeMap::from([(1, low), (2, high)]),
            surfaces: crate::entities::records_of([(10, z), (11, n)].map(|(id, normal)| {
                let frame = geom::Frame::new(Point3::new(c0, 0.0, 0.0), normal, y);
                (id, Surface::Plane { frame })
            })),
            band_seams: std::collections::BTreeSet::new(),
        }
    }

    /// **A crease between two faces is read over the faces' run, from
    /// below, not over their vertices.** Two unit discs folded along a
    /// 2 cm chord have the chord's ends for their only vertices, and
    /// stand `2 + 2c₀ ≈ 4` m across, rim to rim. Folded by `1e-5` they
    /// are `2e-5` apart at their far side, twenty times the file's
    /// `1e-6`: two planes, and the chord is their crease. Folded by
    /// `1e-7` they are one plane to the file over that run, and the
    /// refusal's gap is the fold over it: never more than the fold over
    /// the faces' true diameter, and most of it. Red against the
    /// vertices or the edge's own length for the reach (the `1e-5` fold
    /// then refuses as parallel planes), against one face's boundary for
    /// both (a gap of `2e-7`), and against the region's box diagonal, a
    /// bound from above, for the gap (`4.47e-7`) or for the decision.
    #[test]
    fn a_crease_is_read_over_its_faces_run() {
        let read = |fold: f64| {
            let solid = folded_discs(fold);
            choose(
                &solid,
                1,
                &solid.edges[&1],
                (0, 1),
                Tol::witness(),
                FileBand::test(1e-6),
            )
        };
        match read(1e-5) {
            Ok(Chosen {
                description: Description::Intersection,
                carrier: Some(Curve3::Line(line)),
                ..
            }) => assert!(
                line.dir.get().cross(Vec3::new(0.0, 1.0, 0.0)).norm() < 1e-12,
                "the crease runs along the chord: {line:?}"
            ),
            other => panic!(
                "two planes the faces' run tells apart meet in their crease: {:?}",
                other.map(|c| c.carrier)
            ),
        }
        match read(1e-7) {
            Err(StepImportError::Interpretation {
                refusal: crate::interpret::Refusal::Touching { reading, gap, .. },
                ..
            }) => {
                assert_eq!(reading, STEP_PLANES_CROSS);
                let across = 2.0 + 2.0 * (1.0 - 1e-4_f64).sqrt();
                assert!(
                    gap <= 1e-7 * across * (1.0 + 1e-9) && gap >= 1e-7 * 0.95 * across,
                    "the fold over the two discs' run, never past their {across} m: {gap:e}"
                );
            }
            other => panic!(
                "one plane to the file over the faces' run: {:?}",
                other.map(|c| c.carrier)
            ),
        }
        // Folded by `2.4e-7` the planes read one within the file's band
        // over the run from below (`9.6e-7`), where the region's box
        // diagonal would read them in the band (`1.07e-6`).
        assert!(
            matches!(
                read(2.4e-7),
                Err(StepImportError::Interpretation {
                    refusal: crate::interpret::Refusal::Touching { .. },
                    ..
                })
            ),
            "decided at the run from below"
        );
    }

    /// **A pair of faces with no region refuses naming the edge at
    /// fault**, whichever edge asked: with edge 2's interval unreadable,
    /// edge 1, whose own carrier and interval are sound, answers with
    /// edge 2's refusal. Red against the asking edge named.
    #[test]
    fn a_face_pair_with_no_region_names_the_edge_whose_carrier_does_not_read() {
        let mut solid = folded_discs(1e-5);
        solid.edges.get_mut(&2).unwrap().t0 = f64::NAN;
        let refused = choose(
            &solid,
            1,
            &solid.edges[&1],
            (0, 1),
            Tol::witness(),
            FileBand::test(1e-6),
        );
        assert!(
            matches!(refused, Err(StepImportError::Topology { id: 2, .. })),
            "{:?}",
            refused.map(|c| c.carrier)
        );
    }

    /// **A plane's cut of a spline wall is read over the faces' run from
    /// below.** The folded discs with the folded face's surface a
    /// bilinear wall in its plane, 30 m long past the chord: the faces'
    /// boundaries stand 4 m across and their region, which holds the
    /// wall's net, over 30. The wall's column crosses the plane at the
    /// fold, and the section's row is built on that crossing's definite
    /// sign: a fold of `1e-6` is `4e-6` over the boundaries' run, in the
    /// file's `1e-6` band, and refuses; a fold of `1e-5` builds the
    /// chord. Red against the section read from above (the `1e-6` fold
    /// reads `3.2e-5` over the region and builds).
    #[test]
    fn a_planes_cut_of_a_spline_wall_is_read_over_its_faces_run_from_below() {
        let read = |fold: f64| {
            let mut solid = folded_discs(fold);
            let c0 = (1.0 - 1e-4_f64).sqrt();
            let (y, z) = (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
            let away = Vec3::new(fold.cos(), 0.0, -fold.sin());
            let mid = Point3::new(c0, 0.0, 0.0);
            let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
            let control = vec![
                mid - y * 2.0 - away * 0.5,
                mid - y * 2.0 + away * 30.0,
                mid + y * 2.0 - away * 0.5,
                mid + y * 2.0 + away * 30.0,
            ];
            let wall = Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(k(), k(), control, vec![1.0; 4]).unwrap(),
            ));
            let plane = Surface::Plane {
                frame: geom::Frame::new(mid, z, y),
            };
            solid.surfaces = crate::entities::records_of([(10, plane), (11, wall)]);
            let run = crate::reach::faces_run(&solid, &[0, 1]).unwrap();
            assert!(
                run.below().length() < 4.0 && run.above().length() > 30.0,
                "the bounds stand apart: {run:?}"
            );
            choose(
                &solid,
                1,
                &solid.edges[&1],
                (0, 1),
                Tol::witness(),
                FileBand::test(1e-6),
            )
        };
        assert!(
            matches!(
                read(1e-5),
                Ok(Chosen {
                    description: Description::Intersection,
                    carrier: Some(_),
                    ..
                })
            ),
            "a fold the boundaries' run tells from a graze builds the cut"
        );
        match read(1e-6) {
            Err(StepImportError::NotYetSupported { what, .. }) => assert!(
                what.contains("the spline-wall section refused"),
                "the crossing is in the file's band over the boundaries' run: {what}"
            ),
            other => panic!(
                "a crossing in the band over the run from below builds nothing: {:?}",
                other.map(|c| c.carrier)
            ),
        }
    }

    /// A closed solid whose lid is the folded pair: two unit cylinders
    /// about `z`, their axes `2c₀` apart so the walls cross along two
    /// rulings 2 cm apart, capped below by one plane and above by two,
    /// `z = 1` over the first lobe and a plane turned `fold` about the
    /// chord over the second. Faces: the two lids (0, 1), the floor (2),
    /// the two walls (3, 4). Edge 1 is the chord, the lids' crease, and
    /// its ends are the lids' only shared vertices.
    fn folded_lid(fold: f64) -> SolidSpec {
        let (w, h) = (0.01_f64, 1.0_f64);
        let c0 = (1.0 - w * w).sqrt();
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let cusp = |side: f64, height: f64| Point3::new(c0, side * w, height);
        let vertices = BTreeMap::from([
            (1, cusp(1.0, 0.0)),
            (2, cusp(-1.0, 0.0)),
            (3, cusp(1.0, h)),
            (4, cusp(-1.0, h)),
        ]);
        let phi = w.asin();
        let pi = core::f64::consts::PI;
        let second = Point3::new(2.0 * c0, 0.0, 0.0);
        // The folded lid's plane through the chord, and the ellipse the
        // second wall cuts it in.
        let n = Vec3::new(fold.sin(), 0.0, fold.cos());
        let away = Vec3::new(fold.cos(), 0.0, -fold.sin());
        let rim = |centre: Point3<f64>| {
            Curve3::Circle(geom::Circle::new(geom::Frame::new(centre, z, x), 1.0))
        };
        let lid_rim = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(second + z * (h - c0 * fold.tan()), n, away),
            1.0 / fold.cos(),
            1.0,
        ));
        let (first_span, second_span) = ((phi, 2.0 * pi - phi), (phi - pi, pi - phi));
        let edges = [
            (
                1,
                (4, 3),
                Curve3::line(cusp(-1.0, h), geom::stated_direction(y)),
                (0.0, 2.0 * w),
            ),
            (2, (3, 4), rim(Point3::new(0.0, 0.0, h)), first_span),
            (3, (4, 3), lid_rim, second_span),
            (4, (1, 2), rim(Point3::origin()), first_span),
            (5, (2, 1), rim(second), second_span),
            (
                6,
                (1, 3),
                Curve3::line(cusp(1.0, 0.0), geom::stated_direction(z)),
                (0.0, h),
            ),
            (
                7,
                (2, 4),
                Curve3::line(cusp(-1.0, 0.0), geom::stated_direction(z)),
                (0.0, h),
            ),
        ]
        .into_iter()
        .map(|(id, (start, end), carrier, (t0, t1))| {
            (
                id,
                EdgeSpec {
                    start,
                    end,
                    carrier,
                    t0,
                    t1,
                    reversed: false,
                    curve: None,
                },
            )
        })
        .collect();
        let face = |id: u64, bound: &[(u64, bool)]| FaceSpec {
            id,
            record: id,
            sense: true,
            loops: vec![crate::entities::LoopSpec {
                outer: true,
                uses: bound
                    .iter()
                    .map(|&(edge, forward)| crate::entities::EdgeUse { edge, forward })
                    .collect(),
            }],
            band: false,
        };
        let plane = |origin: Point3<f64>, normal: Vec3<f64>| Surface::Plane {
            frame: geom::Frame::new(origin, normal, y),
        };
        let wall = |origin: Point3<f64>| Surface::Cylinder {
            frame: geom::Frame::new(origin, z, x),
            radius: 1.0,
        };
        SolidSpec {
            id: 1,
            faces: vec![
                face(10, &[(2, true), (1, true)]),
                face(11, &[(3, true), (1, false)]),
                face(12, &[(4, false), (5, false)]),
                face(13, &[(4, true), (7, true), (2, false), (6, false)]),
                face(14, &[(5, true), (6, true), (3, false), (7, false)]),
            ],
            surfaces: crate::entities::records_of([
                (10, plane(Point3::new(c0, 0.0, h), z)),
                (11, plane(Point3::new(c0, 0.0, h), n)),
                (12, plane(Point3::origin(), -z)),
                (13, wall(Point3::origin())),
                (14, wall(second)),
            ]),
            edges,
            vertices,
            band_seams: std::collections::BTreeSet::new(),
        }
    }

    /// **The folded lid imports whole.** Through the build, the
    /// assembly and the at-rest gate, a solid whose two lids fold `1e-2`
    /// along a 2 cm chord finishes as five faces and seven edges, read
    /// from a file whose ε is `1e-3`. The lids stand 4 cm apart at their
    /// far side, forty times that ε; over their vertices' reach, 2 cm,
    /// the fold reads `2e-4`, a fifth of it. So the built body's own
    /// readings, which lever the crease at its own extent, accept what
    /// the import read over the lids' 4 m run. Folded `1e-4` the lids
    /// are one plane to that file, and the crease refuses. Red against
    /// the lids' vertices or the chord's own length for the import's
    /// reach (the `1e-2` fold refuses as parallel planes).
    ///
    /// The fold and the file's ε are coarse so that the row holds at
    /// every run ε: certification reads the crease's crossing over the
    /// edge's own centimetre, which a finer fold leaves in a coarse
    /// run's band, and a finer fold's crease is too ill-conditioned to
    /// certify at a fine one.
    #[test]
    fn the_folded_lid_imports_whole_where_the_file_tells_its_lids_apart() {
        let tol = Tol::witness();
        let import = |fold: f64| {
            let record = crate::entities::SolidRecord {
                id: 1,
                outer: folded_lid(fold),
                voids: Vec::new(),
            };
            crate::assemble::build_one_solid(&record, tol, geom_core::FileCoincidence::new(1e-3))
                .and_then(|(body, _, _)| crate::gate(body, Some(1), tol))
        };
        let done = import(1e-2).unwrap_or_else(|e| panic!("a 1e-2 fold: {e}"));
        assert_eq!((done.faces().count(), done.edges().count()), (5, 7));
        assert!(
            matches!(
                import(1e-4),
                Err(StepImportError::Interpretation {
                    id: 1,
                    refusal: crate::interpret::Refusal::Touching {
                        reading: STEP_PLANES_CROSS,
                        ..
                    },
                    ..
                })
            ),
            "one plane to the file over the lids' run"
        );
    }

    /// **A wrap edge is the surface's own generator through the printed
    /// seam**: a cylinder's ruling at the printed azimuth (not its chart's
    /// `u = 0`), and a torus's two isos through a point, each on the torus.
    #[test]
    fn the_generators_through_a_point_lie_on_their_surface() {
        let cylinder = Surface::Cylinder {
            frame: geom::Frame::new(
                Point3::new(1.0, 2.0, 3.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 2.0,
        };
        let p = Point3::new(1.0, 4.0, 5.0);
        let rulings = generators_through(&cylinder, p);
        assert_eq!(
            rulings.len(),
            1,
            "a cylinder has one generator through a point"
        );
        assert!(
            gap_to(&rulings[0], p) < 1e-15,
            "the ruling passes through the point"
        );
        let torus = Surface::Torus {
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            major_radius: 3.0,
            minor_radius: 1.0,
        };
        let v: f64 = 0.7;
        let q = Point3::new(0.0, 3.0 + v.cos(), v.sin());
        let isos = generators_through(&torus, q);
        assert_eq!(isos.len(), 2, "a torus has two generators through a point");
        for iso in &isos {
            assert!(
                gap_to(iso, q) < 1e-14,
                "each passes through the point: {iso:?}"
            );
            for t in [0.3, 1.9, 4.4] {
                let r = geom_brep::implicit_residual(&torus, iso.eval(t)).abs();
                assert!(r < 1e-12, "each lies on the torus: {r:e}");
            }
        }
        assert!(
            generators_through(&cylinder, Point3::new(1.0, 2.0, 9.0)).is_empty(),
            "a point on the axis reads no azimuth"
        );
    }

    /// **A curved column matches by its own law, whatever the printed
    /// kind.** A quarter circle's column and its printed spline with the
    /// knots doubled (the same locus and sense, other bits) match, as the
    /// same locus printed as a circle does; the same spline run the other
    /// way round its circle (the complement's sense), or trimmed short of
    /// its domain, is withheld.
    #[test]
    fn a_curved_column_matches_its_printed_spline_by_locus_not_by_kind() {
        use geom_core::spline::KnotVector;
        let file = FileBand::test(1e-9);
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let net = |knots: Vec<f64>, control: Vec<Point3<f64>>| {
            geom::NurbsCurve3::new(
                KnotVector::clamped(knots, 2).unwrap(),
                control,
                vec![1.0, w, 1.0],
            )
            .unwrap()
        };
        let column = net(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], control.clone());
        let reknotted = net(vec![0.0, 0.0, 0.0, 2.0, 2.0, 2.0], control.clone());
        let ends = (column.eval(0.0), column.eval(1.0));
        let edge = |carrier: Curve3<f64>, (t0, t1): (f64, f64)| EdgeSpec {
            start: 1,
            end: 2,
            carrier,
            t0,
            t1,
            reversed: false,
            curve: None,
        };
        let spline = edge(
            Curve3::Nurbs(std::sync::Arc::new(reknotted.clone())),
            (0.0, 2.0),
        );
        let circle = edge(
            Curve3::circle(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                1.0,
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (0.0, core::f64::consts::FRAC_PI_2),
        );
        assert!(
            matches!(
                column_match(&circle, &column, ends, file),
                Some(ColumnMatch::Column)
            ),
            "the printed circle matches"
        );
        assert!(
            matches!(
                column_match(&spline, &column, ends, file),
                Some(ColumnMatch::Column)
            ),
            "the re-knotted spline of the same locus matches"
        );
        let short = edge(Curve3::Nurbs(std::sync::Arc::new(reknotted)), (0.0, 1.5));
        assert!(
            column_match(&short, &column, ends, file).is_none(),
            "a spline trimmed short of its domain is withheld"
        );
        // The three-quarter arc the long way round between the same ends,
        // as three rational quarters.
        let long = geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, -1.0, 0.0),
                Point3::new(0.0, -1.0, 0.0),
                Point3::new(-1.0, -1.0, 0.0),
                Point3::new(-1.0, 0.0, 0.0),
                Point3::new(-1.0, 1.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
            ],
            vec![1.0, w, 1.0, w, 1.0, w, 1.0],
        )
        .unwrap();
        let complement = edge(Curve3::Nurbs(std::sync::Arc::new(long)), (0.0, 3.0));
        assert!(
            column_match(&complement, &column, ends, file).is_none(),
            "the arc the column does not cover is withheld"
        );
    }

    /// **A straight column carries its own spread into the gap**: a
    /// printed line exactly on the segment between a bowed column's ends
    /// stands off the column by the bow, and the gap holds it.
    #[test]
    fn a_line_on_a_bowed_columns_segment_reads_the_bow() {
        use geom_core::spline::KnotVector;
        let file = FileBand::test(1e-9);
        let bow = 1e-3;
        let column = geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(bow, 0.0, 0.3),
                Point3::new(0.0, 0.0, 1.0),
            ],
            vec![1.0; 3],
        )
        .unwrap();
        let (a, b) = (Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 1.0));
        let line = EdgeSpec {
            start: 1,
            end: 2,
            carrier: Curve3::line(a, geom_core::UnitVec3::exact_z()),
            t0: 0.0,
            t1: 1.0,
            reversed: false,
            curve: None,
        };
        let gap = segment_gap(&line, &column, (a, b), file).unwrap();
        let samples: Vec<Point3<f64>> = (0..=4000)
            .map(|k| column.eval(f64::from(k) / 4000.0))
            .collect();
        let true_sup = (0..=400)
            .map(|k| line.carrier.eval(f64::from(k) / 400.0))
            .map(|p| {
                samples
                    .iter()
                    .map(|q| q.distance(p))
                    .fold(f64::INFINITY, f64::min)
            })
            .fold(0.0, f64::max);
        assert!(true_sup > 0.3 * bow, "the bow shows: {true_sup:e}");
        assert!(gap >= true_sup, "sound: {gap:e} < {true_sup:e}");
    }
}
