#!/usr/bin/env -S cargo +nightly -Zscript

use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
   let output = File::create("romantable.txt")?;
   let mut output = BufWriter::new(output);

   output.write(b"test")?;

   Ok(())
}
