import Test_01_08.Day_01
import Test_01_08.Day_02
import Test.Hspec

main :: IO ()
main = hspec $ do
    day_01_tests
    day_02_tests