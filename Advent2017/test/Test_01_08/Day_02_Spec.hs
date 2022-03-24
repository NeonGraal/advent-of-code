module Test_01_08.Day_02_Spec (spec) where

import AdventTest (runTests)
import Day_01_08.Day_02 (day_02)

sample1 = lines "5 1 9 5\n7 5 3\n2 4 6 8"

sample2 = lines "5 9 2 8\n9 4 7 3\n3 8 6 5"

spec = runTests day_02 t1 t2
  where
    t1 t = t sample1 18
    t2 t = t sample2 9