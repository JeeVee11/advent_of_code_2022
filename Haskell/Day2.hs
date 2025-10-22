module Day2 where

import Data.List
import Data.List.Split

main :: IO ()
main = do
  input <- readFile "in2.txt"
  putStrLn $ run input

run :: String -> String
run = show . sum . map (getScore . parse) . lines

parse :: String -> Round
parse s =
  let ws = words s
      opponent = parseChoice (ws !! 0)
      me = parseChoice (ws !! 1)
   in Round {opponent, me}

data Round = Round {opponent :: Choice, me :: Choice}

data Choice = Rock | Paper | Scissor
  deriving (Eq, Show)

parseChoice s = case s of
  "A" -> Rock
  "X" -> Rock
  "B" -> Paper
  "Y" -> Paper
  "C" -> Scissor
  "Z" -> Scissor

getScore (Round opponent me) =
  let choice = case me of
        Rock -> 1
        Paper -> 2
        Scissor -> 3
      won
        | opponent == me = 3
        | beats opponent me = 0
        | otherwise = 6
   in choice + won

beats Paper Rock = True
beats Rock Scissor = True
beats Scissor Paper = True
beats _ _ = False
