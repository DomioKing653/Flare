//! This is template for executable generation
const std = @import("std");
extern fn vm_entry(ptr: [*]const u8, len: usize) void;
const program = @embedFile("path/to/bytcode");
// Entry point for final executable 
// Build by using 'zig build-exe launcher.zig path/to/libvm_runtime.a -lc
pub fn main() !void {
    vm_entry(program.ptr, program.len);
}
