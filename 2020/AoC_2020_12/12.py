import sys
import re
import copy
import math

def switcher(c):
    switch = { 'N': 0,
               'S': 1,
               'E': 2,
               'W': 3,
               'L': 4,
               'R': 5,
               'F': 6 }
    return swtich[c]

def part1( instructions ):
    direction = ['E', 'S', 'W', 'N']
    directionIndex = 0
    longitude = 0
    latitude = 0
    for instruction in instructions:
        d = instruction[0]
        v = instruction[1]
        while True:
            if d == 'N':
                latitude += v
                break
            elif d == 'S':
                latitude -= v
                break
            elif d == 'E':
                longitude += v
                break
            elif d == 'W':
                longitude -= v
                break
            elif d == 'L':
                inc = int(v / 90)
                directionIndex = (directionIndex - inc ) % 4
                break
            elif d == 'R':
                inc = int(v / 90 )
                directionIndex = (directionIndex + inc ) % 4
                break
            elif d == 'F':
                d = direction[directionIndex]
    
    return abs( longitude ) + abs( latitude )

def part2( instructions ):
    sin = {0:0, 90:1, 180:0, 270:-1}
    cos = {0:1, 90:0, 180:-1, 270:0}
    latitude = 0
    longitude = 0
    wx = 10
    wy = 1
    for op, val in instructions:
        if op == 'N':
            wy += val
        elif op == 'S':
            wy -= val
        elif op == 'E':
            wx += val
        elif op == 'W':
            wx -= val
        elif op == 'R':
            nwx = wx*round(math.cos(math.radians(-val))) - wy*round(math.sin(math.radians(-val)))
            nwy = wx*round(math.sin(math.radians(-val))) + wy*round(math.cos(math.radians(-val)))
            wx, wy = nwx, nwy
        elif op == 'L':
            nwx = wx*round(math.cos(math.radians(val))) - wy*round(math.sin(math.radians(val)))
            nwy = wx*round(math.sin(math.radians(val))) + wy*round(math.cos(math.radians(val)))
            wx, wy = nwx, nwy
        else:
            longitude += val*wx
            latitude += val*wy
        print(wx, wy)
    return abs( latitude ) + abs( longitude )

def readFile(fileName):
    instructions = []
    file = open(fileName, 'r')
    lines = file.readlines()
    regex = re.compile( "([NSEWLRF])(\d+)" )
    for line in lines:
        temp = regex.match( line )
        instructions.append((temp.group(1), int(temp.group(2))))
    file.close()
    return instructions

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    instructions= readFile(sys.argv[1])
    p1 = part1(instructions)
    p2 = part2(instructions)
    print( "Part 1:{}".format(p1) )
    print( "Part 2:{}".format(p2) )
    
main()
