local List = require 'pl.List'
local Map = require 'pl.Map'
local seq = require 'pl.seq'
local func = require 'pl.func'

local rs = List {}
for l in seq.lines('/tmp/advent02.input') do
    for a, b in l:gmatch("(%d+)-(%d+)") do
        rs:append({ a = tonumber(a), b = tonumber(b) })
    end
end

local _, max = seq(rs):map(func._1.b):minmax()

local function in_range(id)
    for r in rs:iter() do
        if id >= r.a and id <= r.b then
            return true
        end
    end
    return false
end

local found = Map {}
local i = 1
while tonumber(i .. i) <= max do
    local id = tonumber(i .. i)
    while id <= max do
        if in_range(id) then
            found[id] = true
        end
        id = tonumber(id .. i)
    end
    i = i + 1
end

print(found:keys():reduce('+'))
