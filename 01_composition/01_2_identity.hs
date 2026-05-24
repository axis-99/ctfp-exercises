-- 恒等射
-- id::a -> a
-- id x = x

main::IO()
main = do
  let f x = x * 2

-- left unit
  print $ (id . f) 5

-- right unit
  print $ (f . id) 5 

  print $ f 5
