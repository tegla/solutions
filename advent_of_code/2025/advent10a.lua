local List = require 'pl.List'
local seq = require 'pl.seq'
local Map = require 'pl.Map'

local ms = List {}
for l in io.lines('/tmp/advent10.input.txt') do
    local light, buttons = l:match('[[](.*)[]] (.*)')
    local m = Map { lights = 0, buttons = List {} }
    light = seq.copy(light:gmatch(".")):reverse()
    for c in light:iter() do
        m.lights = m.lights << 1
        if c == '#' then
            m.lights = m.lights + 1
        end
    end
    m.buttons = List {}
    for b in buttons:gmatch("[(]([^)]+)") do
        local n = 0
        for i in seq.copy(b:gmatch("%d+")):map(tonumber):iter() do
            n = n + (1 << i)
        end
        m.buttons:append(n)
    end
    ms:append(m)
end

local sum = 0
for m in ms:iter() do
    local function minswitch(l, i)
        if l == m.lights then
            return 0
        end
        if i > #m.buttons then
            return 10000000
        end
        return math.min(
            minswitch(l, i + 1),
            minswitch(l ~ m.buttons[i], i + 1) + 1
        )
    end
    sum = sum + minswitch(0, 1)
end
print(sum)
