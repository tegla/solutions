local List = require 'pl.List'
local seq = require 'pl.seq'

local rs = List {}
local ids = List {}
for l in seq.lines('/tmp/advent05.input.txt') do
    local a, b = string.match(l, "^(%d+)-(%d+)$")
    if a then
        rs:append({ tonumber(a), tonumber(b) })
    end
    local id = string.match(l, "^(%d+)$")
    if id then
        ids:append(tonumber(id))
    end
end

local function is_fresh(id)
    for r in rs:iter() do
        local a, b = table.unpack(r)
        if a <= id and id <= b then
            return true
        end
    end
    return false
end

print(seq(ids):map(is_fresh):sum(function(b) if b then return 1 else return 0 end end))
