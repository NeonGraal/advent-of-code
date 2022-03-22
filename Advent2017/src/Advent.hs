module Advent where
    
data Advent i o = Advent
    { day :: String
    , parse :: String -> i
    , part1 :: i -> o
    , part2 :: i -> o
    }

run :: Show o => Advent i o -> IO ()
run a = do
    let d = day a
    putStrLn $ "Advent 2017 - " ++ d
    contents <- readFile $ "input/" ++ d ++ ".input"
    let input = parse a contents
    putStr $ "  " ++ d ++ " Part1: "
    print $ part1 a input
    putStr $ "  " ++ d ++ " Part2: "
    print $ part2 a input
