import sys
import re
import copy
import math
from itertools import count

def part1(addr):
    total = 0
    for key, val in addr.items():
        total += val 
    return total

def part2( busIds ):
    pass

def createMask( mask ):
    oneMask = 0x000000000
    zeroMask = 0xFFFFFFFFF
    maskOrder = list( reverse( mask ) )
    for i in range( len(maskOrder) ):
        if maskOrder[i] == '1':
            oneMask = oneMask | ( 1 << i ) 
        elif maskOrder[i] == '0':
            oneMask = oneMask & ( 0 << i ) 
    return ( zeroMask, oneMask )        

def useMask( val, zeroMask, oneMask ):
    val = val & zeroMax
    val = val | oneMask
    return val

def readFile(fileName):
    currMaskZero = 0xFFFFFFFFF
    currMaskOne = 0x000000000
    addr = {}
    regex = re.compile("(mem(\[\d+\])|mask) = ([\dX]+)")
    file = open(fileName, 'r')
    lines = file.readlines()
    for line in lines:
        r = regex.match(line)
        if r.group(1) == "mask":
            # For 1, Bitwise OR
            # For 0, Bitwise AND
            currMaskZero, currMaskOne = createMask( r.group( 3 ) )
        else:
            addr[ int( r.group(2) ) ] = useMask( int( r.group( 3 ) ),
                                                 currMaskZero,
                                                 currMaskOne )
        
    file.close()
    return addr

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    addr = readFile(sys.argv[1])
    p1 = part1(earliestTime, busIds)
    #p2 = part2(busIds)
    print( "Part 1:{}".format(p1) )
    #print( "Part 2:{}".format(p2) )
    
main()
