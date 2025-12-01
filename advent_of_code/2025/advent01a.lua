local s = 50
local count = 0
for l in io.lines('/tmp/advent01.input') do
    local d, n = l:match("(%a)(%d+)")
    n = tonumber(n)
    if d == "L" then
        s = s - n
    else
        s = s + n
    end
    s = s % 100
    print(d, n, s)
    if s == 0 then
        count = count + 1
    end
end

print(count)
