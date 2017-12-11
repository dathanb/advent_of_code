{-|
Read an initial state; or I might cheat and hard-code the input
That initial state is just N integers.
We insert that state into a set
Then we redistribute (it'll be interesting to figure out how to do that in Haskell
Then we add that to the set and recurse
We return the step count when that state has been seen before
-}

import qualified Data.Map as Map
import Data.List
import Data.Maybe

-- | My AoC day 6 input
state = [2, 8, 8, 5, 4, 2, 3, 1, 5, 5, 1, 2, 15, 13, 5, 14]
--state = [0,2,7,0]

history = Map.empty

cycleLength :: [Int] -> (Map.Map [Int] Int) -> Int -> Int
redistribute :: [Int] -> [Int]
distribute :: Int -> [Int] -> Int -> [Int]
replace :: [a] -> Int -> a -> [a]

cycleLength state history counter
  | (Map.member state history) == True = if isJust location then counter - (fromJust location) else error "Expected Just but got Nothing" 
  | otherwise                          = cycleLength (redistribute state) (Map.insert state counter history) (counter + 1)
    where location = Map.lookup state history


redistribute state
  | isJust maybeIndex = (distribute count startState index)
  | otherwise = error "expected Just index, but got Nothing"
  where count = maximum state
        maybeIndex = Data.List.elemIndex count state
        index = fromJust maybeIndex
        startState = replace state index 0

distribute count state index
  | count == 0 = state
  | otherwise = distribute newCount newState newIndex
  where newIndex = (index+1) `mod` (length state)
        newCount = count - 1
        newState = replace state newIndex ((state!!newIndex) + 1)

replace collection index newItem =
  (take index collection) ++ [newItem] ++ (drop (index+1) collection)

steps = cycleLength state history 0
