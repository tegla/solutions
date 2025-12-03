local List = require 'pl.List'
local Map = require 'pl.Map'
local seq = require 'pl.seq'
local func = require 'pl.func'

local sum = 0
for l in seq.lines('/tmp/advent03.input') do
    print(l)
    l = List(l:gmatch("(.)")):map(tonumber)
    local p1 = 0
    local tens = 0
    for p = #l - 1, 1, -1 do
        if l[p] >= tens then
            p1, tens = p, l[p]
        end
    end
    local ones = 0
    for p = p1 + 1, #l do
        if l[p] > ones then
            ones = l[p]
        end
    end
    print(tens * 10 + ones)

    sum = sum + tens * 10 + ones
end
print(sum)
