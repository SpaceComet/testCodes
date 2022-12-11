Benchmark comparing the ndarray dot function with a scalar code 

Latest results:
```
Running 100000 tests with an array of 21613 elements...
--- Scalar ---
Elapsed (μs): 23.19781

--- Scalar Rayon ---
Elapsed (μs): 43.91314

--- Vectorized ndarray ---
Elapsed (μs): 62.98
```