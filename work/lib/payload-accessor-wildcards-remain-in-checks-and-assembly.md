---
id: payload-accessor-wildcards-remain-in-checks-and-assembly
kind: issue
title: CheckEvidence and RefusedRef accessors still wildcard into None, the shape py/mate.rs just lost
status: closed
opened: 2026-09-08
closed: 2026-09-08
refs: [mate-fault-accessors-wildcard-into-silence, LIB-WILDCARDS]
---

Found by LIB-PROJ's sweep for the shape
`mate-fault-accessors-wildcard-into-silence` describes: a bound
value's per-arm payload accessor that ends `_ => None`, so an arm
added kernel-side answers `None` from every accessor silently instead
of failing the build. That item's ground was `py/mate.rs`, which is
now clean. **Two files still carry the shape.**

## The hits

The sweep was `grep -rn "_ => None\|_ => none()\|_ => \[" crates/pncad-py/src/`
over the merge base. Every hit, with its disposition:

- `crates/pncad-py/src/py/mate.rs` — 27 hits. **Fixed** by LIB-PROJ
  (`MateFault`, `MatePrimitive`, `Subgroup`, `ClusterMaintenance`).
- `crates/pncad-py/src/py/checks.rs:299`, `:311`, `:322`, `:331`,
  `:346` — five accessors over `d::CheckEvidence` (`actual`,
  `expected`, `other_root`, `other_output`, `reason`). **This item.**
  Same class exactly: `CheckEvidence` is a bound value whose class doc
  promises "the arm's payload as attributes", `check_evidence_tag` is
  the one exhaustive site, and an evidence arm added kernel-side would
  reach Python with five silent `None`s.
- `crates/pncad-py/src/py/assembly.rs:245`, `:255` — two accessors
  over `d::RefusedRef` (`width`, `kind`). **This item.** The file
  already has one exhaustive accessor immediately above them
  (`:235`), so the three disagree with each other about the same
  three-arm enum.
- `crates/pncad-py/src/validation.rs:73`, `:91` — **not a defect, and
  argued at the site.** The doc comment above `census_subject` states
  the extract licence: a site that picks one variant out and answers
  `None` to the rest is asking a question rather than mapping the enum
  onto a smaller vocabulary, and `validation_error_tag` is the
  exhaustive site that forces the decision. Left as is; it is the
  sentence the two rows above should either adopt or refuse.
- `crates/pncad-py/src/py/doc.rs:574`, `:589` — the same extract shape
  over `d::Node`, a ~40-arm recipe-node enum queried by node id
  ("or `None` for any other node" is the accessor's whole question).
  Not this class, and `py/doc.rs` is another lane's ground.
- `crates/pncad-py/src/prose_census.rs:1188` — a `filter_map` inside
  the crate's own source census, not a payload accessor.

**What the pattern could not match**: an accessor that spells its
catch-all as a named binding (`other => None`) or as a rest pattern,
and one whose fall-through is a `Some` of a default rather than
`None`. Neither spelling appears in `crates/pncad-py/src/` today, but
the grep would not have found them.

## Why it is a separate item

`py/checks.rs` and `py/assembly.rs` are LIB's ground and neither is on
LIB-PROJ's two-file fence. The decision each needs is the same one
LIB-PROJ made: for `CheckEvidence`'s five, whether one flattening
record earns its place or five exhaustive matches are cheaper; for
`RefusedRef`'s two, simply to match the file's own third accessor.

## Closed

LIB-WILDCARDS (`work/lib/LIB-WILDCARDS.md`). Both rows are exhaustive:
`CheckEvidence`'s five accessors read one record,
`crates/pncad-py/src/check_payload.rs`, whose match over the six arms
has no wildcard; `RefusedRef`'s `width` and `kind` name all four arms
in place, as `at` above them already did. `grep -n "_ => None"` over
the two files finds nothing.

The two dispositions this item argued are unchanged and were re-read:
`crates/pncad-py/src/validation.rs:73`/`:91` keeps the extract licence
its own doc comment states, and `py/doc.rs`'s two `Node` extracts (now
`:916`/`:931`, the file having moved under them) keep theirs.

The sweep was re-run at the unit's merge base with this item's stated
blind spot closed — the named-binding and `Some`-of-default spellings
added to the pattern — and found no hit the original pattern missed.
Its hit list is in the unit's PR.

Residue: `check-evidence-shell-refusal-crosses-as-prose-only`.
