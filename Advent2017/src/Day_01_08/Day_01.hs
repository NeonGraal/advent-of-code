module Day_01_08.Day_01 (day_01) where

import Advent (Advent (Advent))
import Data.Char (digitToInt)
import Data.Tuple (swap)

day_01 :: Advent String Int
day_01 = Advent "Day_01" id day01_part1 day01_part2

sumZip :: String -> String -> Int
sumZip z =
  let eq (a, b) = a == b
   in sum . map (digitToInt . fst) . filter eq . zip z

day01_part1 :: String -> Int
day01_part1 s =
  sumZip s $ tail s ++ [head s]

day01_part2 :: String -> Int
day01_part2 s =
  let half = length s `div` 2
      halves = uncurry (++) $ swap $ splitAt half s
   in sumZip s halves
