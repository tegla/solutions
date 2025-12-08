local List = require 'pl.List'
local Set = require 'pl.Set'
local Map = require 'pl.Map'

local ls = List(io.lines('/tmp/advent08.input.txt')):map(string.gmatch, "%d+"):map(List)

local function dist_squared(p1, p2)
    return (p2[1] - p1[1]) * (p2[1] - p1[1]) + (p2[2] - p1[2]) * (p2[2] - p1[2]) + (p2[3] - p1[3]) * (p2[3] - p1[3])
end

print("start")
local ds = List {}
for i = 1, #ls - 1 do
    for j = i + 1, #ls do
        ds:append({ i, j, dist_squared(ls[i], ls[j]) })
    end
end
ds:sort(function(a, b) return a[3] < b[3] end)
local junction_map = Map {}

for d = 1, 1000 do
    local i, j = table.unpack(ds[d])
    print("connecting", i, j)
    local ji = junction_map[i]
    if not ji then
        ji = Set { i }
        junction_map[i] = ji
    end
    local jj = junction_map[j]
    if not jj then
        jj = Set { j }
        junction_map[j] = jj
    end
    if ji ~= jj then
        local u = ji + jj
        for k in pairs(u) do
            junction_map[k] = u
        end
    end
end
print("end", #junction_map)

local js = Set.values(Set(junction_map:values()))
js:sort(function(a, b) return #a > #b end)
for j in js:iter() do
    print(j)
end
print(#js[1] * #js[2] * #js[3])
