---
id: s90-tightening-cost-measured
kind: issue
title: The S90 fillet tightening costs E4's door: measured, and DL5 already discharged the obligation
status: open
opened: 2026-09-03
track: M
refs: [S90-impl, H5, 886]
---

## What

`S90-impl` still reads *"what stays open is the tightening itself, at
these three doors"*, and carries #883 as its parked prototype. This item
records a **re-derivation of that tightening against `main` at
`8433129a`** — applied, compiled, and followed hop by hop to its fixed
point — and one consequence the row does not yet carry: **the tightening
cannot terminate anywhere short of `EvalScalar`, and at `EvalScalar` it
deletes E4's door and M10's dual corpus.**

It also records a **tension between two of Ev's own rulings** at this
seam, which is a doc question rather than an implementation one and is
flagged for Ev rather than resolved here.

## The measurement

Method: `Bounds` → `CertifiedBounds` at every generic signature in
`crates/sweep/src/blend/{battery,build,surgery}.rs` (35 signatures —
`battery.rs` 14, `build.rs` 6, `surgery.rs` 15; `arms.rs`, the geometry
builder, is `T: Real` and has none), then `cargo check --workspace
--all-targets`, then the same at each site the compiler named, until it
stopped naming new ones. Every count below is a compile, not a grep.

| Hop | Site | Diagnostics |
|---|---|---|
| 0 | `sweep::blend::{battery,build,surgery}`, 35 signatures | **0** — `sweep` lib, tests, examples and doctests all compile clean |
| 1 | `verbs::Verb::<T>::run` (`crates/verbs/src/run.rs:151,154`) | 2 |
| 2 | `editor_core::eval::wire::wire_blend` (`wire.rs:1131`) | 1 |
| 3 | `editor_core::eval::wire::run_op` (`wire.rs:168,180` — the `Fillet` and `Chamfer` arms) | 2 |
| 4 | `editor_core::eval::wire::wire_instantiate_part` (`wire.rs:276`) | 1 |
| 5 | `EvalScalar` (`eval/mod.rs:1238`), hence `evaluate` | **39, all in tests, none in `src`** |

**Hop 0 is the finding's good news and it is new.** The whole blend
module — including `chamfer_edges`, which VERBS-CHAMFER wrote into these
same three files after #883 was parked — tightens without a single
in-crate error. #883's *"there is no compiling middle"* was about the
module and it is still true there; what has changed is everything
downstream, which is now a **three-hop chain of single-site errors**
rather than twelve red jobs of unmeasured shape.

**Hop 1 needs an impl-block split, not a bound.** `run` (blend) and
`run_pair` (boolean) share one `impl<T: Decide + Bounds +
PcurveFittedLane> Verb<T>` block, so tightening the block drags the
boolean door along for no reason — `run_pair` reaches
`topo::boolean_op_with`, which certifies nothing. Splitting the block
keeps the tightening on the operation it describes. Anyone re-attempting
this should split first; it is not optional and it is not visible from
the signature.

**Hop 5 is the price, and it is entirely M10's.** 39 `E0277` primary
sites, **zero in `crates/*/src`**, distributed:

| File | Sites |
|---|---|
| `crates/editor-core/tests/m10_4_seed.rs` | 12 |
| `crates/editor-core/tests/r2_m10_di_probes.rs` | 8 |
| `crates/editor-core/tests/m10_di_dual_corpus.rs` | 7 |
| `crates/editor-core/tests/r1_dual_probes.rs` | 6 |
| `crates/editor-core/tests/e4_dual_door.rs` | 2 |
| `m10_2_r1_probes.rs`, `r2_m10_2_probes.rs`, `m10_p_lift.rs`, `cert_m2r1_corpus.rs` | 1 each |

`e4_dual_door.rs:102` is `evaluate::<Dual64>` itself — the two sites
there are E4's door, asserted open. So the tightening's terminal cost is
not *"a fillet stops being differentiable"* as an abstraction: it is
`evaluate::<Dual64>` ceasing to be a well-typed function of this
library.

**And fillets really do run at a dual today.** `die_fillet`,
`die_chamfer`, `die_composed` and `die_composed_tour` are all registered
in `corpus::documents()` (`tests/corpus/mod.rs:147-198`), and
`m10_di_dual_corpus::every_document_evaluates_at_dual64_with_the_f64_value_channel`
iterates that registry asserting arm-for-arm and bit-for-bit agreement
with the `f64` run. Verified passing at `8433129a`: 6 tests run, 6
passed. This is the thing #883 priced as a hypothetical
(*"`Filleted<T>` carries a `Body<T>`, so a fillet stops being
differentiable"*) and it is now a live, asserted capability with named
tests.

## The tension, for Ev

Two rulings, eight days apart, point opposite ways at this one seam.

- **`H-R3` (2026-08-21, #867)** — *"tightening to `CertifiedBounds`
  works at least for now."* Implemented at two of three sites in #886;
  the third is this one.
- **`DL5` (2026-08-29, `docs/DUAL-DESIGN.md`, ratified in #1146)** —
  titled *"The fillet seam's lapsed justification: discharge by
  ratifying the delegation rule, not by building an empty lane"*. It
  takes the **same obligation** (`real.rs`'s *"a lane, or a written
  reason it needs none"*) and discharges it with the written reason: a
  `Bounds` read is lane-exempt when it feeds an error payload, or
  selects among constructions whose classification is
  value-channel-decided **and whose selected quantity is locally
  constant in the parameters**. It has landed —
  `crates/geom-core/src/real.rs:688-711` carries it, and
  `scripts/gates/bounds-allowlist.sh` points at it.

`DL1` also closed, permanently, the *"at least for now"* hedge that
`H-R3` and #883 both lean on (*"a Dual never certifies — permanently …
`CertifiedEnclosure` keeps no `Dual` impl"*), naming *"the fillet seam,
the lane splits"* as among the four standing hesitations it converts
into one revisitable decision.

So the seam's obligation is **discharged**, and the remaining case for
tightening is not soundness but **durability**: an audit is fourteen
reads that a future edit silently grows to fifteen, and a bound cannot
be grown past. That is a real argument — it is #883's own, and the best
sentence in its body — but it now has a measured price (E4's door) and a
ratified alternative (DL5's criterion, which is a *standing* rule rather
than a one-time enumeration, and which already caught the thing the
one-time audit missed: the locally-constant condition, `geom::projection`'s
`mid` freeze, issue 874's class).

**What is not viable** is the middle: stopping the cascade at hop 3 by
giving the evaluator's blend arm a refusing lane. That mints a fifth
lane trait exactly where `DL5` calls an empty-refusing-side lane *"the
dead-code pattern the M5 reviews punished"* and where `H-R16`'s target
for the collapse is **zero** lane traits. `S3` already records this site
as one of the two where the pattern was deliberately declined.

## Proposal

1. **Do not land the tightening at this seam**, and retire `S90-impl`'s
   *"what stays open is the tightening itself"* line — the obligation it
   tracks was discharged by `DL5`, and the row was re-read on 2026-09-02
   without that being noticed.
2. **Close #883.** Its branch is ~1100 commits behind, its module was
   renamed (`sweep/src/fillet/` → `sweep/src/blend/`) and its analytical
   content has been absorbed: the locally-constant observation in its
   `real.rs` rewrite is `DL5`'s load-bearing clause.
3. **If the durability argument is to be honoured**, honour it as a
   gate rather than a bound: `scripts/gates/bounds-allowlist.sh` already
   owns this seam and already points at the rule, and a check that the
   blend files' bracket reads stay payload-or-selection shaped buys the
   fifteenth-read protection without costing `evaluate::<Dual64>`.
   Not built here; it is a proposal, not a claim that it is easy.

Items 1 and 2 revise the standing of a ruling, so they are Ev's call,
not this item's.

## Reproduction

At `8433129a`:

```sh
for f in battery build surgery; do
  sed -i 's/Decide + Bounds/Decide + CertifiedBounds/g' crates/sweep/src/blend/$f.rs
done
# add CertifiedBounds to each file's `use geom_core::{...}`
cargo check -p sweep --all-targets     # clean
cargo check --workspace --all-targets  # 2 diagnostics, crates/verbs/src/run.rs
```

then follow the compiler: `verbs/src/run.rs:134` (splitting the impl
block first), `editor-core/src/eval/wire.rs:1116`, `:140`, `:276`,
`editor-core/src/eval/mod.rs:1238`.
