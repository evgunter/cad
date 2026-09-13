---
id: sweep-emits-no-contact-record-for-declared-cusps
kind: issue
title: extrude returns a body whose declared cusps have no contact record, so tier 3 must be re-declared by the caller
status: open
opened: 2026-09-08
---


Found by the BLEND unit 2 style review (PR 2122), reading the doc comment on
`Extruded::body` against the code it describes.

## The mechanic

`crates/sweep/src/extrude.rs:125-136` documents `Extruded::body` as tiers 1–2
by construction and NOT unconditionally tier 3: a profile carrying a DECLARED
cusp joint builds a rim edge that subtends material wedge 0, which the at-rest
gate refuses as `topo::ValidationError::UndeclaredCusp` unless a declaration
accompanies the body. The escape hatch the doc names is real —
`topo::validate_geometric_declared` (`crates/topo/src/validate.rs:2661`) takes
`&[topo::contact::DeclaredContact]` — but the caller has to supply the
declarations itself.

The builder supplies none. `crates/sweep/src/extrude.rs` contains no
`DeclaredContact`, no `ContactClass` and no cusp handling of any kind: the
only occurrences of the word in the file are in that doc comment (grep
`cusp|Cusp|DeclaredContact` over `crates/sweep/src/` returns those three doc
lines and nothing else). `Extruded` (`crates/sweep/src/extrude.rs:124-166`)
has no field for them either. So the profile's own `.cusp()` declaration —
authored through `crates/profile/src/path.rs`'s cusp door and carried into the
validated profile — is dropped at the extrude boundary, and every caller that
wants a declared-cusp body validated has to rebuild the face pairing by hand
off `Extruded::side_faces`.

The three in-tree callers all do exactly that — each builds the face pair
itself and hands it in as a literal array:
`crates/sweep/tests/r1_mate3_probes.rs:73-79`,
`crates/sweep/tests/r2_mate3_probes.rs:97-102` and
`crates/sweep/tests/m9_3_zip.rs:369, 387`.

## Why it is filed now, and not as a MATE-3 residue

The doc sentence deferred the fix with a forward citation — "until the sweep
lane emits one (the declaration-emission handoff, MATE-3)". MATE-3 is closed
(PR 1423, merged `c7e48ff89`; ledger row `MATE-3-SPEC.md` in
`docs/DOC-LEDGER.md`), so the citation points at a finished program and reads
as scheduled work that is not scheduled.

What MATE-3 actually shipped, from its diff (`git diff e102fc3e9 b2a69f1f1`):

- the material-side wedge check and the declared arm in
  `crates/topo/src/validate.rs` (`validate_geometric_declared`, the
  `UndeclaredCusp` verdict) with `DeclaredContact` in `crates/topo/src/contact.rs`;
- the `.cusp()` authoring door in `crates/profile/src/path.rs` (+
  `path/program.rs`, `declared_tangency.rs`);
- the ASSEMBLY-side emitter — `crates/editor-core/src/assembly.rs` turns a
  document's mate declarations into the kernel's contact record set and gates
  through `gate_at_rest_declared`;
- in `crates/sweep/src/extrude.rs`, TWO comment edits only: this `Extruded::body`
  paragraph and the `Smooth` arm's note in `upgrade_rim`.

So the emitter MATE-3 shipped is the assembly's, over mates. The sweep's own
declared-cusp emission was never in its fence, and nothing since has taken it.

PR 2122 rewrote the doc sentence to state the present situation without the
closed-program citation (the builder emits no contact record; validate such a
body through `validate_geometric_declared`). This file is the residue that
sentence used to carry.

## What a fix looks like

Either `Extruded` grows a `declared_contacts: Vec<DeclaredContact>` field that
`extrude` fills from the validated profile's declared joints (the pairing is
known at mint time — the two side faces adjacent to the strut of a declared
joint), or the decision is ratified the other way and the doc says the caller
owns the declaration permanently. The first needs the `Lofted` twin considered
with it; the second needs no code.
