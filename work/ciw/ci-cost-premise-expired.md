---
id: ci-cost-premise-expired
kind: ruling
title: The repo is public: the billed-minutes premise under the F3 trim and the one-point draw no longer holds
status: open
opened: 2026-09-04
needs_ev: true
---


## Question

**Two ratified CI decisions were bought with billed Actions minutes, and
the repo is now public.** GitHub bills nothing for Actions on a public
repository's standard hosted runners, and `ci.yml` runs on
`ubuntu-latest` throughout (the one exception is `vars.BUILD_RUNNER`,
which defaults to `ubuntu-latest`; whether it is set to a larger runner
could not be read from here and is the one input this question is
missing). If the bill is gone, the price side of both decisions is gone
with it, and neither was decided on any other ground.

**(a) The F3 trim — a push to main carries no gates.** `ci.yml`'s own
note states the cost it accepted: *"The commit that actually lands on
main is now never itself built or tested… a semantic conflict between
two independently-green PRs surfaces at the NEXT PR's merge-ref rather
than at the merge that caused it… THE COST THAT REMAINS, stated rather
than mitigated: the conflict surfaces on an INNOCENT PR."* The pairing
that would close it — a run on main — was **declined 2026-08-22**
because it *"buys a second discovery of the same fact, at the price of a
full gate per period whether or not anything landed."* That price was
the argument. (A gate on the PUSH is also strictly better than the
scheduled run that was declined: it runs when something lands rather
than per period, so the "whether or not anything landed" half never
applies to it.)

**(b) The one-point draw.** A run gates one point of {lane} × {eps} ×
{k-lint row}, and `ci.yml` says what buys it: *"billed minutes are what
it buys."* The sampling is why a green job name can sit over a skipped
step.

**The incidence, which is the new fact.** `main` went red twice on
2026-09-04, once by each hole:

- **#1756 → #1775.** A signature change left two no-op field accesses in
  `demos/tour`; the k-lint job was green with `demos tour fmt + clippy`
  **skipped**, because the drawn `klint_row` did not carry it. That is
  (b).
- **#1725 → #1792.** `crates/viewer`'s deliberately exhaustive match on
  `MateFault` and a new `MateFault::Unleverable` variant merged 22
  minutes apart, each green against a base without the other. Nothing
  ever compiled the pair, and the red surfaced on an unrelated PR. That
  is (a), exactly as the note predicts.

Each cost a lane a diagnosis and a repair PR, and the second one blocked
every open PR in the repo until it was found. The decisions were made
when the incidence was unknown; it is now measured at two in one day.

**The question is whether the trade re-opens, and it is only Ev's**, both
because the declination is his and because the remaining costs are not
zero even when minutes are: wall-clock queue depth against the account's
concurrency limit, cache churn, and `BUILD_RUNNER` if it points at a
larger runner. Precedent for the shape rather than for the answer: #449
reversed #52/#53's "opt 2 is net-slower on CI" verdict once its premises
expired, and said so in those terms.

## Gates

`work/ciw/program.md`'s `keep_out` already reserves this: *"what a main
push re-gates is an `[ev]` ruling before any change to the F3 trim."*
This is that ruling, and nothing in `.github/workflows/ci.yml`'s F3 trim
or its sampling note may move until it lands. Raised from outside CIW
(the code-quality Track T orchestrator, while repairing the second of
the two reds); filed here because the paths and the reservation are
CIW's.
