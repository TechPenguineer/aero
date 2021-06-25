/*
 * Copyright (C) 2021 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */

#![no_std]
#![feature(custom_test_frameworks)]

use core::{ops, slice};

use aero_gfx::FrameBuffer;
use x86_64::{PhysAddr, VirtAddr};

/// Represents the different types of memory.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[repr(C)]
pub enum MemoryRegionType {
    /// Unused conventional memory, can be used by the kernel.
    Usable,
    /// Memory mappings created by the bootloader, including the kernel and boot info mappings.
    ///
    /// This memory should **not** be used by the kernel.
    Bootloader,
    UnknownUefi(u32),
    UnknownBios(u32),
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct UnwindInfo {
    /// The base address of the kernel. The kernel base is required
    /// for stack unwinding during kernel panics.
    pub kernel_base: VirtAddr,
    /// The size of the kernel, required to calculate the end of the
    /// kernel base.
    pub kernel_size: usize,

    //// The stack end for the kernel. Stack on `x86_64` grows backwards.
    pub stack_top: VirtAddr,
}

#[derive(Debug)]
#[repr(C)]
pub struct BootInfo {
    pub rsdp_address: PhysAddr,
    pub physical_memory_offset: VirtAddr,
    pub framebuffer: FrameBuffer,
    pub memory_regions: MemoryRegions,
    pub unwind_info: UnwindInfo,
}

#[derive(Debug)]
#[repr(C)]
pub struct MemoryRegions {
    pub(crate) ptr: *mut MemoryRegion,
    pub(crate) len: usize,
}

impl ops::Deref for MemoryRegions {
    type Target = [MemoryRegion];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl ops::DerefMut for MemoryRegions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl From<&'static mut [MemoryRegion]> for MemoryRegions {
    fn from(regions: &'static mut [MemoryRegion]) -> Self {
        MemoryRegions {
            ptr: regions.as_mut_ptr(),
            len: regions.len(),
        }
    }
}

impl From<MemoryRegions> for &'static mut [MemoryRegion] {
    fn from(regions: MemoryRegions) -> &'static mut [MemoryRegion] {
        unsafe { slice::from_raw_parts_mut(regions.ptr, regions.len) }
    }
}

/// Represent a physical memory region.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct MemoryRegion {
    /// The physical start address of the region.
    pub start: u64,
    /// The physical end address (exclusive) of the region.
    pub end: u64,
    /// The memory type of the memory region.
    pub kind: MemoryRegionType,
}