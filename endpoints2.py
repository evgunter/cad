import subprocess, re
sha = "146ef44906f515a9716364b4cf206a86d5ab7bc6"
WT = "/home/user/cad/.claude/worktrees/agent-a011cad2bb870ef53"
files = [l.strip() for l in open('/home/user/scalar-ring0-r2-scratch/ringsrc.txt') if l.strip()]
ep_re = re.compile(r'\.(lo|hi)\(\)')
op_re = re.compile(r'(==|!=|<=|>=|<|>)')


def prod_lines(txt):
    """Yield (lineno, line) for lines outside any #[cfg(test)] mod block."""
    lines = txt.split("\n")
    out = []
    i = 0
    n = len(lines)
    while i < n:
        l = lines[i]
        if l.strip().startswith("#[cfg(test)]"):
            # find the item it attaches to; if it's a mod, skip the block
            j = i + 1
            while j < n and (lines[j].strip().startswith("#[") or lines[j].strip() == ""):
                j += 1
            if j < n and re.match(r'\s*(pub\s+)?mod\s', lines[j]):
                # skip to matching brace
                depth = 0
                started = False
                k = j
                while k < n:
                    depth += lines[k].count("{") - lines[k].count("}")
                    if "{" in lines[k]:
                        started = True
                    if started and depth <= 0:
                        break
                    k += 1
                i = k + 1
                continue
        out.append((i + 1, l))
        i += 1
    return out


total = 0
for f in files:
    txt = subprocess.run(["git", "show", sha + ":" + f], capture_output=True, text=True, cwd=WT).stdout
    for ln, l in prod_lines(txt):
        s = l.strip()
        if s.startswith("//") or s.startswith("///"):
            continue
        if not ep_re.search(l):
            continue
        tag = []
        if op_re.search(l):
            tag.append("CMP")
        if "from_bounds" in l:
            tag.append("REMINT")
        for kw in ("is_finite", "is_nan", "sqrt", ".max(", ".min(", "abs()", "next_up", "next_down"):
            if kw in l:
                tag.append(kw)
        print("%s:%d: [%s] %s" % (f, ln, "/".join(tag) or "plain", s[:150]))
        total += 1
print("TOTAL lines with an endpoint read:", total)
