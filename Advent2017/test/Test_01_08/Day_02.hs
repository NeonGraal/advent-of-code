module Test_01_08.Day_02 (day_02_tests) where

import Advent
import Day_01_08.Day_02
import Test.Hspec

test_run p i e =
  let descr = "'" ++ (show i) ++ "' -> " ++ (show e)
      test = p day_02 i
   in it descr $ test `shouldBe` e

sample1 = lines "5 1 9 5\n7 5 3\n2 4 6 8"

sample2 = lines "5 9 2 8\n9 4 7 3\n3 8 6 5"

day_02_tests = do
  describe "Day 02 - Part1" $ do
    test_run part1 sample1 18
  describe "Day 02 - Part2" $ do
    test_run part2 sample2 9
