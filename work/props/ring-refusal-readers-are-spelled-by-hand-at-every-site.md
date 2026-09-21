---
id: ring-refusal-readers-are-spelled-by-hand-at-every-site
kind: issue
title: "The mignitude and the refuse-then-read reader are spelled by hand at every site: twelve copies of two three-liners"
status: open
opened: 2026-09-21
priority: P3
cost: D
---

## What

RING-2 (PR #3032) made the C9 ring's refusal a decoration, so a
refused bracket carries ordinary endpoints and every one-sided read
has to ask `is_poison()` by name. The rewrite is correct and the
census (`crates/geom-core/tests/ring_endpoint_census.rs`) holds it —
but it landed as **twelve hand-written copies of two three-line
bodies**, one per site, and both reviewers found the same thing
independently.

**The mignitude — `is_poison → 0`, then `lo > 0 ? lo : hi < 0 ? −hi : 0`
— byte-identical at three sites and partial at a fourth:**

| site | name |
| --- | --- |
| `crates/geom-brep/src/offset_meters.rs` | `mig` |
| `crates/geom-brep/src/ssi/certify.rs` | `zero_free_lower_bound` |
| `crates/geom-brep/src/props/quad.rs` | `norm_lo`'s `comp` closure |
| `crates/geom-brep/src/ssi/exhaust.rs` | `excludes_zero` — the same comparison pair, answering `bool` |

The first two declare the duplication at both sites ("Kept separate
rather than shared… the shared body is four comparisons"); the third
and fourth declare neither.

**The refuse-then-read reader — `is_poison → NaN`, else the endpoint —
eight spellings, seven of them anonymous:**

| site | name |
| --- | --- |
| `crates/geom-brep/src/props/quad.rs` | `lo_or_refuse`, `hi_or_refuse` — **named**, and the candidate home |
| `crates/geom-core/src/spline/compose/tensor.rs` | the channel closure |
| `crates/geom-brep/src/ssi/enclose.rs` | `Box3::center`'s `mid` |
| `crates/mesh/src/chords.rs` | the second-difference hull, and the pcurve speed fold |
| `crates/mesh/src/nurbs_cert.rs` | `cell_component` |
| `crates/geom-brep/src/offset_meters.rs` | `norm_sup` |

## Why it is a row and not a fix

The natural home is `RingInterval` itself — `lo_or_refuse` /
`hi_or_refuse` as inherent readers beside `lo`/`hi`, and one
`mignitude` beside `mag` — which is `crates/geom-core/src/ring_interval.rs`,
inside RING-2's fence. What is NOT inside it is the shape of the
consumer crates: folding twelve sites means editing `geom-brep`'s
props, ssi and offset modules, `mesh`'s chords and cert modules and
`geom-core`'s compose module in one pass, each of which is another
program's ground, for a change that moves no number. RING-2 declined
it deliberately and filed this instead.

Two things to weigh when it is taken:

* **The census cannot see which read a guard covers** (its blind spot
  2: a refusal anywhere in the enclosing function counts). Named
  readers would make the guard and the read one token, which is
  strictly more than the census can check today.
* **RING-3 dissolves the newtype into `Interval`**, so whatever is
  added to `RingInterval` has to survive that. Doing this fold as part
  of RING-3, at the type that remains, may be cheaper than doing it
  twice.

## Disposition

PROPS': `crates/geom-core/src/` and `quad.rs` are this program's, and
the candidate home is in both. The sites in `ssi/*` (SSI), `mesh/*`
(CHORD, TESS) and `offset_meters.rs` (ENCL, OFFSET, SHELL) are named
above rather than filed separately, because one fold is one change and
twelve rows would be twelve merge conflicts. Filed by RING-2 (SCALAR),
which minted the copies and says so.
