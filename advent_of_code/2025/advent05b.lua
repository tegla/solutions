local List = require 'pl.List'
local seq = require 'pl.seq'

local rs = List {}
for l in seq.lines('/tmp/advent05.input.txt') do
    local a, b = string.match(l, "^(%d+)-(%d+)$")
    if a then
        rs:append({ a = tonumber(a), b = tonumber(b) })
    end
end


rs:sort(function(l, r)
    return l.a < r.a
end)

local seen = 0
local count = 0
for r in rs:iter() do
    local a, b = r.a, r.b
    if seen < a then
        seen = a - 1
    end
    if seen < b then
        count = count + b - seen
        seen = r.b
    end
end

print(count)
