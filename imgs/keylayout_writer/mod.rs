use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;
use std::mem::MaybeUninit;
use crate::keylayout_writer::svg_writer::SvgWriter;

pub mod prelude;
mod svg_writer;

const ALPHANUMERIC_COL_COUNT: usize = 6;
const ALPHANUMERIC_ROW_COUNT: usize = 3;
const THUMB_KEY_COUNT: usize = 4;

const CANVAS_WIDTH:  usize = 900;
const CANVAS_HEIGHT: usize = 340;

const KEY_WIDTH:  usize = 50;
const KEY_HEIGHT: usize = 50;

const ALPHANUMERIC_STAGGER_RATE: [f64; ALPHANUMERIC_COL_COUNT]
   = [1.0, 0.8, 0.3, 0.0, 0.2, 0.3];
const THUMB_KEY_WIDTH: [f64; THUMB_KEY_COUNT] = [1.0, 1.0, 1.25, 1.0];

pub fn generate_keylayout(
   output_path: impl AsRef<Path>,
   left_alphanumeric_keys: [[Option<Key>; ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT * 2],
   left_thumb_keys: [[Option<Key>; THUMB_KEY_COUNT]; 2],
   right_alphanumeric_keys: [[Option<Key>; ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT * 2],
   right_thumb_keys: [[Option<Key>; THUMB_KEY_COUNT]; 2]
) -> io::Result<()> {
   let output = File::create(output_path)?;
   KeyLayoutWriter::write(output, |writer| {
      let left_alphanumeric_keys = zip_keys(left_alphanumeric_keys);
      let left_thumb_keys = zip_keys(left_thumb_keys);
      let right_alphanumeric_keys = zip_keys(right_alphanumeric_keys);
      let right_thumb_keys = zip_keys(right_thumb_keys);

      writer.left_alphanumeric(left_alphanumeric_keys)?;
      writer.left_thumb(left_thumb_keys)?;
      writer.right_alphanumeric(right_alphanumeric_keys)?;
      writer.right_thumb(right_thumb_keys)?;

      Ok(())
   })?;

   Ok(())
}

pub const X: Option<Key> = None;

pub fn n(text: &'static str) -> Option<Key> {
   Some(Key::Normal(text))
}

pub fn o(text: &'static str) -> Option<Key> {
   Some(Key::Oneshot(text))
}


pub fn h(text: &'static str) -> Option<Key> {
   Some(Key::Hold(text))
}

fn zip_keys<const C: usize, const R: usize>(
   keys: [[Option<Key>; C]; R * 2]
) -> [[(Option<Key>, Option<Key>); C]; R] {
   keys.chunks(2)
      .map(|row_pair| {
         let [primary_key, secondary_key] = row_pair else { panic!(); };

         primary_key.iter().zip(secondary_key.iter())
            .map(|(&p, &s)| (p, s))
            .collect::<Vec<_>>()
            .try_into().unwrap()
      })
      .collect::<Vec<_>>()
      .try_into().unwrap()
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

#[derive(Copy, Clone, Debug)]
pub enum Key {
   Normal(&'static str),
   Oneshot(&'static str),
   Hold(&'static str),
}

impl Key {
   fn text(&self) -> &str {
      match &self {
         Key::Normal(text) => text,
         Key::Oneshot(text) => text,
         Key::Hold(text) => text,
      }
   }

   fn class(&self) -> Option<&str> {
      match &self {
         Key::Normal(_) => None,
         Key::Oneshot(_) => Some("oneshot"),
         Key::Hold(_) => Some("hold"),
      }
   }
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
               writer.append_style(".oneshot", |writer| {
                  writer.append_prop("fill", "dodgerblue")?;
                  Ok(())
               })?;
               writer.append_style(".hold", |writer| {
                  writer.append_prop("fill", "green")?;
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

   fn text(
      &mut self,
      x: usize,
      y: usize,
      class: Option<&str>,
      content: &str
   ) -> io::Result<()> {
      self.svg_writer.append_raw_element(
         "text",
         |writer| {
            writer.append_attr("x", &x.to_string())?;
            writer.append_attr("y", &y.to_string())?;
            if let Some(class) = class {
               writer.append_attr("class", class)?;
            }
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
      class: Option<&str>,
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

            if let Some(class) = class {
               writer.append_attr("class", class)?;
            }

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

   fn alphanumeric_column<const N: usize>(
      &mut self,
      x: usize,
      y: usize,
      keys: &[(Option<Key>, Option<Key>); N]
   ) -> io::Result<()> {
      if N == 0 { return Ok(()); }

      self.rect(x, y, KEY_WIDTH, KEY_HEIGHT * N)?;

      for i in 0..N {
         let y = y + i * KEY_HEIGHT;

         if i >= 1 {
            self.line(x, y, x + KEY_WIDTH, y)?;
         }

         let (primary_key, secondary_key) = keys[i];
         if let Some(key) = primary_key {
            self.text(x + 4, y + 16, key.class(), key.text())?;
         }
         if let Some(key) = secondary_key {
            self.text(x + 4, y + 32, key.class(), key.text())?;
         }
      }

      Ok(())
   }

   fn alphanumeric(
      &mut self,
      keys: [[(Option<Key>, Option<Key>); ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT],
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

   fn left_alphanumeric(
      &mut self,
      keys: [[(Option<Key>, Option<Key>); ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT]
   ) -> io::Result<()> {
      self.alphanumeric(keys, ALPHANUMERIC_STAGGER_RATE, (10, 10))?;
      Ok(())
   }

   fn right_alphanumeric(
      &mut self,
      keys: [[(Option<Key>, Option<Key>); ALPHANUMERIC_COL_COUNT]; ALPHANUMERIC_ROW_COUNT]
   ) -> io::Result<()> {
      let x = CANVAS_WIDTH - ALPHANUMERIC_COL_COUNT * KEY_WIDTH;
      let mut reversed_stagger_rate = ALPHANUMERIC_STAGGER_RATE;
      reversed_stagger_rate.reverse();
      self.alphanumeric(keys, reversed_stagger_rate, (x - 10, 10))?;
      Ok(())
   }

   fn thumb(
      &mut self,
      keys: [(Option<Key>, Option<Key>); THUMB_KEY_COUNT],
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
         .map(|(i, (((primary_key, secondary_key), arc), width))| {
            let (x1, y1) = point(outer_radius, arc);
            let (x2, y2) = point(inner_radius, arc);

            if i > 0 {
               self.line(x1, y1, x2, y2)?;
            }

            let angle = angle(arc + width / 3.0) + PI / 2.0;
            if let Some(key) = primary_key {
               let (x, y) = point(outer_radius - 16.0, arc + 4.0);
               self.rotated_text(x, y, angle, key.class(), key.text())?;
            }
            if let Some(key) = secondary_key {
               let (x, y) = point(outer_radius - 32.0, arc + 4.0);
               self.rotated_text(x, y, angle, key.class(), key.text())?;
            }

            Ok(())
         })
         .collect::<io::Result<()>>()?;

      Ok(())
   }

   fn left_thumb(
      &mut self,
      keys: [[(Option<Key>, Option<Key>); THUMB_KEY_COUNT]; 1]
   ) -> io::Result<()> {
      let center = (
         (4.8 * KEY_WIDTH  as f64) as usize,
         (7.8 * KEY_HEIGHT as f64) as usize
      );
      let start_angle = f64::to_radians(-90.0 - 15.0);
      let arc_radius = 4.0 * KEY_WIDTH as f64;

      self.thumb(keys[0], THUMB_KEY_WIDTH, center, arc_radius, start_angle)?;
      Ok(())
   }

   fn right_thumb(
      &mut self,
      keys: [[(Option<Key>, Option<Key>); THUMB_KEY_COUNT]; 1]
   ) -> io::Result<()> {
      let mut reversed_thumb_key_width = THUMB_KEY_WIDTH;
      reversed_thumb_key_width.reverse();

      let center = (
         CANVAS_WIDTH - (4.8 * KEY_WIDTH  as f64) as usize,
         (7.8 * KEY_HEIGHT as f64) as usize
      );
      let arc_radius = 4.0 * KEY_WIDTH as f64;

      let arc_length = reversed_thumb_key_width.iter().sum::<f64>() * KEY_WIDTH as f64;
      let arc_angle = arc_length / (arc_radius - KEY_HEIGHT as f64);
      let start_angle = f64::to_radians(-90.0 + 15.0) - arc_angle;

      self.thumb(keys[0], reversed_thumb_key_width, center, arc_radius, start_angle)?;
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
