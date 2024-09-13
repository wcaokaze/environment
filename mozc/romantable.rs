#!/usr/bin/env -S cargo +nightly -Zscript

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};

const BASIC_CHAR_TABLE: [(&'static str, [&'static str; 5]); 17] = [
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
];

const SPECIAL_CHARS: [(&'static str, &'static str); 22] = [
   ("nn",  "ん"),
   ("'",   "っ"),
   ("ltu", "っ"),
   ("lwa", "ゎ"),
   ("-",   "ー"),
   ("p",   "ー"),
   (",",   "、"),
   (".",   "。"),
   ("~",   "〜"),
   ("[",   "「"),
   ("]",   "」"),
   ("y-",  "〜"),
   ("yp",  "〜"),
   ("y.",  "…"),
   ("y,",  "‥"),
   ("y/",  "・"),
   ("y[",  "『"),
   ("y]",  "』"),
   ("yd",  "←"),
   ("yh",  "↓"),
   ("yt",  "↑"),
   ("yn",  "→"),
];

fn main() -> io::Result<()> {
   let mut roman_map = HashMap::new();

   basic_characters(&mut roman_map);
   nasal(&mut roman_map);
   special_characters(&mut roman_map);

   let mut roman_map = roman_map.iter().collect::<Vec<_>>();
   roman_map.sort();

   let output = File::create("romantable.txt")?;
   let mut output = BufWriter::new(output);

   for (stroke, char) in roman_map {
      output.write(stroke.as_bytes())?;
      output.write(b"\t")?;
      output.write(char.as_bytes())?;
      output.write(b"\n")?;
   }

   Ok(())
}

fn basic_characters(map: &mut HashMap<String, String>) {
   let second_strokes = ["a", "i", "u", "e", "o"];

   for (first_stroke, chars) in &BASIC_CHAR_TABLE {
      for (second_stroke, char) in second_strokes.iter().zip(chars.iter()) {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str(second_stroke);

         map.insert(stroke, char.to_string());
      }
   }
}

fn nasal(map: &mut HashMap<String, String>) {
   let second_strokes = ["z", "x", "k", "j", "q"];

   for (first_stroke, chars) in &BASIC_CHAR_TABLE {
      for (second_stroke, char) in second_strokes.iter().zip(chars.iter()) {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str(second_stroke);

         let mut char = char.to_string();
         char.push_str("ん");

         map.insert(stroke, char);
      }
   }
}

fn special_characters(map: &mut HashMap<String, String>) {
   for (stroke, char) in &SPECIAL_CHARS {
      map.insert(stroke.to_string(), char.to_string());
   }
}
