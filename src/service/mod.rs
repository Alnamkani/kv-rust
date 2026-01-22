pub mod in_memory;
pub mod interface;

#[cfg(test)]
mod tests;

pub use in_memory::InMemoryStorage;
pub use interface::Storage;
