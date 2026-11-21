# Design fork: turn-lever composite verdicts (ONEROUND, 2026-11-21)

Raised by the `oneround/turn-lever` lane (PR 7262) at 08:59Z: the turn-lever doors now return both bounds, and every consumer but certification makes several decisions of different kinds on the one lever, with no home in the tree for "the bounds disagree" on a composite verdict.

Draws, `/dev/urandom`, taken at 08:59Z before dispatch (protocol `ddd201f41`, rule 3):
- first byte 149: not below 86, so TWO designers;
- second byte 22: even, so **Opus is A, Fable is B**.

The problem statement as handed to both is `problem-as-handed-to-designers.md` beside this file.
