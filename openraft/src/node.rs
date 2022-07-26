use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

/// Essential trait bound for node-id, except serde.
#[doc(hidden)]
pub trait NodeIdEssential:
    Sized + Send + Sync + Eq + PartialEq + Ord + PartialOrd + Debug + Display + Hash + Copy + Clone + Default + 'static
{
}

impl<T> NodeIdEssential for T where T: Sized
        + Send
        + Sync
        + Eq
        + PartialEq
        + Ord
        + PartialOrd
        + Debug
        + Display
        + Hash
        + Copy
        + Clone
        + Default
        + 'static
{
}

/// A Raft node's ID.
///
/// A `NodeId` uniquely identifies a node in the Raft cluster.
#[cfg(all(feature = "rkyv", feature = "serde"))]
mod node {
    use rkyv::ser::serializers::AllocSerializer;

    use super::NodeIdEssential;

    /// Number of bytes used as the base buffer for rkyv AllocSerializer
    const ALLOC_SERILIZER_BASE: usize = 1024;

    pub trait NodeId:
        NodeIdEssential
        + serde::Serialize
        + for<'a> serde::Deserialize<'a>
        + rkyv::Archive
        + rkyv::Serialize<AllocSerializer<ALLOC_SERILIZER_BASE>>
    {
    }

    impl<T> NodeId for T where T: NodeIdEssential
            + serde::Serialize
            + for<'a> serde::Deserialize<'a>
            + rkyv::Archive
            + rkyv::Serialize<AllocSerializer<ALLOC_SERILIZER_BASE>>
    {
    }
}

#[cfg(feature = "rkyv")]
#[cfg(not(feature = "serde"))]
mod node {
    use rkyv::ser::serializers::AllocSerializer;

    use super::NodeIdEssential;

    /// Number of bytes used as the base buffer for rkyv AllocSerializer
    const ALLOC_SERILIZER_BASE: usize = 1024;

    pub trait NodeId: NodeIdEssential + rkyv::Archive + rkyv::Serialize<AllocSerializer<ALLOC_SERILIZER_BASE>> {}

    impl<T> NodeId for T where T: NodeIdEssential + rkyv::Archive + rkyv::Serialize<AllocSerializer<ALLOC_SERILIZER_BASE>> {}
}

#[cfg(feature = "serde")]
#[cfg(not(feature = "rkyv"))]
mod node {
    use super::NodeIdEssential;

    pub trait NodeId: NodeIdEssential + serde::Serialize + for<'a> serde::Deserialize<'a> {}

    impl<T> NodeId for T where T: NodeIdEssential + serde::Serialize + for<'a> serde::Deserialize<'a> {}
}

#[cfg(not(any(feature = "serde", feature = "rkyv")))]
mod node {
    use super::NodeIdEssential;

    pub trait NodeId: NodeIdEssential {}

    impl<T> NodeId for T where T: NodeIdEssential {}
}

pub use node::NodeId;

/// Additional node information.
///
/// The most common usage is to store the connecting address of a node.
/// So that an application does not need an additional store to support its RaftNetwork implementation.
///
/// An application is also free not to use this storage and implements its own node-id to address mapping.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "rkyv", derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize))]
pub struct Node {
    pub addr: String,
    /// Other User defined data.
    pub data: BTreeMap<String, String>,
}

impl Node {
    pub fn new(addr: impl ToString) -> Self {
        Self {
            addr: addr.to_string(),
            ..Default::default()
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; ", self.addr)?;
        for (i, (k, v)) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{}:{}", k, v)?;
        }
        Ok(())
    }
}
