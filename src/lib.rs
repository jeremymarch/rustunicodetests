//#![no_std]
#![deny(unsafe_code)]

#[macro_use]
extern crate alloc;
use alloc::string::String;
//use alloc::string::ToString;
use std::cmp::Ordering;

use core::cmp;

//extern crate tinyvec;
//use tinyvec::TinyVec;

//use core::fmt::Display;
extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub use crate::tables::*;
mod tables;

const MACRON_AND_SMOOTH:u32 = HGK_MACRON | HGK_SMOOTH;
const MACRON_AND_SMOOTH_AND_ACUTE:u32 = HGK_MACRON | HGK_SMOOTH | HGK_ACUTE;
const MACRON_AND_SMOOTH_AND_GRAVE:u32 = HGK_MACRON | HGK_SMOOTH | HGK_GRAVE;
const MACRON_AND_ROUGH:u32 = HGK_MACRON | HGK_ROUGH;
const MACRON_AND_ROUGH_AND_ACUTE:u32 = HGK_MACRON | HGK_ROUGH | HGK_ACUTE;
const MACRON_AND_ROUGH_AND_GRAVE:u32 = HGK_MACRON | HGK_ROUGH | HGK_GRAVE;
const MACRON_AND_ACUTE:u32 = HGK_MACRON | HGK_ACUTE;
const MACRON_AND_GRAVE:u32 = HGK_MACRON | HGK_GRAVE;

const BREVE_AND_SMOOTH:u32 = HGK_BREVE | HGK_SMOOTH;
const BREVE_AND_SMOOTH_AND_ACUTE:u32 = HGK_BREVE | HGK_SMOOTH | HGK_ACUTE;
const BREVE_AND_SMOOTH_AND_GRAVE:u32 = HGK_BREVE | HGK_SMOOTH | HGK_GRAVE;
const BREVE_AND_ROUGH:u32 = HGK_BREVE | HGK_ROUGH;
const BREVE_AND_ROUGH_AND_ACUTE:u32 = HGK_BREVE | HGK_ROUGH | HGK_ACUTE;
const BREVE_AND_ROUGH_AND_GRAVE:u32 = HGK_BREVE | HGK_ROUGH | HGK_GRAVE;
const BREVE_AND_ACUTE:u32 = HGK_BREVE | HGK_ACUTE;
const BREVE_AND_GRAVE:u32 = HGK_BREVE | HGK_GRAVE;

fn get_pua_index(letter:char, diacritics:u32) -> i32 {
    //turn off iota subscript and underdot temporarily 
    //since these are added as combining diacritics later
    let i = match (diacritics & !HGK_IOTA_SUBSCRIPT) & !HGK_UNDERDOT {
        MACRON_AND_SMOOTH           => 0,
        MACRON_AND_SMOOTH_AND_ACUTE => 1,
        MACRON_AND_SMOOTH_AND_GRAVE => 2,
        MACRON_AND_ROUGH            => 3,
        MACRON_AND_ROUGH_AND_ACUTE  => 4,
        MACRON_AND_ROUGH_AND_GRAVE  => 5,
        MACRON_AND_ACUTE            => 6,
        MACRON_AND_GRAVE            => 7,
        BREVE_AND_SMOOTH            => 8,
        BREVE_AND_SMOOTH_AND_ACUTE  => 9,
        BREVE_AND_SMOOTH_AND_GRAVE  => 10,
        BREVE_AND_ROUGH             => 11,
        BREVE_AND_ROUGH_AND_ACUTE   => 12,
        BREVE_AND_ROUGH_AND_GRAVE   => 13,
        BREVE_AND_ACUTE             => 14,
        BREVE_AND_GRAVE             => 15,
        _                           => return -1, //yes, return here
    };
    
    match letter {
        '??' => i,
        '??' => i + 16,
        '??' => i + 32,
        _ => -1,
    }
}

pub enum HgkLetterType {
    HgkLongVowel,
    HgkShortVowel,
    HgkConsonant
}

#[derive(Copy, Clone)]
pub enum HgkUnicodeMode {
    Precomposed,
    CombiningOnly,
    PrecomposedPUA
}

#[derive(PartialEq, Debug)]
pub struct HGKLetter {
    pub letter: char,
    pub diacritics: u32
}

pub trait GreekLetters {
    fn gkletters(&self) -> GreekLetterHolder;
}

impl GreekLetters for str {
    #[inline]
    fn gkletters(&self) -> GreekLetterHolder {
        new_gkletters(self)
    }
}

#[derive(Clone, Debug)]
pub struct GreekLetterHolder<'a> {
    string: &'a str,
    cursor: GreekLetterCursor,
    cursor_back: GreekLetterCursor,
}
/*
impl<'a> GreekLetterHolder<'a> {
    /*
    #[inline]
    ///pub fn as_str(&self) -> &'a str {
    ///    &self.string[self.cursor.cur_cursor()..self.cursor_back.cur_cursor()]
    ///}
    */
}
*/
impl<'a> Iterator for GreekLetterHolder<'a> {
    type Item = HGKLetter;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.cursor_back.cur_cursor() - self.cursor.cur_cursor();
        (cmp::min(slen, 1), Some(slen))
    }

    #[inline]
    fn next(&mut self) -> Option<HGKLetter> {
        let start = self.cursor.cur_cursor();
        if start == self.cursor_back.cur_cursor() {
            return None;
        }

        let r = self.cursor.next_boundary(self.string, 0);
        //println!("next: {} {} {}", start, self.cursor_back.cur_cursor(), r.as_ref().unwrap().as_ref().unwrap().letter);

        Some(r.unwrap().unwrap())
    }
}

impl<'a> DoubleEndedIterator for GreekLetterHolder<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<HGKLetter> {
        let end = self.cursor_back.cur_cursor();
        if end == self.cursor.cur_cursor() {
            return None;
        }
        let prev = self.cursor_back.prev_boundary(self.string, 0);
        Some(prev.unwrap().unwrap())
    }
}

#[inline]
pub fn new_gkletters(s: &str) -> GreekLetterHolder {
    let len = s.len();
    GreekLetterHolder {
        string: s,
        cursor: GreekLetterCursor::new(0, len),
        cursor_back: GreekLetterCursor::new(len, len),
    }
}

#[derive(Clone, Debug)]
pub struct GreekLetterCursor {
    offset: usize,
    len: usize
}

#[derive(PartialEq, Eq, Debug)]
pub enum GreekLetterError {
    InvalidOffset
}

impl GreekLetterCursor {
    pub fn new(offset: usize, len: usize) -> GreekLetterCursor {
        GreekLetterCursor {
            offset,
            len
        }
    }

    // Not sure I'm gonna keep this, the advantage over new() seems thin.
    /// Set the cursor to a new location in the same string.
    pub fn set_cursor(&mut self, offset: usize) {
        if offset != self.offset {
            self.offset = offset;
        }
    }

    #[inline]
    /// The current offset of the cursor. Equal to the last value provided to
    /// `new()` or `set_cursor()`, or returned from `nxext_boundary()` or
    /// `prev_boundary()`.
    pub fn cur_cursor(&self) -> usize {
        self.offset
    }

    #[inline]
    pub fn next_boundary(&mut self, chunk: &str, chunk_start: usize) -> Result<Option<HGKLetter>, GreekLetterError> {

        if self.offset >= self.len {
            unreachable!("should never reach here");
            //return Ok(None);
        }

        let mut the_letter = '\u{0000}';
        let mut diacritics:u32 = 0;

        let mut iter = chunk[self.offset - chunk_start..].chars(); //nfd()
        let mut ch = iter.next().unwrap();
        //println!("next boundary: offset: {} {}", self.offset, ch);
        
        loop {
                if the_letter == '\u{0000}' && !hgk_is_combining(ch) {
                    if ch as u32 >= 0x0370 && ch as u32 <= 0x03FF {
                        //basic greek conversion
                        the_letter = GREEK_BASIC[ch as usize - 0x0370].0;
                        diacritics |= GREEK_BASIC[ch as usize - 0x0370].1;

                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else if ch as u32 >= 0x1F00 && ch as u32 <= 0x1FFF {
                        //extended greek conversion
                        the_letter = GREEK_EXTENDED[ch as usize - 0x1F00].0;
                        diacritics |= GREEK_EXTENDED[ch as usize - 0x1F00].1;
                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else if ch as u32 >= 0xEAF0 && ch as u32 <= 0xEB8A {
                        //PUA conversion
                        the_letter = GREEK_PUA[ch as usize - 0xEAF0].0;
                        diacritics |= GREEK_PUA[ch as usize - 0xEAF0].1;
                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else {
                       the_letter = ch;
                    }
                }
                else if hgk_is_combining(ch) {
                    match ch {
                        '\u{0300}' => diacritics |= HGK_GRAVE,
                        '\u{0301}' => diacritics |= HGK_ACUTE,
                        '\u{0304}' => diacritics |= HGK_MACRON,
                        '\u{0306}' => diacritics |= HGK_BREVE,
                        '\u{0308}' => diacritics |= HGK_DIAERESIS,
                        '\u{0313}' => diacritics |= HGK_SMOOTH,
                        '\u{0314}' => diacritics |= HGK_ROUGH,
                        '\u{0323}' => diacritics |= HGK_UNDERDOT,
                        '\u{0342}' => diacritics |= HGK_CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT,
                        _ => {}
                    }
                }
                else {
                    //self.offset += ch.len_utf8();
                    //else boundary character, return
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics}));
                }

                self.offset += ch.len_utf8();
                if let Some(next_ch) = iter.next() {        
                    ch = next_ch;

                } else if self.offset == self.len {
                    //at the end
                    //println!("herehere2: {}", self.offset);
                    //return Ok(None);
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics}));
                }
                else {
                    return Ok(None);
                }
            }    
        }
    
    #[inline]
    pub fn prev_boundary(&mut self, chunk: &str, chunk_start: usize) -> Result<Option<HGKLetter>, GreekLetterError> {

        if self.offset == 0 {
            unreachable!("should never reach here");
            //return Ok(None);
        }

        let mut the_letter = '\u{0000}';
        let mut diacritics:u32 = 0;

        let mut iter = chunk[..self.offset - chunk_start].chars().rev(); //nfd()
        let mut ch = iter.next().unwrap();
        
        loop {
                if the_letter == '\u{0000}' && !hgk_is_combining(ch) {
                    if ch as u32 >= 0x0370 && ch as u32 <= 0x03FF {
                        //basic greek conversion
                        the_letter = GREEK_BASIC[ch as usize - 0x0370].0;
                        diacritics |= GREEK_BASIC[ch as usize - 0x0370].1;

                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else if ch as u32 >= 0x1F00 && ch as u32 <= 0x1FFF {
                        //extended greek conversion
                        the_letter = GREEK_EXTENDED[ch as usize - 0x1F00].0;
                        diacritics |= GREEK_EXTENDED[ch as usize - 0x1F00].1;
                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else if ch as u32 >= 0xEAF0 && ch as u32 <= 0xEB8A {
                        //PUA conversion
                        the_letter = GREEK_PUA[ch as usize - 0xEAF0].0;
                        diacritics |= GREEK_PUA[ch as usize - 0xEAF0].1;
                        if the_letter == NOT_ACCENTABLE_CHAR || the_letter == '\u{0000}' {
                            the_letter = ch;
                        }
                    }
                    else {
                       the_letter = ch;
                    }

                    //found letter: move offset and return
                    self.offset -= ch.len_utf8();
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics}));
                }
                else if hgk_is_combining(ch) {
                    match ch {
                        '\u{0300}' => diacritics |= HGK_GRAVE,
                        '\u{0301}' => diacritics |= HGK_ACUTE,
                        '\u{0304}' => diacritics |= HGK_MACRON,
                        '\u{0306}' => diacritics |= HGK_BREVE,
                        '\u{0308}' => diacritics |= HGK_DIAERESIS,
                        '\u{0313}' => diacritics |= HGK_SMOOTH,
                        '\u{0314}' => diacritics |= HGK_ROUGH,
                        '\u{0323}' => diacritics |= HGK_UNDERDOT,
                        '\u{0342}' => diacritics |= HGK_CIRCUMFLEX,
                        '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT,
                        _ => {}
                    }
                }
                /*
                is this even reachable??
                else {
                    //self.offset += ch.len_utf8();
                    //else boundary character, return
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics}));
                }*/
                self.offset -= ch.len_utf8();

                if let Some(next_ch) = iter.next() {      
                    ch = next_ch;

                } else if self.offset == 0 {
                    //at the end
                    //println!("herehere2: {} {}", self.offset, diacritics);
                    //return Ok(None);
                    return Ok(Some(HGKLetter{letter:the_letter, diacritics}));
                }
                else {
                    return Ok(None);
                }
            }    
        }

}
/************************************************/

impl HGKLetter {
    pub fn letter_type(&self) -> HgkLetterType {
        if self.letter.is_long() {
            return HgkLetterType::HgkLongVowel;
        }
        else if self.letter.is_long_or_short() {
            if (self.diacritics & HGK_MACRON) == HGK_MACRON {
                return HgkLetterType::HgkLongVowel;
            }
            else {
                return HgkLetterType::HgkShortVowel;
            }
        }
        else if self.letter.is_short() {
            return HgkLetterType::HgkShortVowel;
        }
        else {
            return HgkLetterType::HgkConsonant;
        }
    }

    fn from_str(l:&str) -> HGKLetter {
        let mut diacritics:u32 = 0;
        let mut the_letter: char = '\u{0000}';
        for (i, ch) in l.chars().enumerate() {
            if i == 0 {
                assert!( !hgk_is_combining(ch) ); //"First char of letter is a combining mark."); just ignore it?

                if ch as u32 >= 0x0370 && ch as u32 <= 0x03FF {
                    //basic greek conversion
                    the_letter = GREEK_BASIC[ch as usize - 0x0370].0;
                    diacritics = GREEK_BASIC[ch as usize - 0x0370].1;

                    if the_letter == NOT_ACCENTABLE_CHAR {
                        the_letter = ch;
                    }
                }
                else if ch as u32 >= 0x1F00 && ch as u32 <= 0x1FFF {
                    //extended greek conversion
                    the_letter = GREEK_EXTENDED[ch as usize - 0x1F00].0;
                    diacritics = GREEK_EXTENDED[ch as usize - 0x1F00].1;
                }
                else if ch as u32 >= 0xEAF0 && ch as u32 <= 0xEB8A {
                    //PUA conversion
                    the_letter = GREEK_PUA[ch as usize - 0xEAF0].0;
                    diacritics = GREEK_PUA[ch as usize - 0xEAF0].1;
                }
                else {
                    the_letter = ch;
                }                
            }
            else {
                match ch {
                    '\u{0300}' => diacritics |= HGK_GRAVE,
                    '\u{0301}' => diacritics |= HGK_ACUTE,
                    '\u{0304}' => diacritics |= HGK_MACRON,
                    '\u{0306}' => diacritics |= HGK_BREVE,
                    '\u{0308}' => diacritics |= HGK_DIAERESIS,
                    '\u{0313}' => diacritics |= HGK_SMOOTH,
                    '\u{0314}' => diacritics |= HGK_ROUGH,
                    '\u{0323}' => diacritics |= HGK_UNDERDOT,
                    '\u{0342}' => diacritics |= HGK_CIRCUMFLEX,
                    '\u{0345}' => diacritics |= HGK_IOTA_SUBSCRIPT,
                    _ => break
                }
            }
        }
        
        HGKLetter { letter: the_letter, diacritics }
    }
/*
order:
COMBINING_MACRON, 
COMBINING_BREVE, 
COMBINING_DIAERESIS, 
COMBINING_ROUGH_BREATHING, 
COMBINING_SMOOTH_BREATHING, 
COMBINING_ACUTE, 
COMBINING_GRAVE, 
COMBINING_CIRCUMFLEX, 
COMBINING_IOTA_SUBSCRIPT, 
COMBINING_UNDERDOT
*/
    pub fn to_string(&self, unicode_mode:HgkUnicodeMode) -> String {
        let mut s = vec![self.letter];
        if (self.diacritics & HGK_MACRON) == HGK_MACRON {
            s.push('\u{0304}');
        }
        if (self.diacritics & HGK_BREVE) == HGK_BREVE {
            s.push('\u{0306}');
        }
        if (self.diacritics & HGK_DIAERESIS) == HGK_DIAERESIS {
            s.push('\u{0308}');
        }
        if (self.diacritics & HGK_ROUGH) == HGK_ROUGH {
            s.push('\u{0314}');
        }
        if (self.diacritics & HGK_SMOOTH) == HGK_SMOOTH {
            s.push('\u{0313}');
        }    
        if (self.diacritics & HGK_ACUTE) == HGK_ACUTE {
            s.push('\u{0301}');
        }
        if (self.diacritics & HGK_GRAVE) == HGK_GRAVE {
            s.push('\u{0300}');
        }
        if (self.diacritics & HGK_CIRCUMFLEX) == HGK_CIRCUMFLEX {
            s.push('\u{0342}');
        }
        if (self.diacritics & HGK_IOTA_SUBSCRIPT) == HGK_IOTA_SUBSCRIPT {
            s.push('\u{0345}');
        }
        if (self.diacritics & HGK_UNDERDOT) == HGK_UNDERDOT {
            s.push('\u{0323}');
        }
        match unicode_mode {
            HgkUnicodeMode::CombiningOnly => s.into_iter().collect::<String>(),
            HgkUnicodeMode::PrecomposedPUA => {
                let idx = get_pua_index(self.letter, self.diacritics);
                if (0..=GREEK_LOWER_PUA.len() as i32 - 1 ).contains(&idx) {
                    s.clear();
                    s.push( GREEK_LOWER_PUA[idx as usize] );

                    if (self.diacritics & HGK_IOTA_SUBSCRIPT) == HGK_IOTA_SUBSCRIPT {
                        s.push('\u{0345}');
                    }
                    if (self.diacritics & HGK_UNDERDOT) == HGK_UNDERDOT {
                        s.push('\u{0323}');
                    }
                    s.into_iter().collect::<String>() 
                }
                else {
                    s.into_iter().nfc().collect::<String>() 
                }
            },
            _ => s.into_iter().nfc().collect::<String>()
        }  
    }

    pub fn toggle_diacritic(&mut self, d:u32, on_only:bool) {
        if !self.is_legal(d) {
            return;
        }

        if self.diacritics & d != d || on_only {
            self.diacritics |= d;
        }
        else {
            self.diacritics &= !d; //turn off: rust uses !, C uses ~
            //return;
        }

        //turn off clashing diacritics:
        match d {
            HGK_ROUGH => {
                self.diacritics &= !(HGK_SMOOTH | HGK_DIAERESIS);
            },
            HGK_SMOOTH => {
                self.diacritics &= !(HGK_ROUGH | HGK_DIAERESIS);
            },
            HGK_ACUTE => {
                self.diacritics &= !(HGK_GRAVE | HGK_CIRCUMFLEX);
            },
            HGK_GRAVE => {
                self.diacritics &= !(HGK_ACUTE | HGK_CIRCUMFLEX);
            },
            HGK_CIRCUMFLEX => {
                self.diacritics &= !(HGK_ACUTE | HGK_GRAVE | HGK_MACRON | HGK_BREVE);
            },
            HGK_MACRON => {
                self.diacritics &= !(HGK_BREVE | HGK_CIRCUMFLEX);
            },
            HGK_BREVE => {
                self.diacritics &= !(HGK_MACRON | HGK_CIRCUMFLEX | HGK_IOTA_SUBSCRIPT);
            },
            HGK_IOTA_SUBSCRIPT => {
                self.diacritics &= !(HGK_BREVE);
            },
            HGK_DIAERESIS => {
                self.diacritics &= !(HGK_ROUGH | HGK_SMOOTH);
            },
            HGK_UNDERDOT => { },
            _ => {
                debug_assert!(false, "Unknown Diacritic passed")
            }
        }
    }

    fn is_legal(&mut self, d:u32) -> bool {
        match d {
            HGK_ROUGH => {
                self.letter.is_greek_vowel() || self.letter == '??' || self.letter == '??'
            },
            HGK_SMOOTH => {
                self.letter.is_greek_vowel() || self.letter == '??' || self.letter == '??'
            },
            HGK_ACUTE => {
                self.letter.is_greek_vowel()
            },
            HGK_GRAVE => {
                self.letter.is_greek_vowel()
            },
            HGK_CIRCUMFLEX => {
                self.letter.is_long_or_short() || self.letter.is_long()
            },
            HGK_MACRON => {
                self.letter.is_long_or_short()
            },
            HGK_BREVE => {
                self.letter.is_long_or_short()    
            },
            HGK_IOTA_SUBSCRIPT => {
                matches!(self.letter, '??' | '??' | '??') 
            },
            HGK_DIAERESIS => {
                matches!(self.letter, '??' | '??')                
            },
            HGK_UNDERDOT => { 
                true
            },
            _ => false
        }
    }
}
/*
//https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html
impl Display for HGKLetter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::result::Result {
        write!(f, "{}", self.to_string2(HgkUnicodeMode::Precomposed));
    }
}
*/
pub trait HGKIsLong {
    fn is_long(&self) -> bool;
}

impl HGKIsLong for char {
    fn is_long(&self) -> bool {
        matches!(self, '??' | '??' | '??' | '??')
    }
}

pub trait HGKIsShort {
    fn is_short(&self) -> bool;
}

impl HGKIsShort for char {
    fn is_short(&self) -> bool {
        matches!(self, '??' | '??' | '??' | '??')
    }
}

pub trait HGKIsLongOrShort {
    fn is_long_or_short(&self) -> bool;
}

impl HGKIsLongOrShort for char {
    fn is_long_or_short(&self) -> bool {
        matches!(self, '??' | '??' | '??' | '??' | '??' | '??')
    }
}

pub trait HGKIsGreekVowel {
    fn is_greek_vowel(&self) -> bool;
}

impl HGKIsGreekVowel for char {
    fn is_greek_vowel(&self) -> bool {
        //let letter2 = self.to_lowercase();
        matches!(self, '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??' | '??')
    }
}

pub fn hgk_strip_diacritics(l:&str, turnoff_diacritics:u32) -> String {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    l.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:a.diacritics & !turnoff_diacritics}.to_string(HgkUnicodeMode::Precomposed)).collect::<String>()
}

pub fn hgk_strip_diacritics_and_replace_circumflex_with_macron(l:&str, turnoff_diacritics:u32) -> String {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    l.gkletters().map(|a| {let d = if (a.diacritics & HGK_CIRCUMFLEX ) == HGK_CIRCUMFLEX && (a.letter == '??' ) { a.diacritics | HGK_MACRON } else {a.diacritics}; HGKLetter{letter:a.letter, diacritics: d & !turnoff_diacritics}.to_string(HgkUnicodeMode::Precomposed)}).collect::<String>()
}

//returns true if one or more of the bits in check_diacritics is/are set
pub fn hgk_has_diacritics(l:&str, check_diacritics:u32) -> bool {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    
    //turn off all other bits, see if it equals 0 or not
    for a in l.gkletters() { //.map(|a| HGKLetter{letter:a.letter, diacritics:a.diacritics & !turnoff_diacritics}.to_string(HgkUnicodeMode::PrecomposedPUA)).collect::<String>()
        if (a.diacritics & check_diacritics) != 0 {
            return true;
        }
    }
    false
}

pub fn hgk_convert(l:&str, mode:HgkUnicodeMode) -> String {
    //let b = l.gkletters();
    //println!("num: {}", b.collect::<Vec<HGKLetter>>().len() );
    l.gkletters().map(|a| a.to_string(mode)).collect::<String>()
}

pub fn hgk_toggle_diacritic_str(l:&str, d:u32, on_only:bool, mode:HgkUnicodeMode) -> String {
    let mut letter = HGKLetter::from_str(l);
    letter.toggle_diacritic(d, on_only);
    letter.to_string(mode)
}

pub fn hgk_compare_sqlite(s1: &str, s2: &str) -> Ordering {
    match hgk_compare(s1, s2, 0xFFFFFFFF) {
        1 => Ordering::Greater,
        -1 => Ordering::Less,
        _ => Ordering::Equal
    }
}

//set compare_type to 0xFFFF for diacritic insensitive
pub fn hgk_compare(a:&str, b:&str, compare_type:u32) -> i32 {
    let mut a1 = a.gkletters();
    let mut b1 = b.gkletters();

    let mut a_letter:Option<HGKLetter>;
    let mut b_letter:Option<HGKLetter>;

    loop  {
        a_letter = a1.next();
        b_letter = b1.next();
        if a_letter.is_none() || b_letter.is_none() {
            break;
        }

        /*
        //skip non-greek chars if option is set
        if ((compareType & _HK_IGNORE_UNKNOWN_CHARS) == _HK_IGNORE_UNKNOWN_CHARS && type1 == NOCHAR) {
            continue;
        }
        else if ((compareType & _HK_IGNORE_UNKNOWN_CHARS) == _HK_IGNORE_UNKNOWN_CHARS && type2 == NOCHAR) {
            continue;
        }
        */

        let lettera = a_letter.as_ref().unwrap().letter as usize;
        let letterb = b_letter.as_ref().unwrap().letter as usize;

        //if one or both characters are out of the greek range
        if !(0x0370..=0x03FF).contains(&lettera) && !(0x0370..=0x03FF).contains(&letterb) {
            match lettera.cmp(&letterb) {
                 Ordering::Less => return -1,
                 Ordering::Greater => return 1,
                 Ordering::Equal => return 0
            }
        }
        else if !(0x0370..=0x03FF).contains(&lettera) { //non-greek sorts before greek 
            return -1;
        }
        else if !(0x0370..=0x03FF).contains(&letterb) { //non-greek sorts before greek 
            return 1;
        }

        let a_sort:u32 = GREEK_BASIC[lettera - 0x0370].2;
        let b_sort:u32 = GREEK_BASIC[letterb - 0x0370].2;

        //if one letter sorts less than the other
        match a_sort.cmp(&b_sort) {
             Ordering::Less => return -1,
             Ordering::Greater => return 1,
             Ordering::Equal => ()
        }

        if (a_letter.as_ref().unwrap().diacritics & !compare_type) != (b_letter.as_ref().unwrap().diacritics & !compare_type) {
            if (a_letter.unwrap().diacritics & !compare_type) < (b_letter.unwrap().diacritics & !compare_type) {
                return -1;
            }
            else {
                return 1;
            }
        }
    }
    //here we have reached the end of one or both strings and they are still completely equal

    if a_letter.is_none() && b_letter.is_none() { //both at end
        0
    }
    else if a_letter.is_none() {//1 at end
        -1
    }
    else { //2 at end
        1
    }
}

#[inline]
pub fn hgk_is_combining(c:char) -> bool {
    match c {
        '\u{0300}' => true,
        '\u{0301}' => true,
        '\u{0304}' => true,
        '\u{0306}' => true,
        '\u{0308}' => true,
        '\u{0313}' => true,
        '\u{0314}' => true,
        '\u{0323}' => true,
        '\u{0342}' => true,
        '\u{0345}' => true,
        _ => { false }
    }
}

pub fn hgk_transliterate(input:usize) -> char {
    if (0x0061..=0x007A).contains(&input) {
        GREEK_LOWER[input - 0x0061]
    }
    else if (0x0041..=0x005A).contains(&input) {
        GREEK_UPPER[input - 0x0041]
    }
    else {
        '\u{0000}'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use unicode_normalization::char::compose;
    use alloc::vec::Vec;
    use csv;
    use std::error::Error;
    use std::path::Path;

    fn do_csv_test() -> Result<(), Box<dyn Error>> {
        //println!("{:?}", env::current_dir().unwrap());
        let csvfile = "gktest.csv";
        if !Path::new(csvfile).is_file() {
            Err("CSV tests file does not exist")? //or: return Err("Bad request".into());
        }

        let mut rdr = csv::Reader::from_path(csvfile)?; //Reader::from_reader(io::stdin());
        let mut line_number = 2; //start on line 2 because header row
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the error here.
            let record = result?;

            let diacritic = match record[1].trim() {
                //"none" => HGK_NO_DIACRITICS,
                "rough" => HGK_ROUGH,
                "smooth" => HGK_SMOOTH,
                "acute" => HGK_ACUTE,
                "grave" => HGK_GRAVE,
                "circumflex" => HGK_CIRCUMFLEX,
                "macron" => HGK_MACRON,
                "breve" => HGK_BREVE,
                "iotasub" => HGK_IOTA_SUBSCRIPT,
                "diaeresis" => HGK_DIAERESIS,
                "underdot" => HGK_UNDERDOT,
                _ => panic!("Invalid diacritic on line: {}.", line_number)
            };

            let only_on = match record[2].trim() {
                "onlyon" => true,
                "toggleoff" => false,
                _ => panic!("Invalid toggle off on line: {}.", line_number)
            };

            let mode = match record[3].trim() {
                "CombiningOnly" => HgkUnicodeMode::CombiningOnly,
                "PrecomposedPUA" => HgkUnicodeMode::PrecomposedPUA,
                "Precomposed" => HgkUnicodeMode::Precomposed,
                _ => panic!("Invalid unicode mode on line: {}.", line_number)
            };

            let is_equal = match record[5].trim() {
                "equal" => true,
                "notequal" => false,
                _ => panic!("Invalid equal on line: {}.", line_number)
            };

            if is_equal {
                assert_eq!(hgk_toggle_diacritic_str(&hex_to_string(&record[0]), diacritic, only_on, mode), hex_to_string(&record[4]), "Error on line: {}.", line_number);
            }
            else {
                assert_ne!(hgk_toggle_diacritic_str(&hex_to_string(&record[0]), diacritic, only_on, mode), hex_to_string(&record[4]), "Error on line: {}.", line_number);
            }
            line_number += 1;
        }

        Ok(())
    }

    //make string from utf16 hex codepoints
    fn hex_to_string(s:&str) -> String {
        //https://stackoverflow.com/questions/3408706/hexadecimal-string-to-byte-array-in-c
        let b = hex::decode(s.replace(" ", "")).unwrap();

        let res: Vec<u16> = b
        .chunks_exact(2)
        .into_iter()
        .map(|a| u16::from_be_bytes([a[0], a[1]]))
        .collect();

        String::from_utf16( res.as_slice() ).unwrap()
    }

    #[test]
    fn test_compare() {

        assert_eq!( hgk_compare("??", "??", 0), 0);
        assert_eq!( hgk_compare("??", "??", 0), 0);
        assert_eq!( hgk_compare("??", "??", 0), 0);
        assert_eq!( hgk_compare("??", "??", 0), -1);
        assert_eq!( hgk_compare("??", "??", 0), 1);
        assert_eq!( hgk_compare("??", "???", 0), 1);

        assert_eq!( hgk_compare("??", "??", 0), 0);
        assert_eq!( hgk_compare("??", "??", 0), -1);
        assert_eq!( hgk_compare("??", "??", 0), 1);


        assert_eq!( hgk_compare("????", "??", 0), 1);
        assert_eq!( hgk_compare("??", "????", 0), -1);
        assert_eq!( hgk_compare("????", "??", 0), -1);
        assert_eq!( hgk_compare("??", "????", 0), 1);
      
        assert_eq!( hgk_compare("???", "??", 0xFFFFFFFF), 0);

        assert_eq!( hgk_compare_sqlite("??", "??"), Ordering::Less );
        assert_eq!( hgk_compare_sqlite("??", "??"), Ordering::Greater );
        assert_eq!( hgk_compare_sqlite("???", "??"), Ordering::Equal );
        assert_eq!( hgk_compare_sqlite("??", "???"), Ordering::Equal );

        //custom sort
        let mut v = vec!["??????", "??????", "??????,??????", "\u{EB07}??????", "???? ??????"];
        v.sort_by(|a, b| hgk_compare_sqlite(a, b));
        assert_eq!(v, vec!["???? ??????", "??????", "??????,??????", "\u{EB07}??????", "??????"]);
    }

    #[test]
    fn csv_tests() {
        match do_csv_test() {
            Ok(()) => (),
            Err(error) => panic!("Error: {:?}", error)
        };
    }

    #[test]
    fn native_unicode() {
        //nfd-> nfc -> nfd round trip
        assert_eq!("\u{1F04}".nfd().collect::<String>(), "\u{03B1}\u{0313}\u{0301}");
        assert_eq!("\u{03B1}\u{0313}\u{0301}".nfc().collect::<String>(), "\u{1F04}");

        assert_eq!("\u{EAF0}".nfd().next(), Some('\u{EAF0}'));
        assert_eq!("\u{EAF0}".nfd().count(), 1);

        let s = "???????".to_string();
        let a = s.nfd();
        assert_eq!(a.count(), 6);
        
        //let z4 = "\u{EAF0}".nfd();
        //println!("test pua: {}", z4);

        //let str = "?????????";
        //let str2 = str.nfd().chars().iter().filter(|x| !unicode_normalization::char::is_combining_mark(x))

        assert_eq!(compose('A','\u{30a}'), Some('??'));

        let s = "????";
        let c = s.nfc().collect::<String>();
        assert_eq!(c, "????");

        assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
        assert_eq!(compose('\u{03B1}','\u{0301}'), Some('\u{03AC}'));
        assert_eq!('a','a');

        let a = "\u{03B1}\u{0301}";
        let b = "\u{03AC}";
        assert_ne!(a, b);

        let s = String::from("???");
        let _v: Vec<char> = s.chars().collect();
    }

    #[test]
    fn vowel_lengths() {
        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), true);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), true);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), true);
        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), true);
        assert_eq!('??'.is_short(), false);

        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), true);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), true);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), true);
        assert_eq!('??'.is_long_or_short(), true);
        assert_eq!('??'.is_long(), false);
        assert_eq!('??'.is_short(), false);
        assert_eq!('??'.is_long_or_short(), false);
        assert_eq!('??'.is_long(), true);
        assert_eq!('??'.is_short(), false);
    }

    #[test]
    fn iterator_tests() {
        let s = "??\u{0304}\u{0313}\u{0301}????\u{0313}\u{0301}";//"\u{EB07}?????";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "??\u{0304}\u{0313}\u{0301}????????";//"\u{EB07}?????";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}?????";//"????????????????";//
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{1F04}?????";//"????????????????";//
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        
        let mut aaa = "????????".gkletters();
        assert_eq!(aaa.next().unwrap().letter, '??');
        assert_eq!(aaa.next().unwrap().letter, '??');
        assert_eq!(aaa.next().unwrap().letter, '??');
        assert_eq!(aaa.next(), None);

        let mut aaa = "????????".gkletters();
        assert_eq!(aaa.next_back().unwrap().letter, '??');
        assert_eq!(aaa.next_back().unwrap().letter, '??');
        assert_eq!(aaa.next_back().unwrap().letter, '??');
        assert_eq!(aaa.next_back(), None);


        let mut aaa = "\u{1FE1}".gkletters();
        assert_eq!(aaa.next().unwrap(), HGKLetter{letter:'??', diacritics:HGK_MACRON});

        let mut aaa = "??\u{0304}".gkletters();
        assert_eq!(aaa.next_back().unwrap(), HGKLetter{letter:'??', diacritics:HGK_MACRON});

        let mut aaa = "????\u{0304}??".gkletters();
        assert_eq!(aaa.next_back().unwrap(), HGKLetter{letter:'??', diacritics:0});
        assert_eq!(aaa.next_back().unwrap(), HGKLetter{letter:'??', diacritics:HGK_MACRON});
        assert_eq!(aaa.next_back().unwrap(), HGKLetter{letter:'??', diacritics:0});
        assert_eq!(aaa.next_back(), None);

        let s = "??????";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:0} ];
        assert_eq!(g, b);

        let s = "????????";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}?????";
        let g = s.gkletters().collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_MACRON | HGK_SMOOTH},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:HGK_ACUTE | HGK_SMOOTH} ];
        assert_eq!(g, b);

        let s = "\u{EB07}??????";
        let xxx = s.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:0} ).collect::<Vec<HGKLetter>>();
        let b: &[_] = &[HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:0},HGKLetter{letter:'??', diacritics:0} ];
        assert_eq!(xxx, b);

        let s = "\u{EB07}?????";
        let xxx = s.gkletters().map(|a| HGKLetter{letter:a.letter, diacritics:0}.to_string(HgkUnicodeMode::PrecomposedPUA)).collect::<String>();
        assert_eq!(xxx, "??????");
    }

    
    #[test]
    fn convert_tests() {
        for l in 0x0370..0x03FF {
            let letter = std::char::from_u32(l).unwrap().to_string();

            let a = letter.nfd().collect::<String>();
            let b = a.nfc().collect::<String>();
            println!("{:X}, {}, {}, {}", l, letter, a, b);

            //where the round trip should not be equal
            match l {
                0x0374 => (), // numeral sign
                0x037E => (), // question mark
                0x0387 => (), // raised dot
                _ => {             
                    assert_eq!(letter, b);
                    
                    let aa = hgk_convert(&letter, HgkUnicodeMode::CombiningOnly);
                    //where hgk_convert is different from nfd()
                    match l {
                            0x0385 => (), // GREEK DIALYTIKA TONOS
                            _ => {
                            assert_eq!(aa, a);
                        }
                    }                       
                }
            }
        }

        for l in 0x1F00..0x1FFF {
            let letter = std::char::from_u32(l).unwrap().to_string();

            let a = letter.nfd().collect::<String>();
            let b = a.nfc().collect::<String>();
            println!("{:X}, {}, {}, {}", l, letter, a, b);

            //where the round trip should not be equal
            match l  {
                0x1F71 => (), //alpha with acute -> tonos
                0x1F73 => (), //epsilon with acute -> tonos
                0x1F75 => (), //eta with acute -> tonos
                0x1F77 => (), //iota with acute -> tonos
                0x1F79 => (), //omicron with acute -> tonos
                0x1F7B => (), //upsilon with acute -> tonos
                0x1F7D => (), //omega with acute -> tonos
                0x1FBB => (), //cap alpha with acute -> tonos
                0x1FBE => (), //iota adscript -> small iota (03B9)
                0x1FC9 => (), //cap epsilon with acute -> tonos
                0x1FCB => (), //cap eta with acute -> tonos
                0x1FD3 => (), //iota diaeresis acute -> tonos
                0x1FDB => (), //cap iota with acute -> tonos
                0x1FE3 => (), //upsilon diaeresis acute -> tonos
                0x1FEB => (), //cap upsilon with acute -> tonos
                0x1FEE => (), //diaeresis tonos
                0x1FEF => (), //grave
                0x1FF9 => (), //cap omicron with acute -> tonos
                0x1FFB => (), //cap omega with acute -> tonos
                0x1FFD => (), //acute
                _ => {
                    //otherwise round trip will be equal
                    assert_eq!(letter, b);

                    let aa = hgk_convert(&letter, HgkUnicodeMode::CombiningOnly);
                    //where hgk_convert is different from nfd()
                    match l  {
                        0x1FC1 => (), //circumflex diaeresis
                        0x1FCD => (), //smooth grave
                        0x1FCE => (), //smooth acute
                        0x1FCF => (), //smooth cirumflex
                        0x1FDD => (), //rough grave
                        0x1FDE => (), //rough acute
                        0x1FDF => (), //rough circumflex
                        0x1FED => (), //grave diaeresis
                        _ => {
                            assert_eq!(aa, a);
                        }
                    }
                }
            }
        }

        for l in 0xEAF0..0xEB8A {
            let letter = std::char::from_u32(l).unwrap().to_string();

            let a = hgk_convert(&letter, HgkUnicodeMode::CombiningOnly);
            let b = hgk_convert(&a, HgkUnicodeMode::PrecomposedPUA);
            assert_eq!(letter, b);
        }
    }
    
    #[test]
    fn mytest() {
        //println!("{:?}", env::current_dir().unwrap());

        assert_eq!(hex_to_string("03B1 0304 03B2"), "??\u{0304}??");

        assert_eq!( hgk_strip_diacritics("?????", 0xFFFFFFFF), "????" );
        assert_eq!( hgk_strip_diacritics("\u{EB07}", 0xFFFFFFFF), "??" );
        assert_eq!( hgk_strip_diacritics("??\u{0304}\u{0313}\u{0301}", 0xFFFFFFFF), "??" );


        assert_eq!( hgk_has_diacritics("?????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), true);
        assert_eq!( hgk_has_diacritics("?????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), true);
        assert_eq!( hgk_has_diacritics("????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), true);
        assert_eq!( hgk_has_diacritics("?????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), false);
        assert_eq!( hgk_has_diacritics("?????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), true);
        assert_eq!( hgk_has_diacritics("?????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), true);
        assert_eq!( hgk_has_diacritics("????", (HGK_ACUTE | HGK_CIRCUMFLEX | HGK_GRAVE)), false);
        
        assert_eq!( hgk_convert("\u{EB07}", HgkUnicodeMode::CombiningOnly), "??\u{0304}\u{0313}\u{0301}");
        assert_eq!( hgk_convert("??\u{0304}\u{0313}\u{0301}", HgkUnicodeMode::PrecomposedPUA), "\u{EB07}");

        assert_eq!(GREEK_LOWER_PUA.len() as i32 - 1, 47);

        assert_eq!(MACRON_AND_SMOOTH, HGK_MACRON | HGK_SMOOTH);

        assert_eq!(hgk_transliterate(0x0000), '\u{0000}');
        assert_eq!(hgk_transliterate(0x0040), '\u{0000}');
        assert_eq!(hgk_transliterate(0x0061), '\u{03B1}');
        assert_eq!(hgk_transliterate(0x007B), '\u{0000}');
        
        let _aa = HGKLetter::from_str("\u{EAF0}");

        let a2 = HGKLetter::from_str("\u{03B1}\u{0301}");
        assert_eq!(a2.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_eq!(a2.letter, '\u{03B1}');
        let a3 = HGKLetter::from_str("\u{03AC}");
        assert_eq!(a3.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_eq!(a3.letter, '\u{03B1}');

        let mut s: HGKLetter = HGKLetter { letter: '??', diacritics: HGK_ACUTE | HGK_GRAVE };
        assert_eq!(s.diacritics & HGK_ACUTE, HGK_ACUTE);
        assert_ne!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);

        s.toggle_diacritic(HGK_CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);
        //don't toggle off, if on_only is set
        s.toggle_diacritic(HGK_CIRCUMFLEX, true);
        assert_eq!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);
        //turn off
        s.toggle_diacritic(HGK_CIRCUMFLEX, false);
        assert_ne!(s.diacritics & HGK_CIRCUMFLEX, HGK_CIRCUMFLEX);

 
        let mut a1 = HGKLetter::from_str("??");
        assert_eq!(a1.letter, '??');
        assert_eq!(a1.diacritics, HGK_NO_DIACRITICS);
        a1.toggle_diacritic(HGK_MACRON, false);
        assert_eq!(a1.letter, '??');
        assert_eq!(a1.diacritics, HGK_MACRON);
        assert_eq!(get_pua_index(a1.letter, a1.diacritics), -1);
        assert_eq!(a1.to_string(HgkUnicodeMode::PrecomposedPUA), "\u{1FE1}");

        assert_eq!(hgk_toggle_diacritic_str("??", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{1FE1}");

        assert_eq!(hgk_toggle_diacritic_str("??", HGK_UNDERDOT, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{03B1}\u{0323}");

        assert_eq!(hgk_toggle_diacritic_str("???", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB04}");
        assert_eq!(hgk_toggle_diacritic_str("???", HGK_MACRON, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB07}");
        assert_eq!(hgk_toggle_diacritic_str("???", HGK_BREVE, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{EB83}");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_ACUTE, false, HgkUnicodeMode::PrecomposedPUA), 
            "\u{03AC}");

        assert_eq!(hgk_toggle_diacritic_str("??", HGK_ACUTE, false, HgkUnicodeMode::Precomposed), 
            "\u{03AC}");//??");
        assert_eq!(hgk_toggle_diacritic_str("???", HGK_ACUTE, false, HgkUnicodeMode::Precomposed), 
            "\u{03AC}");//??");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_ACUTE, false, HgkUnicodeMode::CombiningOnly), 
            "\u{03B1}\u{0301}");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_CIRCUMFLEX, false, HgkUnicodeMode::CombiningOnly), 
            "\u{03B1}\u{0342}");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "\u{1FB6}");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "??");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_CIRCUMFLEX, false, HgkUnicodeMode::Precomposed), 
            "???");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "???");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "???");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "???");
        assert_eq!(hgk_toggle_diacritic_str("??", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "??\u{0313}"); //there is no precomposed capital rho with smooth breathing
        assert_eq!(hgk_toggle_diacritic_str("??\u{0313}", HGK_SMOOTH, false, HgkUnicodeMode::Precomposed), 
            "??");
        assert_eq!(hgk_toggle_diacritic_str("??\u{0313}", HGK_ROUGH, false, HgkUnicodeMode::Precomposed), 
            "???");
        assert_eq!(hgk_toggle_diacritic_str("??\u{0313}", HGK_ROUGH, false, HgkUnicodeMode::CombiningOnly), 
            "??\u{0314}");
    }
}
