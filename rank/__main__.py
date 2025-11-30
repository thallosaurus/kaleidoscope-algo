import rank
import sys

path = sys.argv[-1]
print(path + ": " + str(rank.score_image(path)))