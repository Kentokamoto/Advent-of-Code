import sys

def countCollision( mat, row, col ):
    currRow = 0
    currCol = 0
    count = 0;
    while currRow < len(mat):
        if mat[currRow][currCol] == '#':
            count += 1
        if currRow == len(mat) - 1:
            break;
        currRow = currRow + row if currRow + row < len(mat) else len(mat) - 1
        currCol = (currCol + col) % len(mat[currCol])

    return count

def readFile(fileName):
    f = open( fileName, 'r' )
    lines = f.readlines()
    output = []
    for line in lines:
        row = []
        for c in line:
            if c != '\n':
                row.append(c)
        output.append(row)
    f.close() 
    return output

def main():
    if len(sys.argv) != 2:
        print("Looking for 2 arguments")
        return
    
    mat = readFile(sys.argv[1])
    combo = [(1, 1),
             (3, 1),
             (5, 1),
             (7, 1),
             (1, 2)]
    count = 1
    for c in combo:
        count *= countCollision( mat, c[1], c[0] )
    print( count )

main()
