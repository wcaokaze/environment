#!/usr/bin/env -S cargo +nightly -Zscript

#![feature(generic_const_exprs)]

use std::io;
use crate::keylayout_writer::prelude::*;

mod keylayout_writer;

fn main() -> io::Result<()> {
   generate_keylayout("keylayout_roman_template.svg",
      [
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ],
         [X     , X     , X     , X     , X     , X      ]
      ], [
                                   [n("Esc"), n("_"), n("Space"), n("Backspace")],
                                   [X       , X     , X         , X             ]
      ],

      [
                    [X     , X        , n("削除"), X     , X     , X      ],
                    [X     , X        , X        , X     , X     , X      ],
                    [X     , n("確認"), n("変更"), X     , X     , X      ],
                    [X     , X        , X        , X     , X     , X      ],
                    [X     , X        , X        , X     , X     , X      ],
                    [X     , X        , X        , X     , X     , X      ]
      ], [
         [n("Ctrl+["), n("Enter"), n("F13"), n("Super")],
         [X          , X         , X       , X         ]
      ]
   )
}
