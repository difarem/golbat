pub mod op;
pub use self::op::Op;

mod disasm;
pub use self::disasm::parse_op;

mod display;

#[cfg(test)]
mod tests;