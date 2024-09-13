#!/usr/bin/env -S cargo +nightly -Zscript

use std::fs::File;
use std::io::{self, BufWriter, Write};

const BASIC_CHAR_TABLE: ([&'static str; 5], [(&'static str, [&'static str; 5]); 17]) = (
             ["a",    "i",    "u",    "e",    "o"],
   [
      ( "",  ["あ",   "い",   "う",   "え",   "お"]),
      ("c",  ["か",   "き",   "く",   "け",   "こ"]),
      ("s",  ["さ",   "し",   "す",   "せ",   "そ"]),
      ("t",  ["た",   "ち",   "つ",   "て",   "と"]),
      ("n",  ["な",   "に",   "ぬ",   "ね",   "の"]),
      ("h",  ["は",   "ひ",   "ふ",   "へ",   "ほ"]),
      ("m",  ["ま",   "み",   "む",   "め",   "も"]),
      ("v",  ["や",   "い",   "ゆ",   "いぇ", "よ"]),
      ("r",  ["ら",   "り",   "る",   "れ",   "ろ"]),
      ("w",  ["わ",   "うぃ", "う",   "うぇ", "を"]),
      ("g",  ["が",   "ぎ",   "ぐ",   "げ",   "ご"]),
      (";",  ["ざ",   "じ",   "ず",   "ぜ",   "ぞ"]),
      ("d",  ["だ",   "ぢ",   "づ",   "で",   "ど"]),
      ("b",  ["ば",   "び",   "ぶ",   "べ",   "ぼ"]),
      ("f",  ["ぱ",   "ぴ",   "ぷ",   "ぺ",   "ぽ"]),
      ("l",  ["ぁ",   "ぃ",   "ぅ",   "ぇ",   "ぉ"]),
      ("lv", ["ゃ",   "ぃ",   "ゅ",   "ぃぇ", "ょ"]),
   ]
);

const SPECIAL_CHARS: [(&'static str, &'static str); 6] = [
   ("nn",  "ん"),
   ("'",   "っ"),
   ("ltu", "っ"),
   ("lwa", "ゎ"),
   ("-",   "ー"),
   ("p",   "ー"),
];

fn main() -> io::Result<()> {
   let output = File::create("romantable.txt")?;
   let mut output = BufWriter::new(output);

   basic_characters(&mut output)?;
   special_characters(&mut output)?;

   Ok(())
}

fn basic_characters(output: &mut dyn Write) -> io::Result<()> {
   let (second_strokes, table) = &BASIC_CHAR_TABLE;

   for (first_stroke, chars) in table {
      for (second_stroke, char) in second_strokes.iter().zip(chars.iter()) {
         output.write(first_stroke.as_bytes())?;
         output.write(second_stroke.as_bytes())?;
         output.write(b"\t")?;
         output.write(char.as_bytes())?;
         output.write(b"\n")?;
      }
   }

   Ok(())
}

fn special_characters(output: &mut dyn Write) -> io::Result<()> {
   for (strokes, char) in &SPECIAL_CHARS {
      output.write(strokes.as_bytes())?;
      output.write(b"\t")?;
      output.write(char.as_bytes())?;
      output.write(b"\n")?;
   }

   Ok(())
}
