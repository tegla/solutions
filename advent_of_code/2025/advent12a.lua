local List = require 'pl.List'
local Map = require 'pl.Map'
local stringx = require 'pl.stringx'

local shapes = List {}
local questions = List {}
for l in io.lines('/tmp/advent11.input.txt') do
    if l:find('x') then
        local a, b, r = l:match("(%d+)x(%d+): (.*)")
        r = List(r:gmatch("%d+"))
        questions:append(Map { a = tonumber(a), b = tonumber(b), r = r:map(tonumber) })
    elseif l:find(':') then
        shapes:append(0)
    elseif l:find('#') then
        shapes[#shapes] = shapes[#shapes] + stringx.count(l, "#")
    end
end

local count = 0
for q in questions:iter() do
    local area = q.a * q.b
    local min = 0
    for i, s in ipairs(q.r) do
        min = min + shapes[i] * s
    end
    if min <= area then
        count = count + 1
    end
end
print(count)
