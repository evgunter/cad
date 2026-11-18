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
use crate::error::StepImportError;
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
/// interpretation left them, every edge's description and built caches,
/// and every built vertex point. No field holds a value the file printed.
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
    carrier: Curve3<f64>,
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

/// A wall iso's image as a map of the carrier's parameter.
#[derive(Clone, Copy, Debug)]
struct IsoMap {
    /// The fixed chart coordinate: `u` of a column, `v` of a row.
    fixed: f64,
    /// Column (`u` fixed) or row (`v` fixed).
    column: bool,
    /// `None` where the carrier is the iso on its own parameter; else
    /// `(from, to, len)` for a straight carrier on its arc length: the
    /// running coordinate is `from` at `t = 0` and `to` at `t = len`, each
    /// end exactly.
    affine: Option<(f64, f64, f64)>,
}

/// **Builds one shell** (module docs). `moves` receives a record for each
/// vertex the build moved past rounding.
///
/// # Errors
///
/// An interpretation refusal where the file's statements are no
/// description at its band ([`StepImportError::Interpretation`]), and
/// [`StepImportError::NotYetSupported`] where the kernel constructs none.
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
        let mut c = choose(solid, id, spec, (plus, minus), tol, file)?;
        c.carrier = admitted(c.carrier, &spec.carrier);
        chosen.insert(id, c);
    }
    let vertices = build_vertices(solid, &sides, &chosen, tol, file, moves)?;
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
        let carrier = crate::signed_zero::plus_zero_curve(rebuilt_in_surface(
            between_ends,
            carrier,
            p_start,
            p_end,
        ));
        let whole = matches!(description, Description::Iso { .. });
        let (carrier, t0, t1) = params(
            id,
            carrier,
            &spec.carrier,
            (p_start, p_end),
            spec.start == spec.end,
            whole,
        )?;
        let description = match (description, iso) {
            (Description::Iso { wall, .. }, Some(map)) => Description::Iso {
                wall,
                image: map.image(t0, t1),
            },
            (other, _) => other,
        };
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
        let at = |t: f64| match self.affine {
            None => t,
            // Held to the iso's own ends: an end the projection reads a
            // rounding past the chord's length is the iso's end.
            Some((from, to, len)) => {
                (from + (to - from) * (t / len)).clamp(from.min(to), from.max(to))
            }
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
        None => StepImportError::BuildEscalated {
            id,
            source: *source,
            file: geom_core::FileCoincidence::new(file.eps_in()),
        },
    }
}

/// The not-yet-supported refusal of edge `id`, read at the import door of
/// a file declaring `eps_in`.
fn unbuilt(file: FileBand, id: u64, what: String, lane: &'static str) -> StepImportError {
    StepImportError::NotYetSupported {
        id,
        what,
        lane,
        file: geom_core::FileCoincidence::new(file.eps_in()),
    }
}

/// The lane a relation the file band reads `Zero` and the run reads apart
/// waits on.
const SNAP_LANE: &str = "D7 step 3's snap catalogue, which makes a relation between surfaces \
                         that reads Zero at the file's band exact";

/// The lane a pair with no section construction waits on.
const SECTION_LANE: &str = "the C5 section table's arm for this pair (`geom_brep::intersect`)";

/// **Selection and the carrier's construction** for one edge (module
/// docs, steps 1 and 2).
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
        return one_surface(id, spec, sa, plus == minus, file);
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
    // as a wedge between them is (`topo::wedge_reach`): the farthest two
    // of their vertices stand apart, and the edge's own extent where that
    // is longer (a closed edge's whole turn).
    let extent = geom_brep::edge_extent_inf(
        &spec.carrier,
        spec.t0,
        spec.t1,
        (spec.carrier.eval(spec.t0), spec.carrier.eval(spec.t1)),
    )
    .max(faces_reach(solid, [plus, minus]));
    // A cone's pose readings are levered from its apex (the station of a
    // coaxial cylinder's circles is a distance from it): the edge's own
    // diameter is no reach from there.
    let extent = [sa, sb].into_iter().fold(extent, |lever, s| match *s {
        Surface::Cone { apex, .. } => lever.max(region.lever_from(apex)),
        _ => lever,
    });
    let spline = |s: &Surface<f64>| s.spline_chart().is_some();
    if spline(sa) || spline(sb) {
        return spline_pair(id, spec, (sa, sb), &region, extent, (file, eps_band));
    }
    analytic_pair(id, spec, (sa, sb), &region, extent, (file, eps_band))
}

/// The diameter of the box about the vertices the faces `faces` bound,
/// as a lever: how far the two faces of an edge run. Read through the
/// extent's own door ([`geom_brep::points_diameter`]) on the box's two
/// far corners, whose distance is its diagonal.
fn faces_reach(solid: &SolidSpec, faces: [usize; 2]) -> geom_core::Lever<f64> {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for f in faces {
        for l in &solid.faces[f].loops {
            for u in &l.uses {
                let e = &solid.edges[&u.edge];
                for v in [e.start, e.end] {
                    let p = solid.vertices[&v];
                    for (i, c) in [p.x, p.y, p.z].into_iter().enumerate() {
                        lo[i] = lo[i].min(c);
                        hi[i] = hi[i].max(c);
                    }
                }
            }
        }
    }
    if !lo.iter().chain(&hi).all(|c| c.is_finite()) {
        return geom_brep::points_diameter(&[]);
    }
    geom_brep::points_diameter(&[
        Point3::new(lo[0], lo[1], lo[2]),
        Point3::new(hi[0], hi[1], hi[2]),
    ])
}

/// A spline wall in the pair: the plane × wall section in the plane's
/// chart (the native rim's description), or the two walls' section.
fn spline_pair(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_core::Lever<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    let plane = match (sa, sb) {
        (Surface::Plane { .. }, _) => Some(Side::Plus),
        (_, Surface::Plane { .. }) => Some(Side::Minus),
        _ => None,
    };
    let both_spline = sa.spline_chart().is_some() && sb.spline_chart().is_some();
    if plane.is_none() && !both_spline {
        return Err(unbuilt(
            file,
            id,
            format!(
                "an edge between a {} and a {}, whose section the kernel builds only against a \
                 plane or a second spline wall",
                sa.kind().name(),
                sb.kind().name()
            ),
            SECTION_LANE,
        ));
    }
    // An arc on a cap plane is the cap's rim (below). Otherwise the
    // pair's section, where the kernel builds it, and the edge is the
    // pair's intersection; a straight section is the line it is, read by
    // the recognition that promotes a printed spline
    // ([`crate::recognize_curve::try_line`], at the file's band).
    let arc = matches!(spec.carrier, Curve3::Circle(_) | Curve3::Ellipse(_));
    let section = if arc && plane.is_some() {
        Err(unbuilt(
            file,
            id,
            "a cap's arc rim".to_owned(),
            SECTION_LANE,
        ))
    } else {
        section(file, id, spec, (sa, sb), region, extent, eps_band)
    };
    let analytic = arc || matches!(spec.carrier, Curve3::Line(_));
    let chart = match (section, plane) {
        (Ok(carrier), _) => {
            let carrier = match &carrier {
                Curve3::Nurbs(n) => crate::recognize_curve::try_line(n, file)
                    .map_or(carrier.clone(), |(line, _)| line),
                _ => carrier,
            };
            on_description(id, spec, &carrier, file)?;
            // A straight printed edge whose section the recognition does
            // not certify straight at the band (a marched branch's fit)
            // is the segment between its built ends, which the section
            // through them holds.
            let between_ends =
                matches!(spec.carrier, Curve3::Line(_)) && matches!(carrier, Curve3::Nurbs(_));
            return Ok(Chosen {
                description: Description::Intersection,
                carrier,
                intrinsic: true,
                iso: None,
                between_ends,
            });
        }
        (Err(StepImportError::NotYetSupported { .. }), Some(chart)) if analytic => chart,
        (Err(e), _) => return Err(e),
    };
    // **A cap's rim** the section does not build (a plane holding a
    // rational wall's boundary row, or meeting the wall along its edge
    // only): the native rim's description, a curve in the cap plane's
    // chart, is the cap's own profile, which no surface pair defines —
    // so, like a curve one surface holds, its description is the curve
    // as interpretation built it. What interpretation checks is that the
    // wall holds it: its ends and middle lie on the wall at the file's
    // band.
    let wall = match chart {
        Side::Plus => sb,
        Side::Minus => sa,
    };
    let Some(net) = wall.spline_chart() else {
        unreachable!("a plane × spline pair's other side is a spline")
    };
    let c = &spec.carrier;
    let gap = [
        c.eval(spec.t0),
        c.mid_point(spec.t0, spec.t1),
        c.eval(spec.t1),
    ]
    .into_iter()
    .map(|p| net.project(p).map_or(f64::INFINITY, |f| f.distance))
    .fold(0.0_f64, f64::max);
    file.agree_at(id, STEP_RIM_ON_WALL, gap)?;
    Ok(Chosen {
        description: Description::Chart { chart },
        carrier: spec.carrier.clone(),
        intrinsic: false,
        iso: None,
        between_ends: true,
    })
}

/// Whether two planes an edge lies on cross: the sine of their normals'
/// angle levered at the edge's extent, read at the file's band.
pub(crate) const STEP_PLANES_CROSS: &str = "step_planes_cross";

/// Whether a cap's printed rim lies on its spline wall: the farthest of
/// its ends and middle from the wall, read at the file's band.
pub(crate) const STEP_RIM_ON_WALL: &str = "step_rim_on_wall";

/// [`topo::section_near`] at `band`, its verdicts refused for edge `id`.
fn section(
    file: FileBand,
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_core::Lever<f64>,
    band: geom_core::Band,
) -> Result<Curve3<f64>, StepImportError> {
    match topo::section_near(
        Some(topo::SectionLane::f64()),
        (sa, sb),
        (&spec.carrier, (spec.t0, spec.t1)),
        region,
        extent,
        band,
    ) {
        Ok(Ok(c)) => Ok(c),
        Ok(Err(verdict)) => Err(unbuilt(
            file,
            id,
            format!(
                "the section of its {} and its {} near it: {verdict}",
                sa.kind().name(),
                sb.kind().name()
            ),
            SECTION_LANE,
        )),
        Err(source) => Err(StepImportError::BuildEscalated {
            id,
            source,
            file: geom_core::FileCoincidence::new(file.eps_in()),
        }),
    }
}

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
    let read = |band| {
        topo::section_near(
            Some(topo::SectionLane::f64()),
            (sa, sb),
            (&spec.carrier, (spec.t0, spec.t1)),
            region,
            extent,
            band,
        )
    };
    // An escalation at the file's band is the file's ambiguity: its
    // statements do not decide the reading at their own precision.
    let at_file = read(file.band()).map_err(|source| ambiguous(id, &source, file))?;
    let crossing = |carrier| -> Result<Chosen, StepImportError> {
        on_description(id, spec, &carrier, file)?;
        Ok(Chosen {
            description: Description::Intersection,
            carrier,
            intrinsic: true,
            iso: None,
            between_ends: false,
        })
    };
    match at_file {
        Ok(_) => match read(eps_band) {
            Ok(Ok(carrier)) => crossing(carrier),
            Ok(Err(V::Tangent)) => Err(unbuilt(
                file,
                id,
                format!(
                    "its {} and {} read crossing at the file's band and tangent at the run's",
                    sa.kind().name(),
                    sb.kind().name()
                ),
                SNAP_LANE,
            )),
            Ok(Err(verdict)) => Err(unbuilt(
                file,
                id,
                format!(
                    "the section of its {} and its {} near it: {verdict}",
                    sa.kind().name(),
                    sb.kind().name()
                ),
                SECTION_LANE,
            )),
            Err(source) => Err(StepImportError::BuildEscalated {
                id,
                source,
                file: geom_core::FileCoincidence::new(file.eps_in()),
            }),
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
                "the section of its {} and its {} near it, read at the file's band: {verdict}",
                sa.kind().name(),
                sb.kind().name()
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
/// classification at the file band picks crossing or tangency, and its
/// carried component through the printed middle is the carrier.
fn served_pair(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_core::Lever<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    let ball = Some(region.ball());
    let read =
        |band| geom_brep::pair_section(sa, sb, ball, Some(geom_brep::SsiLane::marched()), band);
    let classified = |band| -> Result<Option<geom_brep::PairSection<f64>>, StepImportError> {
        match read(band) {
            Ok(geom_brep::SectionRead::Built(section)) => Ok(Some(section)),
            Ok(geom_brep::SectionRead::Elsewhere(_)) => Ok(None),
            Err(geom_brep::SectionError::CoincidentSurfaces) => Err(unbuilt(
                file,
                id,
                format!(
                    "an edge between two records of one {}, a curve inside one surface the \
                     two records state twice",
                    sa.kind().name()
                ),
                "one record of a non-planar surface \
                 (`work/importgate/two-records-of-one-non-planar-surface-stay-two-at-import.md`)",
            )),
            Err(e) => Err(unbuilt(
                file,
                id,
                format!(
                    "the section of its {} and its {}: {e}",
                    sa.kind().name(),
                    sb.kind().name()
                ),
                SECTION_LANE,
            )),
        }
    };
    let Some(at_file) = classified(file.band())? else {
        return Err(unbuilt(
            file,
            id,
            format!(
                "an edge between a {} and a {}, a pair whose section the kernel does not build",
                sa.kind().name(),
                sb.kind().name()
            ),
            SECTION_LANE,
        ));
    };
    let tangency = at_file
        .decided()
        .iter()
        .any(|z| matches!(z.decision, geom_brep::SectionDecision::Tangency(_)));
    if tangency {
        return tangent(id, spec, (sa, sb), region, extent, (file, eps_band));
    }
    let Some(at_eps) = classified(eps_band)? else {
        unreachable!("a pair the section value builds at one band it builds at every band")
    };
    let carried = at_eps
        .carry(Some(geom_brep::SsiLane::marched()), eps_band)
        .map_err(|e| {
            unbuilt(
                file,
                id,
                format!(
                    "the section of its {} and its {} carries no component: {e}",
                    sa.kind().name(),
                    sb.kind().name()
                ),
                SECTION_LANE,
            )
        })?;
    let mid = spec.carrier.mid_point(spec.t0, spec.t1);
    let component = match carried.site(mid, file.band()) {
        Ok(geom_brep::SiteReading::OnArc(foot)) => &carried.components()[foot.component],
        Ok(geom_brep::SiteReading::AtNode(_)) | Err(_) => {
            // The printed middle on no component at the file's band: the
            // file's statements are no description there.
            let nearest = carried
                .components()
                .iter()
                .map(|c| gap_to(&c.carrier, mid))
                .fold(f64::INFINITY, f64::min);
            file.agree_at(id, STEP_EDGE_ON_DESCRIPTION, nearest)?;
            return Err(unbuilt(
                file,
                id,
                format!(
                    "the printed carrier's middle names no one component of the section of its \
                     {} and its {}",
                    sa.kind().name(),
                    sb.kind().name()
                ),
                SECTION_LANE,
            ));
        }
    };
    // A marched loop is periodic; its edge is an arc of the loop's whole
    // turn, read on the clamped spline of that turn.
    let carrier = match &component.carrier {
        Curve3::Periodic(p) => Curve3::Nurbs(std::sync::Arc::new(p.whole_loop().clone())),
        other => other.clone(),
    };
    let carrier = running_with(id, carrier, spec, file)?;
    on_description(id, spec, &carrier, file)?;
    Ok(Chosen {
        description: Description::Intersection,
        carrier,
        intrinsic: true,
        iso: None,
        between_ends: false,
    })
}

/// A tangency read at the file band: the tangent locus at the run, a
/// ruling of a plane and a cylinder or of two parallel cylinders, or the
/// parallel of a coaxial pair. A pair the run reads apart where the file
/// reads it tangent waits on the snap catalogue.
fn tangent(
    id: u64,
    spec: &EdgeSpec,
    (sa, sb): (&Surface<f64>, &Surface<f64>),
    region: &geom_brep::Reach<f64>,
    extent: geom_core::Lever<f64>,
    (file, eps_band): (FileBand, geom_core::Band),
) -> Result<Chosen, StepImportError> {
    let names = || format!("{} and {}", sa.kind().name(), sb.kind().name());
    let locus = if geom_brep::tangent_locus_kinds(sa.kind(), sb.kind()) {
        match geom_brep::tangent_locus(sa, sb, region.ball(), eps_band) {
            Ok(geom_brep::TangentLocus::Line { line, .. }) => Curve3::Line(line),
            Err(geom_brep::TangentLocusError::NotTangent { .. }) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!(
                        "its {} read tangent at the file's band and apart at the run's",
                        names()
                    ),
                    SNAP_LANE,
                ));
            }
            Err(geom_brep::TangentLocusError::Escalated(source)) => {
                return Err(StepImportError::BuildEscalated {
                    id,
                    source,
                    file: geom_core::FileCoincidence::new(file.eps_in()),
                });
            }
            Err(geom_brep::TangentLocusError::Unsupported { what }) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!("the tangency of its {}: {what}", names()),
                    SNAP_LANE,
                ));
            }
        }
    } else {
        match geom_brep::coaxial_tangent_circle(sa, sb, extent, eps_band) {
            Ok(Some(circle)) => circle,
            Ok(None) => {
                let coaxial_at_file =
                    geom_brep::coaxial_tangent_circle(sa, sb, extent, file.band())
                        .ok()
                        .flatten()
                        .is_some();
                let (what, lane) = if coaxial_at_file {
                    (
                        format!(
                            "its {} read coaxial and tangent at the file's band and not at the \
                             run's",
                            names()
                        ),
                        SNAP_LANE,
                    )
                } else {
                    (
                        format!("a tangency of its {} with no closed-form locus", names()),
                        "a tangent-locus construction for this pose (C7)",
                    )
                };
                return Err(unbuilt(file, id, what, lane));
            }
            Err(e) => {
                return Err(unbuilt(
                    file,
                    id,
                    format!("the tangency of its {}: {e}", names()),
                    SECTION_LANE,
                ));
            }
        }
    };
    let carrier = running_with(id, locus, spec, file)?;
    on_description(id, spec, &carrier, file)?;
    Ok(Chosen {
        description: Description::Tangent,
        carrier,
        intrinsic: true,
        iso: None,
        between_ends: false,
    })
}

/// One surface record on both sides: a periodic face's wrap edge on its
/// seam generator, a closed spline patch's seam (taken by
/// [`iso_of_wall`]), or a curve the surface holds between two faces.
fn one_surface(
    id: u64,
    spec: &EdgeSpec,
    surface: &Surface<f64>,
    one_face: bool,
    file: FileBand,
) -> Result<Chosen, StepImportError> {
    let mid = spec.carrier.mid_point(spec.t0, spec.t1);
    let seams = if one_face {
        generators_through(surface, mid)
    } else {
        Vec::new()
    };
    if !seams.is_empty() {
        // The generator the printed carrier lies on at the file's band:
        // a torus has two through its middle.
        // Both pass through it; the one it runs along is the one whose
        // tangent there lies along the printed carrier's.
        let tangent = spec.carrier.deriv(geom::mid_param(spec.t0, spec.t1));
        let along = |c: &Curve3<f64>| {
            param_at(c, mid).map_or(0.0, |t| {
                let d = c.deriv(t);
                (d.dot(tangent) / (d.norm() * tangent.norm())).abs()
            })
        };
        let on = (0..seams.len())
            .max_by(|&i, &j| along(&seams[i]).total_cmp(&along(&seams[j])))
            .unwrap_or(0);
        let carrier = running_with(id, seams[on].clone(), spec, file)?;
        on_description(id, spec, &carrier, file)?;
        return Ok(Chosen {
            description: Description::Wrap,
            carrier,
            intrinsic: true,
            iso: None,
            between_ends: false,
        });
    }
    Ok(Chosen {
        description: Description::Chart { chart: Side::Plus },
        carrier: spec.carrier.clone(),
        intrinsic: false,
        iso: None,
        between_ends: true,
    })
}

/// **A periodic surface's own generators through `p`**: the ruling of a
/// cylinder or a cone, the meridian of a sphere, and a torus's tube
/// circle and parallel, each the surface's iso through the azimuth (and,
/// on a torus, the tube angle) of `p`. A wrap edge is one of these; the
/// printed carrier only says which. None where `p` stands on the axis,
/// where no azimuth is read, or on a surface with no periodic chart
/// coordinate.
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
            let (s, c) = half_angle.sin_cos();
            vec![Curve3::line(apex, axis * c + radial * s)]
        }
        Surface::Sphere { center, radius, .. } => {
            vec![Curve3::circle(center, radial.cross(axis), radius, radial)]
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
/// An end standing on a domain end within its rounding reads that end
/// exactly: a projection lands ulps inside it, and an interval a hair
/// short of an iso's own domain is no iso of the wall.
fn spline_end_param(
    id: u64,
    n: &geom::NurbsCurve3<f64>,
    p: Point3<f64>,
    seed: f64,
) -> Result<f64, StepImportError> {
    let (lo, hi) = n.domain();
    for end in [lo, hi] {
        if n.eval(end).distance(p) <= rounding(p) {
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

/// The samples a spline foot is seeded from.
const FOOT_SAMPLES: u32 = 64;

/// **`p`'s foot on a spline carrier**, its parameter and distance: the
/// projection ([`geom::NurbsCurve3::project`]), or, where its own seed
/// leaves Newton inconclusive, the projection seeded at the nearest of a
/// fixed schedule of samples, or that sample itself, whose distance is
/// then an upper bound on the foot's.
fn spline_foot(n: &geom::NurbsCurve3<f64>, p: Point3<f64>) -> Option<(f64, f64)> {
    if let Ok(f) = n.project(p) {
        return Some((f.t, f.distance));
    }
    let (lo, hi) = n.domain();
    let (t, d) = (0..=FOOT_SAMPLES)
        .map(|i| {
            let t = lo + (hi - lo) * (f64::from(i) / f64::from(FOOT_SAMPLES));
            (t, n.eval(t).distance(p))
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))?;
    match n.project_from_seed(p, t) {
        Ok(f) if f.distance <= d => Some((f.t, f.distance)),
        _ => d.is_finite().then_some((t, d)),
    }
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
        description: Description::Iso {
            wall,
            image: map.image(0.0, 1.0),
        },
        carrier,
        intrinsic: true,
        iso: Some(map),
        between_ends: false,
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
                            affine: None,
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
                                affine: Some((from, to, len)),
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

/// **Every vertex built** (module docs, step 3): the root of a surface
/// around it along an intrinsic carrier through it that the surface does
/// not hold, else the foot of its printed point on a carrier through it.
fn build_vertices(
    solid: &SolidSpec,
    sides: &BTreeMap<u64, (usize, usize)>,
    chosen: &BTreeMap<u64, Chosen>,
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
    let mut built = BTreeMap::new();
    for (&v, &printed) in &solid.vertices {
        let Some((edges, faces)) = around.get(&v) else {
            // A vertex no edge ends at (a vertex loop's): its printed point
            // is the description, a point on its face's surface.
            built.insert(v, printed);
            continue;
        };
        let point = rooted(solid, sides, chosen, (edges, faces), printed, eps_band)
            .or_else(|| footed(chosen, edges, printed))
            .unwrap_or(printed);
        let moved = point.distance(printed);
        file.agree_at(v, STEP_VERTEX_BUILT, moved)?;
        // A built point the printed one states within the rounding of the
        // construction that built it (its neighbourhood's coordinates) is
        // that point as the file states it, kept bit for bit, as a frame
        // the file states rigid within its rounding is: rebuilding it
        // would move it by rounding alone.
        let scale = edges
            .iter()
            .flat_map(|e| [solid.edges[e].start, solid.edges[e].end])
            .map(|w| rounding(solid.vertices[&w]))
            .fold(rounding(printed), f64::max);
        let point = if moved <= scale { printed } else { point };
        if moved > scale {
            moves.push(StructureNormalization {
                face: v,
                kind: NormalizationKind::VertexBuilt { moved_by: moved },
                file_census: FaceCensus::EMPTY,
                kernel_census: FaceCensus::EMPTY,
            });
        }
        built.insert(v, crate::signed_zero::plus_zero_point(point));
    }
    Ok(built)
}

/// The coordinates' own rounding at `p`: a move within it is no move.
fn rounding(p: Point3<f64>) -> f64 {
    ROUNDING * p.x.abs().max(p.y.abs()).max(p.z.abs())
}

/// **A value's own rounding, relative to its size**: sixteen units of its
/// last place, what a handful of products and sums move a coordinate by,
/// each rounding once. A move within it is no move: the build's
/// reconstruction of a value the file printed differs from it by its
/// arithmetic alone.
const ROUNDING: f64 = 16.0 * f64::EPSILON;

/// **A built carrier admitted as the printed one** where the printed one
/// states it within its own rounding, field by field (a frame the file
/// states rigid within its rounding is admitted the same way): rebuilding
/// it would move it by rounding alone, and the writer's own output would
/// not be a fixed point of the import. A built line is first anchored at
/// its foot of the printed line's origin, a point of the locus the printed
/// line names. Every other carrier as built.
fn admitted(built: Curve3<f64>, printed: &Curve3<f64>) -> Curve3<f64> {
    let same_point = |a: Point3<f64>, b: Point3<f64>| a.distance(b) <= rounding(a).max(rounding(b));
    let along = |a: Vec3<f64>, b: Vec3<f64>| (a - b).norm() <= ROUNDING;
    let length = |a: f64, b: f64| (a - b).abs() <= ROUNDING * a.abs().max(b.abs());
    match (&built, printed) {
        (Curve3::Line(b), Curve3::Line(p)) => {
            let anchored = b.foot(p.origin);
            if same_point(anchored, p.origin) && along(b.dir, p.dir) {
                printed.clone()
            } else {
                Curve3::line(anchored, b.dir)
            }
        }
        (Curve3::Circle(b), Curve3::Circle(p))
            if same_point(b.frame.origin, p.frame.origin)
                && along(b.frame.axis, p.frame.axis)
                && along(b.frame.u_ref, p.frame.u_ref)
                && length(b.radius, p.radius) =>
        {
            printed.clone()
        }
        (Curve3::Ellipse(b), Curve3::Ellipse(p))
            if same_point(b.frame.origin, p.frame.origin)
                && along(b.frame.axis, p.frame.axis)
                && along(b.frame.u_ref, p.frame.u_ref)
                && length(b.semi_u, p.semi_u)
                && length(b.semi_v, p.semi_v) =>
        {
            printed.clone()
        }
        _ => built,
    }
}

/// The first root, in edge then face order, of a face's surface along an
/// intrinsic carrier ending at the vertex that the surface does not hold.
fn rooted(
    solid: &SolidSpec,
    sides: &BTreeMap<u64, (usize, usize)>,
    chosen: &BTreeMap<u64, Chosen>,
    (edges, faces): (&[u64], &[usize]),
    printed: Point3<f64>,
    band: geom_core::Band,
) -> Option<Point3<f64>> {
    for &e in edges {
        let c = &chosen[&e];
        if !c.intrinsic {
            continue;
        }
        let (plus, minus) = sides[&e];
        let held = [
            surface_sig(&solid.faces[plus].surface),
            surface_sig(&solid.faces[minus].surface),
        ];
        let seed = param_at(&c.carrier, printed)?;
        let spec = &solid.edges[&e];
        let extent = geom_brep::edge_extent_inf(
            &spec.carrier,
            spec.t0,
            spec.t1,
            (spec.carrier.eval(spec.t0), spec.carrier.eval(spec.t1)),
        );
        for &f in faces {
            let surface = &solid.faces[f].surface;
            if held.contains(&surface_sig(surface)) {
                continue;
            }
            if let Ok(Ok(t)) = topo::root_along(surface, &c.carrier, seed, extent, band) {
                return Some(c.carrier.eval(t));
            }
        }
    }
    None
}

/// The printed point's foot on the first carrier ending at the vertex,
/// intrinsic carriers first.
fn footed(
    chosen: &BTreeMap<u64, Chosen>,
    edges: &[u64],
    printed: Point3<f64>,
) -> Option<Point3<f64>> {
    let by_rank = edges
        .iter()
        .filter(|e| chosen[*e].intrinsic)
        .chain(edges.iter().filter(|e| !chosen[*e].intrinsic));
    by_rank.into_iter().find_map(|e| {
        let c = &chosen[e].carrier;
        param_at(c, printed).map(|t| c.eval(t))
    })
}

/// A straight edge rebuilt between its built ends (`between_ends`): a
/// segment one surface holds, whose description is its two ends, or a
/// straight section the band does not certify straight. Every other
/// carrier as built.
fn rebuilt_in_surface(
    between_ends: bool,
    carrier: Curve3<f64>,
    p_start: Point3<f64>,
    p_end: Point3<f64>,
) -> Curve3<f64> {
    match (between_ends, &carrier) {
        (true, Curve3::Line(_) | Curve3::Nurbs(_)) => match (p_end - p_start).checked_direction() {
            Ok((dir, _)) => Curve3::line(p_start, dir),
            Err(_) => carrier,
        },
        _ => carrier,
    }
}

/// **The built interval**: the built ends read on the built carrier by its
/// own inverse ([`crate::geometry::endpoint_params`] for an analytic
/// carrier, the projection for a spline). A spline carrier is cut
/// ([`geom::NurbsCurve3::split_at`]) to the extent the printed carrier
/// states — the printed spline's own domain ends, where it is a spline,
/// and the edge's ends otherwise — as the printed carrier chooses the
/// section's component. A wall's own iso (`whole`) is not cut: its image
/// is on its own parameter, which the interval is read on.
fn params(
    id: u64,
    carrier: Curve3<f64>,
    printed: &Curve3<f64>,
    (p_start, p_end): (Point3<f64>, Point3<f64>),
    self_loop: bool,
    whole: bool,
) -> Result<(Curve3<f64>, f64, f64), StepImportError> {
    let Curve3::Nurbs(n) = &carrier else {
        let (t0, t1) = crate::geometry::endpoint_params(id, &carrier, p_start, p_end, self_loop)?;
        return Ok((carrier, t0, t1));
    };
    let (lo, hi) = n.domain();
    if self_loop {
        return Ok((carrier, lo, hi));
    }
    let at = |p: Point3<f64>, seed: f64| spline_end_param(id, n, p, seed);
    let (t0, t1) = (at(p_start, lo)?, at(p_end, hi)?);
    if t1 <= t0 {
        return Err(StepImportError::Topology {
            id,
            what: "a built spline carrier whose end reads at or before its start",
        });
    }
    let cut = |c: &geom::NurbsCurve3<f64>, u: f64, keep_left: bool| {
        c.split_at(u)
            .map(|(left, right)| if keep_left { left } else { right })
            .map_err(|_| StepImportError::Topology {
                id,
                what: "a built spline carrier that does not cut at its edge's end",
            })
    };
    if whole {
        return Ok((carrier, t0, t1));
    }
    let (c0, c1) = match printed {
        Curve3::Nurbs(p) => {
            let (a, b) = p.domain();
            (at(p.eval(a), lo)?.min(t0), at(p.eval(b), hi)?.max(t1))
        }
        _ => (t0, t1),
    };
    let mut piece = n.as_ref().clone();
    if c1 < hi {
        piece = cut(&piece, c1, true)?;
    }
    if c0 > lo {
        piece = cut(&piece, c0, false)?;
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
