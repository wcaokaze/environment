#!/usr/bin/env -S cargo +nightly -Zscript

use std::fs::File;
use std::io::{self, BufWriter};
use std::mem::MaybeUninit;
use crate::svg_writer::SvgWriter;

const CANVAS_WIDTH:  usize = 300;
const CANVAS_HEIGHT: usize = 300;

const KEY_WIDTH:  usize = 50;
const KEY_HEIGHT: usize = 50;

fn main() -> io::Result<()> {
   let output = File::create("keylayout.svg")?;
   KeyLayoutWriter::write(output, |writer| {
      let left_alphanumeric_keys = [
         [  "Tab", "'", ",", ".", "P", "Y"],
         [ "Ctrl", "A", "O", "E", "U", "I"],
         ["Shift", "Z", "Q", "J", "K", "X"]
      ];

      writer.left_alphanumeric(left_alphanumeric_keys)?;
      Ok(())
   })?;

   Ok(())
}

fn transpose<T: Copy, const X: usize, const Y: usize>(m: [[T; X]; Y]) -> [[T; Y]; X] {
   let mut transposed = [[MaybeUninit::uninit(); Y]; X];

   for (y, row) in m.into_iter().enumerate() {
      for (x, t) in row.into_iter().enumerate() {
         transposed[x][y].write(t);
      }
   }

   unsafe { transposed.map(|row| row.map(|t| t.assume_init())) }
}

struct KeyLayoutWriter<'svg> {
   svg_writer: &'svg mut SvgWriter<BufWriter<File>>
}

impl KeyLayoutWriter<'_> {
   fn new<'svg>(
      svg_writer: &'svg mut SvgWriter<BufWriter<File>>
   ) -> KeyLayoutWriter<'svg> {
      KeyLayoutWriter { svg_writer }
   }

   fn write(
      file: File,
      content: impl FnOnce(&mut KeyLayoutWriter) -> io::Result<()>
   ) -> io::Result<()> {
      let output = BufWriter::new(file);
      let mut svg_writer = SvgWriter::new(output);

      svg_writer.append_element(
         "svg",
         |writer| {
            writer.append_attr("xmlns", "http://www.w3.org/2000/svg")?;
            writer.append_attr("width",  &CANVAS_WIDTH .to_string())?;
            writer.append_attr("height", &CANVAS_HEIGHT.to_string())?;
            writer.append_attr("viewBox", &format!("0 0 {CANVAS_WIDTH} {CANVAS_HEIGHT}"))?;
            Ok(())
         },
         |writer| {
            writer.append_style(|writer| {
               writer.append_style("rect", |writer| {
                  writer.append_prop("fill", "transparent")?;
                  writer.append_prop("stroke", "black")?;
                  Ok(())
               })?;
               writer.append_style("line", |writer| {
                  writer.append_prop("fill", "transparent")?;
                  writer.append_prop("stroke", "black")?;
                  Ok(())
               })?;
               writer.append_style("text", |writer| {
                  writer.append_prop("font-size", "12px")?;
                  writer.append_prop("fill", "black")?;
                  Ok(())
               })?;
               Ok(())
            })?;

            writer.append_empty_element("rect", |writer| {
               writer.append_attr("width",  &CANVAS_WIDTH .to_string())?;
               writer.append_attr("height", &CANVAS_HEIGHT.to_string())?;
               writer.append_attr("style", "fill: white;")?;
               Ok(())
            })?;

            let mut key_layout_writer = Self::new(writer);
            content(&mut key_layout_writer)?;

            Ok(())
         }
      )?;

      Ok(())
   }

   fn left_alphanumeric<'a>(&mut self, keys: [[&'a str; 6]; 3]) -> io::Result<()> {
      let col_top_positions = [1.0, 0.8, 0.3, 0.0, 0.2, 0.3];
      let keys = transpose(keys);

      for (x, (column, pos)) in keys.iter().zip(col_top_positions).enumerate() {
         let x = x * KEY_WIDTH;
         let y = (pos * KEY_HEIGHT as f64) as usize;
         self.alphanumeric_column(x, y, column)?;
      }

      Ok(())
   }

   fn rect(
      &mut self,
      x: usize,
      y: usize,
      width: usize,
      height: usize
   ) -> io::Result<()> {
      self.svg_writer.append_empty_element("rect", |writer| {
         writer.append_attr("x", &x.to_string())?;
         writer.append_attr("y", &y.to_string())?;
         writer.append_attr("width",  &width .to_string())?;
         writer.append_attr("height", &height.to_string())?;
         Ok(())
      })?;
      Ok(())
   }

   fn line(
      &mut self,
      x1: usize,
      y1: usize,
      x2: usize,
      y2: usize
   ) -> io::Result<()> {
      self.svg_writer.append_empty_element("line", |writer| {
         writer.append_attr("x1", &x1.to_string())?;
         writer.append_attr("y1", &y1.to_string())?;
         writer.append_attr("x2", &x2.to_string())?;
         writer.append_attr("y2", &y2.to_string())?;
         Ok(())
      })?;
      Ok(())
   }

   fn text<'a>(
      &mut self,
      x: usize,
      y: usize,
      content: &'a str
   ) -> io::Result<()> {
      self.svg_writer.append_raw_element(
         "text",
         |writer| {
            writer.append_attr("x", &x.to_string())?;
            writer.append_attr("y", &y.to_string())?;
            Ok(())
         },
         content
      )?;
      Ok(())
   }

   fn alphanumeric_column<'a, const N: usize>(
      &mut self,
      x: usize,
      y: usize,
      keys: &[&'a str; N]
   ) -> io::Result<()> {
      if N == 0 { return Ok(()); }

      self.rect(x, y, KEY_WIDTH, KEY_HEIGHT * N)?;

      for i in 0..N {
         let y = y + i * KEY_HEIGHT;

         self.text(x + 4, y + 16, keys[i])?;

         if i >= 1 {
            self.line(x, y, x + KEY_WIDTH, y)?;
         }
      }

      Ok(())
   }
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

      pub fn append_raw_element<'a>(
         &mut self,
         element: &'a str,
         attrs: impl FnOnce(&mut AttrWriter<W>) -> io::Result<()>,
         str: &'a str
      ) -> io::Result<()> {
         self.append_indent()?;
         self.write(b"<")?;
         self.write(element.as_bytes())?;

         let mut attr_writer = AttrWriter::new(self);
         attrs(&mut attr_writer)?;

         self.write(b">")?;

         self.write(str.as_bytes())?;

         self.write(b"</")?;
         self.write(element.as_bytes())?;
         self.write(b">\n")?;
         Ok(())
      }

      pub fn append_style<'a>(
         &mut self,
         styles: impl FnOnce(&mut StyleWriter<W>) -> io::Result<()>
      ) -> io::Result<()> {
         self.append_indent()?;
         self.write(b"<style>\n")?;

         self.indent += 1;
         let mut style_writer = StyleWriter::new(self);
         styles(&mut style_writer)?;
         self.indent -= 1;

         self.append_indent()?;
         self.write(b"</style>\n")?;
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

   pub struct StyleWriter<'svg, W: Write> {
      svg_writer: &'svg mut SvgWriter<W>
   }

   impl<W: Write> StyleWriter<'_, W> {
      fn new<'a>(svg_writer: &'a mut SvgWriter<W>) -> StyleWriter<'a, W> {
         StyleWriter {
            svg_writer
         }
      }

      pub fn append_style<'a>(
         &mut self,
         selector: &'a str,
         props: impl FnOnce(&mut StylePropWriter<W>) -> io::Result<()>
      ) -> io::Result<()> {
         self.svg_writer.append_indent()?;
         self.svg_writer.write(selector.as_bytes())?;
         self.svg_writer.write(b" {\n")?;

         self.svg_writer.indent += 1;
         let mut prop_writer = StylePropWriter::new(self.svg_writer);
         props(&mut prop_writer)?;
         self.svg_writer.indent -= 1;

         self.svg_writer.append_indent()?;
         self.svg_writer.write(b"}\n")?;
         Ok(())
      }
   }

   pub struct StylePropWriter<'svg, W: Write> {
      svg_writer: &'svg mut SvgWriter<W>
   }

   impl<W: Write> StylePropWriter<'_, W> {
      fn new<'a>(svg_writer: &'a mut SvgWriter<W>) -> StylePropWriter<'a, W> {
         StylePropWriter {
            svg_writer
         }
      }

      pub fn append_prop<'a>(
         &mut self,
         prop: &'a str,
         value: &'a str
      ) -> io::Result<()> {
         self.svg_writer.append_indent()?;
         self.svg_writer.write(prop.as_bytes())?;
         self.svg_writer.write(b": ")?;
         self.svg_writer.write(value.as_bytes())?;
         self.svg_writer.write(b";\n")?;
         Ok(())
      }
   }
}
