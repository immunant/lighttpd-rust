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
pub mod lshpack;
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

pub unsafe fn run_static_initializers() {
    src::configfile::run_static_initializers();
    src::configparser::run_static_initializers();
    src::gw_backend::run_static_initializers();
    src::http_header_glue::run_static_initializers();
    src::http_header::run_static_initializers();
    src::http_kv::run_static_initializers();
    src::lshpack::run_static_initializers();
    src::mod_access::run_static_initializers();
    src::mod_accesslog::run_static_initializers();
    src::mod_ajp13::run_static_initializers();
    src::mod_alias::run_static_initializers();
    src::mod_auth::run_static_initializers();
    src::mod_authn_file::run_static_initializers();
    src::mod_cgi::run_static_initializers();
    src::mod_deflate::run_static_initializers();
    src::mod_dirlisting::run_static_initializers();
    src::mod_evasive::run_static_initializers();
    src::mod_evhost::run_static_initializers();
    src::mod_expire::run_static_initializers();
    src::mod_extforward::run_static_initializers();
    src::mod_fastcgi::run_static_initializers();
    src::mod_indexfile::run_static_initializers();
    src::mod_proxy::run_static_initializers();
    src::mod_redirect::run_static_initializers();
    src::mod_rewrite::run_static_initializers();
    src::mod_rrdtool::run_static_initializers();
    src::mod_scgi::run_static_initializers();
    src::mod_secdownload::run_static_initializers();
    src::mod_setenv::run_static_initializers();
    src::mod_simple_vhost::run_static_initializers();
    src::mod_sockproxy::run_static_initializers();
    src::mod_ssi::run_static_initializers();
    src::mod_staticfile::run_static_initializers();
    src::mod_status::run_static_initializers();
    src::mod_uploadprogress::run_static_initializers();
    src::mod_userdir::run_static_initializers();
    src::mod_usertrack::run_static_initializers();
    src::mod_vhostdb::run_static_initializers();
    src::mod_webdav::run_static_initializers();
    src::mod_wstunnel::run_static_initializers();
    src::network::run_static_initializers();
}
