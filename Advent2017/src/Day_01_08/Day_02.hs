module Day_01_08.Day_02 (day_02) where

import Advent (Advent (Advent))

day_02 :: Advent [String] Integer
day_02 = Advent "Day_02" lines p1 p2
  where
    nums = map read . words
    p1 s =
      let minmax a (x, n) = (max a x, min a n)
          diff = uncurry (-) . foldr minmax (-1, 99999) . nums
       in sum $ map diff s
    p2 s =
      let evenDiv (a, b) = if a `mod` b == 0 then a `div` b else 0
          pairs (a : l) = [(max a b, min a b) | b <- l] ++ pairs l
          pairs [] = []
          divs = sum . map evenDiv . pairs . nums
       in sum $ map divs s
