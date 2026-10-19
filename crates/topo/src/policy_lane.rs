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
//! **The plane × NURBS lane is the scalar's policy** in [`certify`],
//! [`certify_stated`] and [`recertify`], through
//! [`AtRestPolicy::nurbs_lane`]. `geom_brep` answers
//! [`CertifyError::NurbsLaneNotSupplied`] when handed `None`, which is all
//! that layer can know; here the `None` is the policy's answer, so the
//! absence is the scalar's, and the result type says so:
//! [`ByPolicy::NoLane`] is not a [`CertifyError`], so a door cannot pass
//! it on as a certification failure without naming it. [`certify_via`]
//! and [`recertify_via_stated`] take the lane as their caller holds it.

use geom::Surface;
use geom_brep::{Certificate, CertifyError, EdgeCurve, EdgeCurveSpec, NurbsLane, StatedImage};
use geom_core::{Band, Decide, Point3, Real};

use crate::geometry::SurfaceKey;
use crate::props::AtRestPolicy;

/// **Whose data a certification reads**, named at the call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Origin {
    /// A caller's: a spec a door was handed, or a body a door reads. A
    /// span refusal is that input's, and stays typed.
    Callers,
    /// The kernel's own: an edge the body stores, re-minted or
    /// re-certified at the band it was minted at, at the named site. Its
    /// span was decided forward and short of a full turn at that band;
    /// its parameters travel bitwise and its carrier's rate stays a
    /// positive bound, so the span's sign does (a split child's interval
    /// is one side of a parameter the split decided interior at that
    /// band). A span refusal is a kernel bug, and panics.
    Kernels(&'static str),
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

/// Why a certification through the scalar's policy did not certify.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum ByPolicy {
    /// The edge's certificate needs the NURBS lane and `scalar`'s
    /// policy holds none (a dual, DL1).
    NoLane {
        /// [`Real::NAME`] of the scalar the door ran at.
        scalar: &'static str,
    },
    /// Every other refusal, as the certifier raised it.
    Refused(CertifyError),
}

impl ByPolicy {
    fn of<T: Real>(error: CertifyError) -> Self {
        if error == CertifyError::NurbsLaneNotSupplied {
            Self::NoLane { scalar: T::NAME }
        } else {
            Self::Refused(error)
        }
    }
}

/// [`EdgeCurve::certify_via`] with `T`'s lane, the wedge levered at
/// `reach` ([`crate::wedge_reach`]).
pub(crate) fn certify<T: AtRestPolicy>(
    origin: Origin,
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
) -> Result<EdgeCurve<T>, ByPolicy> {
    certify_stated(origin, spec, start, end, surfaces, reach, band, None)
}

/// [`EdgeCurve::certify_via_stated`] with `T`'s lane: `stated` is a chart
/// image the body already holds for the edge on one of its surfaces, and
/// the wedge is levered at `reach` ([`crate::wedge_reach`]).
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
) -> Result<EdgeCurve<T>, ByPolicy> {
    EdgeCurve::certify_via_stated(
        spec,
        start,
        end,
        surfaces,
        reach,
        band,
        T::nurbs_lane(),
        stated,
    )
    .map_err(|error| ByPolicy::of::<T>(origin.read(error)))
}

/// [`EdgeCurve::certify_via`] with the lane its caller holds.
#[allow(clippy::too_many_arguments)] // the door's, plus the origin
pub(crate) fn certify_via<T: Decide>(
    origin: Origin,
    spec: EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    lane: Option<NurbsLane<T>>,
) -> Result<EdgeCurve<T>, CertifyError> {
    EdgeCurve::certify_via(spec, start, end, surfaces, reach, band, lane)
        .map_err(|error| origin.read(error))
}

/// [`EdgeCurve::recertify_via`] with `T`'s lane, the wedge levered at
/// `reach` ([`crate::wedge_reach`]).
pub(crate) fn recertify<T: AtRestPolicy>(
    origin: Origin,
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
) -> Result<Certificate<T>, ByPolicy> {
    recertify_via_stated(
        origin,
        curve,
        start,
        end,
        surfaces,
        reach,
        band,
        T::nurbs_lane(),
        None,
    )
    .map_err(ByPolicy::of::<T>)
}

/// [`EdgeCurve::recertify_via_stated`] with the lane its caller holds.
#[allow(clippy::too_many_arguments)] // the door's, plus the origin
pub(crate) fn recertify_via_stated<T: Decide>(
    origin: Origin,
    curve: &EdgeCurve<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    band: Band,
    lane: Option<NurbsLane<T>>,
    stated: Option<&StatedImage<T>>,
) -> Result<Certificate<T>, CertifyError> {
    curve
        .recertify_via_stated(start, end, surfaces, reach, band, lane, stated)
        .map_err(|error| origin.read(error))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve3};
    use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair};
    use geom_core::spline::KnotVector;
    use geom_core::{Dual64, Real, Tol};

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
                witness: carrier.eval(T::from_f64(0.5)),
            },
            carrier: Curve3::Nurbs(Arc::new(carrier)),
            param_start: T::zero(),
            param_end: T::one(),
        };
        body.set_edge_curve(edge_key, spec, tol).map(|_| edge_key)
    }

    /// **The attach door takes the scalar's lane**: an analytic rung-3
    /// edge attaches at `f64`, and at the dual — whose policy holds no
    /// lane — refuses naming the edge and the scalar.
    #[test]
    fn the_attach_door_refuses_an_analytic_rung3_edge_at_the_dual() {
        respec_as_spline::<f64>().expect("f64 holds the lane");
        let dual = respec_as_spline::<Dual64>();
        assert!(
            matches!(
                dual,
                Err(EulerOpError::NurbsLaneUnsupported {
                    edge: Some(_),
                    scalar,
                }) if scalar == <Dual64 as Real>::NAME
            ),
            "{dual:?}"
        );
    }
}

/// **No production edge certification but through this module.** The
/// compiler makes every entry above take an [`Origin`]; this census
/// makes the entries the only way `topo`'s production code reaches
/// `geom_brep`'s doors, so a new kernel re-mint cannot skip the choice.
///
/// **What it matches.** A door named by path (`EdgeCurve::certify`,
/// `geom_brep::EdgeCurve::recertify_via`, an alias `E::certify`, a
/// function value with no call) whatever its qualifier, unless the
/// qualifier is one of [`census::NOT_DOORS`], the types and modules
/// whose own `certify`/`recertify` is not an edge door (a path that
/// continues, `geom_brep::certify::NOT_A_SAMPLE`, names the module). A method call
/// `.recertify_via(` or `.recertify_via_stated(`, and `.recertify(` with
/// five arguments or more, the door's arity: the other `recertify`
/// methods `topo` calls take two (`OffsetFitLane`) or four
/// (`PcurveCache`), and a counted argument list only over-counts (a
/// closure's `|a, b|`), so the arity reading errs loud.
///
/// **Production** is every file a test-only `cfg` does not gate, with
/// every item such a `cfg` gates in it blanked: `cfg(test)`, and
/// `cfg(any(..))` over `test`, `feature = "test-support"` and
/// `-testing` features, the naming `scripts/gates/test-features-dev-only.sh`
/// holds to test builds.
///
/// **What it cannot match.** A door reached through a macro's
/// expansion, an `include!`d file, or a file a `#[path]` attribute
/// mounts; a `.recertify(` of fewer than five arguments on an
/// [`EdgeCurve`] reached through a trait or a wrapper that renames it;
/// a `geom_brep` door other than the six (none other returns a
/// [`CertifyError`] today). It reads spelling, not types, so a site's
/// [`Origin`] being the right one is a reviewer's call and the panic
/// rows' (`crate::tier3_tests`).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod census {
    use crate::source_walk::{CodeOnly, crate_sources, src_root};
    use test_utils::source::{balanced_end, boundary_after, line, skip_ws};

    /// `geom_brep`'s edge certification doors.
    const DOORS: [&str; 6] = [
        "certify",
        "certify_via",
        "certify_via_stated",
        "recertify",
        "recertify_via",
        "recertify_via_stated",
    ];

    /// Qualifiers whose `certify`/`recertify` by path is not an edge
    /// door: this module, other types' doors of those names, and
    /// `topo`'s own functions of those names.
    pub(super) const NOT_DOORS: [&str; 3] = ["policy_lane", "PcurveCache", "ApproxSurface"];

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
            let mut item = skip_ws(code, attr_end + 1);
            while code[item..].starts_with("#[") {
                item = skip_ws(code, balanced_end(code, item + 1).expect("closes") + 1);
            }
            let mut depth = 0usize;
            let mut end = None;
            for (off, c) in code[item..].char_indices() {
                match c {
                    '(' | '[' => depth += 1,
                    ')' | ']' => depth = depth.saturating_sub(1),
                    ';' if depth == 0 => {
                        end = Some(item + off);
                        break;
                    }
                    '{' if depth == 0 => {
                        end = balanced_end(code, item + off);
                        break;
                    }
                    _ => {}
                }
            }
            let end = end.expect("a gated item ends");
            let head: Vec<&str> = code[item..end].split_whitespace().collect();
            if code.as_bytes()[end] == b';'
                && let Some(at) = head.iter().position(|w| *w == "mod")
            {
                gated.push(head[at + 1].to_owned());
            }
            for c in &mut kept[at..=end] {
                if *c != b'\n' {
                    *c = b' ';
                }
            }
            from = end + 1;
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
                    "recertify_via" | "recertify_via_stated" => true,
                    "recertify" => arity() >= 5,
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
    let a = geom_brep::EdgeCurve::certify_via(spec, s, e, f, 1.0, band, None);
    let b = E::certify(spec, s, e, f, 1.0, band);
    let d = specs.map(EdgeCurve::recertify);
    let g = c.recertify(s, e, |k| lookup(k, x), 1.0, band);
    let h = c.recertify_via_stated(s, e, f, 1.0, band, lane, None);
    let fine = crate::policy_lane::certify(Origin::Callers, spec, s, e, f, 1.0, band);
    let cache = PcurveCache::certify(p, t0, t1, &carrier, surface, band);
    let lane = lane.recertify(approx, tol);
    let row = cache.recertify(&carrier, surface, band, lane,);
    let sample = geom_brep::certify::NOT_A_SAMPLE;
    // EdgeCurve::certify(spec) in a comment
    let s = "EdgeCurve::certify(";
}
#[cfg(test)]
mod tests {
    fn plant() { let _ = EdgeCurve::certify(spec, s, e, f, 1.0, band); }
}
#[cfg(any(test, feature = "sweep-testing"))]
fn injector() { let _ = EdgeCurve::recertify_via(c, s, e, f, 1.0, band, None); }
"#;
        let (code, gated) = production(src);
        assert_eq!(gated, ["fixtures", "support"], "the gated file modules");
        assert_eq!(
            bypasses(&code),
            [
                "11: E::certify",
                "10: EdgeCurve::certify_via",
                "12: EdgeCurve::recertify",
                "13: .recertify(",
                "14: .recertify_via_stated(",
            ]
            .map(String::from)
            .into_iter()
            .collect::<Vec<_>>(),
            "the bypasses, in door order"
        );
    }
}
