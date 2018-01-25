spinlock = {}
spinlock.table = {0}
spinlock.position = 0 -- but tables are 1-indexed, so we'll have to do some wrangling
spinlock.step_size = tonumber(io.read("*number"))
spinlock.num = 0

print(spinlock.step_size)

function step(spinlock)
  spinlock.position = circular_index(spinlock.table, spinlock.position + spinlock.step_size)
  spinlock.num = spinlock.num + 1
  print(spinlock.position)

  table.insert(spinlock.table, to_table_index(spinlock.position + 1), spinlock.num)

  spinlock.position = (spinlock.position + 1) % #spinlock.table
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

t = {[0] = 0}
print(#t+1)

--[[
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
step(spinlock)
print_table(spinlock.table)
--]]

---[[
for i = 1,2017 do
  print(i)
  step(spinlock)
end

print(spinlock.table[(spinlock.position + 2) % #spinlock.table])
--]]
