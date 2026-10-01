"""How a sentence names a node, read off the binding's machine spelling.

A sentence names a node by its TAG: twelve lowercase hex digits of the
id (`editor_core::spoken`). `NodeId`'s repr prints the whole id, sixteen
digits, so a test that expects a sentence to name a node takes the tag
from that repr HERE, and the tag's rule has one home on this side.
"""


def tag(node):
    """The tag a sentence names `node` by: the first twelve of the
    sixteen hex digits `repr(node)` prints, the id's prefix."""
    digits = repr(node)[len("NodeId(") : -1]
    assert len(digits) == 16, repr(node)
    return digits[:12]
