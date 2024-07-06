module Main where

import Text.Megaparsec
import Text.Megaparsec.Char
import Data.Void(Void)
import Data.Maybe (fromMaybe)

data Thing
    = Garbage Int
    | Group [Thing]

getScore :: Int -> Thing -> Int
getScore _ (Garbage _) = 0
getScore currScore (Group things) =
    currScore + sum (map (getScore (currScore + 1)) things)

getGarbageLength :: Thing -> Int
getGarbageLength (Garbage n) = n
getGarbageLength (Group things) = sum (map getGarbageLength things)

type Parser = Parsec Void String

thing :: Parser Thing
thing = garbage <|> group

garbage :: Parser Thing
garbage = do
    char '<'
    n <- sum <$> many (bang <|> garbageInner)
    char '>'
    return $ Garbage n

bang :: Parser Int
bang = do
    char '!'
    anySingle
    return 0

garbageInner :: Parser Int
garbageInner = length <$> some (noneOf ">!")

group :: Parser Thing
group = do
    char '{'
    things <- thing `sepBy` char ','
    char '}'
    return $ Group things

main :: IO ()
main = do
    input <- readFile "input.txt"
    let g = fromMaybe (error "Can't parse input") $
            parseMaybe (group <* char '\n') input
    putStrLn $ "score: " ++ show (getScore 1 g)
    putStrLn $ "garbage length: " ++ show (getGarbageLength g)
    
