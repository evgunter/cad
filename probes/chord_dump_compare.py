import sys,collections
def load(p):
    d=collections.OrderedDict()
    for l in open(p):
        n,pid,w=l.rstrip("\n").split("\t",2)
        d.setdefault(n,[]).append(w)
    return d
a=load(sys.argv[1]); b=load(sys.argv[2])
only_a=[k for k in a if k not in b]; only_b=[k for k in b if k not in a]
print("base tests",len(a),"head tests",len(b),"only base",len(only_a),"only head",len(only_b))
same=diff=0; out=[]
for k in a:
    if k in b:
        if a[k]==b[k]: same+=1
        else:
            diff+=1
            # classify
            sa=[x for x in a[k] if x.startswith("SPEC")]; sb=[x for x in b[k] if x.startswith("SPEC")]
            kind=[]
            common=set(sa)&set(sb)
            kind.append(f"spec base={len(sa)} head={len(sb)} common={len(common)} base-only={len(set(sa)-set(sb))} head-only={len(set(sb)-set(sa))}")
            ea=[x[:90] for x in a[k] if x.startswith("ERR")]; eb=[x[:90] for x in b[k] if x.startswith("ERR")]
            if ea!=eb: kind.append(f"errs base={ea[:2]} head={eb[:2]}")
            out.append((k,kind))
print("same",same,"diff",diff)
for k,kind in out: print(k, *kind, sep="\n   ")
print("ONLY BASE:",*only_a[:40],sep="\n  ")
print("ONLY HEAD:",*only_b[:60],sep="\n  ")
