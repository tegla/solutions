List = require 'pl.List'
Map = require 'pl.Map'

local g = Map {}
for l in io.lines('/tmp/advent11.input.txt') do
    local a, bs = l:match("(.*): (.*)")
    bs = List(bs:gmatch("[a-z]+"))
    print(a, bs)
    g[a] = bs
end

-- there's no Map in lua with tuples as keys
local function make_state(server, fft, dac)
    if fft then
        fft = "#"
    else
        fft = "_"
    end
    if dac then
        dac = "#"
    else
        dac = "_"
    end
    return server .. fft .. dac
end

local function unpack_state(state)
    local a, b, c = state:match("(...)(.)(.)")
    return a, b == "#", c == "#"
end

local rs = Map {}
rs[make_state("svr", false, false)] = 1

local count = 0
while rs:len() > 0 do
    local rsc = Map()
    for st, c in pairs(rs) do
        local a, fft, dac = unpack_state(st)
        if a == "out" then
            if fft and dac then
                count = count + c
            end
        else
            for b in g[a]:iter() do
                local st2 = make_state(b, fft or a == "fft", dac or a == "dac")
                if not rsc[st2] then
                    rsc[st2] = 0
                end
                rsc[st2] = rsc[st2] + c
            end
        end
    end
    print(rsc)
    rs = rsc
end
print(count)
