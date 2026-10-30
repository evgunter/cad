---
id: the-topo-readme-still-states-declared-contact
kind: issue
title: crates/topo/README.md still states CONTACT-DESIGN C4's declared contact and a mate declaration at rest, after stages 4 and 5 landed
status: closed
opened: 2026-10-29
closed: 2026-10-30
priority: P4
cost: E
refs: [d10-exit-walk]
pr: 6079
branch: contacthold/glued-pair-reads-its-record
---


Found by the D10 exit walk (`work/intent/d10-exit-walk-report.md`, companion pages).

`crates/topo/README.md` still states CONTACT-DESIGN's declaration model:

- `:51`: "a **declaration** is recipe data asserting a contact class on
  a named face pair". No recipe data declares a pair (stage 4 F, #4641;
  a mate's class, stage 4 I, #5637).
- `:422` (C7): "the at-rest door: `validate_pseudomanifold` with mate
  declarations landed in `ContactRecords`". Mates declare no contact
  (#5637).
- `:949-1010`, "Until the intent refactor lands", and its **C4 —
  Declared contact as data** (`:957`). The section says "INTENT stage 4
  and stage 5 … delete them as they land". Both stages have landed. Its
  "declaration bridges only the residue" is the rung
  `a-glued-pair-is-re-read-through-the-declared-rung-which-bridges-in-band`
  asks about.
- `:1184` and `:377`: "no declaration admits it".

DESIGN's state-of-the-build list retires C4's declared seat, so this is
not a new design decision. Once that row is answered, re-word C4 and the
preamble to say what the glue table is and how its pairs are read, and
remove the fence.

## Closed

Closed by PR 6079: crates/topo/README.md states C4 as a glued pair reading its record, and the declaration lines are re-worded.
