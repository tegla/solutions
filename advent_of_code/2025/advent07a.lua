local List = require 'pl.List'
local utils = require 'pl.utils'
local L = utils.string_lambda

local ls = List(utils.readlines('/tmp/advent07.input.txt')):map(string.gmatch, "."):map(List)

local count = 0
for i = 1, #ls - 1 do
    for j = 1, #ls[i] do
        local c1 = ls[i][j]
        local c2 = ls[i + 1][j]
        if c1 == '|' and c2 == '^' then
            count = count + 1
            ls[i + 1][j - 1] = '|'
            ls[i + 1][j + 1] = '|'
        elseif c1 == 'S' or c1 == '|' then
            ls[i + 1][j] = '|'
        end
    end
end
print(count)
