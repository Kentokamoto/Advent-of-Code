import sys
import re
import copy
import math

def part1( instructions ):
    pass

def part2( instructions ):
    pass

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
