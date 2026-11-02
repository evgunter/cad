//! **`topo`'s one home of edge certification.** Every call `topo`'s
//! production code makes into `geom_brep`'s certification doors
//! ([`EdgeCurve::certify`], [`EdgeCurve::recertify`] and their `_via`
//! and `_via_stated` forms) is made here, and each entry takes an
//! [`Origin`] first: the call site says whose data it certifies, and a
//! site cannot certify without saying. A census below holds the rest of
//! the crate's production code to these entries.
//!
//! **The origin decides what a span refusal is** (`[failure]`). The doors
//! report a span check's reading certainly on its refused side as their
//! input's ([`CertifyError::StatedInterval`]). Under [`Origin::Callers`]
//! that input is a caller's (a spec a door was handed, a body a validator
//! reads), and the refusal stays typed. Under [`Origin::Kernels`] it is
//! an edge the body stores, re-minted or re-certified by the kernel at
//! the band it was minted at, and the refusal is a kernel bug: it panics
//! naming the site.
//!
//! **The edge door is the scalar's policy** ([`AtRestPolicy::attach_edge`],
//! [`AtRestPolicy::reattach_edge`]): a certifying scalar's door mints and
//! certifies in full, a tangent-transport scalar's only mints, and no
//! entry here takes a lane, so no caller can certify at a scalar whose
//! policy does not, or omit the certificate at one whose policy does.

use geom::Surface;
use geom_brep::{Certificate, CertifyError, EdgeCurve, EdgeCurveSpec, NurbsLane, StatedImage};
use geom_core::{Band, Decide, Point3, TangentTransport};

use crate::geometry::SurfaceKey;
use crate::props::AtRestPolicy;

/// **Whose data a certification reads**, named at the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Origin {
    /// A caller's: a spec a door was handed, or a body a door reads. A
    /// span refusal is that input's, and stays typed.
    Callers,
    /// The kernel's own: an edge the body stores, re-minted or
    /// re-certified at the band it was minted at, at the named site.
    /// That band is the run's: a run holds one [`geom_core::Tol`], and
    /// every `topo` edge mint certifies at its `Band::linear(tol)`. The span
    /// was decided forward and short of a full turn at that band; its
    /// parameters travel bitwise and its carrier's rate stays a positive
    /// bound, so the span's sign does (a split child's interval is one
    /// side of a parameter the split decided interior at that band). A
    /// span refusal is a kernel bug, and panics.
    Kernels(KernelSite),
}

/// **The kernel's re-mints of stored edges**, one per call site that
/// certifies with [`Origin::Kernels`]; a panic names its site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KernelSite {
    /// [`crate::transform_rigid`]'s re-mint of each moved edge.
    RigidTransform,
    /// [`crate::Body::split_edge`](crate::construction::Construct::split_edge)'s two children.
    SplitChild,
    /// A graft's re-certification of the source body's edges.
    Graft,
    /// A described move's re-certification of a stored carrier.
    ReDescription,
    /// A re-based run's re-certification of its edges.
    ReBasedRun,
    /// [`crate::boolean::describe_edges`]' transverse seam, described
    /// as its pair's `Intersection` on its own curved carrier.
    SeamIntersection,
    /// [`crate::boolean::describe_edges`]' smooth seam whose surfaces
    /// determine it, described as their `TangentIntersection`.
    SeamTangency,
    /// [`crate::boolean::describe_edges`]' smooth seam whose description
    /// went stale, restated where it rests.
    SeamAtRest,
    /// A split's transverse section boundary edge, described as its
    /// pair's `Intersection` on its own curved carrier.
    SectionIntersection,
    /// A split's smooth section boundary edge, described as the
    /// must-carry rule demands.
    SectionSmooth,
    /// A split's re-chart of a section face, its stranded edges restated.
    SectionRechart,
    /// A shell's re-chart of a promoted rim face, its edges restated.
    RimRechart,
    /// [`crate::Body::describe_at_rest`](crate::construction::Construct::describe_at_rest)'s restatement.
    AtRest,
    /// The restatement of a wrap edge whose halves an operation parted.
    PartedWrap,
    /// [`crate::Body::restate_stored_edge`](crate::construction::Construct::restate_stored_edge)'s restatement.
    StoredRestatement,
}

impl core::fmt::Display for KernelSite {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::RigidTransform => "transform_rigid",
            Self::SplitChild => "a split child",
            Self::Graft => "a graft",
            Self::ReDescription => "a re-description",
            Self::ReBasedRun => "a re-based run's edge",
            Self::SeamIntersection => "a seam's intersection",
            Self::SeamTangency => "a seam's tangency",
            Self::SeamAtRest => "a seam at rest",
            Self::SectionIntersection => "a section boundary's intersection",
            Self::SectionSmooth => "a section boundary's smooth join",
            Self::SectionRechart => "a section re-chart",
            Self::RimRechart => "a rim re-chart",
            Self::AtRest => "describe_at_rest",
            Self::PartedWrap => "a parted wrap edge",
            Self::StoredRestatement => "restate_stored_edge",
        })
    }
}

impl Origin {
    /// `error` as this origin reads it.
    fn read(self, error: CertifyError) -> CertifyError {
        if let (Self::Kernels(site), CertifyError::StatedInterval { refused }) = (self, &error) {
            unreachable!(
                "{site}: a stored edge's interval read as refused at the band it was minted at \
                 ({refused}): a kernel bug"
            );
        }
        error
    }
}

/// [`AtRestPolicy::attach_edge`], the wedge levered at `reach`
/// ([`fn@crate::wedge_reach`]).
pub(crate) fn certify<T: AtRestPolicy>(
    origin: Origin,
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
) -> Result<EdgeCurve<T>, CertifyError> {
    certify_stated(origin, spec, start, end, surfaces, reach, band, None)
}

/// [`AtRestPolicy::attach_edge`] with a chart image the body already
/// holds for the edge on one of its surfaces, the wedge levered at
/// `reach` ([`fn@crate::wedge_reach`]).
#[allow(clippy::too_many_arguments)] // `certify`'s, plus the stated image
pub(crate) fn certify_stated<T: AtRestPolicy>(
    origin: Origin,
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    stated: Option<&StatedImage<T>>,
) -> Result<EdgeCurve<T>, CertifyError> {
    T::attach_edge(spec, start, end, surfaces, reach, band, stated)
        .map_err(|error| origin.read(error))
}

/// [`AtRestPolicy::reattach_edge`], the wedge levered at `reach`
/// ([`fn@crate::wedge_reach`]).
pub(crate) fn recertify<T: AtRestPolicy>(
    origin: Origin,
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
) -> Result<T::Carries<Certificate<T>>, CertifyError> {
    recertify_stated(origin, curve, start, end, surfaces, reach, band, None)
}

/// [`AtRestPolicy::reattach_edge`] with the chart image the body stores
/// for the edge.
#[allow(clippy::too_many_arguments)] // `recertify`'s, plus the stated image
pub(crate) fn recertify_stated<T: AtRestPolicy>(
    origin: Origin,
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    stated: Option<&StatedImage<T>>,
) -> Result<T::Carries<Certificate<T>>, CertifyError> {
    T::reattach_edge(curve, start, end, surfaces, reach, band, stated)
        .map_err(|error| origin.read(error))
}

/// **A certifying scalar's edge door** ([`AtRestPolicy::attach_edge`]):
/// the mint and every check of the certificate, C2's limbs through the
/// certification lane.
#[allow(clippy::too_many_arguments)] // the door's own inputs
pub(crate) fn certifying_attach<T: Decide + geom_core::CertifiedBounds>(
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    stated: Option<&StatedImage<T>>,
) -> Result<EdgeCurve<T>, CertifyError> {
    let lane = NurbsLane::certified();
    EdgeCurve::certify_stated(spec, start, end, surfaces, reach, band, lane, stated)
}

/// **A certifying scalar's edge door at rest**
/// ([`AtRestPolicy::reattach_edge`]): the certificate re-derived in full.
#[allow(clippy::too_many_arguments)] // the door's own inputs
pub(crate) fn certifying_reattach<T: Decide + geom_core::CertifiedBounds>(
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    stated: Option<&StatedImage<T>>,
) -> Result<T::Carries<Certificate<T>>, CertifyError> {
    let lane = NurbsLane::certified();
    curve
        .recertify_stated(start, end, surfaces, reach, band, lane, stated)
        .map(<T as geom_core::CertifiedEnclosure>::carried)
}

/// **A tangent-transport scalar's edge door** ([`AtRestPolicy::attach_edge`]):
/// the mint, held with no certificate (DUAL-DESIGN DL1, DL3).
pub(crate) fn minting_attach<T: Decide + TangentTransport>(
    spec: EdgeCurveSpec<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<EdgeCurve<T>, CertifyError> {
    EdgeCurve::mint(spec, surfaces, band).map(EdgeCurve::uncertified)
}

/// **A tangent-transport scalar's edge door at rest**
/// ([`AtRestPolicy::reattach_edge`]): the mint, run again on the edge as
/// it stands.
pub(crate) fn minting_reattach<T: Decide + TangentTransport>(
    curve: &EdgeCurve<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<T::Carries<Certificate<T>>, CertifyError> {
    EdgeCurve::mint(curve.restated_spec(), surfaces, band).map(|_| T::uncertified())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve3};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::spline::KnotVector;
    use geom_core::{Dual64, Tol};

    use crate::props::AtRestPolicy;
    use crate::test_support_fixtures::{describe_as_intersections, geometric_cube};
    use crate::{EulerOpError, entity::EdgeKey};

    /// Re-carries one plane × plane edge of the unit cube at `T` as a
    /// degree-1 spline between its two planes (an analytic rung-3
    /// edge) through the attach door.
    fn respec_as_spline<T: AtRestPolicy>() -> Result<EdgeKey, EulerOpError> {
        let tol = Tol::witness();
        fn cube<S: AtRestPolicy>(tol: Tol) -> crate::Body<S> {
            let mut body = geometric_cube::<S>(tol).body;
            describe_as_intersections(&mut body, tol);
            body
        }
        let mut body = cube::<T>(tol);
        // The same cube at f64 (D9: the same keys) gives the edge's
        // corners as f64, which the carrier is lifted from.
        let reference = cube::<f64>(tol);
        let (edge_key, edge) = reference
            .edges()
            .next()
            .map(|(k, e)| (k, e.clone()))
            .unwrap();
        let (s1, s2) = crate::readback::edge_sides(&body, edge_key)
            .unwrap()
            .surfaces();
        let corner = |v| {
            *reference
                .get_point(reference.get_vertex(v).unwrap().point)
                .unwrap()
        };
        let p0 = corner(reference.get_half_edge(edge.he_plus).unwrap().start);
        let p1 = corner(reference.half_edge_end(edge.he_plus).unwrap());
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0])
            .unwrap()
            .map_scalar(T::from_f64);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(s1, s2),
                selector: geom_brep::CurveSelector::Witness(carrier.eval(T::from_f64(0.5))),
            },
            carrier: Curve3::Nurbs(Arc::new(carrier)),
            param_start: T::zero(),
            param_end: T::one(),
        };
        body.set_edge_curve(edge_key, spec, tol).map(|_| edge_key)
    }

    /// **`Origin::read` keeps a refusal typed but where the kernel owns
    /// a span refusal**: under `Kernels` every other refusal passes
    /// through as it came, and under `Callers` a span refusal does too.
    /// (The `Kernels` span panic has its rows in `crate::tier3_tests`.)
    #[test]
    fn an_origin_types_every_refusal_but_a_kernel_owned_span() {
        use super::{KernelSite, Origin};
        use geom_brep::{CertifyError, StatedSpan, WindingReading};
        let span = CertifyError::StatedInterval {
            refused: StatedSpan::Winding(WindingReading::Exceeded),
        };
        assert_eq!(Origin::Callers.read(span), span, "a caller's span refusal");
        for other in [CertifyError::Unimplemented, CertifyError::WindingExceeded] {
            assert_eq!(
                Origin::Kernels(KernelSite::Graft).read(other),
                other,
                "a kernel site's non-span refusal"
            );
        }
    }

    /// **The attach door is the scalar's**: an analytic rung-3 edge
    /// attaches at `f64`, certified, and at the dual, minted.
    #[test]
    fn the_attach_door_attaches_an_analytic_rung3_edge_at_every_scalar() {
        respec_as_spline::<f64>().expect("f64's door certifies the edge");
        respec_as_spline::<Dual64>().expect("the dual's door mints the edge");
    }
}

/// **No production edge certification but through this module.** The
/// compiler makes every entry above take an [`Origin`]; this census
/// makes the entries the only way `topo`'s production code reaches
/// `geom_brep`'s doors, so a new kernel re-mint cannot skip the choice.
///
/// **What it matches.** A door named by path (`EdgeCurve::certify`,
/// `geom_brep::EdgeCurve::recertify_stated`, an alias `E::certify`, a
/// function value with no call) whatever its qualifier, unless the
/// qualifier is one of [`origin_census::NOT_DOORS`], the types and modules
/// whose own `certify`/`recertify` is not an edge door (a path that
/// continues, `geom_brep::certify::NOT_A_SAMPLE`, names the module). A method call
/// `.recertify_stated(`, and `.recertify(` with the door's six arguments
/// or more: the other `recertify` methods `topo` calls take two
/// (`OffsetFitLane`) or five (`PcurveCache`, the edge's span among
/// them), and a counted argument list only over-counts (a closure's
/// `|a, b|`), so the arity reading errs loud.
///
/// **Production** is every file a test-only `cfg` does not gate, with
/// everything such a `cfg` gates in it blanked — an item, or a field, a
/// variant or an arm, each to its own end
/// ([`test_utils::source::attributed_extent`]): `cfg(test)`, and
/// `cfg(any(..))` over `test`, `feature = "test-support"` and
/// `-testing` features, the naming `scripts/gates/test-features-dev-only.sh`
/// holds to test builds.
///
/// **What it cannot match.** A door reached through a macro's
/// expansion, an `include!`d file, or a file a `#[path]` attribute
/// mounts; a `.recertify(` of fewer than six arguments on an
/// [`EdgeCurve`] reached through a trait or a wrapper that renames it;
/// a `geom_brep` door other than the six (none other returns a
/// [`CertifyError`] today). The door bodies the scalars' policies run
/// are this module's own ([`certifying_attach`], [`minting_attach`] and
/// their re-attach twins), so the policy impls name no door either. It reads spelling, not types, so a site's
/// [`Origin`] being the right one is a reviewer's call and the panic
/// rows' (`crate::tier3_tests`).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod origin_census {
    use crate::source_walk::{CodeOnly, crate_sources, src_root};
    use test_utils::source::{attributed_extent, balanced_end, boundary_after, line};

    /// `geom_brep`'s edge certification doors.
    const DOORS: [&str; 6] = [
        "certify",
        "certify_stated",
        "mint",
        "recertify",
        "recertify_stated",
        "uncertified",
    ];

    /// Qualifiers whose door-named function by path is not an edge door:
    /// this module, whose entries share the doors' names, the two other
    /// types with a `certify`/`recertify` of their own, and the offset
    /// derivation's chart, which has a `mint`.
    pub(super) const NOT_DOORS: [&str; 4] = [
        "policy_lane",
        "PcurveCache",
        "ApproxSurface",
        "ChartedNurbs",
    ];

    /// Whether `attr` (`#[cfg(..)]`, as written) gates its item to test
    /// builds.
    fn test_only(attr: &str) -> bool {
        let compact: String = attr.chars().filter(|c| !c.is_whitespace()).collect();
        let Some(inner) = compact
            .strip_prefix("#[cfg(")
            .and_then(|s| s.strip_suffix(")]"))
        else {
            return false;
        };
        let terms = inner
            .strip_prefix("any(")
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(inner);
        terms.split(',').all(|term| {
            term == "test"
                || term == "feature=\"test-support\""
                || (term.starts_with("feature=\"") && term.ends_with("-testing\""))
        })
    }

    /// `text`'s production code — comments and literals blanked, every
    /// item a test-only `cfg` gates blanked too — and the names of the
    /// file modules it declares under such a `cfg`.
    pub(super) fn production(text: &str) -> (String, Vec<String>) {
        let code = CodeOnly::of(text);
        let code = code.as_str();
        let mut kept = code.as_bytes().to_vec();
        let mut gated = Vec::new();
        let mut from = 0;
        while let Some(at) = code[from..].find("#[").map(|i| i + from) {
            let attr_end = balanced_end(code, at + 1).expect("an attribute closes");
            from = attr_end + 1;
            if !test_only(&text[at..=attr_end]) {
                continue;
            }
            let span = attributed_extent(code, at).expect("a gated item ends");
            let head: Vec<&str> = code[span.clone()].split_whitespace().collect();
            if code[span.clone()].ends_with(';')
                && let Some(at) = head.iter().position(|w| *w == "mod")
            {
                gated.push(head[at + 1].trim_end_matches(';').to_owned());
            }
            for c in &mut kept[span.clone()] {
                if *c != b'\n' {
                    *c = b' ';
                }
            }
            from = span.end;
        }
        (
            String::from_utf8(kept).expect("only ASCII spans are blanked"),
            gated,
        )
    }

    /// Every site in production `code` that reaches an edge door other
    /// than through this module, as `line: spelling`.
    pub(super) fn bypasses(code: &str) -> Vec<String> {
        let ident = |c: char| c.is_alphanumeric() || c == '_';
        let mut out = Vec::new();
        for door in DOORS {
            let path = format!("::{door}");
            for (at, _) in code.match_indices(&path) {
                let after = at + path.len();
                // `geom_brep::certify::NOT_A_SAMPLE` names the module.
                if !boundary_after(code, after) || code[after..].starts_with("::") {
                    continue;
                }
                let qualifier = code[..at]
                    .rsplit(|c: char| !ident(c))
                    .next()
                    .unwrap_or_default();
                if !NOT_DOORS.contains(&qualifier) {
                    out.push(format!("{}: {qualifier}{path}", line(code, at)));
                }
            }
            let method = format!(".{door}");
            for (at, _) in code.match_indices(&method) {
                let open = at + method.len();
                if !code[open..].starts_with('(') {
                    continue;
                }
                let arity = || {
                    let close = balanced_end(code, open).expect("a call closes");
                    let mut depth = 0usize;
                    let mut args = 0;
                    let mut filled = false;
                    for c in code[open + 1..close].chars() {
                        match c {
                            '(' | '[' | '{' => depth += 1,
                            ')' | ']' | '}' => depth -= 1,
                            ',' if depth == 0 => {
                                args += 1;
                                filled = false;
                                continue;
                            }
                            _ => {}
                        }
                        filled |= !c.is_whitespace();
                    }
                    args + usize::from(filled)
                };
                let reached = match door {
                    "recertify_stated" => true,
                    "recertify" => arity() >= 6,
                    _ => false,
                };
                if reached {
                    out.push(format!("{}: .{door}(", line(code, at)));
                }
            }
        }
        out
    }

    /// **Row: `topo`'s production code certifies an edge only through
    /// this module**, where the call names its [`super::Origin`].
    #[test]
    fn every_production_edge_certification_names_its_origin() {
        let files = crate_sources();
        let src = src_root();
        let read = |f: &std::path::Path| std::fs::read_to_string(f).expect("a readable source");
        // The file modules a test-only `cfg` gates, by the path each loads.
        let mut gated = std::collections::BTreeSet::new();
        for file in &files {
            let dir = match file.file_stem().and_then(|s| s.to_str()) {
                Some("lib" | "mod") => file.parent().expect("a file has a directory").to_owned(),
                Some(stem) => file.with_file_name(stem),
                None => continue,
            };
            for name in production(&read(file)).1 {
                gated.insert(dir.join(format!("{name}.rs")));
                gated.insert(dir.join(&name));
            }
        }
        let mut violations = Vec::new();
        let mut read_here = false;
        for file in &files {
            if gated.iter().any(|g| file.starts_with(g)) {
                continue;
            }
            if file.ends_with("policy_lane.rs") {
                read_here = true;
                continue;
            }
            for hit in bypasses(&production(&read(file)).0) {
                let shown = file.strip_prefix(&src).unwrap_or(file).display();
                violations.push(format!("src/{shown}:{hit}"));
            }
        }
        assert!(read_here, "the walk did not reach policy_lane.rs");
        assert!(
            gated.iter().any(|g| g.ends_with("tier3_tests.rs")),
            "the walk read no test-only module, so it cannot tell one: {gated:?}"
        );
        assert!(
            violations.is_empty(),
            "production code reaches a `geom_brep` certification door other than through \
             `policy_lane`, where the call names its `Origin`:\n{}",
            violations.join("\n")
        );
    }

    /// **The census's reading, on text that has something to get wrong.**
    #[test]
    fn the_census_reads_doors_by_shape_and_passes_test_code() {
        let src = r#"
use geom_brep::EdgeCurve as E;
#[cfg(test)]
mod fixtures;
#[cfg(any(test, feature = "test-support"))]
pub(crate) mod support;
#[cfg(debug_assertions)]
mod kept;
fn site(c: &EdgeCurve<f64>) {
    let a = geom_brep::EdgeCurve::certify_stated(spec, s, e, f, 1.0, band, lane, None);
    let b = E::certify(spec, s, e, f, 1.0, band, lane);
    let d = specs.map(EdgeCurve::recertify);
    let g = c.recertify(s, e, |k| lookup(k, x), 1.0, band, lane);
    let h = c.recertify_stated(s, e, f, 1.0, band, lane, None);
    let fine = crate::policy_lane::certify(Origin::Callers, spec, s, e, f, 1.0, band);
    let cache = PcurveCache::certify(p, t0, t1, &carrier, surface, band);
    let lane = lane.recertify(approx, tol);
    let row = cache.recertify(&carrier, span, surface, band, lane,);
    let sample = geom_brep::certify::NOT_A_SAMPLE;
    // EdgeCurve::certify(spec) in a comment
    let s = "EdgeCurve::certify(";
}
#[cfg(test)]
mod tests {
    fn plant() { let _ = EdgeCurve::certify(spec, s, e, f, 1.0, band, lane); }
}
#[cfg(any(test, feature = "sweep-testing"))]
fn injector() { let _ = EdgeCurve::recertify_stated(c, s, e, f, 1.0, band, lane, None); }
struct Depth {
    depth: u32,
    #[cfg(test)]
    torn: bool,
}
impl Clone for Depth { fn clone(&self) -> Self { EdgeCurve::mint(spec, f, band) } }
enum Exempt {
    #[cfg(any(test, feature = "test-support"))]
    Nothing,
}
impl Exempt { fn f() { E::uncertified(minted); } }
fn arms(x: u8) {
    match x {
        #[cfg(test)]
        0 => EdgeCurve::certify(spec, s, e, f, 1.0, band, lane),
        _ => EdgeCurve::certify_stated(spec, s, e, f, 1.0, band, lane, None),
    }
}
struct Last {
    #[cfg(test)]
    torn: bool }
mod later;
"#;
        let (code, gated) = production(src);
        // `later` follows a gated last field, and is not gated with it.
        assert_eq!(gated, ["fixtures", "support"], "the gated file modules");
        assert_eq!(
            bypasses(&code),
            [
                "11: E::certify",
                "10: EdgeCurve::certify_stated",
                "44: EdgeCurve::certify_stated",
                "34: EdgeCurve::mint",
                "12: EdgeCurve::recertify",
                "13: .recertify(",
                "14: .recertify_stated(",
                "39: E::uncertified",
            ]
            .map(String::from)
            .into_iter()
            .collect::<Vec<_>>(),
            "the bypasses, in door order: a gated field, variant or arm takes \
             only itself, and the item after it is read"
        );
    }
}
