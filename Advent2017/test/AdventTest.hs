module AdventTest where

import Advent
import Test.Hspec

type AdventTest i o = (i -> o -> SpecWith ()) -> SpecWith ()

runTest :: (Show i, Eq o, Show o) => (i -> o) -> i -> o -> SpecWith ()
runTest t i e = it descr $ t i `shouldBe` e
  where
    descr = "'" ++ show i ++ "' -> " ++ show e

runTests ::
  (Show i, Eq o, Show o) =>
  Advent i o ->
  AdventTest i o ->
  AdventTest i o ->
  Spec
runTests d p1 p2 = do
  describe (day d ++ " - Part 1") $ p1 (runTest $ part1 d)
  describe (day d ++ " - Part 2") $ p2 (runTest $ part2 d)
