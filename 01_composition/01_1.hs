-- f::A->B
-- g::B->C
--
-- g_after_f :: A -> C
-- g_after_f a = g . f

double :: Int -> Int
double x = x * 2

addOne :: Int -> Int
addOne x = x + 1

-- 合成
doubleThenAdd :: Int -> Int
doubleThenAdd = addOne . double

main :: IO ()
main = do
  print (doubleThenAdd 3)   -- (3*2)+1 = 7
  print (doubleThenAdd 10)  -- (10*2)+1 = 21
