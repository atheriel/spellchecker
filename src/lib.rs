#![feature(libc, cstr_to_str)]

extern crate libc;
extern crate encoding;

use std::ffi::{CStr, CString};

use encoding::types::{EncodingRef, EncoderTrap};
use encoding::label::encoding_from_whatwg_label;

mod ffi;

pub struct Hunspell {
    handle: *mut ffi::Hunhandle,
    encoding: EncodingRef,
}

impl Hunspell {
    pub fn create(affpath: &str, dpath: &str) -> Hunspell {
        // TODO: Check both paths.

        let affpath = CString::new(affpath).unwrap();
        let dpath = CString::new(dpath).unwrap();

        let handle = unsafe {
            ffi::Hunspell_create(affpath.as_ptr(), dpath.as_ptr())
        };

        let enc = get_hunspell_encoding(handle).unwrap();

        Hunspell { handle: handle, encoding: enc }
    }

    pub fn encoding(&self) -> &str {
        self.encoding.name()
    }

    pub fn spelling(&self, word: &str) -> bool {
        let word = self.encoding.encode(word, EncoderTrap::Strict).unwrap();
        let cword = CString::new(word).unwrap();

        match unsafe { ffi::Hunspell_spell(self.handle, cword.as_ptr()) } {
            0 => false,
            _ => true,
        }
    }

    #[allow(unused_variables)]
    pub fn suggestions_for(&self, word: &str) -> Vec<String> {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn add_word(&mut self, word: &str) {
        unimplemented!()
    }

    #[allow(unused_variables)]
    pub fn remove_word(&mut self, word: &str) {
        unimplemented!()
    }
}

impl Drop for Hunspell {
    fn drop(&mut self) {
        unsafe { ffi::Hunspell_destroy(self.handle); }
    }
}

fn get_hunspell_encoding(handle: *mut ffi::Hunhandle) -> Option<EncodingRef> {
    let enc_ptr = unsafe {ffi::Hunspell_get_dic_encoding(handle) };
    if enc_ptr.is_null() {
        return None;
    }
    let enc_str = unsafe { CStr::from_ptr(enc_ptr).to_string_lossy() };
    encoding_from_whatwg_label(&enc_str)
}

#[test]
fn test_spelling() {
    let spellchecker = Hunspell::create("dict/en_CA.aff", "dict/en_CA.dic");

    assert_eq!(spellchecker.spelling("dog"), true);
    assert_eq!(spellchecker.spelling("dogw"), false);
}

#[test]
fn test_nonword() {
    let spellchecker = Hunspell::create("dict/en_CA.aff", "dict/en_CA.dic");

    assert_eq!(spellchecker.spelling("dog w"), false);
}

#[test]
fn test_whitespace() {
    let spellchecker = Hunspell::create("dict/en_CA.aff", "dict/en_CA.dic");

    assert_eq!(spellchecker.spelling(" "), true);
    assert_eq!(spellchecker.spelling(""), true);
}

#[test]
fn test_encoding() {
    let spellchecker = Hunspell::create("dict/en_CA.aff", "dict/en_CA.dic");

    assert_eq!(spellchecker.encoding(), "utf-8");
}
