import sys
import re

class Passport:
    requiredFields = ["byr",
                      "iyr",
                      "eyr",
                      "hgt",
                      "hcl",
                      "ecl",
                      "pid", ]
    def __init__(self):
        self.fields = {"byr": None,
                       "iyr": None,
                       "eyr": None,
                       "hgt": None,
                       "hcl": None,
                       "ecl": None,
                       "pid": None,
                       "cid": None }
    def isValid(self):
        output = True
        for f in self.requiredFields:
            if not self.fields[ f ]:
                return False
            else:
                if f == "byr":
                    output &= self.validByr()
                elif f == "iyr":
                    output &= self.validIyr()
                elif f == "eyr":
                    output &= self.validEyr()
                elif f == "hgt":
                    output &= self.validHgt()
                elif f == "hcl":
                    output &= self.validHcl()
                elif f == "ecl":
                    output &= self.validEcl()
                elif f == "pid":
                    output &= self.validPid()
        return output

    def isBetween(self, key, low, high):
        return low <= int( self.fields[key] ) <= high

    def validByr(self):
        return self.isBetween("byr", 1920, 2002)

    def validIyr(self):
        return self.isBetween("iyr", 2010, 2020)

    def validEyr(self):
        return self.isBetween("eyr", 2020, 2030) 
        
    def validHgt(self):
        regex = re.compile( "(\d+)(cm|in)")
        m = regex.match(self.fields["hgt"])
        if m:
            if m.group(2) == "in":
                return 59 <= int(m.group(1)) <= 76
            else:
                return 150 <= int(m.group(1)) <= 193
        else:
            return False

    def validHcl(self):
        return re.match("^#[0-9a-f]{6}$", self.fields["hcl"]) is not None

    def validEcl(self):
        valid = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
        return self.fields["ecl"] in valid

    def validPid(self):
        return re.match("^\d{9}$", self.fields["pid"]) is not None

def countValidPassports( passports ):
    count = 0
    for passport in passports:
        if passport.isValid():
            print( passport.fields )
            count += 1
    return count

def processLine( passportObj, line ):
    keyValues = line.split(' ')
    regex = re.compile("(\w+):([\w#]+)")
    for kv in keyValues:
        match = regex.match( kv )
        if match.group(1) == "cid":
            continue
        #print( "{} -> {}".format( match.group(1), match.group(2) ) )
        passportObj.fields[match.group(1)] =  match.group(2)
    

def readFile(fileName):
    passports = []
    file = open(fileName, 'r')
    lines = file.readlines()
    temp = Passport()
    for line in lines:
        if line != '\n':
            processLine(temp, line)    
        else:
            #print("blank line")
            passports.append(temp)
            temp = Passport()
    passports.append(temp)
    file.close()
    print( "{} passports imported".format(len(passports)))
    return passports

def main():
    if len( sys.argv ) != 2:
        print( "Missing arguments")
        exit(1)
    
    passports = readFile(sys.argv[1])
    count = countValidPassports(passports)
    print( "{} valid passports".format(count))
main()
