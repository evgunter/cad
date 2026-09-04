---
id: facade-guards-defer-to-rustdoc-json
kind: issue
title: Three facade guards defer to a rustdoc-JSON check that is not scheduled
status: open
opened: 2026-08-20
github: 696
refs: [689]
---

## From GitHub issue 696

Opened 2026-08-20; 0 comments.

Three guards in `crates/pncad/tests/all.rs` are each a source-text fallback for the same unbuilt mechanism, and each says so in its own docs without anyone owning the follow-up:

- `no_arena_key_is_nameable_through_the_facade_document_surface` — "The intended enforcement was a rustdoc-JSON scan of `pncad`'s public API. This toolchain is stable-only and `--output-format json` is nightly-gated… So this is the FALLBACK."
- `no_raw_loop_minting_door_is_nameable_through_the_facade` — same posture, one file wider.
- `every_document_layer_root_export_is_carried_or_listed` — "A rustdoc-JSON check would close [the root-only blind spot] and is nightly-gated."

Each is honest on its own; three deep with no issue number is a deferral nobody is scheduled to discharge, which is the shape the SMELL-SCAN postmortems keep finding.

**What a rustdoc-JSON pass would buy, concretely:**

1. **Aliases and re-spellings.** The LB13 guard's own stated weakness: a key type re-exported under an alias, or reachable as an associated type or a public field of an allowed type, is invisible to a `pub use` text scan.
2. **Below-root reachability.** The completeness guard reads `editor-core`'s *root*. A public name reachable only by module path and never lifted to the root is invisible to it — the exact structural hole the original closure audit's second pass found (`topo::boolean::ContainError`, `geom_curves::EllipseInvalid`).
3. **Direct `pub` items.** The completeness guard reads `pub use` statements only; a `pub struct` written directly in `editor-core/src/lib.rs` would not be seen. That file currently has zero direct `pub` items, so the blind spot is held shut by a coincidence rather than by a rule.

**What it costs:** `--output-format json` is nightly-gated, so this is a CI change — a second toolchain in the workflow, and a decision about whether the JSON format's instability is acceptable for a gate. That is why three units in a row declined it, and it is a fair decline; what is missing is a place for the decision to live.

**Disposition wanted:** either schedule the CI work, or rule that the three text scans are the permanent answer so their docs can stop pointing at a mechanism nobody intends to build.

Filed from the S20/S21 fix pass (PR #689).

## Home

LIB: all three guards live in `crates/pncad/tests/all.rs` and police the `pncad` facade's public surface — the program's territory and the subject of its curation charter.

## The cost half of this question got cheaper (2026-09-04, CIW)

Unchanged: all three guards still defer (`crates/pncad/tests/all.rs`,
the LB13 pair and the completeness guard), and the disposition wanted is
still a ruling — schedule the rustdoc-JSON pass, or declare the three
text scans permanent and rewrite their docs.

What moved is the stated cost. This issue says the decline is fair
because a rustdoc-JSON pass means *"a second toolchain in the workflow,
and a decision about whether the JSON format's instability is
acceptable for a gate"*. The first of those two is now most of a
non-issue:

- the repository went public on 2026-09-03, so standard-runner minutes
  are free and a second toolchain download costs wall clock on one job
  rather than billed minutes on every PR;
- `.github/workflows/nightly.yml` now exists as a real home for exactly
  this shape of check — ungated, once a day, not on anyone's critical
  path — and TCOST-C2/C3 have just moved two comparable passes into it.
  A nightly-only rustdoc-JSON scan does not touch the per-PR gate at
  all.

The **format instability** half is untouched and is still the whole of
the real question: a gate keyed on a nightly-gated, explicitly unstable
JSON schema breaks on a toolchain bump, and the repository pins its
toolchain (`rust-toolchain.toml`), so it would need a second pin with
its own bump discipline. That is the thing for Ev to rule on, and it
should be put to Ev with the cheap-half correction above rather than
with the original framing.

Ordering: CIW schedules this after
`work/ciw/f3-recosting-on-a-public-repo`, whose measurement establishes
what a nightly-seated pass actually costs in wall clock, so the ruling
is asked with a number in it.
