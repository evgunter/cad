---
id: topo-src-cert-m3r1-probes-holds-an-in-src-copy-of-the-cube-family
kind: issue
title: cert_m3r1_probes.rs carries a verbatim in-src copy of the whole geometric-cube fixture family
status: open
opened: 2026-09-16
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
