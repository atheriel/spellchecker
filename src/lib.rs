#![feature(libc, convert, cstr_to_str, path_ext)]

extern crate libc;
extern crate encoding;

use std::ffi::{CStr, CString};
use std::fs::PathExt;
use std::path::Path;
use std::ptr;

use encoding::types::{EncodingRef, DecoderTrap, EncoderTrap};
use encoding::label::encoding_from_whatwg_label;

mod ffi;

pub struct Hunspell {
    handle: *mut ffi::Hunhandle,
    encoding: EncodingRef,
}

impl Hunspell {
    pub fn create<T: AsRef<Path>>(affpath: T, dpath: T) -> Hunspell {
        if !affpath.as_ref().exists() || !dpath.as_ref().exists() {
            panic!("nonexistent dictionary or affix path");
        }

        // These paths should never contain interior \0 bytes (if they were
        // created through Rust), so it is safe to unwrap() them here.
        let affcpath = affpath.as_ref().as_os_str().to_cstring().unwrap();
        let dcpath = dpath.as_ref().as_os_str().to_cstring().unwrap();

        let handle = unsafe {
            ffi::Hunspell_create(affcpath.as_ptr(), dcpath.as_ptr())
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

    pub fn suggestions_for(&self, word: &str) -> Vec<String> {
        let word = self.encoding.encode(word, EncoderTrap::Strict).unwrap();
        let cword = CString::new(word).unwrap();

        // Create an empty array of C strings for Hunspell to fill.
        let mut suggestions: *mut *mut libc::c_char = ptr::null_mut();

        let length = unsafe {
            ffi::Hunspell_suggest(self.handle, &mut suggestions, cword.as_ptr())
        };
        if length <= 0 || suggestions.is_null() {
            return Vec::new()
        }
        let length = length as usize;
        let mut results = Vec::with_capacity(length);

        for i in 0..length {
            let suggestion = unsafe {
                let sug_ptr: *const libc::c_char = *suggestions.offset(i as isize);
                if sug_ptr.is_null() {
                    panic!("null pointer in suggestions lists");
                }
                CStr::from_ptr(sug_ptr)
            };

            let decoded = self.encoding.decode(suggestion.to_bytes(), DecoderTrap::Strict).unwrap();

            results.push(decoded);
        }

        // Deallocate the suggestion list provided by Hunspell. Otherwise, we
        // will leak memory.
        unsafe {
            ffi::Hunspell_free_list(self.handle, &mut suggestions, length as libc::c_int)
        };

        results
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
fn test_suggestions() {
    let spellchecker = Hunspell::create("dict/en_CA.aff", "dict/en_CA.dic");
    let slist = vec![
        "dog",
        "doe",
        "doges",
        "dogie",
        "dodge",
        "dogs",
        "dose",
        "done",
        "dote",
        "dole",
        "loge",
        "dome",
        "dope",
        "dogy",
        "dove"];

    assert_eq!(spellchecker.suggestions_for("doge"), slist);
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
