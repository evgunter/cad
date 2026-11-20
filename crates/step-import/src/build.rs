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
//!    seam generator. A carrier the file band reads off the printed one
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

use crate::entities::{EdgeSpec, FaceSpec, SolidSpec};
use crate::error::{BuiltEntity, StepImportError};
use crate::interpret::{FileBand, Read};
use crate::{FaceCensus, NormalizationKind, StructureNormalization};

/// Whether a printed carrier lies on the description built for it: the
/// farthest of its two ends and its middle from the built carrier, read at
/// the file's band.
pub(crate) const STEP_EDGE_ON_DESCRIPTION: &str = "step_edge_on_description";

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
    /// Every edge, keyed by `EDGE_CURVE` id.
    pub(crate) edges: BTreeMap<u64, Edge>,
    /// Every vertex's built point, keyed by `VERTEX_POINT` id.
    pub(crate) vertices: BTreeMap<u64, Point3<f64>>,
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
        edges,
        vertices,
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

/// [`choose`] of edge `id` alone: a unit test's reading of one edge's
/// selection and carrier.
#[cfg(test)]
pub(crate) fn choose_edge(
    solid: &SolidSpec,
    id: u64,
    tol: Tol,
    file: FileBand,
) -> Result<(), StepImportError> {
    let sides = edge_faces(solid)?;
    choose(solid, id, &solid.edges[&id], sides[&id], tol, file).map(|_| ())
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
    let (sa, sb) = (&solid.faces[plus].surface, &solid.faces[minus].surface);
    let p_start = solid.vertices[&spec.start];
    let p_end = solid.vertices[&spec.end];
    let one_record = surface_sig(sa) == surface_sig(sb);
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
    // their region's diameter from above, and the edge's own extent where
    // that is longer.
    let faces = crate::reach::Reach::of(solid, [plus, minus]).ok_or(StepImportError::Topology {
        id,
        what: "an edge whose faces have no region to read their reach from",
    })?;
    let extent = geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1).max(faces.lever());
    // A cone's pose readings are levered from its apex (the station of a
    // coaxial cylinder's circles is a distance from it): the edge's own
    // diameter is no reach from there.
    let extent = [sa, sb].into_iter().fold(extent, |lever, s| match *s {
        Surface::Cone { apex, .. } => lever.max(region.lever_from(apex)),
        _ => lever,
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
        let tangent = spec.carrier.deriv(geom::mid_param(spec.t0, spec.t1));
        canonical = Curve3::line(Point3::origin(), tangent);
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
    extent: geom_core::Lever<f64>,
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
    let read = |band| read_section(spec, (sa, sb), region, extent, band);
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
    // A straight section is the line it is, read by the recognition that
    // promotes a printed spline ([`crate::recognize_curve::try_line`], at
    // the file's band).
    let carrier = match &carrier {
        Curve3::Nurbs(n) => {
            crate::recognize_curve::try_line(n, file).map_or(carrier.clone(), |(line, _)| line)
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
/// angle levered at the edge's extent, read at the file's band.
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
    extent: geom_core::Lever<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    use topo::SectionVerdict as V;
    let read = |band| read_section(spec, (sa, sb), region, extent, band);
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
            let crossing = normals.0.cross(normals.1).norm() * extent.length();
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
        (Surface::Plane { normal: n1, .. }, Surface::Plane { normal: n2, .. }) => Some((*n1, *n2)),
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
    extent: geom_core::Lever<f64>,
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
    extent: geom_core::Lever<f64>,
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
        let coaxial = |band| match geom_brep::coaxial_tangent_circle(sa, sb, extent, band) {
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
        let segment = Curve3::line(p_start, dir);
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
    let off = p - frame.origin;
    let along = off.dot(axis);
    let across = off - axis * along;
    let Ok((radial, rho)) = across.checked_direction() else {
        return Vec::new();
    };
    match *surface {
        Surface::Cylinder { origin, radius, .. } => {
            vec![Curve3::line(origin + radial * radius, axis)]
        }
        Surface::Cone {
            apex, half_angle, ..
        } => {
            // The ruling from the apex to `p`: along `+axis` on the nappe
            // that opens that way, against it on the mirror nappe (the
            // kernel's cone is the complete double-nappe locus).
            let (s, c) = half_angle.sin_cos();
            let c = if along < 0.0 { -c } else { c };
            vec![Curve3::line(apex, axis * c + radial * s)]
        }
        Surface::Sphere { center, radius, .. } => {
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
            center,
            major_radius,
            minor_radius,
            ..
        } => {
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

/// **D7 step 2's verification**: the printed carrier's ends and middle lie
/// on the built one within the file's band, else the file's statements
/// are no description there.
fn on_description(
    id: u64,
    spec: &EdgeSpec,
    built: &Curve3<f64>,
    file: FileBand,
) -> Result<(), StepImportError> {
    let c = &spec.carrier;
    let gap = [
        c.eval(spec.t0),
        c.mid_point(spec.t0, spec.t1),
        c.eval(spec.t1),
    ]
    .into_iter()
    .map(|p| gap_to(built, p))
    .fold(0.0_f64, f64::max);
    file.agree_at(id, STEP_EDGE_ON_DESCRIPTION, gap)
}

/// **A spline wall's own boundary column or row the printed carrier
/// repeats** (the iso rung): bit for bit where the carrier is a spline
/// (the writer emitted the column twice), and through its ends where
/// recognition promoted it to a line ([`line_iso_match`], read at the file
/// band). The carrier is the wall's iso, or the straight line its ends
/// span; the image is on the carrier's own parameter.
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
                match &spec.carrier {
                    // A spline row is the bitwise arm between two walls
                    // only: beside an analytic face the row is a section.
                    Curve3::Nurbs(n) if (column || both) && bitwise_iso_match(n, &iso) => {
                        let map = IsoMap {
                            fixed,
                            column,
                            running: Running::Own,
                        };
                        return Ok(Some(chosen(
                            wall,
                            Curve3::Nurbs(std::sync::Arc::new(iso)),
                            map,
                        )));
                    }
                    Curve3::Line(_) if column || both => {
                        if let Some((origin, chord, forward)) =
                            line_iso_match(&iso, p_start, p_end, file)
                        {
                            let (dir, len) = chord.direction_and_length();
                            let (from, to) = if forward { (r0, r1) } else { (r1, r0) };
                            let map = IsoMap {
                                fixed,
                                column,
                                running: Running::Chord { from, to, len },
                            };
                            return Ok(Some(chosen(wall, Curve3::line(origin, dir), map)));
                        }
                    }
                    _ => {}
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
/// the rational iso lies in it) and the printed edge lies on (its ends and
/// middle, at the file's band). The wall's exact iso is the carrier, on its
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
            (Side::Plus, _, Surface::Plane { origin, normal, .. })
            | (Side::Minus, Surface::Plane { origin, normal, .. }, _) => (*origin, *normal),
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
                if let Some((origin, chord, forward)) = line_iso_match(&iso, p_start, p_end, file) {
                    let (dir, len) = chord.direction_and_length();
                    let line = Curve3::line(origin, dir);
                    if file.read(STEP_EDGE_ON_DESCRIPTION, held_gap(&line, spec)) != Read::Zero {
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
                let c = &spec.carrier;
                let held = [
                    c.eval(spec.t0),
                    c.mid_point(spec.t0, spec.t1),
                    c.eval(spec.t1),
                ]
                .into_iter()
                .map(|p| spline_foot(&iso, p).map_or(f64::INFINITY, |(_, d)| d))
                .fold(0.0_f64, f64::max);
                if file.read(STEP_EDGE_ON_DESCRIPTION, held) != Read::Zero {
                    continue;
                }
                // The edge runs with the iso, or against it: then the
                // carrier is the iso of the wall reversed along it, whose
                // parameter `t` is the wall's running coordinate
                // `lo + hi − t` (a knot vector that is its own reflection).
                let (carrier, running) = if runs_along(&iso, spec, file) {
                    (iso, Running::Own)
                } else {
                    let reversed = if column {
                        net.reversed_v()
                    } else {
                        net.reversed_u()
                    };
                    let Ok(reversed) = reversed else {
                        continue;
                    };
                    let back = if column {
                        geom_brep::boundary_iso_u(&reversed, end)
                    } else {
                        geom_brep::boundary_iso_v(&reversed, end)
                    }
                    .map_err(structure)?;
                    if !runs_along(&back, spec, file) {
                        continue;
                    }
                    let (lo, hi) = running_domain;
                    (back, Running::Mirror { lo, hi })
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

/// **A straight polynomial iso whose ends are the edge's printed vertex
/// pair at the file's band**: the line from the iso's start control to its
/// end control in the edge's sense (`forward` where the edge starts at the
/// iso's start), and the chord it spans. One constant weight and every
/// control on the end-to-end segment within the band, so the iso is that
/// segment; the ends then read Zero against the vertices. `None` withholds
/// the iso.
fn line_iso_match(
    iso: &geom::NurbsCurve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
    file: FileBand,
) -> Option<(Point3<f64>, Vec3<f64>, bool)> {
    let control = iso.control();
    let (first, last) = (*control.first()?, *control.last()?);
    iso.constant_weight()?;
    let chord = last - first;
    let len2 = chord.dot(chord);
    if !(len2.is_finite() && len2 > 0.0) {
        return None;
    }
    let line = geom::Line {
        origin: first,
        dir: chord,
    };
    let zero = |name, gap: f64| file.read(name, gap) == Read::Zero;
    for &p in control {
        if !line.param(p).is_finite()
            || !zero(STEP_ISO_STRAIGHT, line.distance_within(p, (0.0, 1.0)))
        {
            return None;
        }
    }
    let ends = |a: Point3<f64>, b: Point3<f64>| {
        zero(STEP_ISO_ENDS, a.distance(p_start)) && zero(STEP_ISO_ENDS, b.distance(p_end))
    };
    if ends(first, last) {
        return Some((first, chord, true));
    }
    if ends(last, first) {
        return Some((last, -chord, false));
    }
    None
}

/// Whether a wall's polynomial iso is straight: each control's distance
/// from its end-to-end segment, read at the file's band.
pub(crate) const STEP_ISO_STRAIGHT: &str = "step_iso_straight";

/// Whether a straight iso's ends are an edge's printed vertices, read at
/// the file's band.
pub(crate) const STEP_ISO_ENDS: &str = "step_iso_ends";

/// The bitwise iso match: the carrier's knots, controls and weights are
/// the iso's, bit for bit.
fn bitwise_iso_match(carrier: &geom::NurbsCurve3<f64>, iso: &geom::NurbsCurve3<f64>) -> bool {
    let bits3 = |p: &Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    carrier.knots_f64_interim().degree() == iso.knots_f64_interim().degree()
        && carrier.knots_f64_interim().knots().len() == iso.knots_f64_interim().knots().len()
        && carrier
            .knots_f64_interim()
            .knots()
            .iter()
            .zip(iso.knots_f64_interim().knots())
            .all(|(a, b)| a.to_bits() == b.to_bits())
        && carrier.control().len() == iso.control().len()
        && carrier
            .control()
            .iter()
            .zip(iso.control())
            .all(|(a, b)| bits3(a) == bits3(b))
        && carrier
            .weights()
            .iter()
            .zip(iso.weights())
            .all(|(a, b)| a.to_bits() == b.to_bits())
}

/// **A surface's exact structural signature**: variant tag and field bits,
/// the key that restores writer-side surface sharing (bitwise identity, no
/// ε). An analytic kind's fields are [`Surface::data`]'s, so a field a
/// variant gains is a field of the key; a spline's are its degrees, knots,
/// controls and weights, counts leading each section so two payloads of
/// different shapes cannot alias by concatenation.
pub(crate) fn surface_sig(surface: &Surface<f64>) -> Vec<u64> {
    use geom::SurfaceData;
    let p = |p: Point3<f64>| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()];
    let tag: u64 = match surface {
        Surface::Plane { .. } => 0,
        Surface::Cylinder { .. } => 1,
        Surface::Cone { .. } => 2,
        Surface::Sphere { .. } => 3,
        Surface::Torus { .. } => 4,
        Surface::Nurbs(_) => 5,
        Surface::Approx(_) => 6,
        Surface::Spindle { .. } => 7,
    };
    match surface.data() {
        SurfaceData::Analytic(data) => core::iter::once(tag)
            .chain(
                data.into_iter()
                    .flat_map(|(_, value)| value.scalars())
                    .map(f64::to_bits),
            )
            .collect(),
        // No import path mints one (STEP's OFFSET_SURFACE is not read):
        // the tag aliases only with itself.
        SurfaceData::Approx(_) => vec![tag],
        SurfaceData::Nurbs(payload) => {
            let (nu, nv) = payload.control_counts();
            let (ku, kv) = (payload.knots_u_f64_interim(), payload.knots_v_f64_interim());
            let mut sig = vec![
                tag,
                ku.degree() as u64,
                kv.degree() as u64,
                ku.knots().len() as u64,
                kv.knots().len() as u64,
                nu as u64,
                nv as u64,
            ];
            sig.extend(ku.knots().iter().map(|k| k.to_bits()));
            sig.extend(kv.knots().iter().map(|k| k.to_bits()));
            sig.extend(payload.control().iter().flat_map(|q| p(*q)));
            sig.extend(payload.weights().iter().map(|w| w.to_bits()));
            sig
        }
    }
}

/// **A carrier's exact structural signature**: variant tag and field
/// bits, as [`surface_sig`] is a surface's. Two carriers are the same
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
            let surface = &solid.faces[sides[&id].0].surface;
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
    let surface = &solid.faces[first].surface;
    let sig = surface_sig(surface);
    rest.iter()
        .all(|&f| surface_sig(&solid.faces[f].surface) == sig)
        .then_some(surface)
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
        Surface::Cone { apex, .. } => apex,
        Surface::Sphere { center, radius, .. } => {
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
        let held = [
            surface_sig(&solid.faces[plus].surface),
            surface_sig(&solid.faces[minus].surface),
        ];
        let spec = &solid.edges[&e];
        let extent = geom_brep::edge_extent_inf(&spec.carrier, spec.t0, spec.t1);
        for &f in faces {
            let surface = &solid.faces[f].surface;
            if held.contains(&surface_sig(surface)) {
                continue;
            }
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
    if file.read(STEP_EDGE_ON_DESCRIPTION, off) == Read::Zero {
        Curve3::line(p_start, dir)
    } else {
        carrier
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
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    /// **The straight-iso match's selection table**: forward vertex order
    /// answers the iso's start control and its chord, reversed answers its
    /// end control and the chord turned, and every structural miss
    /// withholds the iso: a curved column, a varying weight vector, a
    /// vertex the file band reads apart from the iso's end. Writer noise
    /// inside the band keeps it: a vertex record and a wall's control
    /// record are different writer objects.
    #[test]
    fn a_straight_iso_matches_by_its_ends_at_the_file_band() {
        let file = FileBand::test(1e-6);
        type Match = Option<(Point3<f64>, Vec3<f64>, bool)>;
        let key = |m: Match| m.map(|(o, c, f)| ([o.x, o.y, o.z], [c.x, c.y, c.z], f));
        let (a, b) = (Point3::new(0.1, 0.2, 0.3), Point3::new(0.1, 0.2, 0.9));
        let column = |w: f64| {
            let knots = KnotVector::clamped(vec![2.0, 2.0, 5.0, 5.0], 1).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, b], vec![w, w]).unwrap()
        };
        assert_eq!(
            key(line_iso_match(&column(1.0), a, b, file)),
            key(Some((a, b - a, true))),
            "the forward reading"
        );
        assert_eq!(
            key(line_iso_match(&column(1.0), b, a, file)),
            key(Some((b, a - b, false))),
            "a reversed vertex order starts at the iso's end"
        );
        assert_eq!(
            key(line_iso_match(&column(0.5), a, b, file)),
            key(Some((a, b - a, true))),
            "a constant weight cancels: the polynomial ruling matches"
        );
        let varying = geom::NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![a, b],
            vec![1.0, 0.5],
        )
        .unwrap();
        assert!(
            line_iso_match(&varying, a, b, file).is_none(),
            "a varying weight withholds"
        );
        let ulp_off = Point3::new(0.1, 0.2, 0.9 + 1e-15);
        assert!(
            line_iso_match(&column(1.0), a, ulp_off, file).is_some(),
            "writer noise inside the band keeps the iso"
        );
        let far_off = Point3::new(0.1, 0.2, 0.9 + 3e-6);
        assert!(
            line_iso_match(&column(1.0), a, far_off, file).is_none(),
            "a vertex the band does not read on the iso's end withholds it"
        );
        let straight_deg2 = {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, Point3::new(0.1, 0.2, 0.6), b], vec![1.0; 3])
                .unwrap()
        };
        assert!(
            line_iso_match(&straight_deg2, a, b, file).is_some(),
            "a straight degree-2 iso is a straight ruling"
        );
        let curved = {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
            geom::NurbsCurve3::new(knots, vec![a, Point3::new(0.4, 0.2, 0.6), b], vec![1.0; 3])
                .unwrap()
        };
        assert!(
            line_iso_match(&curved, a, b, file).is_none(),
            "an iso bent past the band is not straight"
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
            origin,
            axis,
            radius: 1.0,
            u_ref,
        };
        let face = |id: u64, surface: Surface<f64>| FaceSpec {
            id,
            surface,
            sense: true,
            loops: Vec::new(),
            band: false,
        };
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let faces = vec![
            face(10, wall),
            face(11, cylinder(Point3::new(0.5, 1.0, 0.0), z, x)),
            face(12, cylinder(Point3::new(0.0, 1.0, 1.0), x, y)),
        ];
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
                Curve3::line(p, z),
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
            (1, (1, 2), Curve3::line(low, y), (0.0, 2.0 * w)),
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
                },
            )
        })
        .collect();
        let face = |id: u64, normal: Vec3<f64>, arc: u64| FaceSpec {
            id,
            surface: Surface::Plane {
                origin: Point3::new(c0, 0.0, 0.0),
                normal,
                u_ref: y,
            },
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
            faces: vec![face(10, z, 2), face(11, n, 3)],
            edges,
            vertices: BTreeMap::from([(1, low), (2, high)]),
            band_seams: std::collections::BTreeSet::new(),
        }
    }

    /// **A crease between two faces is read over the faces' run, not
    /// their vertices' box.** Two unit discs folded along a 2 cm chord
    /// have the chord's ends for their only vertices, and run 4.47 m
    /// corner to corner. Folded by `1e-5` they stand `2e-5` apart at
    /// their far side, twenty times the file's `1e-6`: two planes, and
    /// the chord is their crease. Folded by `1e-7` they are one plane to
    /// the file over that run, and the refusal's gap is the fold over it.
    /// Red against the vertices' box or the edge's own length for the
    /// reach (the `1e-5` fold then refuses as parallel planes), and
    /// against one face's region for both (a gap of `2.8e-7`).
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
                line.dir.cross(Vec3::new(0.0, 1.0, 0.0)).norm() < 1e-12,
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
                assert!(
                    (4.4e-7..4.5e-7).contains(&gap),
                    "the fold over the two discs' run, 4.47 m: {gap:e}"
                );
            }
            other => panic!(
                "one plane to the file over the faces' run: {:?}",
                other.map(|c| c.carrier)
            ),
        }
    }

    /// **A wrap edge is the surface's own generator through the printed
    /// seam**: a cylinder's ruling at the printed azimuth (not its chart's
    /// `u = 0`), and a torus's two isos through a point, each on the torus.
    #[test]
    fn the_generators_through_a_point_lie_on_their_surface() {
        let cylinder = Surface::Cylinder {
            origin: Point3::new(1.0, 2.0, 3.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 2.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
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
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 3.0,
            minor_radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
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
}
