module Day_01_08.Day_02 where

import Advent (Advent (Advent))

day_02 :: Advent [String] Integer
day_02 = Advent "Day_02" lines p1 p2
  where
    nums = map read . words

    minmax a (x, n) = (max a x, min a n)
    diff = uncurry (-) . foldr minmax (-1, 99999) . nums
    p1 s = sum $ map diff s

    evenDiv (a, b) = if a `mod` b == 0 then a `div` b else 0
    pairs (a : l) = [(max a b, min a b) | b <- l] ++ pairs l
    pairs [] = []
    divs = sum . map evenDiv . pairs . nums
    p2 s = sum $ map divs s
