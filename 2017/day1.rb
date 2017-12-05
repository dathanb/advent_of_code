# You're standing in a room with "digitization quarantine" written in LEDs
# along one wall. The only door is locked, but it includes a small interface.
# "Restricted Area - Strictly No Digitized Users Allowed."
# 
# It goes on to explain that you may only leave by solving a captcha to prove
# you're not a human. Apparently, you only get one millisecond to solve the
# captcha: too fast for a normal human, but it feels like hours to you.
# 
# The captcha requires you to review a sequence of digits (your puzzle input)
# and find the sum of all digits that match the next digit in the list. The
# list is circular, so the digit after the last digit is the first digit in the
# list.
#
# For example:
#
# 1122 produces a sum of 3 (1 + 2) because the first digit (1) matches the second digit and the third digit (2) matches the fourth digit.
# 1111 produces 4 because each digit (all 1) matches the next.
# 1234 produces 0 because no digit matches the next.
# 91212129 produces 9 because the only digit that matches the next one is the last digit, 9.
# What is the solution to your captcha?

input = File.read("day1.input")

def captcha(str)
  sum = str.chars.map(&:to_i).reduce({sum: 0, prev: nil}){|a,e| {sum: (e == a[:prev] ? a[:sum]+e : a[:sum]), prev: e}}[:sum]
  if str.length > 2 && str[0] == str[-1]
    sum += str[0].to_i
  end
  sum
end

puts "1122: #{captcha('1122')}"
puts "1111: #{captcha('1111')}"
puts "91212129': #{captcha('91212129')}"

puts "#{input}: #{captcha(input)}"
