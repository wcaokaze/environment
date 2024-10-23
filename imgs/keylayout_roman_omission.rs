#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_roman_omission.svg",
      [
         [n("Tab")  , n("-aく"), n("-oく"), n("-eき"), n("-uく"), n("-iく")],
         [X         , X        , X        , X        , X        , X        ],
         [n("Ctrl") , n("-aつ"), n("-oつ"), n("-eつ"), n("-uつ"), n("-iつ")],
         [X         , X        , X        , X        , X        , X        ],
         [n("Shift"), n("-aっ"), n("-oっ"), n("-eっ"), n("-uっ"), n("-iっ")],
         [X         , X        , X        , X        , X        , X        ]
      ], [
                                   [n("Esc"), n("_"), n("Space"), n("Backspace")],
                                   [X       , X     , X         , X             ]
      ],

      [
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ],
                    [X     , X     , X     , X     , X     , X      ]
      ], [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
