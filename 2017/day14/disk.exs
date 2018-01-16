defmodule Disk do
  def testInput do
    [
      [ true,  true,  false, true,  false, true,  false, false ],
      [ false, true,  false, true,  false, true,  false, true  ],
      [ false, false, false, false, true,  false, true,  false ],
      [ true,  false, true,  false, true,  true,  false, true  ],
      [ false, true,  true,  false, true,  false, false, false ],
      [ true,  true,  false, false, true,  false, false, true  ],
      [ false, true,  false, false, false, true,  false, false ],
      [ true,  true,  false, true,  false, true,  true,  false ]
    ]
  end

  def loadDisk(input) do
    Enum.reduce(0..127, [], &(&2 ++ [hashToRow(hash(buildRowInput(input, &1)))]))
  end

  def hashToRow(knotHash) do
    Enum.reduce(knotHash, [], &(&2 ++ hexCharToSquares(&1)))
  end

  def buildRowInput(input, n) do
    input ++ '-' ++ (to_charlist n)
  end

  def hash(str) do
    IO.write "Hashing "
    IO.puts str
    :hash.solution(str)
  end

  def hexCharToSquares(ch) do
    case ch do
      ?0 -> [false, false, false, false]
      ?1 -> [false, false, false, true]
      ?2 -> [false, false, true,  false]
      ?3 -> [false, false, true,  true]
      ?4 -> [false, true,  false, false]
      ?5 -> [false, true,  false, true]
      ?6 -> [false, true,  true,  false]
      ?7 -> [false, true,  true,  true]
      ?8 -> [true,  false, false, false]
      ?9 -> [true,  false, false, true]
      ?a -> [true,  false, true,  false]
      ?A -> [true,  false, true,  false]
      ?b -> [true,  false, true,  true]
      ?B -> [true,  false, true,  true]
      ?c -> [true,  true,  false, false]
      ?C -> [true,  true,  false, false]
      ?d -> [true,  true,  false, true]
      ?D -> [true,  true,  false, true]
      ?e -> [true,  true,  true,  false]
      ?E -> [true,  true,  true,  false]
      ?f -> [true,  true,  true,  true]
      ?F -> [true,  true,  true,  true]
    end
  end
end
