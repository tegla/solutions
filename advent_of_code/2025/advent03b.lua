local List = require 'pl.List'
local Map = require 'pl.Map'
local seq = require 'pl.seq'
local func = require 'pl.func'

local sum = 0
for l in seq.lines('/tmp/advent03.input') do
    print(l)
    l = List(l:gmatch("(.)")):map(tonumber)
    local function max(p1, p2)
        local m = 0
        local p = 0
        for i = p2, p1, -1 do
            if l[i] >= m then
                m, p = l[i], i
            end
        end
        return p, m
    end
    local length = 12
    local res = List {}
    local p = 0
    while #res < length do
        local n
        p, n = max(p + 1, #l - length + #res + 1)
        res:append(n)
    end
    res = res:reduce(func._1 * 10 + func._2)
    print(res)
    sum = sum + res -- this will not work with Lua 5.1, you'll eventually go to floats
end
print(sum)
