//! **The certified door's quadrature IS the measurement door's, at
//! every scalar that can reach it** — pinned structurally, because at
//! `f64` alone it is a proof and everywhere else it is a convention.
//!
//! # What is being pinned, and why prose could not hold it
//!
//! `topo::validate_geometric_certificate` returns the object check 7
//! decided on, and its doc claims that object is bit-identical to
//! `topo::mass_properties` on the same body at the same `tol`. The two
//! do not reach the quadrature the same way:
//!
//! * `mass_properties` → `mass_properties_with` → **`T::quad_cut_face`**
//!   (the scalar's own lane, dispatched through `PropsQuadLane`);
//! * `mass_properties_certified` → **`quad_lane::cut_face`** (named
//!   directly, no dispatch).
//!
//! They agree because every `PropsQuadLane` impl that can form the
//! certified call defines `quad_cut_face` as `quad_lane::cut_face(..)
//! .map(Some)` — the same function, the same arguments, in the same
//! order. That is a fact about four impl bodies, not a type-system
//! guarantee: an impl could forward to a different quadrature, or
//! reorder the arguments, and every value in the tree would still be
//! self-consistent at that scalar while the door's identity claim
//! quietly became an agreement claim. The `sweep` certificate suite
//! proves the identity at `f64` on a real body; this file pins the
//! reason it holds at the others.
//!
//! # What this covers, and what it does not
//!
//! COVERS: every `PropsQuadLane::quad_cut_face` impl in
//! `crates/topo/src/props.rs` — `f64`, `Probe`, `Interval` and
//! `Sym<T>` (the four scalars with certification rights, which are the
//! ones that can form the certified call) and `Dual` (which cannot, and
//! answers `Ok(None)` so that it never does). A sixth impl, or a third
//! spelling of the body, reds here.
//!
//! `Sym<T>` (the symbolic identity tier, ERROR-DESIGN E12) joined the
//! census with this file's count, which is the argument this pin asks
//! for. It is a WRAPPER over a certifying base scalar, and it changes
//! exactly one thing: how a margin whose expression is identically zero
//! decides — inside `geom_core::Decide`, below every lane. Its
//! quadrature is therefore not a second quadrature at all; the impl is
//! the same call to the same `quad_lane::cut_face`, and this file
//! checks that character-for-character rather than taking the argument
//! on trust. What would break the door's identity claim here is the
//! opposite move: giving the tier `Ok(None)`, which would silently
//! demote a certifying lane the driver's leaf replay depends on.
//!
//! DOES NOT COVER: what `quad_lane::cut_face` itself computes, or the
//! band, face order and schedule the two callers hand it — those are
//! the same call site's arguments and the `sweep` suite's bit-identity
//! row is the evidence for them. It is a SOURCE-TEXT pin, so it also
//! says nothing about an impl living outside this file (there is none;
//! the trait is sealed to this module's five).
//!
//! Ungated: it reads one file and runs no geometry, so there is nothing
//! for a change filter to save.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

/// The accepted `quad_cut_face` bodies, whitespace-normalised.
///
/// Two, and the second is not a weakening: `Dual`'s `Ok(None)` is the
/// STATICALLY-no-quadrature arm the `compile_fail` guarantee on
/// `validate_geometric_certificate` rests on — a lane that answers
/// `None` cannot produce a certificate, so it can never be the source
/// of a divergence with one.
const CERTIFIED_BODY: &str = "quad_lane::cut_face(body, surface, outer, hes, band, tol).map(Some)";
const NO_LANE_BODY: &str = "Ok(None)";

#[test]
fn every_quad_cut_face_impl_forwards_to_the_certified_quadrature() {
    let props = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("props.rs");
    let text = std::fs::read_to_string(&props).expect("topo's props.rs is readable");
    // Comments blanked and literals blanked: a doc comment quoting the
    // call must not answer for an impl, and neither must a string.
    let code = test_utils::source::code_only(&text);

    let mut bodies = Vec::new();
    let mut rest = code.as_str();
    let mut base = 0usize;
    while let Some(at) = rest.find("fn quad_cut_face") {
        let abs = base + at;
        // The signature's argument list, then the body: two balanced
        // regions in a row, so the body's opening brace is the first
        // `{` after the argument list closes.
        let arg_open = code[abs..]
            .find('(')
            .expect("a signature has an argument list")
            + abs;
        let arg_end =
            test_utils::source::balanced_end(&code, arg_open).expect("the argument list closes");
        // The trait's own DECLARATION of the method has no body — its
        // return type is followed by `;`. Skip it: this pin is about
        // the impls, and the declaration is what they answer to.
        let semi = code[arg_end..].find(';').map(|i| i + arg_end);
        let brace = code[arg_end..].find('{').map(|i| i + arg_end);
        let body_open = match (semi, brace) {
            (Some(s), Some(b)) if s < b => {
                base = s;
                rest = &code[base..];
                continue;
            }
            (_, Some(b)) => b,
            (_, None) => panic!("a fn is either declared or defined"),
        };
        let body_end = test_utils::source::balanced_end(&code, body_open).expect("the body closes");
        let body: String = code[body_open + 1..body_end]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        bodies.push(body);
        base = body_end;
        rest = &code[base..];
    }

    assert_eq!(
        bodies.len(),
        5,
        "LANE CENSUS: props.rs must hold exactly the five PropsQuadLane impls this pin \
         reasons about (f64, Probe, Interval, Sym, Dual); found {} — a new scalar's lane \
         is a new place the certified door's identity claim can break, so it is argued \
         here rather than added silently: {bodies:?}",
        bodies.len()
    );
    let certified = bodies.iter().filter(|b| *b == CERTIFIED_BODY).count();
    let no_lane = bodies.iter().filter(|b| *b == NO_LANE_BODY).count();
    assert_eq!(
        (certified, no_lane),
        (4, 1),
        "LANE IDENTITY: each certifying scalar's `quad_cut_face` must BE \
         `{CERTIFIED_BODY}` — the same function the certified door names directly, with \
         the same arguments in the same order — and the one lane that cannot certify \
         must be `{NO_LANE_BODY}`. A body that is neither makes \
         `validate_geometric_certificate`'s bit-identity claim an agreement claim at \
         that scalar. Found: {bodies:?}"
    );
}
