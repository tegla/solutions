local stringx = require 'pl.stringx'
local pretty = require 'pl.pretty'
local utils = require 'pl.utils'
local List = require 'pl.List'
local Set = require 'pl.Set'

local rs = List {}
for r in stringx.split(List(utils.readlines('/tmp/advent02.input')):join(), ","):iter() do
    local a, b = r:match("(%d+)-(%d+)")
    rs:append({ a = tonumber(a), b = tonumber(b) })
end

local max = 0
for r in rs:iter() do
    if r.b > max then
        max = r.b
    end
end

local found = Set {}
local i = 1
local sum = 0
while tonumber(i .. i) <= max do
    local id = tonumber(i .. i)
    while id <= max do
        if not found[id] then
            for r in rs:iter() do
                if id >= r.a and id <= r.b then
                    found[id] = true
                    sum = sum + id
                    break
                end
            end
        end
        id = tonumber(id .. i)
    end
    i = i + 1
end

print(sum)
