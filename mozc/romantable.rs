#!/usr/bin/env -S cargo +nightly -Zscript

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};

const BASIC_CHAR_TABLE: [(&'static str, [&'static str; 5]); 26] = [
   ( "",  ["あ",   "い",   "う",   "え",   "お"  ]),
   ("c",  ["か",   "き",   "く",   "け",   "こ"  ]),
   ("s",  ["さ",   "し",   "す",   "せ",   "そ"  ]),
   ("t",  ["た",   "ち",   "つ",   "て",   "と"  ]),
   ("n",  ["な",   "に",   "ぬ",   "ね",   "の"  ]),
   ("h",  ["は",   "ひ",   "ふ",   "へ",   "ほ"  ]),
   ("m",  ["ま",   "み",   "む",   "め",   "も"  ]),
   ("v",  ["や",   "い",   "ゆ",   "いぇ", "よ"  ]),
   ("r",  ["ら",   "り",   "る",   "れ",   "ろ"  ]),
   ("w",  ["わ",   "うぃ", "う",   "うぇ", "を"  ]),
   ("g",  ["が",   "ぎ",   "ぐ",   "げ",   "ご"  ]),
   (";",  ["ざ",   "じ",   "ず",   "ぜ",   "ぞ"  ]),
   ("d",  ["だ",   "ぢ",   "づ",   "で",   "ど"  ]),
   ("b",  ["ば",   "び",   "ぶ",   "べ",   "ぼ"  ]),
   ("f",  ["ぱ",   "ぴ",   "ぷ",   "ぺ",   "ぽ"  ]),
   ("l",  ["ぁ",   "ぃ",   "ぅ",   "ぇ",   "ぉ"  ]),
   ("lv", ["ゃ",   "ぃ",   "ゅ",   "ぃぇ", "ょ"  ]),
   ("hh", ["ふぁ", "ふぃ", "ふぅ", "ふぇ", "ふぉ"]),
   ("vv", ["ゔぁ", "ゔぃ", "ゔ",   "ゔぇ", "ゔぉ"]),
   ("cn", ["くぁ", "くぃ", "くぅ", "くぇ", "くぉ"]),
   ("tn", ["てゃ", "てぃ", "てゅ", "てぇ", "てょ"]),
   ("ts", ["とぁ", "とぃ", "とぅ", "とぇ", "とぉ"]),
   ("wh", ["うぁ", "うぃ", "う",   "うぇ", "うぉ"]),
   ("gh", ["ぐぁ", "ぐぃ", "ぐぅ", "ぐぇ", "ぐぉ"]),
   ("dh", ["でゃ", "でぃ", "でゅ", "でぇ", "でょ"]),
   ("ds", ["どぁ", "どぃ", "どぅ", "どぇ", "どぉ"]),
];

const SPECIAL_CHARS: [(&'static str, &'static str); 24] = [
   ("nn",  "ん"),
   ("'",   "っ"),
   ("ltu", "っ"),
   ("lwa", "ゎ"),
   ("lca", "ヵ"),
   ("lce", "ヶ"),
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

   let basic_char_table = BASIC_CHAR_TABLE.map(|t| {
      (t.0.to_string(), t.1.map(|c| c.to_string()))
   });

   let special_chars = SPECIAL_CHARS.map(|c| (c.0.to_string(), c.1.to_string()));

   basic_characters(&mut roman_map, &basic_char_table);
   nasal(&mut roman_map, &basic_char_table);
   diphthong(&mut roman_map, &basic_char_table);
   advanced_diphthong(&mut roman_map, &basic_char_table);

   let palatalized_table = create_palatalized_table(&basic_char_table);
   basic_characters(&mut roman_map, &palatalized_table);
   nasal(&mut roman_map, &palatalized_table);
   diphthong(&mut roman_map, &palatalized_table);
   advanced_diphthong(&mut roman_map, &palatalized_table);
   palatalized_diphthong(&mut roman_map, &palatalized_table);

   special_characters(&mut roman_map, &special_chars);

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

fn create_palatalized_table(
   table: &[(String, [String; 5])]
) -> Vec<(String, [String; 5])> {
   let second_strokes = [
      ("c", "h"),
      ("s", "h"),
      ("t", "h"),
      ("n", "h"),
      ("h", "n"),
      ("m", "n"),
      ("r", "h"),
      ("g", "n"),
      (";", "h"),
      ("d", "n"),
      ("b", "n"),
      ("f", "n"),
   ];

   table.iter()
      .flat_map(|(first_stroke, chars)| {
         let (first_stroke, second_stroke)
            = second_strokes.iter().find(|(f, _)| f == first_stroke)?;

         let palatalized_first_stroke = format!("{first_stroke}{second_stroke}");

         let first_char = &chars[1];
         let palatalized_table = [
            format!("{first_char}ゃ"),
            format!("{first_char}ぃ"),
            format!("{first_char}ゅ"),
            format!("{first_char}ぇ"),
            format!("{first_char}ょ"),
         ];

         Some((palatalized_first_stroke, palatalized_table))
      })
      .collect()
}

fn basic_characters(
   dest: &mut HashMap<String, String>,
   table: &[(String, [String; 5])]
) {
   let second_strokes = ["a", "i", "u", "e", "o"];

   for (first_stroke, chars) in table {
      for (second_stroke, char) in second_strokes.iter().zip(chars.iter()) {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str(second_stroke);

         dest.insert(stroke, char.clone());
      }
   }
}

fn nasal(
   dest: &mut HashMap<String, String>,
   table: &[(String, [String; 5])]
) {
   let second_strokes = ["z", "x", "k", "j", "q"];

   for (first_stroke, chars) in table {
      for (second_stroke, char) in second_strokes.iter().zip(chars.iter()) {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str(second_stroke);

         let mut char = String::from(char);
         char.push_str("ん");

         dest.insert(stroke, char);
      }
   }
}

fn diphthong(
   dest: &mut HashMap<String, String>,
   table: &[(String, [String; 5])]
) {
   let second_strokes = [
      ("'", 0, "い"),
      ("p", 2, "う"),
      ("y", 2, "い"),
      (".", 3, "い"),
      (",", 4, "う"),
   ];

   for (first_stroke, base_chars) in table {
      if first_stroke.is_empty() { continue; }

      for (second_stroke, base_char_idx, additional_char) in second_strokes {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str(second_stroke);

         let mut char = String::new();
         char.push_str(&base_chars[base_char_idx]);
         char.push_str(additional_char);

         dest.insert(stroke, char);
      }
   }
}

fn advanced_diphthong(
   dest: &mut HashMap<String, String>,
   table: &[(String, [String; 5])]
) {
   let third_strokes = [
      ("a", 0, "つ"),
      ("i", 1, "つ"),
      ("u", 2, "つ"),
      ("e", 3, "つ"),
      ("o", 4, "つ"),
      ("'", 0, "く"),
      ("y", 1, "く"),
      ("p", 2, "く"),
      (".", 3, "き"),
      (",", 4, "く"),
   ];

   for (first_stroke, base_chars) in table {
      if first_stroke.is_empty() { continue; }

      for (third_stroke, base_char_idx, additional_char) in third_strokes {
         let mut stroke = String::new();
         stroke.push_str(first_stroke);
         stroke.push_str("t");
         stroke.push_str(third_stroke);

         let mut char = String::new();
         char.push_str(&base_chars[base_char_idx]);
         char.push_str(additional_char);

         dest.insert(stroke, char);
      }
   }
}

fn palatalized_diphthong(
   dest: &mut HashMap<String, String>,
   table: &[(String, [String; 5])]
) {
   for (base_stroke, chars) in table {
      let Some(first_stroke) = base_stroke.chars().nth(0) else { continue; };

      let mut insert = |second_stroke, char| {
         let mut stroke = String::new();
         stroke.push(first_stroke);
         stroke.push_str(second_stroke);

         dest.insert(stroke, char);
      };

      let base_second_stroke = &base_stroke[1..];
      if base_second_stroke == "h" {
         insert("g", chars[4].clone() + "う");
         insert("m", chars[2].clone() + "う");
         insert("r", chars[4].clone() + "く");
         insert("v", chars[2].clone() + "く");
      } else if base_second_stroke == "n" {
         insert("r", chars[4].clone() + "う");
         insert("v", chars[2].clone() + "う");
         insert("g", chars[4].clone() + "く");
         insert("m", chars[2].clone() + "く");
      }
      insert("c", chars[4].clone() + "つ");
      insert("w", chars[2].clone() + "つ");
   }
}

fn special_characters(
   dest: &mut HashMap<String, String>,
   table: &[(String, String)]
) {
   for (stroke, char) in table {
      dest.insert(stroke.clone(), char.clone());
   }
}
