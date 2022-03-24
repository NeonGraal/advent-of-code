module Day_01_08.Day_01 (day_01) where

import Advent (Advent (Advent))
import Data.Char (digitToInt)
import Data.Tuple (swap)

day_01 :: Advent String Int
day_01 = Advent "Day_01" id p1 p2
  where
    p1 s = sumZip s $ tail s ++ [head s]
    p2 s =
      let half = length s `div` 2
          halves = uncurry (++) $ swap $ splitAt half s
       in sumZip s halves

sumZip :: String -> String -> Int
sumZip z =
  let eq (a, b) = a == b
   in sum . map (digitToInt . fst) . filter eq . zip z
