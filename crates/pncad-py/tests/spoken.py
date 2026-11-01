"""How a sentence names a node, read off the binding's machine spelling.

A sentence names a node by its TAG: twelve lowercase hex digits of the
id's digest (`editor_core::spoken`). `NodeId`'s repr prints the whole
id, its mint ordinal, a colon and the sixteen digits of its digest, so
a test that expects a sentence to name a node takes the tag from that
repr HERE, and the tag's rule has one home on this side.
"""


def tag(node):
    """The tag a sentence names `node` by: the first twelve of the
    sixteen hex digits of the digest `repr(node)` prints."""
    whole = repr(node)[len("NodeId(") : -1]
    ordinal, digits = whole.split(":")
    assert ordinal.isdigit() and len(digits) == 16, repr(node)
    return digits[:12]
