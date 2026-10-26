import sys
sys.path.insert(0, "/home/user/rv5748/crates/pncad-py/tests")
from test_loft_sections import square, up
from pncad import Doc, DocEdit, EditError, Formula, Node, evaluate
d = Doc()
a = d.insert(Node.profile(square(0.5))); b = d.insert(Node.profile(square(0.4)))
loft = d.insert(Node.loft(a, [(b, up(1.0))], Formula.count(1)))
for name, then in [("one-section", []), ("empty-station", [(b, [])])]:
    try:
        d2 = d  # Doc is mutable? apply in place
        d.apply(DocEdit.set_loft_sections(loft, a, then))
        print("PY", name, "ADMITTED; succeeded:", evaluate(d).succeeded(loft))
        d.apply(DocEdit.set_loft_sections(loft, a, [(b, up(1.0))]))
    except EditError as e:
        print("PY", name, "refused", e.variant)
