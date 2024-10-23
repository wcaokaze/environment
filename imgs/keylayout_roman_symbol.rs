#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_roman_symbol.svg",
      [
         [n("Tab")  , X     , n("‥"), n("…"), n("〜"), X     ],
         [X         , X     , X     , X     , X      , X     ],
         [n("Ctrl") , X     , X     , X     , X      , X     ],
         [X         , X     , X     , X     , X      , X     ],
         [n("Shift"), X     , X     , X     , X      , X     ],
         [X         , X     , X     , X     , X      , X     ]
      ], [
                                [n("Esc"), n("_"), n("Space"), n("Backspace")],
                                [X       , X     , X         , X             ]
      ],

      [
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ],
                    [n("←"), n("↓"), n("↑"), n("→"), X     , n("『")],
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , n("〜")],
                    [X     , X     , X     , X     , X     , X      ]
      ], [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
