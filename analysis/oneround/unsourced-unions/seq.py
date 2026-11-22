import sys,collections
def load(p,pred):
    d=collections.defaultdict(list)
    for l in open(p):
        x=l.rstrip('\n').split('\t')
        if len(x)>=4 and x[1]==pred: d[x[0]].append(x[2][0] if x[2]!='ESC' else 'E')
    return d
pred=sys.argv[3]
h=load(sys.argv[1],pred); b=load(sys.argv[2],pred)
for t in sorted(set(h)&set(b)):
    if h[t]!=b[t] and 'fuzz' not in t and 'random' not in t:
        print(t); print('  base',''.join(b[t])[:120]); print('  head',''.join(h[t])[:120])
