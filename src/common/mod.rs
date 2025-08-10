//! Shared types.
mod str;

use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Formatter};
use std::io::Result as Res;
use std::io::{Read, Write};

use endio::{Deserialize, LE, LERead, LEWrite, Serialize};
use index_vec::{Index, IndexVec};

pub use self::str::*;

/**
    Wraps a `Vec` with a length type so the vector can be (de-)serialized.

    Note: the length type is not checked and the `Vec` still uses `usize` internally. Handle with care.
*/
#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LVec<L: Index, T>(IndexVec<L, T>);

impl<L: Index, T> Default for LVec<L, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<L: Index, T> LVec<L, T> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(IndexVec::new())
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: L) -> Self {
        Self(IndexVec::with_capacity(capacity))
    }

    #[must_use]
    pub fn inner(&self) -> &Vec<T> {
        // self.to_vec()
        todo!("Implement `to_vec`");
    }

    pub(crate) fn deser_content<R: Read>(reader: &mut R, len: L) -> Res<Self>
    where
        L: TryInto<usize>,
        T: Deserialize<LE, R>,
    {
        let mut vec = IndexVec::<L, T>::with_capacity(len);
        for _ in 0..len.try_into().unwrap() {
            vec.push(LERead::read(reader)?);
        }
        Ok(Self(vec))
    }

    pub(crate) fn ser_len<W: LEWrite>(&self, writer: &mut W) -> Res<()>
    where
        L: TryFrom<usize> + Serialize<LE, W>,
    {
        let len = self.0.len();
        let l_len = if let Ok(x) = L::try_from(len) {
            x
        } else {
            panic!()
        };
        writer.write(l_len)
    }

    pub(crate) fn ser_content<W: Write>(&self, writer: &mut W) -> Res<()>
    where
        for<'a> &'a T: Serialize<LE, W>,
    {
        for e in self.0.as_ref() {
            LEWrite::write(writer, e)?;
        }
        Ok(())
    }
}

impl<L: Index, T: Debug> Debug for LVec<L, T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<L: Index, T, R: Read> Deserialize<LE, R> for LVec<L, T>
where
    L: TryInto<usize> + Deserialize<LE, R>,
    T: Deserialize<LE, R>,
{
    fn deserialize(reader: &mut R) -> Res<Self> {
        let len: L = LERead::read(reader)?;
        Self::deser_content(reader, len)
    }
}

impl<L: Index, T, W: Write> Serialize<LE, W> for &LVec<L, T>
where
    L: TryFrom<usize> + Serialize<LE, W>,
    for<'b> &'b T: Serialize<LE, W>,
{
    fn serialize(self, writer: &mut W) -> Res<()> {
        self.ser_len(writer)?;
        self.ser_content(writer)
    }
}

impl<L: Index, T> std::ops::Deref for LVec<L, T> {
    type Target = IndexVec<L, T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<L: Index, T> std::ops::DerefMut for LVec<L, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<L: Index, T> From<IndexVec<L, T>> for LVec<L, T> {
    fn from(vec: IndexVec<L, T>) -> Self {
        Self(vec)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[repr(u16)]
pub enum ServiceId {
    General = 0,
    Auth = 1,
    Chat = 2,
    World = 4,
    Client = 5,
}

pub type ObjId = u64;
pub const OBJID_EMPTY: u64 = 0;
