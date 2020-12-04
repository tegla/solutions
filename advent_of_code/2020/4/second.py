import input
import re

yrPattern = re.compile('\d{4}')
def yrValid(min, max, yr):
    if not yrPattern.fullmatch(yr):
        return False
    yr = int(yr)
    return yr >= min and yr <= max

def byrValid(byr):
    return yrValid(1920, 2002, byr)

assert not byrValid('a')
assert not byrValid('02000')
assert byrValid('2000')
assert not byrValid('2020')

def iyrValid(iyr):
    return yrValid(2010, 2020, iyr)

def eyrValid(eyr):
    return yrValid(2020, 2030, eyr)

hgtPattern = re.compile('(\d+)(in|cm)')
def hgtValid(hgt):
    m = hgtPattern.fullmatch(hgt)
    if not m:
        return False
    h = int(m.group(1))
    if m.group(2) == 'in':
        return h>=59 and h <= 76
    else:
        return h>=150 and h<= 193

assert not hgtValid('124ft')
assert not hgtValid('193in')
assert hgtValid('193cm')

hclPattern = re.compile('#[0-9a-f]{6}')
def hclValid(hcl):
    return hclPattern.fullmatch(hcl)

assert not hclValid('#')
assert not hclValid('#0000000')
assert not hclValid('#g00000')
assert hclValid('#123abc')

eclPattern = re.compile('(amb|blu|brn|gry|grn|hzl|oth)')
def eclValid(ecl):
    return eclPattern.fullmatch(ecl)

assert not eclValid('xxx')
assert eclValid('brn')

pidPattern = re.compile('\d{9}')
def pidValid(pid):
    return pidPattern.fullmatch(pid)

assert not pidValid('12345678')
assert pidValid('012345678')

validators = {
   'byr': byrValid,
   'iyr': iyrValid,
   'eyr': eyrValid,
   'hgt': hgtValid,
   'hcl': hclValid,
   'ecl': eclValid,
   'pid': pidValid, 
}

def pValid(p):
    for k,v in validators.items():
        if not k in p:
            return False
        if not v(p[k]):
            return False
    return True

valid = 0
for p in input.ps():
    if pValid(p):
        valid+=1

print(valid)