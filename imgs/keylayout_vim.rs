#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_vim.svg",
      [
         [n("Tab")  , n("z"), n("w"), n("e"), n("p"), n("u")],
         [X         , X     , X     , X     , X     , X     ],
         [n("Ctrl") , n("a"), n("o"), n("d"), n("f"), n("i")],
         [X         , X     , X     , X     , X     , X     ],
         [n("Shift"), n("q"), n("J"), n("n"), n("v"), n("x")],
         [X         , X     , X     , X     , X     , X     ]
      ], [
                             [n("Esc"), n("_"), n("Space"), n("Backspace")],
                             [X       , X     , X         , X             ]
      ],

      [
                 [n("y"), n("g"), n("c"), n(","), n("."), n("=")],
                 [X     , X     , X     , X     , X     , X     ],
                 [n("h"), n("j"), n("k"), n("l"), n("s"), n("[")],
                 [X     , X     , X     , X     , X     , X     ],
                 [n("b"), n("r"), n("t"), n("'"), n(";"), n("-")],
                 [X     , X     , X     , X     , X     , X     ]
      ],
      [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
