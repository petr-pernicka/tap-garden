# Benches
This document documents results of benchmarks before and after certain 
optimizations were made.

# SSSP
 - Benchmark file: `/benches/sssp.rs`
 - Benchmarked file: `/src/static_/sssp.rs`

1. Initial implementation
    - Commit: `4a15438e`

2. Check for stale entries
   - Commit: `4ffa82b0`
   - Improvement by 7.7% on Sioux Falls, 4% on Chicago Sketch, and 1% on
     Sydney (within noise threshold)

3. Get the allocation of heap out of the method and into the struct
   - Commit: `d8cbcfd8`
   - Improvement by 7.4% on Sioux Falls, 2.6% on Chicago Sketch, and 4% on
     Sydney

4. Get the allocation of distance and predecessor tree vectors out of the 
   method and into the struct
   - Commit: `4679a81a`
   - Improvement by 45.4% on Sioux Falls, 12.8% on Chicago Sketch, and 13.9% 
     on Sydney

5. Use crate `priority_queue` and use its method `change_priority`
   - Regression by 300% on Sioux Falls, 86% on Chicago Sketch, and 96%
     on Sydney
   - Not applied

6. Use `dary_heap` with `D = 4`
   - Uncertain results
   - Not applied