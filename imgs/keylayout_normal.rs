#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout.svg",
      [
         [n("Tab")  , n("'")    , n(","), n("."), n("P"), n("Y")],
         [X         , X         , X     , X     , X     , X     ],
         [n("Ctrl") , n("A")    , n("O"), n("E"), n("U"), n("I")],
         [X         , X         , X     , X     , X     , X     ],
         [n("Shift"), o("Z")    , n("Q"), n("J"), n("K"), n("X")],
         [X         , h("Shift"), X     , X     , X     , X     ]
      ], [
                                [o("Esc"), n("_"), n("Space"), n("Backspace")],
                                [h("Alt"), X     , h("記号L"), X             ]
      ],

      [
                     [n("F"), n("G"), n("C"), n("R"), n("L"), n("=")],
                     [X     , X     , X     , X     , X     , X     ],
                     [n("D"), n("H"), n("T"), n("N"), n("S"), n("[")],
                     [X     , X     , X     , X     , X     , X     ],
                     [n("B"), n("M"), n("W"), n("V"), n(";"), n("-")],
                     [X     , X     , X     , X     , X     , X     ]
      ], [
         [o("Ctrl+["), o("Enter"), o("F13"), n("Super")],
         [h("Shift") , h("数字L"), h("fnL"), X         ]
      ]
   )
}
