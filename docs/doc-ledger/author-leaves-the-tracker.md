# AUTHOR leaves the tracker — 2026-10-02

AUTHOR covered what the GUI could not author: the doors a person needs to build geometry without leaving the viewer. CHROME's 2026-09-20 priority-seam cut opened it at P0, as the GUI half of Ev's standing goal. It closed on 2026-10-02 at Ev's word in chat: close AUTHOR and split the remainder into successor programs. Its plan set no `## Exit criteria`, so under `work/README.md`'s closing rule no exit walk is owed.

**What landed:** sixteen units, AUTH-1 to AUTH-16. The PR numbers are the record.
- **Doors that did not exist:**
  - a picked face's frame, `Datum::FaceFrame` (AUTH-1, #2955);
  - the add-profile form mints its own world-xy frame in one undo, and says which frame it means (AUTH-3, #3023);
  - `AddPart`, and Ev's duplicate-a-body (AUTH-4, #3052);
  - a committed sketch can be reshaped (AUTH-6, #3446);
  - a union declares its contact from the refusal (AUTH-9, #3543);
  - accept a part's updated version (AUTH-15, #3591).
- **Typed into, not dragged:** a parameter's notation at both doors (AUTH-2, #2957).
- **Values the viewer never showed:**
  - a measure's value, or why it has none (AUTH-7, #3528);
  - an assertion's verdict (AUTH-8, #3535);
  - whether a mate placed its child (AUTH-16, #3769).
- **The picture of an authoring in progress:**
  - the path preview draws what replays when a step refuses (AUTH-5, #3440);
  - a held face pick is drawn (AUTH-10, #3556);
  - an unclosable chain draws its prefix (AUTH-11, #3563);
  - a geometry-refused close still draws what was written (AUTH-13, #3579).
- **Honest chrome:**
  - every tool is reachable, and a row says so (AUTH-12, #3573);
  - a blend target whose edges cannot be named says so (AUTH-14, #3585).
- **Three `[ev]` design forks:**
  - #3551, a negative extrude: an extrude is a positive depth with a structural side;
  - #3571, face naming: a face is told apart by its leaf role in words;
  - #3587, a declaration on a live boolean: A2, the declared pairs become the boolean's own payload.

  Their decision records are rows 22–24 of `docs/DESIGN-FORK-LOG.md` and the PRs themselves. Each ruling's kernel half is on RECIPE's slate.

**The lesson the log kept recording.** Every unit minted some duplicate: a hand copy of a kernel string, a second home for a rule, or a parallel test builder. A review caught each one. Both successors' plans carry this forward.

**Where its 14 live rows went.** The rows moved with their ids. AUTHOR measured 35.5 budget points against 30, and its residue was two tracks with different owners:
- **DOORS** (new, `work/doors/`, `blocked`): the viewer halves of the three rulings, plus the GUI's clearance consumer. All five rows are `parked` on their kernel blockers, and weigh 15 points once they unpark:
  - three blockers on RECIPE: `extrude-distance-is-a-depth-and-a-side`, `names-render-a-faces-leaf-role-in-words` and `declared-pairs-are-a-booleans-own-payload`;
  - one on CLEAR: `clearance-refusal-names-one-face-twice-across-bodies`.
- **AUTHTAIL** (new, `work/authtail/`, `ready`, 19.5 points): the unblocked tail, nine rows:
  - the picture of an authoring in progress;
  - the authoring chrome's hygiene;
  - one row new at the sweep, `drawing-on-a-picked-face-is-a-two-form-trip`. AUTH-1 named the residue and scheduled it on AUTH-3's row, which closed without it, and two code comments still cited the closed row.

**Closed at the sweep:**
- `a-last-leg-walked-back-row-fails-off-the-default-eps`, which TOPO's PR 3590 had already fixed, re-verified at ε 1e-6 and 1e-12;
- `a-mate-row-does-not-say-whether-it-placed-its-child`, closed with AUTH-16.

**Filed elsewhere at the sweep:** BIND's `materole-reads-in-words-only-in-rust` (the Python `MateRole` has no `__str__`), found by AUTH-16.

**Bands:** DOORS takes 10600–10699 and AUTHTAIL 10700–10799 in `docs/MODEL-AB-LOG.md`. AUTHOR's 6800–6899 closes unspent.

**Pruned with the directory:** the fifteen unit specs `docs/AUTH-1-SPEC.md` to `docs/AUTH-15-SPEC.md` (AUTH-16 had none), each with its own note beside this one.

**Citations:**
- Citations of moved rows are rewritten to the new paths in code, docs and other programs' rows.
- In code, the two comments naming a closed row now name the new AUTHTAIL row.
- Process-doc citations of closed AUTHOR rows and of `work/author/log.md` are left to dangle, per the docs-ledger convention, except where a live row reads one as its decision record. Those (RECIPE's three ruled rows and its plan, and the DOORS rows) now carry the recovery command.
- **Eighteen `refs:` entries and one `parent:` named closed AUTHOR rows, and were dropped.** They were on DOORS's and AUTHTAIL's own rows, and on CHROME, MSOLVE, PATHS, PROPS, ROUND, VACUITY, VGEOM and ZIP. Their targets are recoverable below.
- Other programs' `keep_out` prose that names AUTHOR is left as written. Read "AUTHOR's" there as DOORS's for the rulings' viewer halves, and as AUTHTAIL's for everything else.

Recover with `git show 29b8874a1:work/author/<file>`, using `plan.md`, `log.md`, `program.md` or an item id.
