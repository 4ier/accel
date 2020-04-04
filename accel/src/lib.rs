//! GPGPU framework for Rust based on [CUDA Driver API]
//!
//! [CUDA Driver API]: https://docs.nvidia.com/cuda/cuda-driver-api/

extern crate cuda_driver_sys as cuda;

pub mod array;
pub mod device;
pub mod error;
pub mod linker;
pub mod memory;
pub mod module;
pub mod stream;

pub use device::{Context, Device};

/// Size of Block (thread block) in [CUDA thread hierarchy]( http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#programming-model )
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Block {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Block {
    /// one-dimensional
    pub fn x(x: u32) -> Self {
        Block { x: x, y: 1, z: 1 }
    }

    /// two-dimensional
    pub fn xy(x: u32, y: u32) -> Self {
        Block { x: x, y: y, z: 1 }
    }

    /// three-dimensional
    pub fn xyz(x: u32, y: u32, z: u32) -> Self {
        Block { x: x, y: y, z: z }
    }
}

/// Size of Grid (grid of blocks) in [CUDA thread hierarchy]( http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#programming-model )
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Grid {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Grid {
    /// one-dimensional
    pub fn x(x: u32) -> Self {
        Grid { x: x, y: 1, z: 1 }
    }

    /// two-dimensional
    pub fn xy(x: u32, y: u32) -> Self {
        Grid { x: x, y: y, z: 1 }
    }

    /// three-dimensional
    pub fn xyz(x: u32, y: u32, z: u32) -> Self {
        Grid { x: x, y: y, z: z }
    }
}
