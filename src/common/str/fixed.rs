use std::{convert::TryFrom, io::{Read, Result as Res, Write}, ops::Not};

use endio::{Deserialize, LE, Serialize};

use super::{AbstractLuStr, AsciiChar, AsciiError, LuChar, LuStrExt, Ucs2Char, Ucs2Error};

// todo[const generics]: const generic strings
// todo: exclude the final null terminator from the array

// todo: runtime type invariants checks (valid, null terminator)
/// A string with a maximum length of $n.
pub struct LuString<C, const N: usize>([C; N]);

impl<C: PartialEq, const N: usize> PartialEq for LuString<C, N> {
    fn eq(&self, other: &Self) -> bool {
        dbg!((&**self) == (&**other))
    }
}

impl<C, const N: usize> std::fmt::Debug for LuString<C, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        (&**self).fmt(f)
    }
}

impl<C: Copy + Not<Output = bool>, const N: usize> std::ops::Deref for LuString<C, N> {
    type Target = AbstractLuStr<C>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let terminator = self.0.iter().position(|&c| !c).unwrap();
        &self.0[..terminator]
    }
}

impl<C: Copy + Not<Output = bool>, const N: usize> std::ops::DerefMut for LuString<C, N> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let terminator = self.0.iter().position(|&c| !c).unwrap();
        &mut self.0[..terminator]
    }
}

impl<R: Read, C, const N: usize> Deserialize<LE, R> for LuString<C, N> {
    fn deserialize(reader: &mut R) -> Res<Self> {
        let mut bytes = [0u8; N * std::mem::size_of::<C>()];
        reader.read(&mut bytes)?;
        Ok(Self(unsafe { std::mem::transmute(bytes) }))
    }
}

impl<W: Write, C, const N: usize> Serialize<LE, W> for &LuString<C, N> {
    fn serialize(self, writer: &mut W) -> Res<()> {
        let x: [u8; _] =
            unsafe { std::mem::transmute(self.0) };
        writer.write_all(&x)
    }
}

impl<C, const N: usize> TryFrom<&[u8]> for LuString<C, N> {
    type Error = AsciiError;

    fn try_from(string: &[u8]) -> Result<Self, Self::Error> {
        if string.len() >= N {
            // actually length error but whatever
            return Err(AsciiError);
        }
        let mut bytes = [0u8; N];
        // todo: ascii range check
        for (i, chr) in string.iter().enumerate() {
            bytes[i] = *chr;
        }
        let bytes = unsafe { std::mem::transmute(bytes) };
        Ok(Self(bytes))
    }
}

impl<C, const N: usize> TryFrom<&[u8; N]> for LuString<C, N> {
    type Error = AsciiError;

    fn try_from(string: &[u8; N]) -> Result<Self, Self::Error> {
        Self::try_from(&string[..])
    }
}

impl<C, const N: usize> TryFrom<&str> for LuString<C, N> {
    type Error = Ucs2Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let mut bytes = [0u16; N];
        for (i, chr) in string.encode_utf16().take(N - 1).enumerate() {
            bytes[i] = chr;
        }
        let bytes = unsafe { std::mem::transmute(bytes) };
        Ok(Self(bytes))
    }
}

impl<C, const N: usize> From<&LuString<C, N>> for String {
    fn from(wstr: &LuString<C, N>) -> Self {
        String::from_utf16(unsafe {
            &*(&**wstr as *const [Ucs2Char] as *const [<Ucs2Char as LuChar>::Int])
        })
        .unwrap()
    }
}

pub type LuString3 = LuString<u8, 3>;
pub type LuString33 = LuString<u8, 33>;
pub type LuString37 = LuString<u8, 37>;
pub type LuWString32 = LuString<u16, 32>;
pub type LuWString33 = LuString<u16, 33>;
pub type LuWString41 = LuString<u16, 41>;
pub type LuWString42 = LuString<u16, 42>;
pub type LuWString50 = LuString<u16, 50>;
pub type LuWString128 = LuString<u16, 128>;
pub type LuWString256 = LuString<u16, 256>;
pub type LuWString400 = LuString<u16, 400>;
