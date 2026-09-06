---
id: LIB-B-FACE-FRAME
kind: unit
title: binding census family B-FACE-FRAME
status: review
branch: lib/b-face-frame
opened: 2026-09-04
pr: 2074
---


Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope (stated before any code changed)

`crates/pncad-py/tests/test_binding_census.py` charters `B-FACE-FRAME`
in `FAMILIES` (DOCM-1, DOCM-REFERENCES-DESIGN DM1/DM1a/DM2) and
**exactly one** `NOT_BOUND` entry cites it: `face_carrier_kind`
(`test_binding_census.py:1585`). Nothing else in the census names the
id — the only other occurrences are the charter itself (`:723`) and
the comment above the entry (`:1583`). That single row IS the census
roster of the family, and the charter names three doors, so the sweep's
first job is to say why the other two are not rows and cannot become
ones:

- **`Pose.sense` is a FIELD of a curated type.** This census's
  alphabet is top-level stub names plus the `Class.member` spellings
  `BOUND_AS` points AT. `Pose` is curated (`crates/pncad/src/select.rs`
  re-exports `topo::readback::Pose`) and `pncad.pyi:2351` spells it
  identically, so rule 1 accounts it WHOLE and a field it fails to
  project is invisible here. That is the blind spot
  `crates/pncad-py/src/py/select.rs`'s growth tripwire already records
  for enum VARIANTS (issue #1309), one level out. `sense` is owed work
  and owes no row.
- **`Datum.face_frame` is a Python spelling, not a curated Rust
  name.** The curated authoring name is `Datum`
  (`crates/pncad/src/document.rs:40`, the `editor_core` enum whose
  `FaceFrame` arm is `crates/editor-core/src/node.rs:704`), and
  `pncad.pyi:2223` declares a top-level `Datum` — so rule 1 accounts
  it on SPELLING ALONE. The two are not the same type: Rust's is the
  AUTHORING enum whose arms Python spells as `Node.datum_*`
  constructors; Python's is the READ-side value `Value.datum()`
  answers with. A whole authoring arm can therefore go unbound behind
  a name-for-name match, and did. The census says outright it checks
  no semantics, so this is a true reading of its rule rather than a
  bug in it; it is banked as a finding, not acted on.

So the census delta is one entry and one charter, and the unit is not
a no-op for the other two names: they are surface that does not exist.

**What `pncad-py` can reach.** It depends on `pncad` and `quantity`
only, and all three doors are inside that:

- `pncad::select::face_carrier_kind` (curated, `select.rs:64`, and in
  the prelude at `prelude.rs:327`); the kernel body is
  `crates/editor-core/src/names/interrogate.rs:265`, which walks the
  same node ladder `face_frame` does and answers a `SurfaceKind`.
- `Datum::FaceFrame { at, face, spin }` through
  `pncad::document::Datum`; `spin` is `SlotId::Spin`, whose dimension
  is `Angle` (`crates/editor-core/src/node.rs:481`).
- `topo::readback::Pose::sense` through `pncad::select::Pose`
  (`crates/topo/src/readback.rs:106`).

`SurfaceKind` — the value `face_carrier_kind` answers with — is a
curated prelude name, spelled identically in `pncad.pyi:1996` and
already crossing INTO the kernel for `GeomPred.surface_kind`
(`py/select.rs::SurfaceKind::to_kernel`). The read door needs no new
vocabulary, only the crossing in the direction that did not exist.

**The census's own SHAPE decisions, honored not relitigated.**
`StableName`, `InterrogateError` and `FaceKey` are `different-shape`
rows in `NOT_BOUND`: a name crosses as opaque `str`, the interrogation
refusal as the typed `ReadbackError` exception, and an arena key not
at all. `face_carrier_kind` binds name-in / tag-out through the same
`name_from_text` / `readback_err` pair `face_frame` already uses, so
none of the three is re-opened.

**Refusal arms.** `NodeErrorKind::{FaceFrameResolve, FaceFrameKind,
FaceFrameNotPlanar, FaceFrameReadback}` already have tags
(`crates/pncad-py/src/tags.rs:416-419`) and are already pinned in
`crates/pncad-py/src/tests.rs`'s `TAG_INVENTORY` under
`node_error_tag`. The read door's own refusals are
`interrogate_error_tag`'s and are pinned arm-by-arm by
`readback_refusal_tags_are_stable` (`tests.rs:169`). So this unit adds
no tag function, no tag value and no inventory row; what it adds is
the first CONSTRUCTION of three of the four authoring arms from
Python, which is the half `TAG_INVENTORY` says outright it does not
cover (`node_error_tag` has no construction pin at all).

**What the Datum READ side owes.** Nothing new.
`Datum::FaceFrame` evaluates to `DatumValue::Frame`
(`crates/editor-core/src/eval/wire.rs:1016-1066` — origin from the
face pose, `u`/`v` from the u-reference turned by the spin about the
outward normal), and `py/value.rs::Value::datum` already projects
`DatumValue::Frame` as `kind == "frame"` with `axes`. A derived frame
reads back through the door an authored one does, by construction, and
the sweep asserts that rather than adding a projection.

## Outcome

All three doors bound, at these spellings:

- `Node.datum_face_frame(at, face, spin)` — a DAG input, an opaque
  face name and an `Angle` with no default. `datum_face_frame`, not
  `face_frame`, because the sibling constructors are `datum_plane` /
  `datum_axis` / `datum_axis_in_plane` and the prefix is what says
  which arm of the authoring enum a `Node` constructor makes.
- `Evaluation.face_carrier_kind(node, name) -> SurfaceKind` — the
  fifth read-back door, beside the three frames and `denotation`, for
  the reason the others hang there: a name is only meaningful against
  the run that minted it.
- `Pose.sense -> bool`, beside `axis`, with the `repr` carrying it.

`SurfaceKind` crosses OUT for the first time through a new
`py/select.rs::surface_kind`, written as `entity_kind` is — an
exhaustive kernel-side match with a caller, which retires the dead
twin in the growth tripwire rather than adding a second one.

`crates/pncad-py/tests/test_face_frame.py` is the positive form: 26
tests whose numbers are all oracles against the pose the read door
answers with (origin, `sense * axis`, `u_ref` turned by the spin), a
sketch-on-a-face scene that extrudes a boss out of a plate, the three
constructible refusal arms, and the DAG edge shown by the delete
refusal. Two ty fixtures in each direction. The census delta is
exactly the one predicted above: `face_carrier_kind` to `BOUND_AS`,
the charter deleted, the closure paragraph recording why a
three-door family had a one-row roster.

Three findings banked:
`work/lib/datum-crosses-name-for-name-as-two-types.md` (the census
blind spot this unit's scope argument uncovered),
`work/lib/pncad-py-comparable-enums-do-not-hash.md` (23 of 23, hit
writing a tally by carrier kind), and one fix taken rather than
banked: `crates/pncad-py/run-python-tests.sh` read a hardcoded
`$root/target` and so exited 1 on a successful build for every lane
that sets `CARGO_TARGET_DIR`, which the implementer discipline
requires of all of them.

## Home

LIB's, filed by DOCM at DOCM-1's merge (the Python surface is outside
DOCM's fence). Same class, same shape, unscheduled alongside it:
B-DISTRIBUTIONS, B-MEASURES, B-NOTATION.
