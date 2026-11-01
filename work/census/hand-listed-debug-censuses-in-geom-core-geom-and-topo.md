---
id: hand-listed-debug-censuses-in-geom-core-geom-and-topo
kind: unit
title: nine hand-written Debug impls across geom-core, geom and topo list their fields by hand and end in finish(), so a new field is silently unrendered under a completeness claim
status: closed
opened: 2026-09-06
refs: [2093]
branch: census/debug-census
closed: 2026-09-15
---

Found by #2093's sweep, which fixed the same class inside
`crates/viewer/`. Filed here on the VIEW orchestrator's direction:
implementer discipline §6 makes reporting the filing act outside a
lane's fence and gives the file to the party with the whole board, and
this program established on 2026-09-06 that a §6 report alone is not a
durable artifact — the same out-of-fence finding was reported twice
through §6 with nothing a later reader could find
(`work/issues/loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed`).

No program obviously owns these three crates' `fmt` impls, which is
what puts the file here rather than on a slate.

## The class

A `Debug` impl that names its value's fields by hand has no compile-time
tie to the declaration: a field added to the struct is silently absent
from every dump, visible only to a reader who reads one and wonders
what is not there. #2093 closed that inside `crates/viewer/` by
destructuring `Self` exhaustively in the impl, so a new field is an
E0027 unbound-pattern error, and by binding a field the dump will not
carry to `_` — a decision a reader can see and the compiler still
forces.

**These are the worse version of it**, because every one ends in
`finish()` rather than `finish_non_exhaustive()`: the rendering claims
to show every field while listing them by hand.

## The hit list

Enumeration rule: `grep -rnE "impl[^=]*\bDebug\b for" crates/`, minus
`crates/viewer/`. **Eight rows, nine concrete impls** —
`crates/geom/src/curves/nurbs.rs:221` is inside `nurbs_curve!`, invoked
twice (`crates/geom/src/curves/nurbs.rs:1492` for `CurveWindow2` and
`:1493` for `CurveWindow3`).

| site | value | fields listed | ends in |
|---|---|---|---|
| `crates/geom-core/src/spline/knots.rs:340` | `Span<'_>` | `kv` (as an address), `index`, `first_control`, `degree` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:262` | `SplineCoeffs<'_, E>` | `knots` (address), `coeffs` (address), `len` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:331` | `RationalCoeffs<'_, E>` | `knots`, `coeffs`, `len`, `weights` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:367` | `CoeffWindow<'_, E>` | `pair`, `span` | `finish()` |
| `crates/geom-core/src/spline/hull.rs:396` | `RationalWindow<'_, E>` | `pair`, `span` | `finish()` |
| `crates/geom/src/curves/nurbs.rs:221` (×2) | `CurveWindow2`, `CurveWindow3` | `curve` (address), `span` | `finish()` |
| `crates/geom/src/surfaces/nurbs.rs:164` | `SurfaceWindow<'_, T>` | `surface` (address), `span_u`, `span_v`, `base`, `stride` | `finish()` |

**One row is NOT this shape and is listed for completeness, not for
fixing.** `crates/topo/src/param_source.rs:84` is
`impl Debug for ParamSource`, where `ParamSource` is
`pub struct ParamSource(Arc<[u8]>)` (`param_source.rs:82`) and the body
is `write!(f, "ParamSource(<{} bytes>)", self.0.len())` — no
`debug_struct`, no `finish()`, and a one-field newtype has no census to
fall behind. #2093's own hit list carried it under a sentence claiming
"every one ends in `finish()`", which was wrong; it is corrected here.

So: **eight rows, nine concrete impls, seven of the eight rows in the
class.**

## Why each one is written out rather than derived

Every one of these is a borrow-carrying token, and each says so in its
own doc: a derived `Debug` would follow the reference and dump the
whole knot vector, control net or coefficient array at every `{:?}`.
That reason is good and is not what is at issue — the same reason
applies to the four walks #2093 rewrote. What is missing is the
exhaustive destructure that makes the hand-listing safe.

## The same files carry the same hazard in `PartialEq`

`hull.rs` and both `nurbs.rs` files write `PartialEq` field by field
beside each `Debug` — `hull.rs:275`, `:344`, `:376`, `:405`,
`crates/geom/src/curves/nurbs.rs:209` and
`crates/geom/src/surfaces/nurbs.rs:176` — on the same address-equality
argument. A field added to one of these values is silently
outside equality too, which is the sharper consequence: a `Debug` that
misses a field misleads a reader, an `Eq` that misses one answers
wrong. Unswept, and named here so the next reader does not have to
re-derive it.

## Specced as CENSUS-DEBUG (2026-09-15), with the hit list corrected

`docs/CENSUS-DEBUG-SPEC.md` binds it; the spec is deleted at merge and
this section survives.

**The hit list above was accurate on 2026-09-06 and is now short by
four.** Its own enumeration rule — `grep -rnE "impl[^=]*\bDebug\b for"
crates/`, minus `crates/viewer/` — gave eight rows and nine concrete
impls then. On 2026-09-15 it gives **14**, and four of the six arrivals
are in the class:

| site | value |
| --- | --- |
| `crates/mesh/src/memo.rs` | `PatchMemo` — `debug_struct` … `finish()`, 4 fields |
| `crates/editor-core/src/resolve/pick.rs` | `PickMemo` — same shape, 6 fields |
| `crates/editor-core/src/names/table.rs` | `NameTable` — same shape, 2 fields |
| `crates/geom-core/src/sym/memo.rs` | `DriveMemo` — same shape, 2 fields |

The other two arrivals are NOT the class and are recorded so a later
reader does not re-derive it: `topo/src/props.rs`'s `SignCertificate`
uses `write!`, and `editor-core/src/names/role.rs`'s `NameRef` delegates
to `self.0.name` — the latter being the PROSE-census rows' shape (a
renderer that delegates), not this one's.

**This is the row's own thesis arriving as evidence about the row.** A
hand-written list of hand-written lists decays at the same rate as its
subject, and nothing observed four arrivals in nine days. It is the
argument the unit's instrument half has to answer.

`crates/geom-core/src/sym/memo.rs` is SYM's and is a fence this row did
not previously reach.

## The `PartialEq` half is in scope (orchestrator's call, 2026-09-15)

The body above names it unswept and gives the reason it should not be:
a `Debug` that misses a field misleads a reader, an `Eq` that misses one
answers wrong. Same impls, same types, same files, same repair. Bounded
to the types whose `Debug` the unit touches; the rest are triaged and
filed rather than swept.

## Re-homed to CENSUS (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

CENSUS collects the rows of one class: a vocabulary spelled by hand in
several places, and the census or instrument that cannot see one of the
spellings. This row is a member of that class.

Its class at the cut was **M** — pattern already proven by #2093, but 9
impls in 3 crates and the PartialEq half needs a scope call. The class
is a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## The enumeration re-taken at implementation (2026-09-15)

The spec's 14-row enumeration holds, and the hit list above is corrected
in three further places the spec did not reach.

**1. `Span`'s `PartialEq` is in the Half-B set and neither the body
above nor the spec lists it.** The body names `hull.rs` (four),
`curves/nurbs.rs` and `surfaces/nurbs.rs` — six sites. There is a
seventh: `crates/geom-core/src/spline/knots.rs`'s
`impl PartialEq for Span<'_>`, sitting directly under the `Debug` the
list DOES carry, hand-listing all four fields on the same
address-equality argument. **Half B is seven sites, eight concrete
impls** (the `nurbs_curve!` one expands twice), not six/seven.

**2. `props.rs`'s `SignCertificate` is in the class**, and the criterion
that ruled it out is the wrong criterion. The spec excluded it because
it *"uses `write!"`*, i.e. on its terminator. But it renders in braced
struct shape —
`SignCertificate { volume in […], surface_area …, open_at …, target_refusal … }`
— so it makes the same completeness claim `finish()` makes, by hand, and
it reads `self.runs` by name. The class is the tie to the declaration,
not the spelling of the terminator. Not fixed (the spec says in terms
not to); held by name in the arrival census and filed on
`work/census/hand-listed-partialeq-siblings-outside-the-census-debug-fence.md`.

**3. A hand-list can hide one level down, behind a delegation.**
`crates/profile/src/lib.rs`'s `impl PartialEq for SketchPlane<f64>`
delegates to `SketchPlane::bit_eq`, which hand-lists twelve
coordinates through `origin()` and `placement.linear`. Neither the item
nor the spec reached it, and no text reader can: a field read through a
method is a call. Filed on the same row, and stated as the arrival
census's principal blind spot.

The enumeration rule's own blind spots, stated: it is line-based, so an
`impl` head spanning lines with `Debug` and `for` on different lines is
invisible (none in the tree — checked); it reads `crates/` only, and the
four excluded cargo roots plus `benches`, `demos/*` and `tools/*` hold
no `debug_struct` or hand-written `Debug`/`PartialEq` at all (checked);
and an impl assembled from macro metavariables is text until expansion.

## The instrument (2026-09-15)

`crates/test-utils/tests/hand_written_impl_census.rs` answers the
ARRIVAL question and nothing else. The per-field question is
compiler-known once an impl destructures — E0027 — and the census does
not re-ask it. Its subject is *"did a new hand-written `Debug` or
`PartialEq` land reading a field by name"*, which is a property of
source text and the thing that let four impls arrive in nine days
unobserved. Its blind spots are at the site.
