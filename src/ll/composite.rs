/*
 * This file generated automatically from composite.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#[allow(unused_imports)];
#[allow(non_camel_case_types)];
use core;
use core::libc::*;
use ll::base::*;
use ll;
use ll::xproto;
use ll::render;
use ll::shape;
use ll::xfixes;

pub static COMPOSITE_MAJOR_VERSION : c_uint = 0;
pub static COMPOSITE_MINOR_VERSION : c_uint = 3;

pub struct query_version_cookie {
    sequence : c_uint
}


pub struct query_version_request {
    major_opcode :           u8,
    minor_opcode :           u8,
    length :                 u16,
    client_major_version :   u32,
    client_minor_version :   u32
}


pub struct query_version_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    major_version :   u32,
    minor_version :   u32,
    pad1 :            [u8,..16]
}



pub struct redirect_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct redirect_subwindows_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct unredirect_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct unredirect_subwindows_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    update :         u8,
    pad0 :           [u8,..3]
}



pub struct create_region_from_border_clip_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    region :         ll::xfixes::region,
    window :         ll::xproto::window
}



pub struct name_window_pixmap_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window,
    pixmap :         ll::xproto::pixmap
}


pub struct get_overlay_window_cookie {
    sequence : c_uint
}


pub struct get_overlay_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window
}


pub struct get_overlay_window_reply {
    response_type :   u8,
    pad0 :            u8,
    sequence :        u16,
    length :          u32,
    overlay_win :     ll::xproto::window,
    pad1 :            [u8,..20]
}



pub struct release_overlay_window_request {
    major_opcode :   u8,
    minor_opcode :   u8,
    length :         u16,
    window :         ll::xproto::window
}

#[link_args="-lxcb-composite"]
pub extern "C" {

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_query_version (c : *connection,
                                       client_major_version :  u32,
                                       client_minor_version :  u32) -> query_version_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_composite_query_version_unchecked (c : *connection,
                                                 client_major_version :  u32,
                                                 client_minor_version :  u32) -> query_version_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_composite_query_version_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_composite_query_version_reply (c : *connection,
                                             cookie : query_version_cookie,
                                             e : **generic_error) -> *query_version_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_redirect_window_checked (c : *connection,
                                                 window :  ll::xproto::window,
                                                 update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_redirect_window (c : *connection,
                                         window :  ll::xproto::window,
                                         update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_redirect_subwindows_checked (c : *connection,
                                                     window :  ll::xproto::window,
                                                     update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_redirect_subwindows (c : *connection,
                                             window :  ll::xproto::window,
                                             update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_unredirect_window_checked (c : *connection,
                                                   window :  ll::xproto::window,
                                                   update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_unredirect_window (c : *connection,
                                           window :  ll::xproto::window,
                                           update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_unredirect_subwindows_checked (c : *connection,
                                                       window :  ll::xproto::window,
                                                       update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_unredirect_subwindows (c : *connection,
                                               window :  ll::xproto::window,
                                               update :  u8) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_create_region_from_border_clip_checked (c : *connection,
                                                                region :  ll::xfixes::region,
                                                                window :  ll::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_create_region_from_border_clip (c : *connection,
                                                        region :  ll::xfixes::region,
                                                        window :  ll::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_name_window_pixmap_checked (c : *connection,
                                                    window :  ll::xproto::window,
                                                    pixmap :  ll::xproto::pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_name_window_pixmap (c : *connection,
                                            window :  ll::xproto::window,
                                            pixmap :  ll::xproto::pixmap) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_get_overlay_window (c : *connection,
                                            window :  ll::xproto::window) -> get_overlay_window_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
unsafe fn xcb_composite_get_overlay_window_unchecked (c : *connection,
                                                      window :  ll::xproto::window) -> get_overlay_window_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_composite_get_overlay_window_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
unsafe fn xcb_composite_get_overlay_window_reply (c : *connection,
                                                  cookie : get_overlay_window_cookie,
                                                  e : **generic_error) -> *get_overlay_window_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
unsafe fn xcb_composite_release_overlay_window_checked (c : *connection,
                                                        window :  ll::xproto::window) -> void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
unsafe fn xcb_composite_release_overlay_window (c : *connection,
                                                window :  ll::xproto::window) -> void_cookie;
}

