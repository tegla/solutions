local seq = require 'pl.seq'
local List = require 'pl.List'
local utils = require 'pl.utils'
local L = utils.string_lambda

local ls = List(utils.readlines('/tmp/advent06.input.txt'))
local os = ls:pop()

local vs = {}
for l in ls:iter() do
    for i, n in seq.enum(l:gmatch(".")) do
        if n ~= " " then
            if vs[i] then
                vs[i] = vs[i] * 10
            end
            if not vs[i] then
                vs[i] = 0
            end
            vs[i] = vs[i] + tonumber(n)
        end
    end
end

local sum = 0
for i, o in seq.enum(os:gmatch(".")) do
    if o == ' ' then
        goto continue
    end

    local j = i
    local vl = List {}
    while vs[j] do
        vl:append(vs[j])
        j = j + 1
    end
    sum = sum + vl:reduce(o)

    ::continue::
end

print(sum)
