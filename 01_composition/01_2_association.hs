-- 3つの関数
f :: Int -> Int
f x = x + 1

g :: Int -> Int
g x = x * 2

h :: Int -> Int
h x = x * x

main :: IO ()
main = do
  let x = 3

  -- 結合性：どう括っても同じ
  print $ (h . (g . f)) x   -- h(g(f(x)))
  print $ ((h . g) . f) x   -- 同上
