module AdventTest where

import Advent
import Test.Hspec

runTests ::
  (Show i, Eq o, Show o) =>
  Advent i o ->
  ((i -> o -> SpecWith ()) -> SpecWith ()) ->
  ((i -> o -> SpecWith ()) -> SpecWith ()) ->
  SpecWith ()
runTests d p1 p2 =
  let descr i e = "'" ++ (show i) ++ "' -> " ++ (show e)
      test p i e = it (descr i e) $ p d i `shouldBe` e
   in do
        describe (day d ++ " - Part 1") $ p1 (test part1)
        describe (day d ++ " - Part 2") $ p2 (test part2)
