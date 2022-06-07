# Implementations

 - Rust CPU Naive
    - ADD Blocked
    - ADD Threaded
    - ADD Cache stuff? (from 344)
    - ADD SIMD


 - Blocking for parallelism  
 - Blocking for cache  
 - Packing data  
 - Blocking for register usage  
 - Microkernel (usually a small implementation of outer product in intrinsics or inline assembly)  

 - CUDA
    - CUDA Naive
    - CUDA (look at cublas impl)

# Exisitng Implementations

 - Rust
    - ndarray
    - nalgebra


 - C++
    - numpy
    - vecLib (mac)
        - https://developer.apple.com/documentation/accelerate/veclib

 - CUDA
    - cuBLAS
    - cuDNN (Uses cuBLAS, so I will not test here)

 - ARM
    - ARM HPC BLAS

