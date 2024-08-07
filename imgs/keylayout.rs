#!/usr/bin/env -S cargo +nightly -Zscript

use std::fs::File;
use std::io::{self, BufWriter};
use crate::svg_writer::SvgWriter;

const CANVAS_WIDTH:  usize = 300;
const CANVAS_HEIGHT: usize = 300;

fn main() -> io::Result<()> {
   let output = File::create("keylayout.svg")?;
   let output = BufWriter::new(output);
   let mut writer = SvgWriter::new(output);

   writer.append_element(
      "svg",
      |writer| {
         writer.append_attr("xmlns", "http://www.w3.org/2000/svg")?;
         writer.append_attr("width",  &CANVAS_WIDTH .to_string())?;
         writer.append_attr("height", &CANVAS_HEIGHT.to_string())?;
         writer.append_attr("viewBox", &format!("0 0 {CANVAS_WIDTH} {CANVAS_HEIGHT}"))?;
         Ok(())
      },
      |writer| {
         writer.append_empty_element("rect", |writer| {
            writer.append_attr("width",  &CANVAS_WIDTH .to_string())?;
            writer.append_attr("height", &CANVAS_HEIGHT.to_string())?;
            writer.append_attr("fill", "#fff")?;
            Ok(())
         })?;

         Ok(())
      }
   )?;

   Ok(())
}

mod svg_writer {
   use std::io::{self, Write};

   pub struct SvgWriter<W: Write> {
      output: W,
      write_count: usize,
      indent: usize
   }

   impl<W: Write> SvgWriter<W> {
      pub fn new(output: W) -> Self {
         SvgWriter {
            output,
            write_count: 0,
            indent: 0
         }
      }

      fn write<'a>(&mut self, buf: &'a [u8]) -> io::Result<()> {
         self.write_count += self.output.write(buf)?;
         Ok(())
      }

      fn append_indent(&mut self) -> io::Result<()> {
         for _ in 0..self.indent {
            self.write(b"   ")?;
         }
         Ok(())
      }

      pub fn append_empty_element<'a>(
         &mut self,
         element: &'a str,
         attrs: impl FnOnce(&mut AttrWriter<W>) -> io::Result<()>
      ) -> io::Result<()> {
         self.append_indent()?;
         self.write(b"<")?;
         self.write(element.as_bytes())?;

         let mut attr_writer = AttrWriter::new(self);
         attrs(&mut attr_writer)?;

         self.write(b"/>\n")?;
         Ok(())
      }

      pub fn append_element<'a>(
         &mut self,
         element: &'a str,
         attrs: impl FnOnce(&mut AttrWriter<W>) -> io::Result<()>,
         children: impl FnOnce(&mut SvgWriter<W>) -> io::Result<()>
      ) -> io::Result<()> {
         self.append_indent()?;
         self.write(b"<")?;
         self.write(element.as_bytes())?;

         let mut attr_writer = AttrWriter::new(self);
         attrs(&mut attr_writer)?;

         self.write(b">\n")?;

         self.indent += 1;
         children(self)?;
         self.indent -= 1;

         self.append_indent()?;
         self.write(b"</")?;
         self.write(element.as_bytes())?;
         self.write(b">\n")?;
         Ok(())
      }
   }

   pub struct AttrWriter<'svg, W: Write> {
      svg_writer: &'svg mut SvgWriter<W>
   }

   impl<W: Write> AttrWriter<'_, W> {
      fn new<'a>(svg_writer: &'a mut SvgWriter<W>) -> AttrWriter<'a, W> {
         AttrWriter {
            svg_writer
         }
      }

      pub fn append_attr<'a>(
         &mut self,
         attr: &'a str,
         value: &'a str
      ) -> io::Result<()> {
         self.svg_writer.write(b" ")?;
         self.svg_writer.write(attr.as_bytes())?;
         self.svg_writer.write(b"=\"")?;
         self.svg_writer.write(value.as_bytes())?;
         self.svg_writer.write(b"\"")?;
         Ok(())
      }
   }
}
