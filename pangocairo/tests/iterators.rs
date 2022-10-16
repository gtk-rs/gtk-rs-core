// Take a look at the license at the top of the repository in the LICENSE file.

use glib::GStr;
use pango::prelude::*;

const TEXT: &GStr = glib::gstr!("AAAA BBBB CCCC DDDD EEEE FFFF gggg hhhh iiii jjjj kkkk llll");

#[test]
fn glyph_item_iter() {
    let item = {
        let map = pangocairo::FontMap::default();
        let pc = map.create_context();
        let layout = pango::Layout::new(&pc);
        layout.set_width(256 * pango::SCALE);
        layout.set_height(256 * pango::SCALE);
        layout.set_text(TEXT);
        let mut layout_iter = layout.iter();
        layout_iter.run().unwrap()
    };
    let iter = pango::GlyphItemIter::new_start(&item, TEXT).unwrap();
    assert_eq!(iter.glyph_item(), &item);
    assert_eq!(iter.text(), TEXT);
    for (i, (sg, si, sc, eg, ei, ec)) in iter.into_iter().enumerate() {
        let i = i as i32;
        // ensure these are all single-byte ASCII characters
        assert_eq!(sg, i);
        assert_eq!(si, i);
        assert_eq!(sc, i);
        assert_eq!(eg, i + 1);
        assert_eq!(ei, i + 1);
        assert_eq!(ec, i + 1);
    }
}
