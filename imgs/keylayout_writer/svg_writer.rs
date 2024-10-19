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
