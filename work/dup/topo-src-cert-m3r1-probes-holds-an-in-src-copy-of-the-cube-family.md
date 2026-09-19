---
id: topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family
kind: issue
title: cert_m3r1_probes.rs carries a verbatim in-src copy of the whole geometric-cube fixture family
status: closed
opened: 2026-09-16
closed: 2026-09-18
refs: [half-edge-to-face-walk-is-spelled-once-per-suite]
---


## Finding

- **Where**: `crates/topo/src/cert_m3r1_probes.rs` — `GeoCube`
  (~`:34`), `line` (~`:41`), `plane` (~`:45`), `geometric_cube`
  (~`:50`) and `describe_as_intersections` (~`:160`), against
  `crates/topo/tests/common/mod.rs`'s members of the same names.
- **Importance**: medium
- **Confidence**: sure they are copies — **the file says so twice**
- **Raised by**: the `dup-cube-seq` lane (S-DUP), 2026-09-16
- **Refs**: `brick-has-two-constructions-and-two-homes` (this is a
  further consumer of that row's third link)

Two doc comments in `cert_m3r1_probes.rs` state it outright:
*"`topo/tests/common::geometric_cube`, copied verbatim (in-crate)"* and
*"`topo/tests/common::describe_as_intersections`, copied verbatim"*.
`GeoCube`, `line` and `plane` come with them unlabelled.

**Both of those sentences went stale on 2026-09-16, in PR #2727.** That
PR reconciled `geometric_cube` and `cube_into` onto a shared `cube_ops`,
so `tests/common::geometric_cube` is now a seven-line wrapper and the
in-`src` copy is a verbatim copy of a shape the tree no longer has.
Nothing broke and no behaviour moved — the copy still builds the body it
always did, and that PR's measurement confirmed it — but the copy and
its original have now diverged in FORM as well as in address, and the
copy's own self-description is the thing that is wrong about it. Worth
recording when it happened, because the next reader will otherwise date
the divergence from whenever they notice it.

The copy's Euler-op call-site counts are identical to the shared
sequence's — `{mvfs: 1, mev: 2, mef: 5, MefSite::Chords: 5,
MevSite::Fan: 1, set_face_surface: 1}` — which is how a shape sweep
found it; a name sweep found it only because the doc comments name the
original.

## Why it cannot be fixed today, and what unblocks it

The module is in `crates/topo/src/`, and its header explains why it has
to be: the corruption route it exists to drive needs `Body::surfaces`,
which is `pub(crate)`. Nothing in `src/` can name
`crates/topo/tests/common/mod.rs`, which compiles into the test binary
only. So the copy is not a lazy one — it is what the current home
layout forces.

**It is unblocked by exactly the move
`brick-has-two-constructions-and-two-homes` adjudicates as its third
link**: once the fixture family lives in
`crates/topo/src/test_support_impl.rs`, an in-crate module can name it
and these five items delete. The mover should count this file in the
consumer set, which the row's *"set that would have to move"* does not
yet list.

One thing does NOT delete with them: `cert_m3r1_probes.rs` also carries
`face_surface_of_he`, which `tests/common`'s
`describe_as_intersections` spells as a local closure. Whichever home
wins, that helper is one function, not two.

## The "copied verbatim" claim went staler again (2026-09-18, `dup/one-prism-builder`)

`cert_m3r1_probes.rs:49`'s *"`topo/tests/common::geometric_cube`, copied
verbatim (in-crate)"* is now wrong **twice over**, and neither is a
defect — the bodies are still equal, proved byte-for-byte by that
unit's before/after dumps. It is doc rot, and it is worth recording
because it is the second time the same sentence has decayed without
anyone touching either file's code:

- **Link 1 (PR #2727)** made `tests/common`'s `geometric_cube` a thin
  caller of a shared `cube_ops`, so the in-`src` copy stopped being a
  copy of the named function and became a copy of what that function
  used to be.
- **This unit** goes further: `tests/common`'s `geometric_cube` is now
  four lines over `prism_ops` at `UNIT_SQUARE`, N-general, while the
  in-`src` copy is still the unrolled eight-corner ladder with `a`,
  `b`, `cc`, `d` spelled out. Nothing about them is verbatim any more
  except the body they build.

Nothing in `crates/topo/src/` was touched to fix it — out of this
unit's fence, and the sentence is the mover's to correct when the
family lands in `src/test_support_impl.rs`. The lesson for whoever
does: **a doc comment naming another file's function as its source
rots every time that function is refactored**, and this one has no
guard. If the in-`src` copy survives the move at all, the claim it
carries should be one a test can check (the two build equal bodies)
rather than one only a reader can.

## The witness gate does not reach this copy (measured 2026-09-18, link 2)

`work/dup/brick-has-two-constructions-and-two-homes.md` names this file
as unblocking with link 2, and that is right about namability and wrong
about the gate. `crates/topo/src/lib.rs:169` mounts it
`#[cfg(test)] mod cert_m3r1_probes;`, and
`scripts/gates/witness-not-ambient.sh` runs `gate_production_sources`,
whose `gate_filter_test_only_paths` takes a `#[cfg(test)] mod x;` module
**out of the file set entirely** — the gate never reads this file. Its
**12** `Tol::witness()` calls are legal where they sit and will stay
legal.

Established by the measurement on the same day at the other mount:
planting one `Tol::witness()` in
`crates/topo/src/test_support_impl.rs` — whose
`#[cfg(any(debug_assertions, test, feature = "test-support"))]` mount
`GATE_CFG_TEST_NOT_RE` refuses to narrow — fires the gate and names the
line, while this file's twelve sit in the same crate untouched. The
difference is the cfg shape, not the directory.

**So a mover gets no help from the gate here.** If this copy is folded
into the shared family after link 3, its call sites have to thread
`tol: Tol` because the family's doors now take it (link 2), not because
anything would red if they did not.


## Closed (2026-09-18, `dup/move-the-fixture-family`)

**Folded.** `crates/topo/src/cert_m3r1_probes.rs` no longer holds a
copy of anything. `GeoCube`, `line`, `plane`, `geometric_cube`,
`describe_as_intersections` and the local `face_surface_of_he` are
deleted from it; it now names
`crate::test_support_fixtures::{geometric_cube, describe_as_intersections,
face_surface_of_he}` and builds nothing box-shaped. The file went from
382 lines to 225. What it keeps is its own subject: `nurbs_wall`,
`m7_8_cube`, `six_doors`, `edge_cert_count`, `DOOR_NAMES` and the row.

**The two stale "copied verbatim" sentences are gone with the code they
described**, so the doc-rot this row tracked over two decays has no
carrier left. The lesson it drew — *if the in-`src` copy survives the
move, the claim it carries should be one a test can check* — is
discharged the other way: the copy did not survive.

**The header's `Body::surfaces` argument still holds and is still
served.** The corruption route needs `pub(crate)` access
(`body.surfaces[wall] = nurbs_wall(0.05)`), which is why this module is
in-crate; the fold does not move it out. `test_support_fixtures` is
mounted `#[cfg(any(test, feature = "test-support"))]`, so it exists in
the `cfg(test)` build this module compiles in, and an in-crate module
names it by path.

**The witness note was right and cost nothing.** This module is still
`#[cfg(test)] mod cert_m3r1_probes;`, so the gate still does not reach
it; its `Tol::witness()` calls stay legal. Two of them are new, at the
two folded doors, because the family's signatures take `tol: Tol` after
link 2 — exactly as this row predicted.

### Correction (2026-09-19, PR 2842's fix pass): the `face_surface_of_he` half is a HALF-fix

This row's *"whichever home wins, that helper is one function, not
two"* was written about the two spellings **this row could see**, and
PR 2842 folded four. That closes the row's own claim and nothing
wider. The helper is a member of a class this row never measured: the
half-edge → loop → face walk is spelled in 56 tracked files, including
two byte-identical closures in `topo/src` (`shell.rs`'s `face_of_he`,
`replace_face.rs`'s `face_of`) that no census in links 1–3 disclosed.
Filed as `work/dup/half-edge-to-face-walk-is-spelled-once-per-suite.md`,
which carries the measurement and names those two as the cheapest next
pair. **A name census closes name collisions; this class is one thing
under many names**, which is why folding four of them is not closing
it.
