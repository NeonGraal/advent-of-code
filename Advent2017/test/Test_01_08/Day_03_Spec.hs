module Test_01_08.Day_03_Spec (spec) where

import AdventTest (runTest, runTests)
import Day_01_08.Day_03 (day_03, toCoords, spiral, coordsToSum)
import Test.Hspec (describe)

spec = do
  runTests day_03 p1 p2
  describe "coords" $ do
    runTest coords 1 []
    runTest coords 2 []
    runTest coords 4 []
    runTest coords 7 []
    runTest coords 10 []
    runTest coords 11 []
    runTest coords 12 []
    runTest coords 13 []
    runTest coords 16 []
    runTest coords 19 []
    runTest coords 21 []
    runTest coords 22 []
    runTest coords 23 []
    runTest coords 24 []
    runTest coords 25 []
  where
    coords i = case spiral i of
      t @ (s, i, a) -> coordsToSum (toCoords t) s
    p1 t = do
      t 1 0
      t 4 1
      t 12 3
      t 16 3
      t 23 2
      t 1024 31
    p2 t = do
      return ()
