# Ev's words, verbatim — part 4 of the 2026-10-03 conversation behind ruling `one-way-to-say-dependency-and-intent`

Continues commit fae23dbc71. Deleted in the next commit; the ruling and
PR #3990 point here.

**Ev, 11:**

> one thing that might not already be in there, but should be added if it's not--to replace "coincidence is never inferred from values", we need "the lint [we need a name for this specific lint] checks every case where a coincidence is inferred from values to ensure that it holds symbolically"

[Assistant: named it `unproven-coincidence`; every coincidence the kernel infers from values (boolean glue, split ON verdicts, at-rest census contacts) is recorded at the one structural door and the lint checks each holds symbolically — today by canonical-form equality, later by any added rung. At-rest contact between copies became the same lint; interference stays its own finding. D10 and the banked coincidence principle state it.]
