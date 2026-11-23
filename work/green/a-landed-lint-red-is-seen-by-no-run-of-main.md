---
id: a-landed-lint-red-is-seen-by-no-run-of-main
kind: issue
title: A lint red two green PRs compose on main is seen by no run of main, only by the next PR's
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by GREEN's `green/gate-reds` lane (PR 7599) closing
`main-red-the-payload-rung-sweep-flags-node-error-class-under-unsettled`.

**What happens.** `581b4e1090` (PR 7458) made `scripts/payload-rung-sweep.py --check`
red on main, and main showed green for about nine hours while every PR
whose test merge held it went red in `lint`'s `gates (scripts/gates)` step.
Four programs logged it (`work/boxfinish/log.md`, `work/formwords/log.md`,
`work/germ/log.md`, `work/green/log.md`) before anyone owned it, and
PR 7580 decided it.

**Why, and that it is deliberate.** `.github/workflows/ci.yml`'s `filter`
job is `if: github.event_name != 'push' …`, and every gated job needs it,
so a push to main runs only `cache-prime` and `cache-prime-lint` ("Pushes
to main run nothing else", above `cache-prime`; `work/ciw/latency-cut.md`,
"Pushes to main run only the cache primer"). Ev declined a scheduled full
run on main (2026-08-22, `docs/CI-MINUTES-2026-08.md`, "a scheduled full
run on main — DECLINED" and "The scheduled full run: still declined"):
the next PR's merge-ref tests the landed tree, and the cost left is
attribution. That ruling stands and this row does not ask to reopen it.

**The part no ruling covers.** The nightly does not run the lint job's
cheap, cargo-free checks either: `.github/workflows/nightly.yml`'s
`gate-selftests` runs `scripts/run-gates.sh --selftest`, not the gates,
and nothing in it runs `payload-rung-sweep.py --check` or
`work.py lint`. So a lint-class red composed on main is seen on no run of
main at all, and its first reader is always an innocent PR — the
attribution cost `docs/CI-MINUTES-2026-08.md` names, paid here by four
programs. Both cheap places to see it are python-only, no cargo:
a nightly step (`run-gates.sh`, `payload-rung-sweep.py --check`,
`work.py lint`), which the nightly's `attribution` job could then name a
merge for; or the same steps on the push run, which the 2026-08-22
ruling's text (a FULL run) does not reach but whose spirit may. Which,
if either, is a CIW-ground choice and possibly Ev's; nothing is changed
here.
