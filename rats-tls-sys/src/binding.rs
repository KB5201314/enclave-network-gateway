/* automatically generated by rust-bindgen 0.60.1 */

pub type __uint8_t = ::std::os::raw::c_uchar;
pub type size_t = ::std::os::raw::c_ulong;
pub const rats_tls_err_t_RATS_TLS_ERR_NONE: rats_tls_err_t = 0;
pub const rats_tls_err_t_RATS_TLS_ERR_UNKNOWN: rats_tls_err_t = 1;
pub const rats_tls_err_t_RATS_TLS_ERR_INVALID: rats_tls_err_t = 2;
pub const rats_tls_err_t_RATS_TLS_ERR_NO_MEM: rats_tls_err_t = 3;
pub const rats_tls_err_t_RATS_TLS_ERR_NOT_REGISTERED: rats_tls_err_t = 4;
pub const rats_tls_err_t_RATS_TLS_ERR_LOAD_CRYPTO_WRAPPERS: rats_tls_err_t = 5;
pub const rats_tls_err_t_RATS_TLS_ERR_LOAD_TLS_WRAPPERS: rats_tls_err_t = 6;
pub const rats_tls_err_t_RATS_TLS_ERR_LOAD_ENCLAVE_ATTESTERS: rats_tls_err_t = 7;
pub const rats_tls_err_t_RATS_TLS_ERR_LOAD_ENCLAVE_VERIFIERS: rats_tls_err_t = 8;
pub const rats_tls_err_t_RATS_TLS_ERR_DLOPEN: rats_tls_err_t = 9;
pub const rats_tls_err_t_RATS_TLS_ERR_INIT: rats_tls_err_t = 10;
pub const rats_tls_err_t_RATS_TLS_ERR_UNSUPPORTED_CERT_ALGO: rats_tls_err_t = 11;
pub const rats_tls_err_t_RATS_TLS_ERR_NO_NAME: rats_tls_err_t = 12;
pub type rats_tls_err_t = ::std::os::raw::c_uint;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_DEBUG: rats_tls_log_level_t = 0;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_INFO: rats_tls_log_level_t = 1;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_WARN: rats_tls_log_level_t = 2;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_ERROR: rats_tls_log_level_t = 3;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_FATAL: rats_tls_log_level_t = 4;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_NONE: rats_tls_log_level_t = 5;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_MAX: rats_tls_log_level_t = 6;
pub const rats_tls_log_level_t_RATS_TLS_LOG_LEVEL_DEFAULT: rats_tls_log_level_t = 3;
pub type rats_tls_log_level_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rtls_core_context_t {
    _unused: [u8; 0],
}
pub type rats_tls_handle = *mut rtls_core_context_t;
pub const rats_tls_cert_algo_t_RATS_TLS_CERT_ALGO_RSA_3072_SHA256: rats_tls_cert_algo_t = 0;
pub const rats_tls_cert_algo_t_RATS_TLS_CERT_ALGO_ECC_256_SHA256: rats_tls_cert_algo_t = 1;
pub const rats_tls_cert_algo_t_RATS_TLS_CERT_ALGO_MAX: rats_tls_cert_algo_t = 2;
pub const rats_tls_cert_algo_t_RATS_TLS_CERT_ALGO_DEFAULT: rats_tls_cert_algo_t = 1;
pub type rats_tls_cert_algo_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rats_tls_conf_t {
    pub api_version: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_ulong,
    pub log_level: rats_tls_log_level_t,
    pub tls_type: [::std::os::raw::c_char; 32usize],
    pub attester_type: [::std::os::raw::c_char; 32usize],
    pub verifier_type: [::std::os::raw::c_char; 32usize],
    pub crypto_type: [::std::os::raw::c_char; 32usize],
    pub cert_algo: rats_tls_cert_algo_t,
    pub enclave_id: ::std::os::raw::c_ulonglong,
    pub quote_sgx_epid: rats_tls_conf_t__bindgen_ty_1,
    pub quote_sgx_ecdsa: rats_tls_conf_t__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rats_tls_conf_t__bindgen_ty_1 {
    pub valid: bool,
    pub spid: [u8; 16usize],
    pub linkable: bool,
}
#[test]
fn bindgen_test_layout_rats_tls_conf_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rats_tls_conf_t__bindgen_ty_1>(),
        18usize,
        concat!("Size of: ", stringify!(rats_tls_conf_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rats_tls_conf_t__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(rats_tls_conf_t__bindgen_ty_1))
    );
    fn test_field_valid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).valid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t__bindgen_ty_1),
                "::",
                stringify!(valid)
            )
        );
    }
    test_field_valid();
    fn test_field_spid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).spid) as usize - ptr as usize
            },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t__bindgen_ty_1),
                "::",
                stringify!(spid)
            )
        );
    }
    test_field_spid();
    fn test_field_linkable() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).linkable) as usize - ptr as usize
            },
            17usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t__bindgen_ty_1),
                "::",
                stringify!(linkable)
            )
        );
    }
    test_field_linkable();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rats_tls_conf_t__bindgen_ty_2 {
    pub valid: bool,
    pub cert_type: u8,
}
#[test]
fn bindgen_test_layout_rats_tls_conf_t__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rats_tls_conf_t__bindgen_ty_2>(),
        2usize,
        concat!("Size of: ", stringify!(rats_tls_conf_t__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<rats_tls_conf_t__bindgen_ty_2>(),
        1usize,
        concat!("Alignment of ", stringify!(rats_tls_conf_t__bindgen_ty_2))
    );
    fn test_field_valid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t__bindgen_ty_2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).valid) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t__bindgen_ty_2),
                "::",
                stringify!(valid)
            )
        );
    }
    test_field_valid();
    fn test_field_cert_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t__bindgen_ty_2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cert_type) as usize - ptr as usize
            },
            1usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t__bindgen_ty_2),
                "::",
                stringify!(cert_type)
            )
        );
    }
    test_field_cert_type();
}
#[test]
fn bindgen_test_layout_rats_tls_conf_t() {
    assert_eq!(
        ::std::mem::size_of::<rats_tls_conf_t>(),
        184usize,
        concat!("Size of: ", stringify!(rats_tls_conf_t))
    );
    assert_eq!(
        ::std::mem::align_of::<rats_tls_conf_t>(),
        8usize,
        concat!("Alignment of ", stringify!(rats_tls_conf_t))
    );
    fn test_field_api_version() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).api_version) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(api_version)
            )
        );
    }
    test_field_api_version();
    fn test_field_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(flags)
            )
        );
    }
    test_field_flags();
    fn test_field_log_level() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).log_level) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(log_level)
            )
        );
    }
    test_field_log_level();
    fn test_field_tls_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tls_type) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(tls_type)
            )
        );
    }
    test_field_tls_type();
    fn test_field_attester_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attester_type) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(attester_type)
            )
        );
    }
    test_field_attester_type();
    fn test_field_verifier_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).verifier_type) as usize - ptr as usize
            },
            84usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(verifier_type)
            )
        );
    }
    test_field_verifier_type();
    fn test_field_crypto_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).crypto_type) as usize - ptr as usize
            },
            116usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(crypto_type)
            )
        );
    }
    test_field_crypto_type();
    fn test_field_cert_algo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cert_algo) as usize - ptr as usize
            },
            148usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(cert_algo)
            )
        );
    }
    test_field_cert_algo();
    fn test_field_enclave_id() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).enclave_id) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(enclave_id)
            )
        );
    }
    test_field_enclave_id();
    fn test_field_quote_sgx_epid() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quote_sgx_epid) as usize - ptr as usize
            },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(quote_sgx_epid)
            )
        );
    }
    test_field_quote_sgx_epid();
    fn test_field_quote_sgx_ecdsa() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rats_tls_conf_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quote_sgx_ecdsa) as usize - ptr as usize
            },
            178usize,
            concat!(
                "Offset of field: ",
                stringify!(rats_tls_conf_t),
                "::",
                stringify!(quote_sgx_ecdsa)
            )
        );
    }
    test_field_quote_sgx_ecdsa();
}
pub type rats_tls_callback_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn rats_tls_init(
        conf: *const rats_tls_conf_t,
        handle: *mut rats_tls_handle,
    ) -> rats_tls_err_t;
}
extern "C" {
    pub fn rats_tls_set_verification_callback(
        handle: *mut rats_tls_handle,
        user_callback: rats_tls_callback_t,
    ) -> rats_tls_err_t;
}
extern "C" {
    pub fn rats_tls_negotiate(handle: rats_tls_handle, fd: ::std::os::raw::c_int)
        -> rats_tls_err_t;
}
extern "C" {
    pub fn rats_tls_receive(
        handle: rats_tls_handle,
        buf: *mut ::std::os::raw::c_void,
        buf_size: *mut size_t,
    ) -> rats_tls_err_t;
}
extern "C" {
    pub fn rats_tls_transmit(
        handle: rats_tls_handle,
        buf: *mut ::std::os::raw::c_void,
        buf_size: *mut size_t,
    ) -> rats_tls_err_t;
}
extern "C" {
    pub fn rats_tls_cleanup(handle: rats_tls_handle) -> rats_tls_err_t;
}
