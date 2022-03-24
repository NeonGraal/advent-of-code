module Day_01_08.Day_01 where

import Advent (Advent (Advent))
import Data.Char (digitToInt)
import Data.Tuple (swap)

day_01 :: Advent String Int
day_01 = Advent "Day_01" id p1 p2
  where
    eq = uncurry (==)
    toInt = digitToInt . fst
    sumZip z = sum . map toInt . filter eq . zip z
    p1 s = sumZip s $ tail s ++ [head s]
    p2 s =
      let half = length s `div` 2
          halves = uncurry (++) $ swap $ splitAt half s
       in sumZip s halves
