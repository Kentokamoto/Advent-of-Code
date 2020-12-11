import sys
import re

combos = 0
def chain( jolts, currJoltage, diff ):
    if len( jolts ) == 0:
        return True
    for i in range( 1, 4 ):
        if currJoltage + i in jolts:
            jolts.remove( currJoltage + i )
            diff[i - 1] += 1
            if not chain( jolts, currJoltage + i, diff ):
                jolts.add( currJoltage + i )
                diff[i - 1] -= 1
            else:
                return True
    return False

def chain2( jolts, currJoltage, currCombo, memo,  maxVal ):
    global combos
    if currJoltage == maxVal:
        return 1
    if currJoltage in memo:
        return memo[currJoltage]
    val = 0
    for i in range( 1, 4 ):
        if currJoltage + i in jolts:
            currCombo.append( currJoltage + i )
            jolts.remove( currJoltage + i )
            val += chain2( jolts, currJoltage+i, currCombo, memo, maxVal )
            jolts.add( currJoltage + i )
            currCombo.pop()
    memo[ currJoltage ] = val
    return val
        
def readFile(fileName):
    jolts = set()
    file = open(fileName, 'r')
    lines = file.readlines()
    for line in lines:
        jolts.add(int(line)) 
    file.close()
    return jolts 

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    jolts = readFile(sys.argv[1])
    jolts2 = jolts.copy()
    maxVal = max(jolts)
    diff = [0, 0, 0]
    global combos
    currCombo = []
    memo = {}
    currJoltage = 0
    chain( jolts, currJoltage, diff)
    diff[2] += 1
    print( diff )
    val = chain2( jolts2, currJoltage, currCombo, memo, maxVal )
    print(val ) 
main()
