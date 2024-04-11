pub mod matrix;

#[cfg(feature = "vector_constgenerics")]
pub mod vector_constgenerics;
#[cfg(feature = "vector_constgenerics")]
pub use vector_constgenerics as vector;

#[cfg(not(feature = "vector_constgenerics"))]
pub mod vector_hardcoded;
#[cfg(not(feature = "vector_constgenerics"))]
pub use vector_hardcoded as vector;
