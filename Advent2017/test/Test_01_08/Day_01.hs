module Test_01_08.Day_01 where

import Test.Hspec
import Advent
import Day_01_08.Day_01

test_run p i e = 
    let descr = "'" ++ i ++ "' -> " ++ show e
        test  = p day_01 i
    in it descr $ test `shouldBe` e

day_01_tests = do
    describe "Day 01 - Part1" $ do
        test_run part1 "1122" 3
        test_run part1 "1111" 4
        test_run part1 "1234" 0
        test_run part1 "91212129" 9
    describe "Day 01 - Part2" $ do
        test_run part2 "1212" 6
        test_run part2 "1221" 0
        test_run part2 "123425" 4
        test_run part2 "123123" 12
        test_run part2 "12131415" 4
