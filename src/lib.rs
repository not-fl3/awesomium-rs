extern crate awesomium_sys as awe;

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_void};
use std::cell::RefCell;
use std::ptr::null_mut;
use std::convert::From;

pub struct AweString(*const awe::awe_string);

fn awe_string_create_from_ascii(string : &str) -> AweString {
    AweString (unsafe {
        let string = CString::new(string).unwrap();
        awe::awe_string_create_from_ascii(string.as_ptr(), string.as_bytes().len())
    })
}

fn awe_string_empty() -> AweString {
    AweString(unsafe { awe::awe_string_empty() })
}


impl<'a> From<Option<&'a str>> for AweString {
    fn from(string : Option<&'a str>) -> Self {
        match string {
            None => awe_string_empty(),
            Some(string) => awe_string_create_from_ascii(string)
        }
    }
}

impl<'a> From<&'a str> for AweString {
    fn from(string : &'a str) -> Self {
        awe_string_create_from_ascii(string)
    }
}

pub fn awe_webcore_initialize_default() {
    unsafe { awe::awe_webcore_initialize_default(); }
}

pub struct AweWebView {
    web_view : *mut awe::awe_webview
}

pub fn awe_webcore_create_webview(width       : isize,
                                  height      : isize,
                                  view_source : bool) -> AweWebView {
    AweWebView {
        web_view : unsafe {
            awe::awe_webcore_create_webview(width as i32, height as i32, view_source as u8)
        }
    }
}

pub struct InitializeParams<'a> {
    pub enable_plugins: bool,
    pub enable_javascript: bool,
    pub enable_databases: bool,
    pub package_path: Option<&'a str>,
    pub locale_path: Option<&'a str>,
    pub user_data_path: Option<&'a str>,
    pub plugin_path: Option<&'a str>,
    pub log_path: Option<&'a str>,
    pub log_level: awe::awe_loglevel,
    pub force_single_process: bool,
    pub child_process_path: Option<&'a str>,
    pub enable_auto_detect_encoding: bool,
    pub accept_language_override: Option<&'a str>,
    pub default_charset_override: Option<&'a str>,
    pub user_agent_override: Option<&'a str>,
    pub proxy_server: Option<&'a str>,
    pub proxy_config_script: Option<&'a str>,
    pub auth_server_whitelist: Option<&'a str>,
    pub save_cache_and_cookies: bool,
    pub max_cache_size: i32,
    pub disable_same_origin_policy: bool,
    pub disable_win_message_pump: bool,
    pub custom_css: Option<&'a str>
}

impl <'a> Default for InitializeParams<'a> {
    fn default() -> InitializeParams<'a> {
        InitializeParams {
            enable_plugins              : false,
            enable_javascript           : true,
            enable_databases            : false,
            package_path                : None,
            locale_path                 : None,
            user_data_path              : None,
            plugin_path                 : None,
            log_path                    : None,
            log_level                   : awe::_awe_loglevel::AWE_LL_NORMAL,
            force_single_process        : false,
            child_process_path          : None,
            enable_auto_detect_encoding : true,
            accept_language_override    : None,
            default_charset_override    : None,
            user_agent_override         : None,
            proxy_server                : None,
            proxy_config_script         : None,
            save_cache_and_cookies      : true,
            max_cache_size              : 0,
            disable_same_origin_policy  : false,
            disable_win_message_pump    : false,
            custom_css                  : None,
            auth_server_whitelist       : None
        }
    }
}
pub fn awe_webcore_initialize<'a>(params : InitializeParams<'a>) {
    unsafe {
        awe::awe_webcore_initialize(
            params.enable_plugins as u8,
            params.enable_javascript as u8,
            params.enable_databases as u8,
            AweString::from(params.package_path).0,
            AweString::from(params.locale_path).0,
            AweString::from(params.user_data_path).0,
            AweString::from(params.plugin_path).0,
            AweString::from(params.log_path).0,
            params.log_level,
            params.force_single_process as u8,
            AweString::from(params.child_process_path).0,
            params.enable_auto_detect_encoding as u8,
            AweString::from(params.accept_language_override).0,
            AweString::from(params.default_charset_override).0,
            AweString::from(params.user_agent_override).0,
            AweString::from(params.proxy_server).0,
            AweString::from(params.proxy_config_script).0,
            AweString::from(params.auth_server_whitelist).0,
            params.save_cache_and_cookies as u8,
            params.max_cache_size,
            params.disable_same_origin_policy as u8,
            params.disable_win_message_pump as u8,
            AweString::from(params.custom_css).0)
    }
}

pub fn awe_webcore_set_base_directory(path : &str) {
    unsafe {
        let path = CString::new(path).unwrap();
        let string = awe::awe_string_create_from_ascii(path.as_ptr(), path.as_bytes().len());
        awe::awe_webcore_set_base_directory(string);
    }
}

pub fn awe_webcore_update() {
    unsafe { awe::awe_webcore_update(); }
}

thread_local!(static LOG_CALLBACK: RefCell<*mut c_void> =
              RefCell::new(null_mut()));

impl AweWebView {
    pub fn set_callback_js_console_message<F>(&self, callback : F)
        where F : Fn(&AweWebView, &str, i32, &str) {
        LOG_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
        });

        unsafe { awe::awe_webview_set_callback_js_console_message(self.web_view,
                                                                  Some(wrapper::<F>));
        }

        unsafe extern "C" fn wrapper<F>(caller: *mut awe::awe_webview,
                                        message: *const awe::awe_string,
                                        line_number: ::std::os::raw::c_int,
                                        source: *const awe::awe_string)
            where F : Fn(&AweWebView, &str, i32, &str) {

        let mut umessage : [i8; 200] = [0; 200];

        awe::awe_string_to_utf8(message, umessage.as_mut_ptr(), 200);

        let mut usource : [i8; 200] = [0; 200];
        awe::awe_string_to_utf8(source, usource.as_mut_ptr(), 200);
        LOG_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)(
                &AweWebView{ web_view : caller },
                CStr::from_ptr(umessage.as_ptr()).to_str().unwrap(),
                line_number,
                CStr::from_ptr(usource.as_ptr()).to_str().unwrap()); });
    }


    }

    pub fn set_callback_finish_loading<F>(&self, callback : F)
        where F : Fn(&AweWebView) {
        FINISH_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut c_void;
        });

        unsafe { awe::awe_webview_set_callback_finish_loading(self.web_view,
                                                              Some(wrapper::<F>));
        }

        unsafe extern "C" fn wrapper<F>(caller: *mut awe::awe_webview)
            where F : Fn(&AweWebView) {

            LOG_CALLBACK.with(|z| {
                let closure = *z.borrow_mut() as *mut F;
                (*closure)( &AweWebView{ web_view : caller } );
            });
        }
    }
    pub fn set_transparent(&self, transparent : bool) {
        unsafe {
            awe::awe_webview_set_transparent(self.web_view, transparent as u8);
        }
    }

    pub fn load_file(&self, path : &str, _frame_name : &str) {
        let path = CString::new(path).unwrap();
        let string = unsafe { awe::awe_string_create_from_ascii(path.as_ptr(), path.as_bytes().len()) };

        unsafe { awe::awe_webview_load_file(self.web_view,
                                            string,
                                            awe::awe_string_empty()) }
    }

    pub fn is_loading_page(&self) -> bool {
        unsafe { awe::awe_webview_is_loading_page(self.web_view) != 0 }
    }

    pub fn render(&self) -> AweRenderBuffer {
        AweRenderBuffer {
            buffer : unsafe {awe::awe_webview_render(self.web_view)}
        }
    }

    pub fn execute_javascript(&self, javascript : &str, frame_name : Option<&str>) {
        unsafe {
            awe::awe_webview_execute_javascript(
                self.web_view,
                AweString::from(javascript).0,
                AweString::from(frame_name).0)
        }
    }
}

thread_local!(static FINISH_CALLBACK: RefCell<*mut c_void> =
              RefCell::new(null_mut()));


pub struct AweRenderBuffer {
    buffer : *const awe::awe_renderbuffer
}

pub fn awe_renderbuffer_get_buffer(buffer : &AweRenderBuffer) -> *const u8 {
    unsafe { awe::awe_renderbuffer_get_buffer(buffer.buffer) }
}

pub fn awe_renderbuffer_get_buffer_slice(buffer : &AweRenderBuffer) -> &[(u8, u8, u8, u8)] {
    let width = awe_renderbuffer_get_width(buffer);
    let height = awe_renderbuffer_get_height(buffer);
    let buffer = unsafe { awe::awe_renderbuffer_get_buffer(buffer.buffer) };
    let buffer : *const (u8, u8, u8, u8) = unsafe { ::std::mem::transmute(buffer)};
    unsafe {::std::slice::from_raw_parts(buffer, (width * height) as usize) }
}
pub fn awe_renderbuffer_get_width(buffer : &AweRenderBuffer) -> i32 {
    unsafe { awe::awe_renderbuffer_get_width(buffer.buffer) }
}
pub fn awe_renderbuffer_get_height(buffer : &AweRenderBuffer) -> i32 {
    unsafe { awe::awe_renderbuffer_get_height(buffer.buffer) }
}
