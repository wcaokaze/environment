#!/usr/bin/env -S cargo +nightly -Zscript

use std::fs::File;
use std::io::{self, BufWriter};
use std::mem::MaybeUninit;
use crate::svg_writer::SvgWriter;

const ALPHANUMERIC_COL_COUNT: usize = 6;
const ALPHANUMERIC_ROW_COUNT: usize = 3;
const THUMB_KEY_COUNT: usize = 4;

const CANVAS_WIDTH:  usize = 1000;
const CANVAS_HEIGHT: usize = 400;

const KEY_WIDTH:  usize = 50;
const KEY_HEIGHT: usize = 50;

const ALPHANUMERIC_STAGGER_RATE: [f64; ALPHANUMERIC_COL_COUNT]
   = [1.0, 0.8, 0.3, 0.0, 0.2, 0.3];
const THUMB_KEY_WIDTH: [f64; THUMB_KEY_COUNT] = [1.0, 1.0, 1.25, 1.0];

fn main() -> io::Result<()> {
   let output = File::create("keylayout.svg")?;
   KeyLayoutWriter::write(output, |writer| {
      let left_alphanumeric_keys = [
         [  "Tab", "'", ",", ".", "P", "Y"],
         [ "Ctrl", "A", "O", "E", "U", "I"],
         ["Shift", "Z", "Q", "J", "K", "X"]
      ];

      let left_thumb_keys = ["Alt", "_", "Space", "Backspace"];

      let right_alphanumeric_keys = [
         ["F", "G", "C", "R", "L", "="],
         ["D", "H", "T", "N", "S", "["],
         ["B", "M", "W", "V", ";", "-"]
      ];

      let right_thumb_keys = ["Ctrl+[", "Enter", "F13", "Super"];

      writer.left_alphanumeric(left_alphanumeric_keys)?;
      writer.left_thumb(left_thumb_keys)?;
      writer.right_alphanumeric(right_alphanumeric_keys)?;
      writer.right_thumb(right_thumb_keys)?;
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
               writer.append_style("path", |writer| {
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

   fn rotated_text<'a>(
      &mut self,
      x: usize,
      y: usize,
      angle: f64,
      content: &'a str
   ) -> io::Result<()> {
      self.svg_writer.append_raw_element(
         "text",
         |writer| {
            writer.append_attr("x", &x.to_string())?;
            writer.append_attr("y", &y.to_string())?;

            let angle_deg = angle.to_degrees();
            writer.append_attr(
               "transform", &format!("rotate({angle_deg:0.2} {x} {y})")
            )?;

            Ok(())
         },
         content
      )?;
      Ok(())
   }

   fn path(
      &mut self,
      content: impl FnOnce(&mut PathBuilder) -> ()
   ) -> io::Result<()> {
      let mut content_builder = PathBuilder::new();
      content(&mut content_builder);

      self.svg_writer.append_empty_element("path", |writer| {
         writer.append_attr("d", &content_builder.path)?;
         Ok(())
      })?;
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

         if i >= 1 {
            self.line(x, y, x + KEY_WIDTH, y)?;
         }

         self.text(x + 4, y + 16, keys[i])?;
      }

      Ok(())
   }

   fn alphanumeric<'a>(
      &mut self,
      keys: [[&'a str; ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT],
      stagger_rate: [f64; ALPHANUMERIC_COL_COUNT],
      left_top: (usize, usize)
   ) -> io::Result<()> {
      let keys = transpose(keys);

      for (x, (column, pos)) in keys.iter().zip(stagger_rate).enumerate() {
         let x = left_top.0 + x * KEY_WIDTH;
         let y = left_top.1 + (pos * KEY_HEIGHT as f64) as usize;
         self.alphanumeric_column(x, y, column)?;
      }

      Ok(())
   }

   fn left_alphanumeric<'a>(
      &mut self,
      keys: [[&'a str; ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT]
   ) -> io::Result<()> {
      self.alphanumeric(keys, ALPHANUMERIC_STAGGER_RATE, (0, 0))?;
      Ok(())
   }

   fn right_alphanumeric<'a>(
      &mut self,
      keys: [[&'a str; ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT]
   ) -> io::Result<()> {
      let x = CANVAS_WIDTH - ALPHANUMERIC_COL_COUNT * KEY_WIDTH;
      let mut reversed_stagger_rate = ALPHANUMERIC_STAGGER_RATE;
      reversed_stagger_rate.reverse();
      self.alphanumeric(keys, reversed_stagger_rate, (x, 0))?;
      Ok(())
   }

   fn thumb<'a>(
      &mut self,
      keys: [&'a str; THUMB_KEY_COUNT],
      key_widths: [f64; THUMB_KEY_COUNT],
      arc_center: (usize, usize),
      arc_radius: f64,
      start_angle: f64
   ) -> io::Result<()> {
      use std::f64::consts::PI;

      let key_widths = key_widths.map(|u| u * KEY_WIDTH as f64);

      let outer_radius = arc_radius;
      let inner_radius = outer_radius - KEY_HEIGHT as f64;

      let angle = |arc| {
         start_angle + arc / inner_radius as f64
      };

      let point = |radius, arc| {
         let angle = angle(arc);

         (
            (arc_center.0 as f64 + radius * f64::cos(angle)) as usize,
            (arc_center.1 as f64 + radius * f64::sin(angle)) as usize
         )
      };

      self.path(|builder| {
         let arc = key_widths.into_iter().sum();

         let (x, y) = point(outer_radius, 0.0);
         builder.move_to(x, y);

         let (x, y) = point(outer_radius, arc);
         builder.arc(
            outer_radius as usize, outer_radius as usize,
            /* x_axis_rotation = */ 0.0,
            /* large_arc_flag = */ false,
            /* sweep_flag = */ true,
            x, y
         );

         let (x, y) = point(inner_radius, arc);
         builder.line_to(x, y);

         let (x, y) = point(inner_radius, 0.0);
         builder.arc(
            inner_radius as usize, inner_radius as usize,
            /* x_axis_rotation = */ 0.0,
            /* large_arc_flag = */ false,
            /* sweep_flag = */ false,
            x, y
         );

         builder.close();
      })?;

      let arcs = [0.0].into_iter()
         .chain(key_widths.into_iter())
         .scan(0.0, |st, w| {
            *st += w;
            Some(*st)
         });

      keys.into_iter()
         .zip(arcs)
         .zip(key_widths)
         .enumerate()
         .map(|(i, ((key, arc), width))| {
            let (x1, y1) = point(outer_radius, arc);
            let (x2, y2) = point(inner_radius, arc);

            if i > 0 {
               self.line(x1, y1, x2, y2)?;
            }

            let (x, y) = point(outer_radius - 16.0, arc + 4.0);
            let angle = angle(arc + width / 2.0) + PI / 2.0;
            self.rotated_text(x, y, angle, key)?;

            Ok(())
         })
         .collect::<io::Result<()>>()?;

      Ok(())
   }

   fn left_thumb<'a>(
      &mut self,
      keys: [&'a str; THUMB_KEY_COUNT]
   ) -> io::Result<()> {
      let center = (
         (4.7 * KEY_WIDTH  as f64) as usize,
         (7.7 * KEY_HEIGHT as f64) as usize
      );
      let start_angle = f64::to_radians(-90.0 - 15.0);
      let arc_radius = 4.0 * KEY_WIDTH as f64;

      self.thumb(keys, THUMB_KEY_WIDTH, center, arc_radius, start_angle)?;
      Ok(())
   }

   fn right_thumb<'a>(
      &mut self,
      keys: [&'a str; THUMB_KEY_COUNT]
   ) -> io::Result<()> {
      let mut reversed_thumb_key_width = THUMB_KEY_WIDTH;
      reversed_thumb_key_width.reverse();

      let center = (
         CANVAS_WIDTH - (4.7 * KEY_WIDTH  as f64) as usize,
         (7.7 * KEY_HEIGHT as f64) as usize
      );
      let arc_radius = 4.0 * KEY_WIDTH as f64;

      let arc_length = reversed_thumb_key_width.iter().sum::<f64>() * KEY_WIDTH as f64;
      let arc_angle = arc_length / (arc_radius - KEY_HEIGHT as f64);
      let start_angle = f64::to_radians(-90.0 + 15.0) - arc_angle;

      self.thumb(keys, reversed_thumb_key_width, center, arc_radius, start_angle)?;
      Ok(())
   }
}

struct PathBuilder {
   path: String
}

impl PathBuilder {
   fn new() -> Self {
      PathBuilder {
         path: String::new()
      }
   }

   fn move_to(&mut self, x: usize, y: usize) {
      self.path.push_str(&format!("M {x} {y} "));
   }

   fn line_to(&mut self, x: usize, y: usize) {
      self.path.push_str(&format!("L {x} {y} "));
   }

   fn arc(
      &mut self,
      rx: usize,
      ry: usize,
      x_axis_rotation: f64,
      large_arc_flag: bool,
      sweep_flag: bool,
      x: usize,
      y: usize
   ) {
      let large_arc_flag = if large_arc_flag { 1 } else { 0 };
      let sweep_flag     = if sweep_flag     { 1 } else { 0 };

      self.path.push_str(&format!(
         "A {rx} {ry} {x_axis_rotation} {large_arc_flag} {sweep_flag} {x} {y} "
      ));
   }

   fn close(&mut self) {
      self.path.push_str("Z");
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
