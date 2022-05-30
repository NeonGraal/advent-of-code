module Day_01_08.Day_03 where

import Advent (Advent (Advent))
import Control.Arrow ((***))
import Control.Monad (join)
import Debug.Trace (trace)
import GHC.Float (sqrtDouble)

day_03 :: Advent Int Int
day_03 = Advent "Day_03" read p1 p2
  where
    p1 s = case spiral s of
      (_, inward, along) -> inward + abs along
    p2 s = s

sideCells =
  [ (0, [(-1, 1), (0, 1), (1, 1), (-1, 0)]),
    (1, [(1, -1), (1, 0), (1, 1), (0, 1)]),
    (2, [(-1, -1), (0, -1), (1, -1), (1, 0)]),
    (3, [(-1, 1), (-1, 0), (-1, 1), (0, -1)])
  ]

toCoords :: (Int, Int, Int) -> (Int, Int)
toCoords (s, i, a) = case s of
  0 -> (- a, - i)
  1 -> (- i, a)
  2 -> (a, i)
  3 -> (i, - a)
  _ -> error $ "Invalid side: " ++ show s

spiral :: Int -> (Int, Int, Int)
spiral 1 = (0, 0, 0)
spiral i = debug $ (side, inward, along)
  where
    root = ceiling . sqrt . fromIntegral $ i
    range = if odd root then root else root + 1
    inward = (range - 2) `div` 2 + 1
    edge = range ^ 2 - i
    side = edge `div` (range - 1)
    place = edge `mod` (range - 1)
    along = place - (range `div` 2)
    labeled l v = l ++ show v
    debug =
      trace
        ( labeled "i:" i
            ++ labeled " r:" range
            ++ labeled " e:" edge
            ++ labeled " s:" side
            ++ labeled " p:" place
        )

{-
spiralSum 1 = 1
spiralSum i = coordsSum $ toCoords i
-}

coordsToSum :: (Int, Int) -> Int -> [(Int, Int)]
coordsToSum (x, y) s = maybe [] (map cellAdd) cells
  where
    cells = lookup s sideCells
    cellAdd (a, b) = (a + x, b + y)
