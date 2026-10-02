import os,collections,difflib,sys
def load(d):
    m=collections.OrderedDict(); last=None
    for f in sorted(os.listdir(d)):
        for l in open(os.path.join(d,f)):
            l=l.rstrip('\n')
            if '\t' in l:
                t,msg=l.split('\t',1); m.setdefault(t,[]).append(msg); last=t
            else: m[last][-1]+=' '+l
    return m
a=load(sys.argv[1]); b=load(sys.argv[2])
print('tests main',len(a),'head',len(b))
for k,m in (('main',a),('head',b)):
    print(k,'REST bodies',sum(1 for t in m for x in m[t] if x.startswith('REST body')),'REST none',sum(1 for t in m for x in m[t] if x.startswith('REST none')),'REST err',sum(1 for t in m for x in m[t] if x.startswith('REST err')))
diff=[t for t in set(a)|set(b) if a.get(t)!=b.get(t)]
print('differing tests',len(diff))
for t in sorted(diff):
    A=a.get(t,[]);B=b.get(t,[])
    print('==',t, len(A), len(B))
    for l in list(difflib.unified_diff(A,B,lineterm='',n=0))[2:10]: print('  ',l[:220])
same_body=[t for t in a if a.get(t)==b.get(t) and any(x.startswith('REST body') for x in a[t])]
print('identical tests with REST bodies:',len(same_body))
print(' '.join(sorted(same_body)))
print('--- main REST bodies preserved in head, per differing test')
for t in sorted(diff):
    A=[x for x in a.get(t,[]) if x.startswith('REST body')]
    B=[x for x in b.get(t,[]) if x.startswith('REST body')]
    from collections import Counter
    ca,cb=Counter(A),Counter(B)
    lost={k:v-cb[k] for k,v in ca.items() if cb[k]<v}
    print(t[:80], 'main bodies',len(A),'head',len(B),'lost',lost)
