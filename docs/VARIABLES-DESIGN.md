# Variables — D10 stage 1 (VARIABLES-DESIGN VR1–VR9)

The scalar half of D10's "every slot holds a variable". Weighed by a
designer pair on `work/intent/variables-are-identities-with-labels`;
the two converged, and this page states the merged design. It extends
to the typed variables of later stages without moving anything stated
here: only the payload shapes are per kind.

**VR1 — Identity is minted.** A variable is a `VarId`, minted from the
document's mint chain beside `RecipeNodeId` and `StepId` (a `var` tag
in the mint log): a function of the edit sequence (D9), never reused,
never positional. The id is what every reader, every coincidence token,
every content key and every symbolic-tier symbol carries, so nothing
that identifies a variable is text.

**VR2 — A name is separate, optional and unique.** A variable may have
a name, held beside it in the document, unique within the document,
resolved only by the formula text door and written only by `unparse`.
Setting, changing or clearing it is one edit that recomputes nothing.
A name is not a node label: a formula reads a name, so it must resolve
and be unique, where a label is never resolved and need not be. The
kernel mints no name and the GUI proposes none: the person types one
when a value is first shared or promoted, and a caller gives one when
it declares. An unnamed variable has exactly one reader, a slot or a
definition, and reads as its value and that reader ("5 mm, the depth of
Extrude "base plate""), and as a hex tag in diagnostics, as a node does.
An output reads as its operation and port, a name optional.

**VR3 — Kind and definition.** `Var { kind, def }`. Stage 1's kinds are
`Length`, `Angle`, `Scalar` and `Count`, fixed at minting: a new kind is
a new variable. `def` is `Free { value, unit, distribution }` (a
`Count` holds an exact integer and nothing else) or `Defined(Expr)`. A
definition may not reach its own variable (refused at the edit door and
at load). Distributions live only on free variables; a defined
variable's uncertainty is the pushforward of its inputs'. Later stages
add kinds (`Point` … `Frame`, the discrete kinds, `Face`, `Edge`,
`Body`) and the `Output { node, port }` definition as arms of the same
enums.

**VR4 — A slot holds a `VarId`.** Every slot — a feature's depth, a
pattern's count or index, a profile step's argument, a placement step,
an assertion's bound — holds one variable id and nothing else. A slot
showing `w * 2` holds an anonymous defined variable; a slot showing
`5 mm` an anonymous free one. Formulas have one home: definitions. The
exception is a `Measure`'s arithmetic over measured primitives, which
stays a formula in the node until stage 2 makes `Measure` an operation;
its value leaves and an assertion's bound are slots.

**VR5 — `Expr` holds no float.** Its leaves are `Var(VarId)` (caching
the kind, which cannot change), exact rational constants (`Scalar`,
reduced; integers for `Count`) and `turn` (`Angle`, one full rotation).
A right angle is `turn/4`. The dimension lattice, the operators and the
nesting bound stay; the `Literal` leaf and every literal constructor
go.

**VR6 — `Formula` is authored, `Expr` is stored.** What a person or a
caller writes is a `Formula`: names, variable ids, written quantities
(`5 mm`, `90 deg`) and constants. The edit door lowers a `Formula` to
an `Expr`, resolving names in the document's scope and minting an
anonymous free variable for each written quantity, so `w + 5 mm` stores
`w + v` with `v` editable, nameable and offerable; until it is named, `v`
belongs to that definition. The spelling decides
the meaning: `90 deg` is a value from a continuous family and becomes a
variable; `turn/4` is the exact constant. The document's types cannot
hold a written quantity. A lone number at a slot's root is a typed value too:
`3` in a count slot mints a free `Count`, `0.5` a free `Scalar`. A
number inside an operator tree is a constant, and `turn/4` alone is a
formula, so a constant.

**VR7 — Lifecycle.** An unnamed variable belongs to its one reader and
goes when that reader is deleted or stops reading it; an edit that would
give it a second reader, or clear the name of a variable two readers
share, refuses until it is named. Deleting a named
variable leaves its readers unresolved and typed, never re-pointed, and
evaluation refuses at each reader (D10's deletion rule; the mint log
keeps the id from being reused). The doors: declare, set value / unit
/ distribution, define (free ↔ defined, same identity), rename, delete,
and set a slot from a `Formula`. Create-or-replace by name goes.

**VR8 — Coincidence tokens and the analysis lanes read ids.**
`ParamSource` lowers a slot to its variable expanded through
definitions to free ids, with constants as exact rationals: two slots
reading one variable lower equal, two typed `5 mm` lower distinct, and
a rename moves no token. The symbolic tier's parameter symbols and the
error-propagation lane's seeds are keyed by `VarId`. A variable is an
analysis axis only if it carries a tolerance — a band or a
distribution — named or anonymous alike. A free variable without one
is a constant in every analysis lane: the box, the dual seeds, the
symbolic tier (which binds its exact nominal, not a symbol), Monte
Carlo and the stackup's entries. A query that names a variable itself
(`range`, an explicit seed) widens it whatever it carries.

**VR9 — Scope, persistence, the façade.** A variable belongs to its
document; `ParamScope::Root` / `Part` prefix its tokens. Persistence
holds the variable table, the names and the mint log; a file this build
cannot read refuses typed with the regenerate recourse, and the load
door checks VR2's uniqueness, VR3's acyclicity and kinds, VR4's slot
kinds and VR7's unnamed-has-one-reader. The façade's slot arguments accept a
variable, a `Formula`, or a written quantity (which mints an anonymous
variable for that call); passing a named variable is how two slots share
one.
Python mirrors it. Split moves a variable with its readers; inline
carries every variable of the part under a new id.
