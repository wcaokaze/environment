#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_roman_first.svg",
      [
         [n("Tab")  , n("っ")  , n("、")  , n("。")  , n("ー")  , n("記号")],
         [X         , X        , X        , X        , X        , X        ],
         [n("Ctrl") , n("あ")  , n("お")  , n("え")  , n("う")  , n("い")  ],
         [X         , X        , X        , X        , X        , X        ],
         [n("Shift"), n("あん"), n("おん"), n("えん"), n("うん"), n("いん")],
         [X         , X        , X        , X        , X        , X        ]
      ], [
                                   [n("Esc"), n("_"), n("Space"), n("Backspace")],
                                   [X       , X     , X         , X             ]
      ],

      [
                    [n("p-"), n("g-"), n("k-"), n("r-"), n("l-"), n("=") ],
                    [X      , X      , X      , X      , X      , X      ],
                    [n("d-"), n("h-"), n("t-"), n("n-"), n("s-"), n("「")],
                    [X      , X      , X      , X      , X      , X      ],
                    [n("b-"), n("m-"), n("w-"), n("y-"), n("z-"), n("ー")],
                    [X      , X      , X      , X      , X      , X      ]
      ], [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
