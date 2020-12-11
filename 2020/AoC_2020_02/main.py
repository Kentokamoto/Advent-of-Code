import sys
import re

class password:
    def __init__( self, l, h, c, pw ):
        self.low = l
        self.high = h
        self.c = c
        self.pw = pw
    
    def isValid(self):
        count = 0
        for c in self.pw:
            if c == self.c:
                count += 1

        return self.low <= count <= self.high

    def positionValid(self):
        left = self.pw[self.low-1] == self.c
        right = self.pw[self.high-1] == self.c if self.high < len( self.pw) else false
        return left != right

def countValid( passwords ):
    count = 0
    for p in passwords:
        if p.isValid():
            count += 1;
    
    return count

def countValidPos( passwords ):
    count = 0
    for p in passwords:
        if p.positionValid():
            count += 1
    return count

def main():
    if len(sys.argv) != 2:
        print("Looking for 2 arguments")
        return
    
    f = open( sys.argv[1], 'r' )
    lines = f.readlines()
    f.close();
    regex = re.compile("(\d+)\-(\d+)\s(\D):\s(\D+)") 
    passwords = []
    for line in lines:
        x = regex.match( line )
        p = password( int(x.group(1)),
                      int(x.group(2)),
                      x.group(3),
                      x.group(4)) 
        passwords.append(p)

    validPasswords = countValid(passwords)
    validPosPasswords = countValidPos(passwords)
    print("Valid Passwords: {}".format(validPasswords))
    print("Valid Position Passwords: {}".format(validPosPasswords))

main()
