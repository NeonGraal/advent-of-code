module Test_01_08.Day_01_Spec (spec) where

import AdventTest (runTests)
import Day_01_08.Day_01 (day_01)

spec = runTests day_01 p1 p2
  where
    p1 t = do
      t "1122" 3
      t "1111" 4
      t "1234" 0
      t "91212129" 9
    p2 t = do
      t "1212" 6
      t "1221" 0
      t "123425" 4
      t "123123" 12
      t "12131415" 4
