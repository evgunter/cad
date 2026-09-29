---
id: viewer-readme-counts-one-tone-function-outside-frame
kind: issue
title: crates/viewer/README.md says RowStatus::tone is the one function outside frame that decides a tone; there are four
status: open
opened: 2026-09-25
priority: P4
cost: E
---

`crates/viewer/README.md:940-944` (*The badges.*) says the rule that a
poisoned row stays `Advisory` "is STATED by `tree::RowStatus::tone`, the
one function outside `frame` that decides a tone". Since PR #3230 there
are four: `tree::RowStatus::tone`, `session::Standing::tone`
(`session/select.rs`), `parts::PartChooser::tone` (`parts.rs`) and
`session::FaceFrameFault::tone` (`session/refuse.rs`), and the rule
#3230 applied says more will follow wherever a caller would otherwise
pick a tone per arm (`work/vnews/a-tree-rows-message-line-picks-its-
affordance-by-hand`, *General rule*).

Found by #3230's review (S5); not fixed there, because the README is
VDOC's carve-out in that lane's `keep_out`. The fix drops the count
rather than updating it: the sentence's point is that `pane::features`
reads the tone off the value, which stays true, and a count of the
values that carry one is the kind of hand-kept list this crate keeps
retiring (`app::toned`'s doc dropped its caller list in the same PR).
