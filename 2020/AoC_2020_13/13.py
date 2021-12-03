import sys
import re
import copy
import math
from itertools import count

def part1(earliestTime, busIds):
    d = dict()
    for v in busIds:
        if not v.isdigit():
            continue
        busId = int(v)
        if earliestTime % busId != 0:
            d[busId] = (math.floor(earliestTime / busId) + 1) * busId
        else:
            d[busId] = 0

    waitTime = d[int(busIds[0])] - earliestTime
    bus = busIds[0]
    for key, val in d.items():
        diff = val - earliestTime
        if diff < waitTime:
            waitTime = diff
            bus = key
    print( "BusId:{}, Diff:{}".format(bus, waitTime))
    return bus * waitTime

def part2( busIds ):
    start = 000000000000000
    mult = int(busIds[0])
    target = round(start / mult)
    repeat = True
    step = 1
    buses = tuple( (int(b), i) for i, b in enumerate( busIds ) if b.isdigit())
    #TODO: Understand why this is the quickest
    for val, index in buses:
        mult = next(c for c in count(mult, step) if (c + index) % val == 0)
        step *= val
    return mult

def readFile(fileName):
    busIds = []
    file = open(fileName, 'r')
    lines = file.readlines()
    earliestTime = int(lines[0])
    for elem in lines[1].strip().split(','):
        busIds.append(elem)
    file.close()
    return earliestTime, busIds

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    earliestTime, busIds= readFile(sys.argv[1])
    p1 = part1(earliestTime, busIds)
    p2 = part2(busIds)
    print( "Part 1:{}".format(p1) )
    print( "Part 2:{}".format(p2) )
    
main()
