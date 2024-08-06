#!/usr/bin/env -S cargo +nightly -Zscript

use std::io::{self, Write};
use std::fs::File;

const CANVAS_WIDTH:  usize = 300;
const CANVAS_HEIGHT: usize = 300;

fn main() -> io::Result<()> {
   let mut svg_content = String::new();
   svg_content.push_str(
      &format!(r##"
         <svg xmlns="http://www.w3.org/2000/svg"
            width="{CANVAS_WIDTH}" height="{CANVAS_HEIGHT}"
            viewBox="0 0 {CANVAS_WIDTH} {CANVAS_HEIGHT}">

            <rect width="{CANVAS_WIDTH}" height="{CANVAS_HEIGHT}" fill="#fff"/>
         </svg>
      "##)
   );

   let mut file = File::create("keylayout.svg")?;
   file.write_all(svg_content.as_bytes())?;

   Ok(())
}
