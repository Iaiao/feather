use derive_more::Deref;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Deref)]
pub struct TickCounter(pub usize);
