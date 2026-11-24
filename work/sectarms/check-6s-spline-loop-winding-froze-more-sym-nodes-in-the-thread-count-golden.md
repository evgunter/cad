---
id: check-6s-spline-loop-winding-froze-more-sym-nodes-in-the-thread-count-golden
kind: issue
title: Since PR 7028 the thread-count golden's two Sym validate_geometric lines freeze 16 nodes (were 14 and 12) at every eps; pinned until the move is accounted for
status: open
opened: 2026-11-17
priority: P3
cost: E
---


Filed by GREEN from nightly 44023697569 (11-17, main `cdee7461`), where
`sweep::all mass_props_are_thread_count_invariant`'s one-thread and
four-thread rows were red at every ε. Bisected by a GREEN lane.

**What moved.** Two lines of `crates/sweep/tests/thread-count-digest/eps-*.txt`,
the same two at every ε, and only their `frozen` count:

- `sym_arc_loft validate_geometric`: `frozen` 14 → 16;
- `sym_thin_strip validate_geometric`: `frozen` 12 → 16.

`decisions`, `sym`/`num`, `chk`, `disp=0` and `shapes` are unchanged,
and the one-thread row reads the same as four threads, so the move is
not width-dependent.

**First bad merge:** `bdb7f6d73`, PR 7028 (`sectarms/planar-spline-winding`).
Its first parent `e5f67cddd` is a `[skip ci]` render over `4a1d55ece`, which
reads 14 / 12. Two runs at `bdb7f6d73` give byte-identical messages.

    git checkout bdb7f6d73 && CAD_TOLERANCE_EPS=1e-6 cargo nextest run --profile ci --ignore-default-filter -p sweep -E 'test(the_walk_matches_the_serial_golden_at_)'

**The likely account, not instrumented.** PR 7028 widened check 6
(`validate.rs`'s `tier3_local_checks_marked`) from loops of
Line/Circle/Ellipse/Spiric carriers to loops of every carrier, and the
planar winding sum to read spline edges by their own term. Both `Sym`
bodies are lofts of rational arc sections, so their planar caps carry
rational spline edges that check 6 skipped before and now winds. That
fits `validate_geometric`'s plain walks freezing a few more nodes with
the decision count unchanged. The PR is about exactly that change, but
it does not name this digest: its sweep subset was cut off by a box
restart and not re-run.

**Pinned.** GREEN set both lines to 16 at every ε (the digest's
`expected` doc lists them as pinned, unexplained).

## Done when

Someone confirms which nodes the new spline-loop winding freezes (+2 on
the arc loft, +4 on the thin strip) and that freezing them is right.
Then the lines are re-cut as explained, their mention leaves `expected`'s
pinned list, and GREEN's
`the-thread-count-goldens-sym-validate-lines-and-arc-lofts-volume-are-pinned-unexplained`
loses this move.

## The golden is gone; the question stands (CIW, 2026-11-23)

PR 7501 deletes the thread-count golden that pinned `frozen` 16. This row's "Done when" — confirm which nodes freeze — was never measured, so the row stays open. Nothing pins these freeze counts now. The suite's serial comparison and its dispute-coverage guard (a floor on the share of contradiction checks that saw a whole form) are what remain.

- 2026-11-23 — accounted by BOXFINISH's PR 7394 (a definite numeric sign asks the symbolic tier nothing): the two lines read `frozen` 14 and 12 again at every ε, and the +2/+4 were freezes of the contradiction check that walked every definite margin's forms at `Sym<f64>` (it ran in every build). Decisions, `sym` and `num` do not move. (BOXFINISH)
