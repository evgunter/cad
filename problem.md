# Problem: what type a clearance cell's enclosure is

You are a designer lane on evgunter/cad, a B-rep CAD kernel written in Rust. Read `docs/prompts/designer.md` IN FULL first; it binds you. Also read `CLAUDE.md`.

**Setup.**
- Clone with `git clone /home/user/cad <your dir>/cad`. Then, in the clone, run `git fetch https://github.com/evgunter/cad.git main && git checkout --detach FETCH_HEAD`.
- Your directory and your `CARGO_TARGET_DIR` are given in your dispatch line. Build only into that target directory.
- Wrap heavy cargo in `local-scripts/with-build-slot.sh -- <cmd>`, with `CARGO_INCREMENTAL=0`.
- Never background builds or arm waiters, and never end your turn with background work running.
- The container can restart without warning. Keep notes in your report draft as you go.
- Do not push, do not comment on GitHub, and do not edit the main checkout.
- Delete your target directory when you are done.

## The ground

- **The row.** `work/clear/clearance-cell-enclosures-are-typed-as-the-interval-scalar.md` (P0) raises the question; read it in full.
  - Ev asked it on PR 6308 (read the PR's comments with the GitHub MCP tools if you have them).
  - The row's bullets were written by an orchestrator before anyone weighed the question. Treat them as context, not as options you must choose between.
  - Ev's words are the question: "it seems wrong to be putting in an f64 as an interval? ... isn't it semantically a different type than the input type? i guess that might be a bit ugly since `Sym` isn't compatible", then "i'm not sure if `Sym` is completely incompatible or just difficult/impractical".
- **What waits on the answer.** `work/clear/the-at-rest-gate-certifies-embedding.md` (P0, parked on this row) is the unit that first brings f64 into the engine.
  - It is ruled by Ev on PR 6301, and the ruling is now in `docs/DESIGN.md` `[topology.tiers]`.
  - The ruling: a finished body's boundary is certified embedded off its recorded contacts, at every scalar with certification rights (f64, Probe, Interval, Sym), by the clearance engine's body-level injectivity question.
  - Whatever you recommend must serve that unit.
- **The engine.**
  - `crates/topo/src/clearance.rs` (module docs, `window_of`, `boundary_of`, the `lift` module);
  - `crates/topo/src/injective.rs`;
  - the `cavity_embedded` path in `crates/topo/src/shell.rs`, which passes `Interval::from_certified` as its lift.
- **The scalars.** `docs/DESIGN.md` `[scalars]`, `[topology.tiers]` (its K·ε band), and the README pages DESIGN's companion table lists for the scalar ladder, rounding and levers (`crates/geom-core/README.md`, "Float rounding").
  - Read what `Interval`, `Probe`, `Sym` and f64 each mean as the scalar a `Body<T>` is written in.
  - Read what `Interval::from_certified` asserts.
- **Earlier forks near this one.** In `docs/DESIGN-FORK-LOG.md`:
  - row 202 (`work/clear/symbolic-tier-and-clearance-engine.md`, decided without an `[ev]` PR);
  - the PR 6301 row.
  - Check the provenance of any clause you lean on, as `designer.md` §2 says.

## What needs deciding

The engine subdivides each face's parameter domain. For each cell it needs a bound on the cell's whole image: a set bound on a function over a domain, which no point evaluation gives. Today it builds that bound in `Interval` arithmetic, and every stored value of the body enters through `Interval::from_certified`. So one type plays two roles:
- the uncertain-number scalar a body can be written in;
- the engine's range bound over a cell.

At f64, a stored value enters as a degenerate interval.

- **Is that conflation a fault?** If it is, what is the fault exactly, in terms of what each value means? If it is not, say why Ev's unease is answered.
- **What should the cell's bound be,** as a type or otherwise, and how does a body written in each scalar (f64, Probe, Interval, Sym) supply its stored values to it? In particular, what does an f64 body's stored value mean when it enters, given `[topology.tiers]`' band?
- **What can `Sym` contribute?** Answer Ev's question directly: is it incompatible with the engine, or only difficult or impractical, and what would each answer cost?
- **What does your answer change** for `the-at-rest-gate-certifies-embedding`'s spec, for the shell's cavity check, and for any ratified text? If a ratified clause would change, say which, and show its provenance.
