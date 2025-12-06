local seq = require 'pl.seq'
local List = require 'pl.List'

local nss = List {}
local os = nil
for l in seq.lines('/tmp/advent06.input.txt') do
    local ns = List(l:gmatch("%d+"))
    if #ns > 0 then
        nss:append(ns)
    else
        os = List(l:gmatch("[^ ]"))
    end
end

local sum = 0
for i = 1, #nss[1] do
    local s = seq.reduce(os[i], seq.map(function(j) return nss[j][i] end, seq.range(1, #nss)))
    sum = sum + s
end
print(sum)
