# MESH-1 — issue 1362: the walk.rs world-origin loop-area anchor

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **S**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 1362 is the primary specification; issue 303's merged fix
(PR 1361, `crates/mesh/src/validate.rs`'s recentred `signed_volume`)
is the established pattern for the class.

## Situation

`crates/mesh/src/walk.rs:1067` folds the `band_u` loop-area vector as
`(p − Point3::origin()).cross(q − Point3::origin())` — anchored at
the world origin, the same "translation-invariant-over-ℝ paid as
cancellation" shape issue 303 fixed. The consumer is direction-only:
the comment at `:1070–1078` explains the `atan2`/`sense_sign` chart-
frame argument that picks the azimuth branch. At large placements the
pick can read a cancellation-noise direction and mis-pick a meridian
branch.

The issue's secondary list (copy-source template sites, explicitly
low-priority but routed with this unit so the templates stop teaching
the shape): `crates/sweep/tests/revolve_common`'s `signed_volume` /
`signed_volume_lifted`, `pncad-py`'s `mesh_signed_volume`, and the
`docs/guide/meshing.md` snippet.

## Deliverables

1. **Establish the fold's actual anchor-dependence first.** For a
   closed loop the summed cross-product fold is anchor-independent in
   exact arithmetic; floating-point is where the anchor matters.
   State in the PR whether this loop is closed by construction at
   this site, because it decides what the fix claims: conditioning
   only, or value too.
2. **Re-anchor the fold at a local point** (the loop's own first
   point, or the pattern issue 303's fix used — an overflow-robust
   local centre). The direction semantics for well-conditioned inputs
   must be preserved; the comment at the site updates to state the
   invariant (why a local anchor, what the consumer reads), not the
   history.
3. **A red-first row** pinning direction honesty at a large
   placement: the same band walked near the origin and at a far
   placement must pick the same meridian branch / azimuth direction;
   demonstrate the old spelling failing it (vivid digits in the PR,
   the issue-303 style) before the fix greens it.
4. **The template sites** corrected to the local-anchor spelling
   (three sites named above; one-line edits, no behavior claims —
   they are near-origin-harmless today, per the issue).
5. **Class sweep** (discipline §5): grep the shape — origin-anchored
   cross/area folds — across `crates/mesh` at minimum; hit list with
   per-hit disposition in the PR body. State what the pattern could
   not match.

## Acceptance

- The red-first row red under the old spelling, green under the fix.
- Existing suites green. **If any committed pin, count, or render
  moves, stop and decide correctness per discipline §3** — expected:
  no movement on the committed corpus (near-origin bodies), and the
  PR says so as a checked claim, not an assumption.
- ε posture: this fold consults no tolerance; say in the PR which CI
  lane/ε the gate drew and why the unit argues lane-independence (the
  issue-1356 discipline — "consults no tolerance" is a claim to
  state, not to leave implicit).

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 1362" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the issue
  after merge.
- Scope fence: `crates/mesh/src/walk.rs` — **the `band_u` area fold
  and its comment only**. `walk.rs` is contended by later S-MESH
  units: do not touch `pole_v`, the ε reads, `closing_column`, or
  `gap_is_noise`. Plus the three template sites and the tests this
  unit adds. Nothing else — no `docs/MODEL-AB-LOG.md`, no
  `docs/S-MESH-*.md`.
- Any refusal minted or changed is classified against the D2
  addendum in the PR body (none is expected).
