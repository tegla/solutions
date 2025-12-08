local List = require 'pl.List'
local Map = require 'pl.Map'

local ls = List {}
local mr = 0
for l in io.lines('/tmp/advent09.input.txt') do
    local c, r = l:match("(%d+),(%d+)")
    r, c = tonumber(r), tonumber(c)
    ls:append({ r, c })
    mr = math.max(r, mr)
end
ls:append(ls[1])

local m = List {}
for _ = 1, mr do
    m:append(List {})
end

-- Go through the sqiggle
for i = 1, #ls - 1 do
    local r1, c1 = table.unpack(ls[i])
    local r2, c2 = table.unpack(ls[i + 1])
    if c1 == c2 then
        assert(r1 ~= r2)
        if r1 < r2 then
            -- assuming right chirality because of lazy programmer syndrome
            for r = r1, r2 do
                m[r]:append(Map { pos = c1, ending = true })
            end
        else
            for r = r1, r2, -1 do
                m[r]:append(Map { pos = c1, starting = true })
            end
        end
    end
end

-- Pack columns
for r, cs in ipairs(m) do
    cs:sort(function(a, b) return a.pos < b.pos end)
    local i = 1
    -- print(r, cs)
    while i < #cs do
        if cs[i].starting and cs[i + 1].starting then
            cs:remove(i + 1)
        elseif cs[i].ending and cs[i + 1].ending then
            cs:remove(i + 1)
        else
            i = i + 1
        end
    end
    i = 1
    while i < #cs do
        assert(cs[i].starting and cs[i + 1].ending)
        cs:insert(i, List { cs[i].pos, cs[i + 1].pos })
        cs:remove(i + 1)
        cs:remove(i + 1)
        i = i + 1
    end
end

-- Pack rows
local rs = List {}
for r, cs in ipairs(m) do
    if #rs == 0 then
        rs:append(List { r, r, cs })
    else
        if cs == rs[#rs][3] then
            rs[#rs][2] = r
        else
            rs:append(List { r, r, cs })
        end
    end
end


-- Find the max block
local max = 0
for i = 1, #ls - 1 do
    for j = i + 1, #ls do
        local r1, c1 = table.unpack(ls[i])
        local r2, c2 = table.unpack(ls[j])
        if r1 > r2 then
            r1, r2 = r2, r1
        end
        if c1 > c2 then
            c1, c2 = c2, c1
        end

        for rr in rs:iter() do
            if math.max(rr[1], r1) <= math.min(rr[2], r2) then
                for c in rr[3]:iter() do
                    if c[1] <= c1 and c[2] >= c2 then
                        goto row_fits
                    end
                end
                goto not_fit
                ::row_fits::
            end
        end

        local s = (r2 - r1 + 1) * (c2 - c1 + 1)
        max = math.max(max, s)
        ::not_fit::
    end
end
print(max)
