//! The mesh door: tessellation, mesh read-back, and STL.
//!
//! Steps 4 and 5 of the guide's ladder — *author → validate → measure
//! → tessellate → cross-check → export*. Rust runs them with
//! `tessellate` plus `mesh::validate`'s re-derivations; this module
//! binds the first and hands Python the mesh's own arrays so the
//! cross-check is a computation the caller can write, which is what
//! makes it INDEPENDENT of the exact measure it is checked against.
//!
//! # What crosses, and what does not
//!
//! `Mesh` is a handle over the kernel's own value. Its two ratified
//! contracts survive the crossing:
//!
//! * **the shared position buffer** — [`Mesh::positions`] is the one
//!   array every triangle indexes into, so adjacent faces sharing a
//!   boundary share indices and watertightness is checkable from
//!   Python by index equality, never by comparing coordinates;
//! * **patch separability** — [`Mesh::patch`] answers one face's
//!   triangles, and [`Mesh::triangles`] is those same patches
//!   concatenated in the fixed order the STL writers walk.
//!
//! What does NOT cross is the mesh's back-references. `FacePatch::face`,
//! `BoundaryPolyline::edge` and the two vertex back-references are
//! arena keys (`FaceKey`, `EdgeKey`, `VertexKey`), and keeping those
//! unnameable is what the whole curation is for — so a patch is
//! addressable by INDEX here and the per-edge boundary polylines,
//! whose only content beside indices is those keys, are not bound at
//! all.
//!
//! **The door from a patch to a `StableName` is the honest shape, and
//! since LIB-B-PICKING it exists on both sides**: `NodePick`'s
//! `patch_names` / `boundary_names` (`py/pick.rs`) answer one name per
//! patch and per polyline, in mesh order, with the key never leaving
//! the kernel. That is what makes a patch INDEX a usable handle rather
//! than a dead end — it is the argument of `NodePick`, not of this
//! module, because only a value that owns its pairing may invert a
//! key at all.
//!
//! # δ is a `Length`
//!
//! The chordal budget is a distance in metres (`0.5 * mm`), not the
//! kernel's ε and not a bare float: D6 puts it in the closed
//! `{Length, Angle, Count}` set, so it crosses as a `Length` like
//! every other distance the surface takes. Nothing is pre-checked —
//! zero, negative and non-finite budgets are the kernel's own
//! `InvalidChordalTolerance`, raised where the call was written.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyString};

use crate::errors::ErrorClass;
use crate::py::quantity::Length;
use crate::py::typed_err;
use crate::tags::{
    binary_header_error_tag, solid_name_error_tag, stl_error_tag, tessellate_error_tag,
};
use pncad::mesh;
use pncad::stl;

/// Raise `TessellateError` mirroring the kernel's refusal: `variant`
/// is the arm's stable tag, and the arms' NUMBERS and prose notes ride
/// as attributes, always present and `None` where inapplicable so a
/// stub-guided read cannot `AttributeError`.
///
/// The human message is the kernel's own `Display` prose, as at every
/// other door on this surface; the machine payload is the `variant`
/// tag (see `crate::tags`).
fn tessellate_err(py: Python<'_>, err: &mesh::TessellateError) -> PyErr {
    use mesh::TessellateError as T;
    let none = || py.None().into_any();
    let mut fields: Vec<(&str, Py<PyAny>)> = vec![
        (
            "variant",
            PyString::new(py, tessellate_error_tag(err))
                .unbind()
                .into_any(),
        ),
        ("value", none()),
        ("bound", none()),
        ("requested", none()),
        ("note", none()),
    ];
    let float = |v: f64| match v.into_pyobject(py) {
        Ok(bound) => Ok(bound.unbind().into_any()),
        Err(failed) => Err(PyErr::from(failed)),
    };
    // Exhaustive on purpose: an arm added kernel-side arrives here as
    // a compile error rather than as a silently unprojected payload.
    let projected = match err {
        T::InvalidChordalTolerance { value } => float(*value).map(|v| fields[1] = ("value", v)),
        T::ResolutionOverflow { count } => float(*count).map(|v| fields[1] = ("value", v)),
        T::CertificateExceeded {
            bound, requested, ..
        } => float(*bound).and_then(|b| {
            fields[2] = ("bound", b);
            float(*requested).map(|r| fields[3] = ("requested", r))
        }),
        T::UnsupportedNurbsFace { note, .. } | T::UnsupportedCurve { note, .. } => {
            fields[4] = ("note", PyString::new(py, note).unbind().into_any());
            Ok(())
        }
        T::MissingEntity { what } => {
            fields[4] = ("note", PyString::new(py, what).unbind().into_any());
            Ok(())
        }
        T::UnsupportedCurvedDomain { max_distance, .. } => {
            float(*max_distance).map(|v| fields[1] = ("value", v))
        }
        // The shape door's refusal is props' own, and its prose names
        // the structural expectation that failed (`props_rim_level`,
        // an incidence name); that sentence is the payload a caller can
        // act on, so it rides as the note. The band arm's payload is
        // the run's configuration failure, likewise a sentence.
        T::UnsupportedCurvedShape { source, .. } => {
            fields[4] = (
                "note",
                PyString::new(py, &source.to_string()).unbind().into_any(),
            );
            Ok(())
        }
        T::Band { error } => {
            fields[4] = (
                "note",
                PyString::new(py, &error.to_string()).unbind().into_any(),
            );
            Ok(())
        }
        // The key-only arms: the offending entity is an arena key, so
        // there is nothing to project that a caller may hold.
        T::UnsupportedSurface { .. }
        | T::NullScaffoldEdge { .. }
        | T::RingOnCurvedFace { .. }
        | T::EmptyLoop { .. }
        | T::Triangulation { .. }
        | T::SelfTouchingTrimLoop { .. } => Ok(()),
    };
    if let Err(failed) = projected {
        return failed;
    }
    typed_err(py, ErrorClass::Tessellate, err.to_string(), &fields)
}

/// Raise `StlError` with the writer's own message and a stable
/// `variant`.
fn stl_err(py: Python<'_>, variant: &str, message: String) -> PyErr {
    typed_err(
        py,
        ErrorClass::StlExport,
        message,
        &[("variant", PyString::new(py, variant).unbind().into_any())],
    )
}

/// A tessellated body: one shared position buffer and one triangle
/// patch per face.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Mesh {
    pub(crate) inner: Arc<mesh::Mesh>,
}

impl Mesh {
    /// Every patch's triangles in the fixed export order — the walk
    /// both STL writers make, so `triangles` and an exported file
    /// agree facet for facet.
    fn all_triangles(&self) -> impl Iterator<Item = (u32, u32, u32)> + '_ {
        self.inner
            .patches
            .iter()
            .flat_map(|patch| patch.triangles.iter().map(|t| (t[0], t[1], t[2])))
    }
}

#[pymethods]
impl Mesh {
    /// The shared position buffer, in the kernel's minting order:
    /// topology vertices first, then per-edge chord points, then
    /// per-face interior grid points.
    #[getter]
    fn positions(&self) -> Vec<(Length, Length, Length)> {
        let len = |x: f64| Length(pncad::quantity::Length::from_meters(x));
        self.inner
            .positions
            .iter()
            .map(|p| (len(p.x), len(p.y), len(p.z)))
            .collect()
    }

    /// Every triangle, as index triples into `positions`, in the fixed
    /// export order. Winding is OUTWARD (counterclockwise seen from
    /// outside the material), which is what makes a divergence-theorem
    /// volume over these triangles positive for a closed body.
    #[getter]
    fn triangles(&self) -> Vec<(u32, u32, u32)> {
        self.all_triangles().collect()
    }

    /// How many faces the mesh carries a patch for.
    #[getter]
    fn patch_count(&self) -> usize {
        self.inner.patches.len()
    }

    /// How many triangles the mesh carries in total.
    #[getter]
    fn triangle_count(&self) -> usize {
        self.inner.patches.iter().map(|p| p.triangles.len()).sum()
    }

    /// One face's triangles, by patch index — the addressability the
    /// `Mesh` value promises. The face itself is an arena key and does
    /// not cross, so the index is the handle.
    fn patch(&self, index: usize) -> PyResult<Vec<(u32, u32, u32)>> {
        let patch = self.inner.patches.get(index).ok_or_else(|| {
            pyo3::exceptions::PyIndexError::new_err(format!(
                "patch index {index} out of range ({} patches)",
                self.inner.patches.len()
            ))
        })?;
        Ok(patch.triangles.iter().map(|t| (t[0], t[1], t[2])).collect())
    }

    /// Write ASCII STL and answer the text.
    ///
    /// `solid_name` is the `solid <name>` name, and it is validated:
    /// a character outside the printable ASCII the single-line grammar
    /// admits refuses here, as `StlError` with a
    /// `solid_name_unrepresentable` variant, rather than being
    /// sanitized into a file no parser can read.
    #[pyo3(signature = (solid_name=""))]
    fn to_stl_ascii(&self, py: Python<'_>, solid_name: &str) -> PyResult<String> {
        let name = stl::SolidName::new(solid_name)
            .map_err(|err| stl_err(py, solid_name_error_tag(&err), err.to_string()))?;
        let mut out = Vec::new();
        stl::write_ascii(
            &self.inner,
            &stl::AsciiOptions { solid_name: name },
            &mut out,
        )
        .map_err(|err| stl_err(py, stl_error_tag(&err), err.to_string()))?;
        // The writer emits ASCII by construction (the name is
        // validated printable-ASCII and the numbers are formatted), so
        // a decode failure here would be a kernel defect, surfaced
        // rather than lossily replaced.
        String::from_utf8(out).map_err(|err| {
            stl_err(
                py,
                "not_utf8",
                format!("stl export: the ASCII writer emitted non-UTF-8 bytes: {err}"),
            )
        })
    }

    /// Write binary STL and answer the bytes.
    ///
    /// `header` is the 80-byte header field's free text —
    /// conventionally the producer. It is validated: a header that
    /// does not fit, or one that would make the file sniff as ASCII
    /// STL, refuses here as `StlError` rather than being truncated or
    /// written.
    #[pyo3(signature = (header=""))]
    fn to_stl_binary<'py>(&self, py: Python<'py>, header: &str) -> PyResult<Bound<'py, PyBytes>> {
        let header = stl::BinaryHeader::new(header)
            .map_err(|err| stl_err(py, binary_header_error_tag(&err), err.to_string()))?;
        let mut out = Vec::new();
        stl::write_binary(&self.inner, &stl::BinaryOptions { header }, &mut out)
            .map_err(|err| stl_err(py, stl_error_tag(&err), err.to_string()))?;
        Ok(PyBytes::new(py, &out))
    }

    fn __repr__(&self) -> String {
        format!(
            "Mesh({} positions, {} patches, {} triangles)",
            self.inner.positions.len(),
            self.patch_count(),
            self.triangle_count()
        )
    }
}

/// Tessellate a body at a chordal budget — the free `mesh::tessellate`
/// as a method on the value it takes, the posture `mass_properties`
/// and the validators already set.
pub(crate) fn tessellate(
    py: Python<'_>,
    body: &Arc<pncad::topo::Body<f64>>,
    chordal: &Length,
) -> PyResult<Mesh> {
    let tol = pncad::tolerance::Tol::witness();
    mesh::tessellate(body, chordal.0.meters(), tol)
        .map(|m| Mesh { inner: Arc::new(m) })
        .map_err(|err| tessellate_err(py, &err))
}

/// Register the mesh surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    Ok(())
}
