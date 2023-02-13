use super::super::*;
use libc::*;

extern "C" {
    pub fn SSL_ech_set1_echconfig(s: *mut SSL, val: *mut c_uchar, len: c_ulong) -> c_int;
    pub fn SSL_CTX_ech_set1_echconfig(s: *mut SSL_CTX, val: *mut c_uchar, len: c_ulong) -> c_int;
    pub fn SSL_CTX_ech_set_outer_alpn_protos(
        s: *mut SSL_CTX,
        protos: *const c_uchar,
        protos_len: c_ulong,
    ) -> c_int;
    pub fn SSL_ech_set_outer_server_name(
        s: *mut SSL,
        outer_name: *const c_char,
        no_outer: c_int,
    ) -> c_int;
}
