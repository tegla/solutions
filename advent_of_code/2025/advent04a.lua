local List = require 'pl.List'
local seq = require 'pl.seq'
local pretty = require 'pl.pretty'

-- matrix with 0 as default
local mtc = {
    __index = function() return 0 end
}
local mtr = {
    __index = function(t, k)
        local c = {}
        setmetatable(c, mtc)
        t[k] = c
        return c
    end
}

local m = {}
setmetatable(m, mtr)

local function directions(r, c)
    return coroutine.wrap(function()
        coroutine.yield(r - 1, c - 1)
        coroutine.yield(r - 1, c)
        coroutine.yield(r - 1, c + 1)
        coroutine.yield(r, c + 1)
        coroutine.yield(r + 1, c + 1)
        coroutine.yield(r + 1, c)
        coroutine.yield(r + 1, c - 1)
        coroutine.yield(r, c - 1)
    end)
end

local rs = List {}
for l in seq.lines('/tmp/advent04.input') do
    rs:append(List(l:gmatch(".")))
end

for r, cs in ipairs(rs) do
    for c, v in ipairs(cs) do
        if v == "@" then
            for dr, dc in directions(r, c) do
                m[dr][dc] = m[dr][dc] + 1
            end
        end
    end
end

local count = 0
for r, cs in ipairs(rs) do
    for c, v in ipairs(cs) do
        if v == "@" and m[r][c] < 4 then
            count = count + 1
        end
    end
end

print(count)
