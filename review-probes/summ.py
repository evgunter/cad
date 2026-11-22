import sys,glob,re,collections
def load(tag):
    first={}
    for f in glob.glob(f'log-{tag}/*.log'):
        for line in open(f):
            c=line.rstrip('\n').split('\t')
            if c[0]!='GATE': continue
            test=c[2]; verdict=c[9]
            if verdict!='Embedded' and test not in first:
                first[test]=(verdict[:230], c[10][:200])
    return first
if __name__=='__main__':
    a=load(sys.argv[1])
    for t in sorted(a):
        if len(sys.argv)<3 or re.search(sys.argv[2],t): print(t,'\n   ',a[t][0],'\n   ',a[t][1])
