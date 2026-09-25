---
id: viewer-tests-assert-ratifications-that-are-not
kind: issue
title: Viewer test docs call something ratified that no ratified clause decides, and the README's ratif lines are unverdicted
status: open
opened: 2026-09-24
priority: P3
cost: E
---

Found by VNEWS's census,
`work/vnews/ratified-is-asserted-across-viewer-src-and-some-was-never-ratified`
(§Population 4), at merge base `f4b178189`.
`grep -rniI ratif crates/viewer/tests` gives the population. The
census verdicts every line there, and the members that are not TRUE
are these:

- **FALSE**: `crates/viewer/tests/frame_policy.rs`, the doc on the
  checks-badge row (:1162), says *"The checks badge is a BUTTON, and
  that is ratified rather than incidental"*. `checks badge` occurs 0
  times in `crates/viewer/GUI-DESIGN.md` and `docs/DESIGN.md`. The only
  backing is `crates/viewer/README.md:944-946`, and the origin is
  `bce8486e9` (agent, 2026-09-01). The src twin, `frame.rs`
  `Affordance::Opens`, is VNEWS's and stays with the census.
- **PARTIAL**: `crates/viewer/tests/story_parametric.rs` module docs
  (:11) and its story step 7 (:391) cite G4 for a NUMERIC WRITE.
  `GUI-DESIGN.md` G4 as Ev wrote it (`5267a9193`) is about a drag.
- **PARTIAL**: `crates/viewer/tests/panel_edits.rs`, an assertion
  message at :657, says *"it renders the ratified wording"*. G4
  ratifies the refusal and its affordance. Ev's parenthesised example
  sentence was illustration, and an agent dropped it (`585b3422f`); no
  wording is decided.

**Not verdicted, handed over with the rule:**
`grep -niI ratif crates/viewer/README.md`. Most of its lines use
*"ratified list"* or *"ratified kind"* as the README gate's own
vocabulary. That is a different sense of the word, and itself a
reader hazard beside a companion table where "Ratified" means Ev's.
The census's sorting test (find the clause, then the commit) applies
to the rest.
