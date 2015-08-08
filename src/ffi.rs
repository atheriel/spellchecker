use libc;

pub enum Hunhandle {}

#[link(name = "hunspell-1.3.0")]
extern {
    pub fn Hunspell_create(affpath: *const libc::c_char, dpath: *const libc::c_char) -> *mut Hunhandle;
    pub fn Hunspell_destroy(pHunspell: *mut Hunhandle);
    pub fn Hunspell_get_dic_encoding(pHunspell: *mut Hunhandle) -> *const libc::c_char;

    pub fn Hunspell_spell(pHunspell: *mut Hunhandle, word: *const libc::c_char) -> libc::c_int;
}
