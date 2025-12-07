local List = require 'pl.List'
local utils = require 'pl.utils'

local ls = List(utils.readlines('/tmp/advent07.input.txt')):map(string.gmatch, "."):map(List)

for i = 1, #ls do
    for j = 1, #ls[i] do
        if ls[i][j] == '.' then
            ls[i][j] = 0
        end
    end
end

for i = 1, #ls - 1 do
    for j = 1, #ls[i] do
        local c1 = ls[i][j]
        local is_beam = type(c1) == "number"
        local c2 = ls[i + 1][j]
        if is_beam and c2 == '^' then
            ls[i + 1][j - 1] = ls[i + 1][j - 1] + c1
            ls[i + 1][j + 1] = ls[i + 1][j + 1] + c1
        elseif c1 == 'S' then
            ls[i + 1][j] = 1
        elseif is_beam then
            ls[i + 1][j] = ls[i + 1][j] + c1
        end
    end
end
print(ls[#ls]:reduce('+'))
