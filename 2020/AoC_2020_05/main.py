import sys
import math

def findCol(colVal):
    low = 0
    high = 7
    for c in colVal:
        mid = low + (high - low) // 2
        if c == "L":
            high = mid
        else:
            low = mid + 1
    return low

def findRow(rowVal):
    low = 0
    high = 127
    mid = 0
    for r in rowVal:
        mid = low + (high - low) // 2
        if r == "F":
            high = mid 
        else:
            low = mid + 1
    return low

def highestSeatId( boardingPasses ):
    highest = 0
    for v in boardingPasses:
        highest = max( highest, v[2])
    return highest

def findMySeat( boardingPasses ):
    occupied = [ 0 for i in range( 127 * 8 + 7 ) ]
    for v in boardingPasses:
        occupied[ v[2] ] = 1
    potentialSeats = []
    for i in range( 8, 127 * 8 ):
        if occupied[i] and not occupied[i - 1] and occupied[ i - 2 ]:
            potentialSeats.append( i - 1 )
    for i in potentialSeats:
        print( i )

def processLine(line, boardingPasses):
    row = line[0:7]
    col = line[7:10]
    #print("{} | {}".format(row, col), end='')
    r = findRow(row)
    c = findCol(col)
    seatId = r * 8 + c
    #print( "\t({}, {}, {})".format( r, c, seatId ) )
    boardingPasses.append(  ( r, c, seatId ))

def readFile(fileName):
    boardingPasses = []
    file = open(fileName, 'r')
    lines = file.readlines()
    for line in lines:
        processLine(line, boardingPasses )    
    file.close()
    return boardingPasses

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    boardingPasses = readFile(sys.argv[1])
    print( highestSeatId( boardingPasses ) )
    findMySeat( boardingPasses )
main()
