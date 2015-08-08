#![feature(libc)]

extern crate libc;

use std::ffi::CString;

mod ffi;

pub struct Hunspell {
    handle: *mut ffi::Hunhandle,
}

impl Hunspell {
    pub fn create(affpath: &str, dpath: &str) -> Hunspell {
        // TODO: Check both paths.

        let affpath = CString::new(affpath).unwrap();
        let dpath = CString::new(dpath).unwrap();

        let handle = unsafe {
            ffi::Hunspell_create(affpath.as_ptr(), dpath.as_ptr())
        };

        Hunspell { handle: handle }
    }

    pub fn spelling(&self, word: &str) -> bool {
        let word = CString::new(word).unwrap();

        match unsafe { ffi::Hunspell_spell(self.handle, word.as_ptr()) } {
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
