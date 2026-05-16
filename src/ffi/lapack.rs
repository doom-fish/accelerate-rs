#![allow(missing_docs, non_camel_case_types, non_upper_case_globals)]

pub type __CLPK_integer = i32;

#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    pub fn sgetrf_(
        m: *mut __CLPK_integer,
        n: *mut __CLPK_integer,
        a: *mut f32,
        lda: *mut __CLPK_integer,
        ipiv: *mut __CLPK_integer,
        info: *mut __CLPK_integer,
    );
    pub fn sgesv_(
        n: *mut __CLPK_integer,
        nrhs: *mut __CLPK_integer,
        a: *mut f32,
        lda: *mut __CLPK_integer,
        ipiv: *mut __CLPK_integer,
        b: *mut f32,
        ldb: *mut __CLPK_integer,
        info: *mut __CLPK_integer,
    );
}
