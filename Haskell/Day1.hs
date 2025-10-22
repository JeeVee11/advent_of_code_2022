module Day1 where

import Data.List
import Data.List.Split

main :: IO ()
main = do
  input <- readFile "in1.txt"
  let out = run input
  putStrLn out

run :: String -> String
run input =
  let elves = splitOn "\n\n" input
      scores = map calc elves
      top3 = sum . take 3 . reverse . sort $ scores
   in show top3

calc :: String -> Int
calc = sum . map read . lines
