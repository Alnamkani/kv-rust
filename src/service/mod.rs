pub mod error;
pub mod in_memory;
pub mod interface;

#[cfg(test)]
mod tests;

pub use error::StorageError;
pub use in_memory::InMemoryStorage;
pub use interface::Storage;
