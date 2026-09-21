---
id: render-mm-overflows-to-inf-for-a-delta-the-door-accepts
kind: issue
title: render_mm renders a legal δ above 1.7977e305 as inf, and the row over it stops at a kilometre
status: closed
opened: 2026-09-16
closed: 2026-09-16
refs: [the-scientific-arm-rounds-out-of-the-type, renders-that-multiply-a-finite-guarded-length-spell-the-product-inf]
pr: 2737
branch: view/render-mm-inf
---


(VIEW) `DisplayTolerance::new` (`crates/viewer/src/scene.rs`) accepts
**any finite δ > 0** — the predicate is `delta.is_finite() && delta > 0.0`
and there is no upper bound. `DisplayTolerance::render_mm` is
`readout::number(self.0 * 1.0e3)`, and that product overflows for every
δ above about `1.7977e305`. So a δ the door accepted renders as `inf`,
which is not a δ the door accepts and is infinitely far from the value.

Measured (`rustc -O`, the module's own predicate replicated):

| δ (m) | `self.0 * 1.0e3` | `render_mm` |
|---|---|---|
| `1.7975e305` | `1.7975000000000001e308` | exact, reads back (after the third arm) |
| `1.7976931348623156e305` | `1.7976931348623155e308` | exact, reads back |
| `1.0e306` | `inf` | `"inf"` |
| `1.0e307` | `inf` | `"inf"` |

**It is a hole in the row, not only in the code.**
`no_delta_renders_as_a_number_a_delta_cannot_be`
(`crates/viewer/tests/display_budget.rs`) asserts exactly the property
this breaks — `DisplayTolerance::new(read * 1.0e-3).is_ok()` — and it is
green, because its sweep runs `1.0e-12` to `1.0e-1` and its named ends
are the two smallest positives and a kilometre. The top of the type is
outside the population, so the row states a universal it never tests
there. A sweep that stops short of an end is that fact recorded as an
absence (this program's own rule, `work/view/plan.md`).

**What a repair has to decide** is where the conversion's overflow is
answered: in `DisplayTolerance::new`, which would make the type refuse a
δ no tessellation could use anyway and give the render a total domain;
or in `render_mm`, which would have to say something true about a δ whose
millimetre value is not an `f64`. The first is a change to a door's
accepted set and is the larger claim; the second leaves the type holding
values its own render cannot spell. Either way the row above owes an end
at the top of the type, which is the cheap half and is independent of
the choice.

Found by `the-scientific-arm-rounds-out-of-the-type`, whose item argued
the carve-out was safe partly on the ground that *"a δ that large does
not survive the millimetre conversion"*. It does not survive it, and
that is this defect rather than a reason there is none.

## Closed — in the door, because no text can repair the render

**The table re-measured, not reasoned about** (`rustc -O`, the module's
predicate and `readout::number` replicated verbatim). Every row of it
holds: `1.7975e305` and `1.7976931348623156e305` render exactly, at
twenty-two characters, and read back; `1.0e306` and `1.0e307` render
`"inf"`, and so does `f64::MAX`. The bisection adds the edge the table
did not name: the coarsest δ whose millimetre product is finite is
`1.7976931348623156e305`, and one `f64` above it the product is `inf`.

**Where the overflow is answered: in `DisplayTolerance::new`.** For a δ
past that edge the millimetre value **is not an `f64` at all**, so no
text of it reads back as one, so no text of it reads back as a δ. The
row's property is therefore unrepairable inside `render_mm`: answering
there means abandoning the universal and writing a second carve-out at
the top of the type — the exact shape
`the-scientific-arm-rounds-out-of-the-type` had just removed one level
down, and whose argument had already been measured false once. The door
is the only place the render's domain can be made total.

**What the narrowing costs, checked at every construction site rather
than argued.** The bound is `f64::MAX * 1.0e-3`, and it is not a number
anyone picked: the δ field parses millimetres and commits `mm * 1.0e-3`
over a finite `mm` (`pane::view`'s `delta_field`), so the coarsest δ a
person can name is exactly the coarsest δ whose millimetre product is
finite. The two coincide, and
`the_door_refuses_a_delta_whose_millimetre_value_is_not_one` measures
that rather than restating it. Of the construction sites — `app`'s
`INITIAL_DELTA` and `set_delta`, `scene`'s `answer` and
`fit_from_probes`, `gpu`'s test fixture, `examples/r1_gallery_probe`,
and the test helpers — only two can reach the bound at all, and both are
inside `fit_delta`: `DisplayTolerance::new(solved)`, which used to
accept a solved δ that then rendered `inf` and now refuses it, and
`requested.scaled(PROBE_FACTOR)`, which refuses a request within a
factor of eight of the coarsest. A fit that refuses leaves δ alone by
design (`ViewerApp::take_fit`), so the second is a typed decline in a
band no picture lives in rather than a new failure.

**Why not `render_mm`.** It would have to say something true about a
value that does not exist. The truthful spellings are all texts no
`f64` parse recovers — `"1.798e309"` parses to infinity, like the
`"inf"` it replaces — so the field would still hand back a number the
door refuses, and the type would still hold values its own render
cannot spell.

**The row's missing end, which was owed either way.** The sweep in
`no_delta_renders_as_a_number_a_delta_cannot_be` now runs the three
decades under the top as well as the twelve under a kilometre, names
`f64::MAX * 1.0e-3` and the band below it, and — past the bound — asks
the door's own answer: a δ it accepts owes the property, a δ it refuses
is the row next door. The character bound moved out of the property and
into `fits_and_reads_back_as_a_delta`, because from `1.7975e305` up the
millimetre value is in the band `readout` spells exactly and that
spelling is twenty-two characters.

**The receipt is a base-tree red, not a mutation.** On `origin/main` at
`bfc577bbdd` both new rows fail:
`the_door_refuses_a_delta_whose_millimetre_value_is_not_one` at the
first refusal, and `no_delta_renders_as_a_number_a_delta_cannot_be`
with *"δ inf mm renders as inf, which is not a δ this door accepts"*.
Both are green with the door's second condition in place.

**Residue, filed rather than disclosed:**
`renders-that-multiply-a-finite-guarded-length-spell-the-product-inf` —
the camera readout, `Bounds::wording` and `props::field_text`, the
three siblings this unit's sweep found and did not take.
