import glob
import os
import shutil

DEST="../procon-input-support/example"
shutil.rmtree(DEST)
os.makedirs(DEST, exist_ok=True)
l=[]
for p in glob.glob("test-data/case/*/parser"):
    l.append(p)
l.sort()
# print(l)
for (i,p) in enumerate(l):
    shutil.copyfile(p, DEST+'/'+str(i+1))