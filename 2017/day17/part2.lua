spinlock = {}
spinlock.table_size = 1
spinlock.second_element = nil
spinlock.position = 0 -- but tables are 1-indexed, so we'll have to do some wrangling
spinlock.step_size = tonumber(io.read("*number"))
spinlock.num = 0

function step(spinlock)
  spinlock.position = (spinlock.position + spinlock.step_size) % spinlock.table_size
  spinlock.num = spinlock.num + 1

  if (spinlock.position == 0) then
    spinlock.second_element = spinlock.num
    io.write("New second element: ")
    io.write(spinlock.second_element)
    io.write("\n")
  end

  spinlock.table_size = spinlock.table_size + 1
  spinlock.position = (spinlock.position + 1) % spinlock.table_size
end

function circular_index(t, i)
  return i % #t
end

function to_table_index(zero_based_index)
  return zero_based_index + 1
end

function print_table(t)
  io.write("{")
  for k,v in ipairs(t) do
    io.write(v)
    io.write(",")
  end
  io.write("}\n")
end

--[[
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
step(spinlock)
print(spinlock.second_element)
--]]

---[[
for i = 1,50000000 do
  step(spinlock)
end

print(spinlock.second_element)
--]]
