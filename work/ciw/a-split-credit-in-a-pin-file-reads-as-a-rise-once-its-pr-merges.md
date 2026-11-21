---
id: a-split-credit-in-a-pin-file-reads-as-a-rise-once-its-pr-merges
kind: issue
title: A pin file's '# from OLD_PATH' split credit passes on the PR that adds it and reds every later PR once it merges
status: open
opened: 2026-11-21
priority: P3
cost: E
---


Found by ONEROUND (2026-11-21): PR 7143 split `geom-brep`'s curvature
arms out of `implicit.rs` into `curvature_arm.rs` and, as
`scripts/gates/cover-kind-match.sh`'s header asks of a split, ended the
new file's pin line `# from crates/geom-brep/src/implicit.rs`. The gate
sums a split's lines with its origin's and compares the sum with the
BASE's pin for the origin. On the PR that is right (the base's
`implicit.rs` pin was 7, the sum 3). After the merge the base is main,
where `implicit.rs` is 2 and the new file has its own line, so the same
sum, 3, reads as a rise of `implicit.rs` on every PR whatever it
touches. Main's lint was red from that merge until PR 7238 took the
suffix off.

So a `# from` line is correct for exactly one PR and wrong from the
moment that PR lands, and nothing on that PR can see it: its own gate
run compares against the pre-merge base. The author would have to
remember to remove the suffix in a follow-up, which is a rule kept by
hand.

Shapes a fix could take (not chosen here): the gate honours a `# from`
credit only where the BASE has no line for the new path (the split is
not on the base yet), so the suffix is inert after the merge and can be
swept at leisure; or the gate drops the suffix mechanically. Whichever,
the gate's selftest wants a case for "split, then judged against a base
that already has the split".

P3 by `work/README.md` Priority (tooling that prevents a red main); E.
