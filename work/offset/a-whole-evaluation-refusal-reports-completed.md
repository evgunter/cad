---
id: a-whole-evaluation-refusal-reports-completed
kind: issue
title: An evaluation that refuses every node up front reports EvalOutcome::Completed
status: open
opened: 2026-10-26
---


Filed from PR 5720's review (finding S7), by OFFSET.

## Finding

`refuse_every_node` (`crates/editor-core/src/eval/mod.rs:3830`) is the
door a run takes when it refuses before building anything: a tolerance
conflict (`refuse_tolerance_conflict`), a parameter box its scalar
cannot carry (`refuse_param_box`), or a seed it cannot carry
(`refuse_seed`). It fails every node with that one refusal, and then:

- records `outcome: EvalOutcome::Completed` (`:3866`), whose doc says
  "Every scheduled node has a result" (`:680`). That is true, but every
  reader that checks `Completed` to mean "this run built" is wrong
  about it;
- records `env: doc.var_env::<T>()`, the nominal environment, and not
  the box or the seed it was asked to bind;
- records `parts: parts::Carried::none()`.

So the run's environment looks like a nominal build's. Before PR 5720,
the E4/E5 report bound a seeded `f64` record as equal to its passes,
and then reported a `MeasuredValueDrift`. PR 5720 closed that one
reader: `Evaluation::refused_whole()` (a new crate-private
`whole_refusal` field), and the binding refuses
`BindingViolation::NotABuild`.

## The question

Should a whole refusal be its own `EvalOutcome` arm, for example
`Refused` beside `Completed` and `Canceled`, so that no reader can
mistake it for a build? `EvalOutcome` is public and matched across the
workspace (pncad-py's census lists it), so this is a vocabulary change
beyond the binding unit. The other readers of `Completed` have not been
audited for this mistake.
