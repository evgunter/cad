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
