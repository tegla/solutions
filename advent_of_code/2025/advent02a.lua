local stringx = require 'pl.stringx'
local pretty = require 'pl.pretty'
local utils = require 'pl.utils'
local List = require 'pl.List'

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

local i = 1
local sum = 0
while tonumber(i .. i) <= max do
    local id = tonumber(i .. i)
    for r in rs:iter() do
        if id >= r.a and id <= r.b then
            sum = sum + id
            break
        end
    end

    i = i + 1
end

print(sum)
