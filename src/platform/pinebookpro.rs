//
// pinebookpro.rs
// Copyright (C) 2020 Ilja Kartašov <ik@lowenware.com>
// Distributed under terms of the MIT license.
//
//

use crate::memory::{PERIPHERALS_BASE};

pub const PERIPHERALS_REAL: usize = 0xf800_0000;
pub const PERIPHERALS_SIZE: usize = 0x0800_0000;
pub const MEMORY_BASE: usize = 0x0000_0000;
pub const MEMORY_SIZE: usize = 0xf800_0000;

pub const PCIE_BASE: usize = PERIPHERALS_BASE + 0xf800_0000;
pub const GIC_BASE: usize = PERIPHERALS_BASE + 0xfee0_0000;
pub const UART_BASE: usize = PERIPHERALS_BASE + 0xff18_0000;

pub fn heap(kernel_base: usize, kernel_size: usize) -> usize {
    kernel_base + kernel_size
}
