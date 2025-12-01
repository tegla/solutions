local s = 50
local count = 0
for l in io.lines('/tmp/advent01.input') do
    local d, n = l:match("(%a)(%d+)")
    n = tonumber(n)
    for _ = 1, n do
        if d == "L" then
            s = s - 1
        else
            s = s + 1
        end
        s = s % 100
        if s == 0 then
            count = count + 1
        end
    end
    print(d, n, s, count)
end

print(count)
