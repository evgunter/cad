---
id: cone-axial-window-widens-a-seamed-frustum-to-its-apex
kind: issue
title: splitting::rules::cone_axial_window widens a seamed frustum wall to its apex (its 'holds the apex' rule is each loop's azimuth window under three quarters of a turn), a second rule beside clearance's winds_nowhere
status: open
opened: 2026-10-31
priority: P2
cost: M
---

Found by FRUSTUM's review (CLEAR, PR 6114); not measured as causing a
refusal yet.

`crates/topo/src/splitting/rules.rs`, `cone_axial_window` (read by
`boolean/join.rs`'s `cone_window`): a face on a cone keeps its
boundary's axial range only where every loop's azimuth window is
decided under three quarters of a turn
(`chord_join::face_loop_azimuth_windows`). A seamed frustum wall, one
face whose loop runs a full rim, the seam, the other rim and back,
spans a whole turn, so its range is widened to the apex though the
face stops short of it. That is the over-widening FRUSTUM removed
from `clearance::window_of`, by reading here.

**Two "holds the apex" rules.** This one reads azimuth windows;
`clearance::winds_nowhere` reads the joint elements of all the face's
loops in total (identity, no reset), with the argument for the window
at `window_of`'s cone arm. The second is a core living inside one
consumer (clearance). The fix moves it to a home both read, and
`cone_axial_window` takes it in place of the azimuth rule, with a row
on a seamed frustum whose axial range stops short of the apex.
`work/shelf/cone-v-window-misses-an-apex-held-inside-the-face.md`
wants the same reading in `replace_face`.
