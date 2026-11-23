---
id: a-check-offers-no-way-to-add-the-assertion-it-asks-for
kind: issue
title: viewer: a check tells the person to add an assertion, and neither the check nor any other door lets them
status: closed
opened: 2026-11-22
priority: P1
cost: H
pr: 7362
branch: offer/a-check-carries-the-assertion-that-quiets-it
closed: 2026-11-23
---

**Ev reported this** (in chat, 2026-11-22): "the checks report that you should add an assertion to clear them, but there's no clear way to add assertions anywhere (and it should be offered from the check itself)".

The advice is the kernel's: `Recourse::words` (`crates/editor-core/src/coincide.rs:612-660`) says "Recourse: assert the coincidence at its site" whenever `checks::at_rest::row_quietable` (`crates/editor-core/src/checks/at_rest.rs:481`) allows it. `OVERCONSTRAINED_RECOURSE` (`crates/editor-core/src/mate.rs:816-818`) says "an assertion says so", and `crates/editor-core/src/checks.rs:537-545` has the matching quiet wording.

The Checks window (`crates/viewer/src/app.rs:1787-1835`, `checks_window`) draws, per row, a select button, the sentence, and only two offers: "add to the product" for an unused finding, and cell-select buttons for a coincidence. `CheckRow` (`crates/viewer/src/frame.rs`, ~2385-2405) has no assertion field. The viewer's only assertion door is the clearance tool (`SessionOp::AddClearance`, `crates/viewer/src/session/op.rs:839`; `crates/viewer/src/pane/create.rs:595-610`, `:1785`). No op authors a coincidence or value assertion, though Python has both (`crates/pncad-py/src/py/doc.rs:3634` `assertion`, `:3679` `clearance`).

The fix: a check row whose recourse is an assertion carries an "Assert" control that commits exactly the assertion the recourse names, at its site. That needs a session op for each assertion kind the checks recommend, and the finding has to carry enough to build the claim. A general add-assertion tool can follow from the same ops. This is `a-refusal-offers-no-action-in-the-viewer`'s refuse-then-offer pattern on the checks surface. Related: `work/chrome/an-unproven-coincidence-is-hard-to-resolve-by-a-variable.md` (open).

## The unit (2026-11-22, after Ev's decision on #7341)

This row carries the kernel half of the decided design: the `## Design` section of `a-refusal-offers-no-action-in-the-viewer`.

- **The coincidence finding** carries, in place of `quietable: bool`, the assertion that says its row. That is the measure (`Gap` or `Distance`), the two references as the measure reads them, and `= 0`, together with the edits that author it. It is minted by `saying`, the same rule that `quiet_row` matches against. Whether an assertion could quiet the row is whether that value exists. A row the kernel cannot mint for (check family rows: `Saying::reference` drops indexed selections) is not quietable. It names the construction alone and does not refuse at `Error`. State that behaviour change in the PR.
- **The interference finding** carries its opposed face pair, also in place of `quietable: bool`. Its bound is the person's to give.
- **The recourse words** are rendered from the value, so the sentence and the value cannot disagree.
- **A corpus law test:** apply every offered assertion, re-run the checks, and the row is quiet and names the new assertion.
- **`pncad-py`** exposes the value (`finding.quieting`), with stubs.

Ground: `crates/editor-core/src/checks/at_rest.rs` (ATREST), `coincide.rs` and `checks.rs` (COINCIDE), `finding.rs`, and `crates/pncad-py`. OFFER carries it, and the unit announces itself on both owners' logs. Re-priced M → H: it sets the pattern every later finding follows.
