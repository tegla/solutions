local List = require 'pl.List'

local ls = List {}
for l in io.lines('/tmp/advent09.input.txt') do
    local a, b = l:match("(%d+),(%d+)")
    ls:append({ tonumber(a), tonumber(b) })
end

local max = 0
for i = 1, #ls - 1 do
    local a1, b1 = table.unpack(ls[i])
    for j = 2, #ls do
        local a2, b2 = table.unpack(ls[j])
        local s = (math.abs(a1 - a2) + 1) * (math.abs(b1 - b2) + 1)
        max = math.max(max, s)
    end
end
print(max)
