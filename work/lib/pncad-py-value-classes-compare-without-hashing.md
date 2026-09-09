---
id: pncad-py-value-classes-compare-without-hashing
kind: issue
title: pncad-py: nine value classes compare without hashing, so a set of findings still raises
status: closed
opened: 2026-09-08
refs: [pncad-py-comparable-enums-do-not-hash, LIB-HASH, LIB-HASH-2]
closed: 2026-09-09
---


Measured by LIB-HASH, which fixed the enum-mirror half of the same
shape (`pncad-py-comparable-enums-do-not-hash`) and is fenced out of
this half: value classes with fields keep whatever hashability they
have, and a lane that fixes the tags does not get to decide the
policy for the values.

## What happens

Same rule, same symptom, one rung up. A class that defines `__eq__`
and no `__hash__` is unhashable, so `{report.findings[0]}` and
`seen: dict[CheckFinding, int]` both raise `TypeError` even though
`==` on two of them works.

## The hit list

Every class the compiled module exposes with `cls.__hash__ is None`,
after LIB-HASH landed, and its `__eq__`:

| class | `__eq__` | disposition |
| --- | --- | --- |
| `Expr` | `crates/pncad-py/src/py/expr.rs:152` | **by design**, stated on the stub (`crates/pncad-py/pncad.pyi:2221`): equality is an IEEE comparison of the literals inside, so `0.0` and `-0.0` are equal trees whose bits are not |
| `MeasureExpr` | `crates/pncad-py/src/py/measure.rs:444` | **by design**, `Expr`'s reason (`crates/pncad-py/pncad.pyi:1770`) |
| `MateFrame` | `crates/pncad-py/src/py/mate.rs:137` | **by design**, the mirror rule (reason on the roster) |
| `MatePrimitive` | `crates/pncad-py/src/py/mate.rs:264` | **by design**, the mirror rule (reason on the roster) |
| `Alignment` | `crates/pncad-py/src/py/mate.rs:373` | **by design**, the mirror rule (reason on the roster) |
| `AnalysisPolicy` | `crates/pncad-py/src/py/analysis.rs:529` | **by design**, the mirror rule (reason on the roster) |
| `FlushFinding` | `crates/pncad-py/src/py/flush.rs:197` | **by design**, the mirror rule (reason on the roster) |
| `ChecksConfig` | `crates/pncad-py/src/py/checks.rs:275` | **by design**, the mirror rule (reason on the roster) |
| `CheckEvidence` | `crates/pncad-py/src/py/checks.rs:359` | **by design**, the mirror rule (reason on the roster) |
| `CheckFinding` | `crates/pncad-py/src/py/checks.rs:411` | **by design**, the mirror rule (reason on the roster) |
| `ChecksReport` | `crates/pncad-py/src/py/checks.rs:460` | **by design**, the mirror rule (reason on the roster) |

Nine undecided. Every one of them says nowhere why it does not hash,
which is the same never-considered shape the mirrors had.

Swept by reading `__hash__` off every class in `vars(pncad)` —
CPython sets it to `None` on a type that defines `__eq__` and no
`__hash__`, so the sweep needs no instance and has no blind spot on
this module's classes. **What it cannot see** is a class that hashes
INCONSISTENTLY with its comparison; that is a different defect and
this row does not claim to have looked for it.

## The decision this needs

Not one answer for nine classes. Three shapes are in there:

- **Small value records** (`MateFrame`, `MatePrimitive`, `Alignment`,
  `AnalysisPolicy`) read like keys — "which alignments did this
  assembly use" is a set.
- **Findings** (`FlushFinding`, `CheckFinding`, `CheckEvidence`) read
  like keys too: deduplicating findings across runs is the natural
  thing to want.
- **Reports and configs** (`ChecksReport`, `ChecksConfig`) hold
  collections, and a hashable aggregate over a list is a decision
  with a cost, not a gimme.

Each also has to answer the float question `Expr` answered: if any
field bottoms out in an `f64` the hash has to fold `-0.0`, exactly as
`DocParam` and `WrittenLength` already do
(`crates/pncad-py/src/py/doc.rs:2758`).

## The guard that is already in place

`crates/pncad-py/tests/test_hashability.py`'s `UNHASHABLE` roster
names all eleven with a reason, and its
`TestNothingComparesWithoutHashing` fails on a TWELFTH — so this row
cannot grow silently while it waits, and closing an entry means
deleting its roster line (the roster's staleness test refuses an entry
for a class that hashes now).

## Closed (LIB-HASH-2)

Answered by the mirror rule Ev ruled as (B') on `[ev]` PR #2233 — **a
Python value class mirrors its Rust type's derives**, and the kernel
omits `Hash` on values in favour of the funnel. All nine Rust types
derive `PartialEq` and no `Hash` (`ChecksConfig` adds `Eq`), so all
nine compare and do not hash, and each of the nine `FILED` lines in
`crates/pncad-py/tests/test_hashability.py`'s `UNHASHABLE` roster is
now a stated reason naming that type and its derives — including why
the mirror answers the three readings in "The decision this needs"
(the small records and the findings read like keys; the reports and
configs hold collections). No class's dunders changed.

**"The decision this needs" is answered, not deferred**: the ruling is
one answer for nine classes because the rule is about the Rust side,
and the float question this row raises does not arise — a class that
does not hash has no `-0.0` to fold.

The two-way walk that closed this row measured the OPPOSITE direction
too, over all 163 classes the module exposes, and found three shapes
where a Python class hashes over a Rust type that derives no `Hash`.
Those are residue and have their own files:
`the-tag-mirrors-hash-over-kernel-enums-that-derive-no-hash`,
`the-value-records-hash-by-hand-over-kernel-types-that-derive-no-hash`,
`sketchplane-compares-and-hashes-over-a-rust-type-that-derives-neither`.
Nothing refuses to hash over a `Hash`-deriving type: that cell is
empty.
