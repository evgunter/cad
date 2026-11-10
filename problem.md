# Problem: a cut whose tool is a seated ⊔, crossing the seat

You are a designer lane on evgunter/cad, a Rust B-rep CAD kernel. Read `docs/prompts/designer.md` IN FULL first: it binds you. Also read `CLAUDE.md`.

**Setup.**
- Clone with `git clone /home/user/cad <your dir>/cad`. Then, in the clone, run `git fetch https://github.com/evgunter/cad.git main && git checkout --detach FETCH_HEAD`.
- Your dir and your `CARGO_TARGET_DIR` are given in your dispatch line. Build only into that target dir.
- Wrap heavy cargo in `local-scripts/with-build-slot.sh`, with `CARGO_INCREMENTAL=0`.
- Never background builds or arm waiters, and never end your turn with background work running.
- Do not push, do not comment on GitHub, and do not edit the main checkout.
- Delete your target dir when you are done.

## The ground

- **The row.** `work/seal/a-cut-across-its-tools-seat-leaves-a-sliver-between-the-seats-two-sections.md` (P1) is the evidence that raised this question. Read it in full. Its last paragraph names two directions. Its filer wrote them before anyone weighed the question: they are hypotheses, not candidates you must choose between. The options are yours to find, and that includes questioning the row's "Done when".
- **Where it came from.** SEAL's lane on `a-vertex-piercing-two-declared-coincident-faces-reads-as-one` (PR 6613) measured it with a prototype. That row's design came from fork `seat-vertex-one-interface`. This case was that fork's "assumption 3".
- **Who governs the area.** Read the topo README clauses on records (`[topology.records]`), the seat / wall reading (`vtxfac::Wall`, `vtxfac::WallRead`) and the finish (`finish::classify_shell`, `setopfinish`'s seam bookkeeping), plus `docs/DESIGN.md`'s at-rest and contact sections. Check the provenance of any clause you lean on, as `designer.md` §2 says.

## The case

- **The operand.** `S = bolt ⊔ plate`: two pieces at rest on each other. The bolt's foot lies on the plate's top, and that seat is recorded as a face–face record. The kernel reads a seat crossed by a vertex of the other operand as one two-sided wall: that vertex mints each face's crossing, so its two sides each take one face's crossing.
- **What builds.** `S − X`, both `∩`, a bead on one piece, and the bridged unions' self-touch (`crates/topo/tests/a_vertex_through_a_declared_seat.rs`).
- **What refuses.** `X − S`, the seat as a cut's tool, refuses `BooleanError::ToolSeatCrossed`, naming the record. It is raised in two places:
  - in the vertex pass (`vtxfac::classify_vertex_on_face`), where a vertex of `A` crosses both faces of a wall of `B`;
  - at the result gate (`ops::Held::apart`), where a shell holds both faces of a carried record.

  It is pinned by `a_cut_across_its_tools_seat_refuses_typed`, and in the editor by `a_seat_as_a_cuts_tool_is_read_whole_and_refuses_where_the_cut_crosses_it` (`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`).
- **The intended bodies.** `post − seat` is empty, `pin − seat` is the pin's stub below the plate, and `bead − seat` is the bead's piece outside both. The fixtures are in the test file above.

## Measured facts (from the prototype; check them)

- **A crack appears.** The two sections cut `X` along each face apart. That leaves a zero-thickness region of `X` between them, the crack.
- **The crack's class depends on order.** Read against each face alone, it is Out of both; with the copies minted the other way round, it is In both. Shells are classed by their section faces (`finish::classify_shell`):
  - read Out, it survives in `X − S` and `∪`, zipped with the reverted seat patches into a lamina (`ResultInvalid` `LaminaWedge` for `post − seat` and `pin − seat`);
  - read In, it survives in `∩` and `S − X` instead.
- **Dropping it breaks the finish's bookkeeping.** Every completed polygon pair contributes a seam whose kept A and B faces must both survive the carve ("kept A section face missing from the carve"). A seamed operand that loses every kept component is a `JoinDesync`.
- **The crack is not always its own shell.** In `bead − seat`, the top's section runs past the bolt's foot edge, an edge of the bolt lying in the plate's top. The crack therefore opens into the bead's piece outside the bolt through one section face, and dropping it needs the top's patch divided along that edge. No reader does that: the prototype measured `EdgeInFace { gap: Crossed }`.
- **A related case.** A cut by a seat crossed by faces alone, with no vertex on it (the slab and the cap), is refused only at the result gate. A curved seat is not read there at all (`work/seal/a-cut-by-a-curved-seat-keeps-its-two-faces-held-apart-by-the-record.md`, with its lane findings). Your answer should say whether it covers these.

## What needs deciding

What should `X − S` be, and what in the kernel has to change to make it so?
- Is the crack something the construction should never mint, something the finish should drop, or something else? What is the fault behind it?
- How is a wall's patch divided where the wall turns one-sided (`bead − seat`)?
- Is the typed refusal, as it stands, an acceptable final state for any of these ops? If so, which ones, and why?
- What does your answer do to the ops that build today (`S − X`, `∩`, `∪`), and to the faces-only and curved seats?
