FORK: intent/an-unread-body-is-no-finding
Dispatched 2026-10-30 ~05:42Z by the INTENT orchestrator.
Blinding byte: 250 (even: A = Opus, B = Fable)
Opus designer: session_01VyvYnJ1HGjffH7GAsKVEUH; Fable designer: session_01VyvYnK4cEK2o3EQNWvRZYm (identical briefs, dispatched together 05:40Z)

## First report, A (delivered 05:46Z)
- **Answer:** one certified `unread` check, Advisory (Off/Warn), default Warn.
  - It reports nodes none of whose outputs anything reads, and named variables nothing reads.
  - The subject is the node or the named variable; family members are not reported.
  - The product slot joins `Doc::reader_counts`.
  - Only sinks are reported. Assertions are readers.
  - An unread Measure: unsure, Ev's call.
- **Argument:**
  - VR7 already removes unread anonymous variables, so only outputs and named variables remain.
  - Making the node the subject keeps the trim idiom quiet; the cost is that a forgotten split half goes silent.
- **Confidence:** likely.
- Rejected the framing: partly. The brief said Separation; the real residents are Connectedness, ChartCoherence, UnprovenCoincidence and AtRest.
- Changes to ratified text: YES. It rewords DESIGN `[operations]` "The product", and the stage 3 spec row 26.

## First report, B (delivered 05:50Z)
- **Answer:** one certified check, `unread-operation`: an operation none of whose outputs anything reads.
  - The product slot is a reader; the world is excepted. The subject is the node.
  - Advisory (Off/Warn), default Warn. Sinks only.
  - Named variables: "same shape, could join later, not built".
  - Measure: included under the general rule (unsure).
- **Argument:**
  - VR7 already removes unread anonymous variables.
  - The sentence's "every unread variable" is false: unread `space` and `axis` outputs are everywhere.
  - Making the node the subject keeps the split idiom quiet.
- **Confidence:** likely.
- Rejected the framing: yes, the sentence as written is wrong.
- Changes to ratified text: YES. `[operations]` "The product", `[variables]` "Names" (the product slot is a reader), VR7, and spec row 193 / test 26.

## Agreement
- Both say: node subject, certified, Advisory, default Warn, sinks only, the product slot joins `reader_counts`, and the sentence is reworded.
- Split on one point: A also reports named variables nothing reads; B leaves them out until a need is shown.
- Measure: both are unsure.
- No reconciliation round: it is one stated sub-point, not a crossover. It goes to Ev as an [ev] PR because ratified text changes.

## Round 2 (after Ev's comment on #6075, 06:09Z: "deliberately made" vs "byproduct"). Both designers were shown Ev's comment verbatim, not each other's reports.
### A, round 2 (06:12Z)
- **Deliberate** means inserted (InsertNode) or named (VR2); byproducts are unnamed output ports. Nothing new is stored.
- **Used** means reaching an end: the product or an assertion. These are ends, not readers, and `reader_counts` is unchanged.
- **What is reported:** the last deliberate thing in an unused chain.
- **Named variables are in.** A named split half fires; an unnamed one is a byproduct and stays quiet.
- **Unchanged:** certified, Advisory, Warn. There is new DESIGN wording.
- **Moved from round 1:** the product is an end, not a reader; used means reaching an end; deliberate means inserted or named; named variables are firmly in.
### B, round 2 (06:14Z)
- **Deliberate** means inserted, named, the product slot, or an assertion; byproducts are unnamed outputs and anonymous variables.
- **Used** is one step: something reads it, or it is the product, an assertion's claim, or a measure's readout. `reader_counts` is unchanged.
- **The check, `unused`,** has two arms: an operation none of whose outputs is read or is the product, and a named variable unread and not the product. Naming a split half makes it deliberate.
- **A Measure is exempt** (its readout is its use). A sketch as a finding: unsure.
- **Unchanged:** certified, Advisory, Warn.
- **Moved from round 1:** the product is a use, not a reader; named variables are in; a named output is a new case; a Measure is exempt; the name is now `unused`.

## Round 2 agreement
- Both agree:
  - deliberate means inserted or named, and byproducts are never findings;
  - the product is a use, not a reader, and `reader_counts` is unchanged;
  - named variables, named outputs included, are in;
  - the finding sits at the tip of a dead chain;
  - certified, Advisory, Warn.
- The split: an unasserted Measure (A: fires, unsure; B: exempt, likely).
- A minor difference in definitions: A's "used" is transitive reach to an end, reporting the last deliberate thing; B's is one step, reporting the tip. On a DAG both put findings at the same tips. A's reach is the tint's query.
