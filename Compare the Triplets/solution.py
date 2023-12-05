#!/bin/python3

import math
import os
import random
import re
import sys

# Complete the compareTriplets function below.
def compareTriplets(a, b):  
  a_score = 0
  b_score = 0

  for (point_a, point_b) in zip(a, b):
    if point_a > point_b:
      a_score += 1
    elif point_a < point_b:
      b_score += 1

  return [a_score, b_score]

if __name__ == '__main__':
    fptr = open(os.environ['OUTPUT_PATH'], 'w')

    a = list(map(int, input().rstrip().split()))

    b = list(map(int, input().rstrip().split()))

    result = compareTriplets(a, b)

    fptr.write(' '.join(map(str, result)))
    fptr.write('\n')

    fptr.close()
