---
id: the-chart-window-walk-refuses-in-the-chord-joins-error
kind: issue
title: "[topo.homes] chart_window, a shared reading, refuses in the chord join's SplitJoinError"
status: open
opened: 2026-11-03
---


Found by `onedoor/carrier-and-window` (2026-11-03), which moved the walk and left its error type alone, because a move does not change refusals.

`[topo.homes]` says *"A shared reading refuses in its own error type, never in an operation's."* `topo::chart_window` (`face_azimuth_images`, `face_azimuth_window`, `face_loop_azimuth_windows`, `run_azimuth_images`, `nearest_branch`, `pole_branch`) refuses in `chord_join::SplitJoinError`. That is the chord join's vocabulary, not an operation's, so the clause's letter holds. But the walk's three non-chord readers each translate it back out:

- `boolean::solid_contain` (the sphere chart trim, at its `chart_window::face_azimuth_window` call) matches `SplitJoinError::Escalated` / `OrderEscalated` and sorts the remaining variants into corruption and class refusals;
- `splitting::rules` (`face_loop_azimuth_windows`) discards every `Err` as "no window" (`Err(_) => None`);
- `review_d18`'s `face_azimuth_window_traces` row and `sweep`'s `pole_slit_window` read it through the public `topo::SplitJoinError`.

The variants the walk raises are `Corrupt`, `Escalated`, `SectionInvariant`, `RunImageNotYet`, `RunImageIncidence`, `ApexUnlifted` and `PoleEdgesCoincide`; the last two are window-only. The question is whether the walk gets a `ChartWindowError` of its own, with `SplitJoinError` wrapping it at the chord join's island winding, which is the one reader that is the chord join. That change moves refusals, which is why it is a row and not part of the move.
