---
id: profile-embed-lift-has-two-homes-anchor-and-loft
kind: issue
title: Profile<f64> -> Profile<T> is written twice, editor-core's anchor::embed_profile and sweep's loft::end_profile, and the home is a lift on the profile types
status: review
opened: 2026-09-08
refs: [2139, 2186, D385]
branch: wire/profile-lift-door
pr: 2409
---

(EVAL orchestrator) From EVAL-1's style review (PR 2139, S1/S4/S8).
`crates/editor-core/src/eval/anchor.rs` `embed_profile` and
`crates/sweep/src/loft.rs:225`–`:245` `end_profile` are the same
function — `Profile<f64> → Profile<T>`: per loop, per vertex `pos` +
`bulge` through `from_f64`, `with_tangent_joints(..to_vec())`,
`Profile::new(plane, loops)` — in two crates. The home is a lift on
the profile types themselves (`ProfileVertex`/`ProfileLoop`/`Profile`,
the `map_scalar` door `D385` names for the test crates),
in `crates/profile`. Two placement facts ride along: `embed_profile`
sits in a module whose doc is "program-anchored profile naming" and
is neither, and it is the file's one `pub` (re-exported at
`eval/mod.rs`) with one caller (`wire.rs`); the door's arrival is the
moment it leaves. Where else to look: `rg 'ProfileVertex::new\(' crates/*/src`.

`crates/profile` is S-BOOL's by the paths lattice and not by its
charter, and the consumers are EVAL's and BLEND's; the owner is
undecided, which is what `work/issues/` is for. Citations accurate at
`bd2fe4289`.

## Narrowed (EVAL-8, PR 2186)

`anchor::embed_profile` is retired: the pinned arm lifts the pre-pass's
VALIDATED form instead (`ValidatedProfile`'s lift door in
`crates/profile/src/validate.rs`), so the production copy is gone.
What remains of the class, at PR 2186's review head:

- `crates/sweep/src/loft.rs:225`–`:245` `end_profile` — the same
  `Profile<f64> → Profile<T>` loop, AND it then re-`validate`s the
  exact lift at `T`, which is exactly the re-decision EVAL-8 removed
  from the pinned arm (EVAL-8's correctness review); the validated
  lift door now exists to replace both halves.
- Two test copies of the raw lift: `crates/editor-core/tests/pinned_lift_validates_once.rs`
  `embed` (born in the same PR that retired the production one) and
  `crates/profile/tests/common/mod.rs` `lift`.

The home is unchanged: a `map`/lift on `Profile`/`ProfileLoop`
themselves in `crates/profile` (D385's `map_scalar`), which the
validated door would then be built over.

## Re-homed to WIRE (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

WIRE is the evaluation seat's successor. `docs/DOC-LEDGER.md` sweep 9
parked three of these rows in `work/issues/` as "the successor's opening
slate" when EVAL closed on 2026-09-08, naming two more already there;
this program is that successor, and it takes the rest of the seat's
ground with them.

Its class at the cut was **M** — new public lift API across three
crates; owner undecided and D385 door shape must be settled. The class
is a dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.

## The `D385` seam, located (2026-09-11, WIRE orchestrator)

`D385` is `work/tint/D385.md`, not `work/tcost/D385.md` — it moved to
S-TINT at that program's opening on 2026-09-11, and the pointer in the
body above was written before the move and is corrected there. What the
move means for this row is that **the two test copies in the class are
not WIRE's to convert**: `D385`'s fence is Track W (`crates/*/tests/`
and `crates/test-utils/`), it names `crates/profile/tests/common/mod.rs`
by line, and its own rule is that *"a W row whose mechanism reaches into
a crate's `src/` is filed on the owning track rather than edited
there."*

So the class splits cleanly along the fence and the ORDER is forced:

1. **WIRE mints the door** in `crates/profile/src/` (announced seam —
   `crates/profile/*` is S-BOOL's glob) and retires
   `sweep::loft::end_profile`'s production copy against it. That is this
   row plus `profile-has-no-scalar-lift-door`, the sequence
   `work/wire/plan.md` already orders together.
2. **S-TINT's `D385` then converts the test copies** —
   `crates/profile/tests/common/mod.rs` `lift` and
   `crates/editor-core/tests/pinned_lift_validates_once.rs` `embed` —
   against a door that by then exists. `D385` today lists the first and
   not the second; that is a gap in `D385`'s list, and announcing this
   sequence to S-TINT is how it closes.

Announce to both S-BOOL (the door's file) and S-TINT (the dependency) at
dispatch, not at merge.
