/*******************************************************************************
 * Copyright (c) 2017 Association Cénotélie (cenotelie.fr)
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General
 * Public License along with this program.
 * If not, see <http://www.gnu.org/licenses/>.
 ******************************************************************************/

//! Module for binary manipulation APIs

/// Reads a `u16` from an array of bytes
///
/// # Panics
///
/// Raise a panic when the given buffer is not long enough
/// from the specified index to accomodate the size of a `u16` value
#[must_use]
pub fn read_u16(buffer: &[u8], index: usize) -> u16 {
    const SIZE: usize = core::mem::size_of::<u16>();
    let bytes: &[u8; SIZE] = buffer[index..(index + SIZE)]
        .try_into()
        .expect("given buffer cannot contain u16 value");
    u16::from_le_bytes(*bytes)
}

/// Reads a `u32` from an array of bytes
///
/// # Panics
///
/// Raise a panic when the given buffer is not long enough
/// from the specified index to accomodate the size of a `u32` value
#[must_use]
pub fn read_u32(buffer: &[u8], index: usize) -> u32 {
    const SIZE: usize = core::mem::size_of::<u32>();
    let bytes: &[u8; SIZE] = buffer[index..(index + SIZE)]
        .try_into()
        .expect("given buffer cannot contain u32 value");
    u32::from_le_bytes(*bytes)
}

/// Converts a slice of bytes to slice of Ts, and return error if
#[must_use]
pub fn read_table<'a, T>(buffer: &'a [u8], count: usize) -> Result<&'a [T], ()> {
    let buffer = &buffer[..count * core::mem::size_of::<T>()];
    // SAFETY: we will guarantee that only prefix and suffix have no item (so data is fully consumed)
    //  we will return the data, otherwise error
    let (prefix, data, suffix) = unsafe { buffer.align_to::<T>() };
    if prefix.len() == 0 && suffix.len() == 0 {
        Ok(data)
    } else {
        Err(())
    }
}
