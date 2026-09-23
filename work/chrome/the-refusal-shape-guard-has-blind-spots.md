---
id: the-refusal-shape-guard-has-blind-spots
kind: issue
title: chrome: the refusal shape guard (test_utils::refusal and its callers) has blind spots that stay green
status: open
opened: 2026-09-23
---


## Finding

The shape guard for refusals the viewer draws (`crates/test-utils/src/refusal.rs`,
called by `crates/editor-core/tests/refusal_concision_chains.rs` and
`crates/viewer/tests/refusal_concision_edits.rs`) passes over several
shapes it exists to catch. Each blind spot stays green:

1. **Where a prefix may start.** `stage_prefixes` opens a clause only at
   the text's start or after `": "`, `"— "`, `"; "` or `"("`
   (`refusal.rs:36`). A prefix after `". "` or `", "` is missed.
2. **How long a prefix may be.** A clause counts only at one or two
   tokens (`refusal.rs:44`), so a prefix of three or more words is
   missed (`declared-REST union zip:`, `curved-section invariant at
   face …:`).
3. **Capitals.** A token must start lowercase (`refusal.rs:47`), so a
   capitalised prefix is missed (`A/B lockstep invariant violated:`).
4. **Allowed labels are global.** `ALLOWED_LABELS`
   (`refusal_concision_chains.rs:92`) admits `check separation` and the
   rest on every row, not only on the checks-window rows that use them.
5. **Exemptions are never required to fire.** `FILED`, `FILED_DECLARE`,
   `FILED_DEBUG` and `KERNEL_KEYED` (`:104`, `:126`, `:130`, `:39`)
   are never checked for use. When the owner's fix lands, the stale
   entry stays and the test stays green; it goes red only when someone
   reads the list.
6. **The blend-detail reader.** It has no count floor: it asserts only
   that each helper is read once (`:2830`). It also reads only the four
   helper calls (`:2908`), so a direct
   `BlendError::UnsupportedChain { .. }` construction outside
   `surgery.rs`'s helpers would not be rendered.
7. **BodyNotIntact rows are keyed by prefix.** They are admitted by
   `starts_with("Blend/BodyNotIntact@")` (`:2862`), not by exact id.
8. **Recourses without a marker are not counted.** `recourse_markers`
   counts only `Recourse:` and `There is no way through`
   (`refusal.rs:87-88`). A second recourse phrased without them is
   invisible, for example:
   - "blend in SEQUENTIAL calls";
   - `Indeterminate`'s "— a near-coincidence; declare …" tail;
   - `StructureRefusal`'s "narrow the parameter box and try again".

   So "one recourse per message" is enforced only between the two
   markers.

Found by the CHROME concision delta review (PR #3108).

## Also found while fixing it

`geom::EllipseInvalid` (`crates/geom/src/curves.rs`, in open PR #2861)
opens every arm with `ellipse construction:`. Its `CircularAxes` and
`Escalated` arms offer `COINCIDENCE_RECOURSE` ("declare …"). Under a
split (`SplitJoinError::Section`), a plane×cylinder section at a
near-circular tilt reaches it, and a split takes no declaration. The
chain test admits exactly this on `Split/Join/Section(Carrier)` and
`Boolean/Join/Section(Carrier)` (`FILED`, `FILED_DECLARE`).
