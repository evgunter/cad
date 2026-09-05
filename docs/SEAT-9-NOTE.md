# SEAT-9 — the shell arm on `Verb` (coordination note, not yet a spec)

**What SEAT owns here.** `docs/VERB-SEAT-DESIGN.md` §2's vocabulary: a
`Verb::Shell` arm (`thickness: T`, the designated open faces as
`FaceKey`s — kernel types, arity One), a `run` door over
`topo::shell_open`, `VerbRecord::Shell(ShellNaming)` carrying the
record `Shelled` already mints (`crates/topo/src/shell.rs:513-546`)
by value, `VerbError::Shell(ShellError<T>)`, and the flow declaration.
No content tag and no lowering: `Node::Shell` does not exist —
**LIB-G17** (`work/lib/LIB-G17.md`, RECIPE-DOORS D5) is the recipe
door and mints its tag and correspondence when it lands; SHELL's
charter (`work/shell/program.md` `keep_out`) says SEAT's record
variant must agree with what G17 consumes. So SEAT-9 is G17's
ENABLER: the kernel ask G17 was parked on (issue 1202, the
`ShellNaming` birth channel) closed 2026-09-04 and the record exists;
what G17 still lacks is the vocabulary arm that carries it. G17's
`blocked_on: [1202]` is an int lint cannot resolve — LIB's to
un-park; noted here as courtesy.

**The flow row.** `thickness` reaches no stored scalar field the
document declared: a shelled cylinder's inner wall carries
`r − thickness`, a KERNEL-DERIVED value, and VS-Q3 says v1 mints no
source for those. The row is explicitly empty with that reason —
never omitted (the SEAT-4 chamfer precedent) — and the cavity walls
read `None` at every consumer, which is P3's permanent fallback.

**The gate before SEAT-9 dispatches — the raw tolerance (item
`shell-doors-take-tolerance-beside-tol`, SEAT-1's residue).**
`topo::shell` / `shell_open` take `tolerance: f64` beside `tol: Tol`;
measured: it is the offset FIT budget handed to
`replace_faces_offset` (`replace_face.rs:1003`) for the fitted offset
surfaces, and every caller in the tree passes a constant
(`FIT_TOL`, `1e-6`; all are tests — the door has no production
caller). A `Verb<T>` payload cannot carry it as written (an `f64`
beside a `T`), so the arm forces the disposition SEAT-1 deferred:
either (i) it is derivable from `Tol` and the parameter drops at the
door — SEAT-1's rule: only after proving every caller passes the
canonical derivation, and `FIT_TOL` is not `Band::linear(tol)`'s
epsilon by inspection, so this needs the offset-fit owner's word
(`geom-brep/src/offset_fit.rs` is S-CERT's then PROPS'); or (ii) it is
a genuine per-call budget, typed `T`, and then it is document-visible
vocabulary — a slot on LIB-G17's `Node::Shell` — which is a design
question for three programs. **This is an `[ev]` shape**: the SEAT
orchestrator opens it jointly named with SHELL (their file) before
SEAT-9's spec is cut; SEAT-9 waits on that answer, not on G17.

**Block placement.** SEAT-9 is block SEAT-B3's fourth slot in waiting
order (difficulty M, logged pre-draw); SEAT-8, SEAT-FW and SEAT-DN
precede it and none depends on it.
