# Design fork: a drive builds and finishes a body again in every leaf

Item: `work/boxfinish/a-drive-finishes-a-body-again-in-every-leaf-whose-box-does-not-move-it.md`. Protocol `ddd201f411`. Orchestrator: BOXFINISH.

## Draws (2026-11-22 05:24 UTC)

| Byte | Value | Meaning |
|---|---|---|
| arm | 217 | 86 or above: two designers |
| labels | 140 | two designers; labels byte 140: even: Opus is A, Fable is B |

## Sessions

Dispatched concurrently 2026-11-22 05:25 UTC, the same prompt to both (the problem, what was measured and what was not, where to look; no options). A framing exposure: the prompt tells both that the owner cut design-doc additions twice this week and to keep any proposed DESIGN text short.

| Label | Model (configured) | Session |
|---|---|---|
| A | Opus 5.5 (`claude-opus-5-5`) | session_013QoYWVSMHPHUfpXd5T7sGr |
| B | Fable 5.2 (`claude-fable-5-2`) | session_013QoYWWk7f5tL6S39THzy39 |

## First reports (recorded as delivered, before any reconciliation)

- **A, 05:30 UTC.** Answer: no store of bodies in the drive; the evaluator's existing node memo (`evaluate`'s `prior`, which the drive passes as `None`) becomes one store per drive, shared by the leaves, and the drive bisects only along variables the refusing node reads. The unit is a document node's result (build, finish and refusal as one value), identified by the node's content key, which at a box scalar includes the intervals of the variables the node reads. One line of argument: a node's result over a leaf depends only on the box's projection onto the variables it reads, so two leaves with the same projection get the same result, and that is DESIGN's ratified content-keyed cache transfer ("the key is the correctness proof"). But at the symbolic scalar the key is not yet a proof, and three defects have to be fixed first: `Sym` feeds only its value's bits to the content key; an opaque value's id is a per-replay counter; the expression table, the registry and the counts are per session. A refusal no bisection can change is recognised by the split rule: split the widest variable in the first undecided node's support, and refuse at once, naming the node, when none can be split. Confidence: likely. Rejected the framing: in part (the holes' refusal is not the same in every leaf: 254 distinct hole bodies; what repeats is the kind, which is the doors-read row's; the 1.6 s a0 row gains nothing from this, it runs one leaf). Proposed changing ratified text: yes, E6's split rule and one sentence of its leaf protocol; no DESIGN text. Weighed and rejected or kept as fallbacks: the parent leaf's evaluation as `prior` for its two children (misses cousins); the support-aware split alone; hoisting unmoved nodes to the root build (a second path).
- **B, 05:31 UTC.** Answer: build no new cache; the drive uses the evaluator's existing per-node memo, each leaf taking its PARENT box's replay as its prior (no shared mutable map, so the result cannot depend on the schedule); three things that stop that at the symbolic scalar are fixed (the expression table belongs to the drive; an opaque unknown is keyed by its minting node and ordinal; what a node writes to the session, its registrations, counts and caps, is part of its value and per node), the content key at the symbolic scalar hashes both channels, a refusal is served like a value, and a box is split only along axes the undecided node reads, with a refusal of its own naming the node when none can help. One line of argument: the issue is evidence that a symbolic value is not a self-contained value (its meaning lives in ambient per-replay state); a node's result is a function of its content key, which is ratified (`[determinism]`, `[scalars]` "no parallel path"), and the repeated finish and the repeated build are one symptom. Confidence: likely as a whole, sure on the unit and on the opaque fix. Rejected the framing: in part (the question "what may a finish be keyed by" is one level down from the answer; the measured tour certifies nothing, so its 22 s / 50 s split is of an all-refusing drive; "same nodes or rebuilt" is answerable: same ids, tables rebuilt, opaque the exception). Proposed changing ratified text: yes, E6's split rule and refusal vocabulary; no DESIGN text. Weighed and rejected: a finish cache in `topo` keyed by body; hoisting unmoved nodes; a drive-wide map from key to result (kept as an easy later addition if measured); either half alone.

**Agreement on first reports:** yes on the final state's shape: no new cache; the node and its content key as the unit; the evaluator's memo; the same defects at the symbolic scalar fixed first (the key hashes the expression; opaque ids; the table per drive with a node's registrations and counts travelling with its value); a refusal served like a value; the support-aware split and a refusal naming the node; E6's text and no DESIGN text. One difference: where the memo lives across leaves: A, one store per drive shared by all leaves (catches cousins), with the parent-as-prior as a fallback; B, the parent's replay as the only prior (no shared state, schedule-independent), with a drive-wide map as a later addition if measured. A round follows on that.
