#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(label_break_value)]
#![feature(register_tool)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
extern crate libc;
pub mod src {
pub mod algo_hmac;
pub mod algo_md5;
pub mod algo_sha1;
pub mod algo_splaytree;
pub mod algo_xxhash;
pub mod array;
pub mod base64;
pub mod buffer;
pub mod burl;
pub mod chunk;
pub mod ck;
pub mod configfile;
pub mod configfile_glue;
pub mod configparser;
pub mod connections;
pub mod data_config;
pub mod fdevent;
pub mod fdevent_fdnode;
pub mod fdevent_impl;
pub mod fdlog;
pub mod fdlog_maint;
pub mod gw_backend;
pub mod h2;
pub mod http_cgi;
pub mod http_chunk;
pub mod http_date;
pub mod http_etag;
pub mod http_header;
pub mod http_header_glue;
pub mod http_kv;
pub mod http_range;
pub mod keyvalue;
pub mod lemon;
pub mod log;
pub mod ls_hpack {
pub mod lshpack;
} // mod ls_hpack
pub mod mod_access;
pub mod mod_accesslog;
pub mod mod_ajp13;
pub mod mod_alias;
pub mod mod_auth;
pub mod mod_auth_api;
pub mod mod_authn_file;
pub mod mod_cgi;
pub mod mod_deflate;
pub mod mod_dirlisting;
pub mod mod_evasive;
pub mod mod_evhost;
pub mod mod_expire;
pub mod mod_extforward;
pub mod mod_fastcgi;
pub mod mod_indexfile;
pub mod mod_proxy;
pub mod mod_redirect;
pub mod mod_rewrite;
pub mod mod_rrdtool;
pub mod mod_scgi;
pub mod mod_secdownload;
pub mod mod_setenv;
pub mod mod_simple_vhost;
pub mod mod_sockproxy;
pub mod mod_ssi;
pub mod mod_staticfile;
pub mod mod_status;
pub mod mod_uploadprogress;
pub mod mod_userdir;
pub mod mod_usertrack;
pub mod mod_vhostdb;
pub mod mod_vhostdb_api;
pub mod mod_webdav;
pub mod mod_wstunnel;
pub mod network;
pub mod network_write;
pub mod plugin;
pub mod rand;
pub mod reqpool;
pub mod request;
pub mod response;
pub mod sock_addr;
pub mod sock_addr_cache;
pub mod stat_cache;
pub mod vector;
} // mod src
