# Delta check, fix pass ec3a48d4 -> 0ef9b1da (merge base f6bab5e5)

- `main-seam-port.patch`: main f6bab5e5 with the capture hook, the fix
  pass's `with_judgement_naming_fault` seam ported into main's judge
  closure (fault checked after judgement k ran, before its rows are
  named), and the fix pass's `union_defects_in_evaluation_order` tests.
  Both pins pass on main, and their four evaluations hash identically
  (body/names/rows/... or error) to head's: head's precedence is main's.
- Mutants on head (run, then reverted):
  - end read before naming (ec3a48d4's order): both precedence pins red.
  - `given_pairs` keeps the last verdict: m2 pin red at 1e-9, 1e-6, 1e-12.
  - `GluedPair` pub: doctest (line 438) red.
  - `given_at` pub, and `boolean_op_given` pub + `pub use ops::boolean_op_given`
    in `topo::boolean`: all four doctests still pass. The `given_at`
    snippet fails on `GluedPair`'s privacy; the `boolean_op_given` snippet
    fails on module `ops` being private, whatever the fn's visibility.
- Capture, editor-core whole suite (default profile, slow set, debug):
  472 tests / 11,558 evaluations / 82 errors each side, identical except
  the known `ZeroWord` rows of `coincidence_door::an_unproven_fold_row_…`.
  Fail set: the same 25 on both sides.
- Suites on head 0ef9b1da: topo/sweep/verbs ci default ε 6007/6007. Union
  filter (1488 tests, ci --ignore-default-filter) at 1e-6: 9 red, at
  1e-12: 11 red; all red on main (7 revalue_corpus; the four sweep reds
  from the first round; `emit_union_borders::the_corner_boss_logs_its_corner_alike…`,
  red on main f6bab5e5 at 1e-9, 1e-6 and 1e-12).
