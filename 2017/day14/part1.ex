defmodule Part1 do
  # read input
  # for each number between 0 and 127 inclusive
    # calculate the knot hash of #{input}-#{n}
    # for each character in the knot hash
      # convert hex to binary (there are only 15 values, we can just hard-code if the bitwise arithmetic is challenging)
      # count the number of "on" bits
  # return the total number of "on" bits

  def countUsedFragments(input) do
    Enum.reduce(0..127, 0, &(Part1.hashSum(buildRowInput(input, &1), &2)))
  end

  def hashSum(input, acc) do
    acc + countBitsInString(hash(input))
  end

  def buildRowInput(input, n) do
    input ++ '-' ++ (to_charlist n)
  end

  def countBitsInString(repr) do
    Enum.reduce(repr, 0, &(countBitsInChar(&1) + &2))
  end

  def hash(str) do
    IO.puts "Hashing "
    IO.puts str
    :hash.solution(str)
  end

  def countBitsInChar(ch) do
    case ch do
      ?0 -> 0
      ?1 -> 1
      ?2 -> 1
      ?3 -> 2
      ?4 -> 1
      ?5 -> 2
      ?6 -> 2
      ?7 -> 3
      ?8 -> 1
      ?9 -> 2
      ?a -> 2
      ?A -> 2
      ?b -> 3
      ?B -> 3
      ?c -> 2
      ?C -> 2
      ?d -> 3
      ?D -> 3
      ?e -> 3
      ?E -> 3
      ?f -> 4
      ?F -> 4
    end
  end

  def hello do
    IO.puts "Hello World"
  end
end
