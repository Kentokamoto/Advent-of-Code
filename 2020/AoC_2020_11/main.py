import sys
import re
import copy

OCCUPIED = '#'
EMPTY = 'L'
FLOOR = '.'

def isInBounds( seats, newRow, newCol ):
    if newRow < 0 or newRow >= len( seats ):
        return False
    if newCol < 0 or newCol >= len( seats[0] ):
        return False
    return True

def adjacentCount2( seats, row, col, status):
    rcombo = [-1, -1, -1,  0, 0,  1, 1, 1 ]
    ccombo = [-1,  0,  1, -1, 1, -1, 0, 1 ]
    count = 0
    for combo in range( len( rcombo ) ):
        newRow = row + rcombo[combo]
        newCol = col + ccombo[combo]
        while isInBounds( seats, newRow, newCol ):
            if seats[newRow][newCol] == status:
                count += 1 
                break
            elif seats[newRow][newCol] == EMPTY:
                break
            newRow += rcombo[combo]
            newCol += ccombo[combo]
    return count

def part2( s ):
    repeat = True
    tempSeats = copy.deepcopy(s)
    while( repeat ):
        repeat = False
        for rowIndex in range(len(s)):
            for colIndex in range(len(s[rowIndex])):
                n = adjacentCount2( s, rowIndex, colIndex, OCCUPIED )
                if s[rowIndex][colIndex] == EMPTY and \
                   n == 0:
                    repeat = True 
                    tempSeats[rowIndex][colIndex] = OCCUPIED 
                elif s[rowIndex][colIndex] == OCCUPIED and \
                    n >= 5:
                    repeat = True
                    tempSeats[rowIndex][colIndex] = EMPTY
                    
        s = copy.deepcopy(tempSeats)
    return s

def adjacentCount( seats, row, col, status):
    rcombo = [-1, -1, -1,  0, 0,  1, 1, 1 ]
    ccombo = [-1,  0,  1, -1, 1, -1, 0, 1 ]
    count = 0
    for combo in range( len( rcombo ) ):
        newRow = row + rcombo[combo]
        newCol = col + ccombo[combo]
        if newRow < 0 or newRow >= len( seats ):
            continue
        if newCol < 0 or newCol >= len( seats[0] ):
            continue

        if seats[newRow][newCol] == status:
            count += 1 
    return count

 
def occupySeats( seats ):
    repeat = True
    tempSeats = copy.deepcopy(seats)
    while( repeat ):
        repeat = False
        for rowIndex in range(len(seats)):
            for colIndex in range(len(seats[rowIndex])):
                if seats[rowIndex][colIndex] == EMPTY and \
                   adjacentCount( seats, rowIndex, colIndex, OCCUPIED ) == 0:
                    repeat = True 
                    tempSeats[rowIndex][colIndex] = OCCUPIED 
                elif seats[rowIndex][colIndex] == OCCUPIED and \
                    adjacentCount( seats, rowIndex, colIndex, OCCUPIED ) >= 4:
                    repeat = True
                    tempSeats[rowIndex][colIndex] = EMPTY
                    
        seats = copy.deepcopy(tempSeats)
#        printSeats( seats )
    return seats

def printSeats( seats ):
    for i in seats:
        for j in i:
            print( j, end = '')
        print()
    print()

def countSeats( seats, status ):
    count = 0
    for i in seats:
        for j in i:
            if j == status:
                count += 1

    return count

def readFile(fileName):
    seats = []
    file = open(fileName, 'r')
    lines = file.readlines()
    row = []
    for line in lines:
        for c in line:
            if c != '\n':
                row.append(c)
        seats.append(row.copy())
        row = []
    file.close()
    return seats

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    seats = readFile(sys.argv[1])
    seats2 = copy.deepcopy( seats )
    newSeats = occupySeats( seats )
    newSeats2 = part2( seats2 )
    print( countSeats( newSeats, OCCUPIED ) )
    print( countSeats( newSeats2, OCCUPIED ) )
    
main()
