import rank
import sys

path = sys.argv[-1]
print(rank.score_image(path))