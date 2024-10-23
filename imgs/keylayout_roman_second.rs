#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_roman_second.svg",
      [
         [n("Tab")  , n("-aい"), n("-oう"), n("-eい"), n("-uう"), n("-uい")],
         [X         , X        , X        , X        , X        , X        ],
         [n("Ctrl") , n("-a")  , n("-o")  , n("-e")  , n("-u")  , n("-i")  ],
         [X         , X        , X        , X        , X        , X        ],
         [n("Shift"), n("-aん"), n("-oん"), n("-eん"), n("-uん"), n("-iん")],
         [X         , X        , X        , X        , X        , X        ]
      ], [
                                   [n("Esc"), n("_"), n("Space"), n("Backspace")],
                                   [X       , X     , X         , X             ]
      ],

      [
                    [X     , n("-yoう"), n("-yoつ"), n("-yoく"), X     , X     ],
                    [X     , X         , X         , X         , X     , X     ],
                    [X     , n("-y-")  , n("省略") , X         , X     , X     ],
                    [X     , X         , X         , X         , X     , X     ],
                    [X     , n("-yuう"), n("-yuつ"), n("-yuく"), X     , X     ],
                    [X     , X         , X         , X         , X     , X     ]
      ], [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
