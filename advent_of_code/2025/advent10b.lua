local List = require 'pl.List'
local seq = require 'pl.seq'
local Map = require 'pl.Map'
local utils = require 'pl.utils'

local ms = List {}
for l in io.lines('/tmp/advent10.input.txt') do
    local m = Map { joltages = List {}, buttons = List {} }
    for b in l:gmatch("[(]([^)]+)") do
        m.buttons:append(seq.copy(b:gmatch("%d+")):map(tonumber))
    end
    m.joltages = seq.copy(l:match("{(.*)}"):gmatch("%d+")):map(tonumber)
    ms:append(m)
end

local sum = 0
for m in ms:iter() do
    print(m)
    local combos = List {}
    for i = 0, (1 << #m.buttons) - 1 do
        local dejolt = m.joltages:map(function() return 0 end)
        local buttons = List {}
        for bp, b in ipairs(m.buttons) do
            if (i >> (bp - 1)) & 1 == 1 then
                buttons:append(b)
                for jp in b:iter() do
                    dejolt[jp + 1] = dejolt[jp + 1] + 1
                end
            end
        end
        combos:append(Map { buttons = buttons, dejolt = dejolt })
    end

    local min_pushes
    local function _min_pushes(joltages)
        local allzero = true
        for j in joltages:iter() do
            if j ~= 0 then
                allzero = false
            end
        end
        if allzero then
            return 0
        end
        local min = nil
        for c in combos:iter() do
            for j = 1, #c.dejolt do
                if joltages[j] < c.dejolt[j] or (joltages[j] - c.dejolt[j]) & 1 == 1 then
                    goto end_combo
                end
            end

            local jc = joltages:clone()
            for j = 1, #c.dejolt do
                jc[j] = (jc[j] - c.dejolt[j]) >> 1
            end
            local mp = min_pushes(jc)
            if mp then
                mp = mp * 2 + #c.buttons
                if not min or mp < min then
                    min = mp
                end
            end

            ::end_combo::
        end
        return min
    end
    min_pushes = utils.memoize(_min_pushes)
    local min = min_pushes(m.joltages)
    print(min)
    sum = sum + min
end
print(sum)
