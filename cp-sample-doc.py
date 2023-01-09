import glob
import os
import shutil

DEST="../procon-input-support/example"
shutil.rmtree(DEST)
shutil.copytree("test-data/case", DEST)