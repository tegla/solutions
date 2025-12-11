List = require 'pl.List'
Map = require 'pl.Map'

local g = Map {}
for l in io.lines('/tmp/advent11.input.txt') do
    local a, bs = l:match("(.*): (.*)")
    bs = List(bs:gmatch("[a-z]+"))
    print(a, bs)
    g[a] = bs
end

local rs = Map { you = 1 }
local count = 0
while rs:len() > 0 do
    local rsc = Map()
    for a, c in pairs(rs) do
        if a == "out" then
            count = count + c
        else
            for b in g[a]:iter() do
                if not rsc[b] then
                    rsc[b] = 0
                end
                rsc[b] = rsc[b] + c
            end
        end
    end
    print(rsc)
    rs = rsc
end
print(count)
