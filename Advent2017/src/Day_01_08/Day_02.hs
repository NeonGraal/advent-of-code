module Day_01_08.Day_02 (day_02) where

import Advent

day_02 = Advent "Day_02" lines day02_part1 day02_part2

nums :: String -> [Int]
nums = map read . words

day02_part1 :: [String] -> Int
day02_part1 s =
  let minmax a (x, n) = (max a x, min a n)
      diff = uncurry (-) . foldr minmax (-1, 99999) . nums
   in sum $ map diff s

day02_part2 :: [String] -> Int
day02_part2 s =
  let evenDiv (a, b) = if a `mod` b == 0 then a `div` b else 0
      pairs (a : l) = [(max a b, min a b) | b <- l] ++ pairs l
      pairs [] = []
      divs = sum . map evenDiv . pairs . nums
   in sum $ map divs s
