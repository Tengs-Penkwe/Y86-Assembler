const std = @import("std");
const fs = @import("std").fs;
const os = @import("std").os;
const print = std.debug.print;

//TODO: arg parse

const file_path = "example.y86";
// const buf_len = 1024;

const content =
    \\xorq %rax, %rax
;

pub fn main() anyerror!void {
    //TODO: read file
    // var file = try std.fs.cwd().openFile(file_path, .{});
    // defer file.close();
    //
    // print("{?}", .{file});
    //
    // var buf_reader = std.io.bufferedReader(file.reader());
    // var in_stream = buf_reader.reader();
    //
    // var buf: [1024]u8 = undefined;
    // while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
    //     print("{?}", .{line});
    // }

}
