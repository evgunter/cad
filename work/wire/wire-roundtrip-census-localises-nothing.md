---
id: wire-roundtrip-census-localises-nothing
kind: issue
title: The wire round-trip census compares whole programs, so a laundered vocabulary member reads as two corpus-sized Debug dumps
status: closed
opened: 2026-09-12
branch: wire/census-localise
pr: 2501
closed: 2026-09-13
---


## Finding

`every_document_verb_survives_the_wire`
(`crates/editor-core/tests/switch_program_vocabulary.rs`, ~:532) is the
persistence clause of the step/mode/target census: it serializes
`corpus()`, deserializes it, and asserts bit-identity with
`assert_eq!(before, after)` and no message. That assertion is correct
and it FIRES — measured, not read (S195's mutant, PR below): a seventh
arc mode whose `WireArcData::from_spec` arm maps it onto a neighbouring
wire variant round-trips to the neighbour, and this clause is the only
one of the five that goes red.

What it does not do is say WHERE. The failure prints two `Debug`
renderings of the whole corpus — one `ProfileProgram` of three loops and
forty-six chain steps, several hundred `Expr { dim: …, kind: Literal(…) }`
records each — and the reader diffs them by eye. The laundering is one
step in one loop.

This is the same shape as the count clause in
`every_enumerated_slot_addresses_a_distinct_expression`, which S195's PR
fixed by re-running the comparison one chain step at a time on the
failure path (`steps_whose_slot_count_disagrees`). The same move applies
here: zip `before.loops` with `after.loops`, and inside a `Chain` zip the
steps, so the message names the step index and the variant that changed
rather than handing over the corpus. The helper is already in the file.

Not done in S195's PR because it is a different failure from the one
that row is about — S195's scenario is a mode whose slots address
nothing, not a mode laundered on the wire — and growing the diff to
reach it is the thing that unit was told not to do.

## Fence

`crates/editor-core/tests/switch_program_vocabulary.rs` only. Test-side;
no kernel or document code moves.

## Closed 2026-09-13 (PR 2501)

`wire_differences` zips the loops and, inside a `Chain`, the steps,
and the clause is `assert!` over `==` rather than `assert_eq!` —
`assert_eq!` renders both operands, which IS the two corpus dumps this
row is about, so the comparator had to change with the message.
Measured with the wire laundering `Sweep` into `ArcLen` on the way
out:

    the corpus did not survive serialization. Changed: [
        "loop 0 chain step 15: ArcTo(Sweep) went over the wire and came back as ArcTo(ArcLen)",
        ... four more, including
        "loop 0 chain step 38: ArcFilletArc(Sweep, Radius) went over the wire and came back as ArcFilletArc(ArcLen, Radius)",
    ]

The helper the row expected to exist did not fit: the count clause's
localiser answers a different question (slots versus expressions of
ONE program), so the shared thing turned out to be the LABEL rather
than the walk — `step_label`, which both now use.

The clause also gained the non-emptiness assertion its equality needs:
a round trip over an empty corpus is bit-identical whatever the wire
does.


## Closed 2026-09-13 (PR 2501)

The clause fired correctly and printed two whole-corpus `Debug`
renderings — a `ProfileProgram` of three loops and forty-six chain steps
— for a laundering that is one step in one loop. It now zips the loops
and the steps and names both:

> `loop 0 chain step 15: ArcTo(Sweep) went over the wire and came back
> as ArcTo(ArcLen)`

and the report is evaluated **inside** the `assert!`'s format arguments,
so the doc's claim that it runs only on the failure path is now true of
the code as well — it was not when the delta round read it.
