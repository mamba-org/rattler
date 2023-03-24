//! Generated file, do not edit by hand, see `crate/tools/src`

#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code,
    clippy::upper_case_acronyms
)]

pub use libc::FILE;

pub const SOLV_VERSION_0: u32 = 0;
pub const SOLV_VERSION_1: u32 = 1;
pub const SOLV_VERSION_2: u32 = 2;
pub const SOLV_VERSION_3: u32 = 3;
pub const SOLV_VERSION_4: u32 = 4;
pub const SOLV_VERSION_5: u32 = 5;
pub const SOLV_VERSION_6: u32 = 6;
pub const SOLV_VERSION_7: u32 = 7;
pub const SOLV_VERSION_8: u32 = 8;
pub const SOLV_VERSION_9: u32 = 9;
pub const SOLV_FLAG_PREFIX_POOL: u32 = 4;
pub const SOLV_FLAG_SIZE_BYTES: u32 = 8;
pub const SOLV_FLAG_USERDATA: u32 = 16;
pub const SOLV_FLAG_IDARRAYBLOCK: u32 = 32;
pub const DISTTYPE_RPM: u32 = 0;
pub const DISTTYPE_DEB: u32 = 1;
pub const DISTTYPE_ARCH: u32 = 2;
pub const DISTTYPE_HAIKU: u32 = 3;
pub const DISTTYPE_CONDA: u32 = 4;
pub const SOLV_FATAL: u32 = 1;
pub const SOLV_ERROR: u32 = 2;
pub const SOLV_WARN: u32 = 4;
pub const SOLV_DEBUG_STATS: u32 = 8;
pub const SOLV_DEBUG_RULE_CREATION: u32 = 16;
pub const SOLV_DEBUG_PROPAGATE: u32 = 32;
pub const SOLV_DEBUG_ANALYZE: u32 = 64;
pub const SOLV_DEBUG_UNSOLVABLE: u32 = 128;
pub const SOLV_DEBUG_SOLUTIONS: u32 = 256;
pub const SOLV_DEBUG_POLICY: u32 = 512;
pub const SOLV_DEBUG_RESULT: u32 = 1024;
pub const SOLV_DEBUG_JOB: u32 = 2048;
pub const SOLV_DEBUG_SOLVER: u32 = 4096;
pub const SOLV_DEBUG_TRANSACTION: u32 = 8192;
pub const SOLV_DEBUG_WATCHES: u32 = 16384;
pub const SOLV_DEBUG_TO_STDERR: u32 = 1073741824;
pub const POOL_FLAG_PROMOTEEPOCH: u32 = 1;
pub const POOL_FLAG_FORBIDSELFCONFLICTS: u32 = 2;
pub const POOL_FLAG_OBSOLETEUSESPROVIDES: u32 = 3;
pub const POOL_FLAG_IMPLICITOBSOLETEUSESPROVIDES: u32 = 4;
pub const POOL_FLAG_OBSOLETEUSESCOLORS: u32 = 5;
pub const POOL_FLAG_NOINSTALLEDOBSOLETES: u32 = 6;
pub const POOL_FLAG_HAVEDISTEPOCH: u32 = 7;
pub const POOL_FLAG_NOOBSOLETESMULTIVERSION: u32 = 8;
pub const POOL_FLAG_ADDFILEPROVIDESFILTERED: u32 = 9;
pub const POOL_FLAG_IMPLICITOBSOLETEUSESCOLORS: u32 = 10;
pub const POOL_FLAG_NOWHATPROVIDESAUX: u32 = 11;
pub const POOL_FLAG_WHATPROVIDESWITHDISABLED: u32 = 12;
pub const REL_GT: u32 = 1;
pub const REL_EQ: u32 = 2;
pub const REL_LT: u32 = 4;
pub const REL_AND: u32 = 16;
pub const REL_OR: u32 = 17;
pub const REL_WITH: u32 = 18;
pub const REL_NAMESPACE: u32 = 19;
pub const REL_ARCH: u32 = 20;
pub const REL_FILECONFLICT: u32 = 21;
pub const REL_COND: u32 = 22;
pub const REL_COMPAT: u32 = 23;
pub const REL_KIND: u32 = 24;
pub const REL_MULTIARCH: u32 = 25;
pub const REL_ELSE: u32 = 26;
pub const REL_ERROR: u32 = 27;
pub const REL_WITHOUT: u32 = 28;
pub const REL_UNLESS: u32 = 29;
pub const REL_CONDA: u32 = 30;
pub const SEARCH_STRINGMASK: u32 = 15;
pub const SEARCH_STRING: u32 = 1;
pub const SEARCH_STRINGSTART: u32 = 2;
pub const SEARCH_STRINGEND: u32 = 3;
pub const SEARCH_SUBSTRING: u32 = 4;
pub const SEARCH_GLOB: u32 = 5;
pub const SEARCH_REGEX: u32 = 6;
pub const SEARCH_ERROR: u32 = 15;
pub const SEARCH_NOCASE: u32 = 128;
pub const SEARCH_NO_STORAGE_SOLVABLE: u32 = 256;
pub const SEARCH_SUB: u32 = 512;
pub const SEARCH_ARRAYSENTINEL: u32 = 1024;
pub const SEARCH_DISABLED_REPOS: u32 = 2048;
pub const SEARCH_KEEP_TYPE_DELETED: u32 = 4096;
pub const SEARCH_SKIP_KIND: u32 = 65536;
pub const SEARCH_FILES: u32 = 131072;
pub const SEARCH_CHECKSUMS: u32 = 262144;
pub const SEARCH_SUBSCHEMA: u32 = 1073741824;
pub const SEARCH_THISSOLVID: u32 = 2147483648;
pub const SEARCH_COMPLETE_FILELIST: u32 = 0;
pub const SEARCH_NEXT_KEY: u32 = 1;
pub const SEARCH_NEXT_SOLVABLE: u32 = 2;
pub const SEARCH_STOP: u32 = 3;
pub const SEARCH_ENTERSUB: i32 = -1;
pub const SOLVER_TRANSACTION_IGNORE: u32 = 0;
pub const SOLVER_TRANSACTION_ERASE: u32 = 16;
pub const SOLVER_TRANSACTION_REINSTALLED: u32 = 17;
pub const SOLVER_TRANSACTION_DOWNGRADED: u32 = 18;
pub const SOLVER_TRANSACTION_CHANGED: u32 = 19;
pub const SOLVER_TRANSACTION_UPGRADED: u32 = 20;
pub const SOLVER_TRANSACTION_OBSOLETED: u32 = 21;
pub const SOLVER_TRANSACTION_INSTALL: u32 = 32;
pub const SOLVER_TRANSACTION_REINSTALL: u32 = 33;
pub const SOLVER_TRANSACTION_DOWNGRADE: u32 = 34;
pub const SOLVER_TRANSACTION_CHANGE: u32 = 35;
pub const SOLVER_TRANSACTION_UPGRADE: u32 = 36;
pub const SOLVER_TRANSACTION_OBSOLETES: u32 = 37;
pub const SOLVER_TRANSACTION_MULTIINSTALL: u32 = 48;
pub const SOLVER_TRANSACTION_MULTIREINSTALL: u32 = 49;
pub const SOLVER_TRANSACTION_MAXTYPE: u32 = 63;
pub const SOLVER_TRANSACTION_SHOW_ACTIVE: u32 = 1;
pub const SOLVER_TRANSACTION_SHOW_ALL: u32 = 2;
pub const SOLVER_TRANSACTION_SHOW_OBSOLETES: u32 = 4;
pub const SOLVER_TRANSACTION_SHOW_MULTIINSTALL: u32 = 8;
pub const SOLVER_TRANSACTION_CHANGE_IS_REINSTALL: u32 = 16;
pub const SOLVER_TRANSACTION_MERGE_VENDORCHANGES: u32 = 32;
pub const SOLVER_TRANSACTION_MERGE_ARCHCHANGES: u32 = 64;
pub const SOLVER_TRANSACTION_RPM_ONLY: u32 = 128;
pub const SOLVER_TRANSACTION_KEEP_PSEUDO: u32 = 256;
pub const SOLVER_TRANSACTION_OBSOLETE_IS_UPGRADE: u32 = 512;
pub const SOLVER_TRANSACTION_ARCHCHANGE: u32 = 256;
pub const SOLVER_TRANSACTION_VENDORCHANGE: u32 = 257;
pub const SOLVER_TRANSACTION_KEEP_ORDERDATA: u32 = 1;
pub const SOLVER_TRANSACTION_KEEP_ORDERCYCLES: u32 = 2;
pub const SOLVER_TRANSACTION_KEEP_ORDEREDGES: u32 = 4;
pub const SOLVER_ORDERCYCLE_HARMLESS: u32 = 0;
pub const SOLVER_ORDERCYCLE_NORMAL: u32 = 1;
pub const SOLVER_ORDERCYCLE_CRITICAL: u32 = 2;
pub const SOLVER_RULE_TYPEMASK: u32 = 65280;
pub const SOLVER_SOLUTION_JOB: u32 = 0;
pub const SOLVER_SOLUTION_DISTUPGRADE: i32 = -1;
pub const SOLVER_SOLUTION_INFARCH: i32 = -2;
pub const SOLVER_SOLUTION_BEST: i32 = -3;
pub const SOLVER_SOLUTION_POOLJOB: i32 = -4;
pub const SOLVER_SOLUTION_BLACK: i32 = -5;
pub const SOLVER_SOLUTION_STRICTREPOPRIORITY: i32 = -6;
pub const SOLVER_SOLVABLE: u32 = 1;
pub const SOLVER_SOLVABLE_NAME: u32 = 2;
pub const SOLVER_SOLVABLE_PROVIDES: u32 = 3;
pub const SOLVER_SOLVABLE_ONE_OF: u32 = 4;
pub const SOLVER_SOLVABLE_REPO: u32 = 5;
pub const SOLVER_SOLVABLE_ALL: u32 = 6;
pub const SOLVER_SELECTMASK: u32 = 255;
pub const SOLVER_NOOP: u32 = 0;
pub const SOLVER_INSTALL: u32 = 256;
pub const SOLVER_ERASE: u32 = 512;
pub const SOLVER_UPDATE: u32 = 768;
pub const SOLVER_WEAKENDEPS: u32 = 1024;
pub const SOLVER_MULTIVERSION: u32 = 1280;
pub const SOLVER_LOCK: u32 = 1536;
pub const SOLVER_DISTUPGRADE: u32 = 1792;
pub const SOLVER_VERIFY: u32 = 2048;
pub const SOLVER_DROP_ORPHANED: u32 = 2304;
pub const SOLVER_USERINSTALLED: u32 = 2560;
pub const SOLVER_ALLOWUNINSTALL: u32 = 2816;
pub const SOLVER_FAVOR: u32 = 3072;
pub const SOLVER_DISFAVOR: u32 = 3328;
pub const SOLVER_BLACKLIST: u32 = 3584;
pub const SOLVER_EXCLUDEFROMWEAK: u32 = 4096;
pub const SOLVER_JOBMASK: u32 = 65280;
pub const SOLVER_WEAK: u32 = 65536;
pub const SOLVER_ESSENTIAL: u32 = 131072;
pub const SOLVER_CLEANDEPS: u32 = 262144;
pub const SOLVER_ORUPDATE: u32 = 524288;
pub const SOLVER_FORCEBEST: u32 = 1048576;
pub const SOLVER_TARGETED: u32 = 2097152;
pub const SOLVER_NOTBYUSER: u32 = 4194304;
pub const SOLVER_SETEV: u32 = 16777216;
pub const SOLVER_SETEVR: u32 = 33554432;
pub const SOLVER_SETARCH: u32 = 67108864;
pub const SOLVER_SETVENDOR: u32 = 134217728;
pub const SOLVER_SETREPO: u32 = 268435456;
pub const SOLVER_NOAUTOSET: u32 = 536870912;
pub const SOLVER_SETNAME: u32 = 1073741824;
pub const SOLVER_SETMASK: u32 = 2130706432;
pub const SOLVER_NOOBSOLETES: u32 = 1280;
pub const SOLVER_REASON_UNRELATED: u32 = 0;
pub const SOLVER_REASON_UNIT_RULE: u32 = 1;
pub const SOLVER_REASON_KEEP_INSTALLED: u32 = 2;
pub const SOLVER_REASON_RESOLVE_JOB: u32 = 3;
pub const SOLVER_REASON_UPDATE_INSTALLED: u32 = 4;
pub const SOLVER_REASON_CLEANDEPS_ERASE: u32 = 5;
pub const SOLVER_REASON_RESOLVE: u32 = 6;
pub const SOLVER_REASON_WEAKDEP: u32 = 7;
pub const SOLVER_REASON_RESOLVE_ORPHAN: u32 = 8;
pub const SOLVER_REASON_RECOMMENDED: u32 = 16;
pub const SOLVER_REASON_SUPPLEMENTED: u32 = 17;
pub const SOLVER_FLAG_ALLOW_DOWNGRADE: u32 = 1;
pub const SOLVER_FLAG_ALLOW_ARCHCHANGE: u32 = 2;
pub const SOLVER_FLAG_ALLOW_VENDORCHANGE: u32 = 3;
pub const SOLVER_FLAG_ALLOW_UNINSTALL: u32 = 4;
pub const SOLVER_FLAG_NO_UPDATEPROVIDE: u32 = 5;
pub const SOLVER_FLAG_SPLITPROVIDES: u32 = 6;
pub const SOLVER_FLAG_IGNORE_RECOMMENDED: u32 = 7;
pub const SOLVER_FLAG_ADD_ALREADY_RECOMMENDED: u32 = 8;
pub const SOLVER_FLAG_NO_INFARCHCHECK: u32 = 9;
pub const SOLVER_FLAG_ALLOW_NAMECHANGE: u32 = 10;
pub const SOLVER_FLAG_KEEP_EXPLICIT_OBSOLETES: u32 = 11;
pub const SOLVER_FLAG_BEST_OBEY_POLICY: u32 = 12;
pub const SOLVER_FLAG_NO_AUTOTARGET: u32 = 13;
pub const SOLVER_FLAG_DUP_ALLOW_DOWNGRADE: u32 = 14;
pub const SOLVER_FLAG_DUP_ALLOW_ARCHCHANGE: u32 = 15;
pub const SOLVER_FLAG_DUP_ALLOW_VENDORCHANGE: u32 = 16;
pub const SOLVER_FLAG_DUP_ALLOW_NAMECHANGE: u32 = 17;
pub const SOLVER_FLAG_KEEP_ORPHANS: u32 = 18;
pub const SOLVER_FLAG_BREAK_ORPHANS: u32 = 19;
pub const SOLVER_FLAG_FOCUS_INSTALLED: u32 = 20;
pub const SOLVER_FLAG_YUM_OBSOLETES: u32 = 21;
pub const SOLVER_FLAG_NEED_UPDATEPROVIDE: u32 = 22;
pub const SOLVER_FLAG_URPM_REORDER: u32 = 23;
pub const SOLVER_FLAG_FOCUS_BEST: u32 = 24;
pub const SOLVER_FLAG_STRONG_RECOMMENDS: u32 = 25;
pub const SOLVER_FLAG_INSTALL_ALSO_UPDATES: u32 = 26;
pub const SOLVER_FLAG_ONLY_NAMESPACE_RECOMMENDED: u32 = 27;
pub const SOLVER_FLAG_STRICT_REPO_PRIORITY: u32 = 28;
pub const SOLVER_ALTERNATIVE_TYPE_RULE: u32 = 1;
pub const SOLVER_ALTERNATIVE_TYPE_RECOMMENDS: u32 = 2;
pub const SOLVER_ALTERNATIVE_TYPE_SUGGESTS: u32 = 3;
pub const SELECTION_NAME: u32 = 1;
pub const SELECTION_PROVIDES: u32 = 2;
pub const SELECTION_FILELIST: u32 = 4;
pub const SELECTION_CANON: u32 = 8;
pub const SELECTION_DOTARCH: u32 = 16;
pub const SELECTION_REL: u32 = 32;
pub const SELECTION_GLOB: u32 = 512;
pub const SELECTION_NOCASE: u32 = 2048;
pub const SELECTION_FLAT: u32 = 1024;
pub const SELECTION_SKIP_KIND: u32 = 16384;
pub const SELECTION_MATCH_DEPSTR: u32 = 32768;
pub const SELECTION_INSTALLED_ONLY: u32 = 256;
pub const SELECTION_SOURCE_ONLY: u32 = 4096;
pub const SELECTION_WITH_SOURCE: u32 = 8192;
pub const SELECTION_WITH_DISABLED: u32 = 65536;
pub const SELECTION_WITH_BADARCH: u32 = 131072;
pub const SELECTION_WITH_ALL: u32 = 204800;
pub const SELECTION_REPLACE: u32 = 0;
pub const SELECTION_ADD: u32 = 268435456;
pub const SELECTION_SUBTRACT: u32 = 536870912;
pub const SELECTION_FILTER: u32 = 805306368;
pub const SELECTION_FILTER_KEEP_IFEMPTY: u32 = 1073741824;
pub const SELECTION_FILTER_SWAPPED: u32 = 2147483648;
pub const SELECTION_MODEBITS: u32 = 805306368;
pub const SOLV_ADD_NO_STUBS: u32 = 256;
pub const CONDA_ADD_USE_ONLY_TAR_BZ2: u32 = 256;
pub const CONDA_ADD_WITH_SIGNATUREDATA: u32 = 512;
pub type Stringpool = s_Stringpool;
pub type Pool = s_Pool;
pub type Id = libc::c_int;
pub type Offset = libc::c_uint;
<<<<<<< HEAD
||||||| parent of e3f02ef (fix: FILE io libc stuff)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut libc::c_void,
}
#[test]
fn bindgen_test_layout__iobuf() {
    const UNINIT: ::std::mem::MaybeUninit<_iobuf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_iobuf>(),
        8usize,
        concat!("Size of: ", stringify!(_iobuf))
    );
    assert_eq!(
        ::std::mem::align_of::<_iobuf>(),
        8usize,
        concat!("Alignment of ", stringify!(_iobuf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Placeholder) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_iobuf),
            "::",
            stringify!(_Placeholder)
        )
    );
}
=======
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = libc::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: usize,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    const UNINIT: ::std::mem::MaybeUninit<_IO_FILE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_ptr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_end) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_base) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_base) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_ptr) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_end) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_buf_base) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_buf_end) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_save_base) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_backup_base) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_save_end) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._markers) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._chain) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._fileno) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags2) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._old_offset) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._cur_column) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._vtable_offset) as usize - ptr as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._shortbuf) as usize - ptr as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lock) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._codecvt) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_codecvt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._wide_data) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_wide_data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._freeres_list) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_list)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._freeres_buf) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_buf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad5) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._mode) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unused2) as usize - ptr as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
>>>>>>> e3f02ef (fix: FILE io libc stuff)
pub type Hashval = libc::c_uint;
pub type Hashtable = *mut Id;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Reldep {
    pub name: Id,
    pub evr: Id,
    pub flags: libc::c_int,
}
pub type Reldep = s_Reldep;
extern "C" {
    pub fn pool_str2id(pool: *mut Pool, arg1: *const libc::c_char, arg2: libc::c_int) -> Id;
}
extern "C" {
    pub fn pool_strn2id(
        pool: *mut Pool,
        arg1: *const libc::c_char,
        arg2: libc::c_uint,
        arg3: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn pool_rel2id(
        pool: *mut Pool,
        arg1: Id,
        arg2: Id,
        arg3: libc::c_int,
        arg4: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn pool_id2str(pool: *const Pool, arg1: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_id2rel(pool: *const Pool, arg1: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_id2evr(pool: *const Pool, arg1: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_dep2str(pool: *mut Pool, arg1: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_shrink_strings(pool: *mut Pool);
}
extern "C" {
    pub fn pool_shrink_rels(pool: *mut Pool);
}
extern "C" {
    pub fn pool_freeidhashes(pool: *mut Pool);
}
extern "C" {
    pub fn pool_resize_rels_hash(pool: *mut Pool, numnew: libc::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Queue {
    pub elements: *mut Id,
    pub count: libc::c_int,
    pub alloc: *mut Id,
    pub left: libc::c_int,
}
pub type Queue = s_Queue;
extern "C" {
    pub fn queue_alloc_one(q: *mut Queue);
}
extern "C" {
    pub fn queue_alloc_one_head(q: *mut Queue);
}
extern "C" {
    pub fn queue_init(q: *mut Queue);
}
extern "C" {
    pub fn queue_init_buffer(q: *mut Queue, buf: *mut Id, size: libc::c_int);
}
extern "C" {
    pub fn queue_init_clone(target: *mut Queue, source: *const Queue);
}
extern "C" {
    pub fn queue_free(q: *mut Queue);
}
extern "C" {
    pub fn queue_insert(q: *mut Queue, pos: libc::c_int, id: Id);
}
extern "C" {
    pub fn queue_insert2(q: *mut Queue, pos: libc::c_int, id1: Id, id2: Id);
}
extern "C" {
    pub fn queue_insertn(q: *mut Queue, pos: libc::c_int, n: libc::c_int, elements: *const Id);
}
extern "C" {
    pub fn queue_delete(q: *mut Queue, pos: libc::c_int);
}
extern "C" {
    pub fn queue_delete2(q: *mut Queue, pos: libc::c_int);
}
extern "C" {
    pub fn queue_deleten(q: *mut Queue, pos: libc::c_int, n: libc::c_int);
}
extern "C" {
    pub fn queue_prealloc(q: *mut Queue, n: libc::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Map {
    pub map: *mut libc::c_uchar,
    pub size: libc::c_int,
}
pub type Map = s_Map;
extern "C" {
    pub fn map_init(m: *mut Map, n: libc::c_int);
}
extern "C" {
    pub fn map_init_clone(target: *mut Map, source: *const Map);
}
extern "C" {
    pub fn map_grow(m: *mut Map, n: libc::c_int);
}
extern "C" {
    pub fn map_free(m: *mut Map);
}
extern "C" {
    pub fn map_and(t: *mut Map, s: *const Map);
}
extern "C" {
    pub fn map_or(t: *mut Map, s: *const Map);
}
extern "C" {
    pub fn map_subtract(t: *mut Map, s: *const Map);
}
extern "C" {
    pub fn map_invertall(m: *mut Map);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Solvable {
    pub name: Id,
    pub arch: Id,
    pub evr: Id,
    pub vendor: Id,
    pub repo: *mut s_Repo,
    pub provides: Offset,
    pub obsoletes: Offset,
    pub conflicts: Offset,
    pub requires: Offset,
    pub recommends: Offset,
    pub suggests: Offset,
    pub supplements: Offset,
    pub enhances: Offset,
}
pub type Solvable = s_Solvable;
extern "C" {
    pub fn solvable_lookup_type(s: *mut Solvable, keyname: Id) -> Id;
}
extern "C" {
    pub fn solvable_lookup_id(s: *mut Solvable, keyname: Id) -> Id;
}
extern "C" {
    pub fn solvable_lookup_num(
        s: *mut Solvable,
        keyname: Id,
        notfound: libc::c_ulonglong,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn solvable_lookup_sizek(
        s: *mut Solvable,
        keyname: Id,
        notfound: libc::c_ulonglong,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn solvable_lookup_str(s: *mut Solvable, keyname: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_str_poollang(s: *mut Solvable, keyname: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_str_lang(
        s: *mut Solvable,
        keyname: Id,
        lang: *const libc::c_char,
        usebase: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_bool(s: *mut Solvable, keyname: Id) -> libc::c_int;
}
extern "C" {
    pub fn solvable_lookup_void(s: *mut Solvable, keyname: Id) -> libc::c_int;
}
extern "C" {
    pub fn solvable_get_location(
        s: *mut Solvable,
        medianrp: *mut libc::c_uint,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_location(
        s: *mut Solvable,
        medianrp: *mut libc::c_uint,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_sourcepkg(s: *mut Solvable) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_bin_checksum(
        s: *mut Solvable,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_uchar;
}
extern "C" {
    pub fn solvable_lookup_checksum(
        s: *mut Solvable,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solvable_lookup_idarray(s: *mut Solvable, keyname: Id, q: *mut Queue) -> libc::c_int;
}
extern "C" {
    pub fn solvable_lookup_deparray(
        s: *mut Solvable,
        keyname: Id,
        q: *mut Queue,
        marker: Id,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_lookup_count(s: *mut Solvable, keyname: Id) -> libc::c_uint;
}
extern "C" {
    pub fn solvable_set_id(s: *mut Solvable, keyname: Id, id: Id);
}
extern "C" {
    pub fn solvable_set_num(s: *mut Solvable, keyname: Id, num: libc::c_ulonglong);
}
extern "C" {
    pub fn solvable_set_str(s: *mut Solvable, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn solvable_set_poolstr(s: *mut Solvable, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn solvable_add_poolstr_array(s: *mut Solvable, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn solvable_add_idarray(s: *mut Solvable, keyname: Id, id: Id);
}
extern "C" {
    pub fn solvable_add_deparray(s: *mut Solvable, keyname: Id, dep: Id, marker: Id);
}
extern "C" {
    pub fn solvable_set_idarray(s: *mut Solvable, keyname: Id, q: *mut Queue);
}
extern "C" {
    pub fn solvable_set_deparray(s: *mut Solvable, keyname: Id, q: *mut Queue, marker: Id);
}
extern "C" {
    pub fn solvable_unset(s: *mut Solvable, keyname: Id);
}
extern "C" {
    pub fn solvable_identical(s1: *mut Solvable, s2: *mut Solvable) -> libc::c_int;
}
extern "C" {
    pub fn solvable_selfprovidedep(s: *mut Solvable) -> Id;
}
extern "C" {
    pub fn solvable_matchesdep(
        s: *mut Solvable,
        keyname: Id,
        dep: Id,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_matchessolvable(
        s: *mut Solvable,
        keyname: Id,
        solvid: Id,
        depq: *mut Queue,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_matchessolvable_int(
        s: *mut Solvable,
        keyname: Id,
        marker: libc::c_int,
        solvid: Id,
        solvidmap: *mut Map,
        depq: *mut Queue,
        missc: *mut Map,
        reloff: libc::c_int,
        outdepq: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_is_irrelevant_patch(s: *mut Solvable, installedmap: *mut Map) -> libc::c_int;
}
extern "C" {
    pub fn solvable_trivial_installable_map(
        s: *mut Solvable,
        installedmap: *mut Map,
        conflictsmap: *mut Map,
        multiversionmap: *mut Map,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_trivial_installable_queue(
        s: *mut Solvable,
        installed: *mut Queue,
        multiversionmap: *mut Map,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_trivial_installable_repo(
        s: *mut Solvable,
        installed: *mut s_Repo,
        multiversionmap: *mut Map,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Stringpool {
    pub strings: *mut Offset,
    pub nstrings: libc::c_int,
    pub stringspace: *mut libc::c_char,
    pub sstrings: Offset,
    pub stringhashtbl: Hashtable,
    pub stringhashmask: Hashval,
}
extern "C" {
    pub fn stringpool_init(ss: *mut Stringpool, strs: *mut *const libc::c_char);
}
extern "C" {
    pub fn stringpool_init_empty(ss: *mut Stringpool);
}
extern "C" {
    pub fn stringpool_clone(ss: *mut Stringpool, from: *mut Stringpool);
}
extern "C" {
    pub fn stringpool_free(ss: *mut Stringpool);
}
extern "C" {
    pub fn stringpool_freehash(ss: *mut Stringpool);
}
extern "C" {
    pub fn stringpool_resize_hash(ss: *mut Stringpool, numnew: libc::c_int);
}
extern "C" {
    pub fn stringpool_str2id(
        ss: *mut Stringpool,
        str_: *const libc::c_char,
        create: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn stringpool_strn2id(
        ss: *mut Stringpool,
        str_: *const libc::c_char,
        len: libc::c_uint,
        create: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn stringpool_shrink(ss: *mut Stringpool);
}
pub const solv_knownid_ID_NULL: solv_knownid = 0;
pub const solv_knownid_ID_EMPTY: solv_knownid = 1;
pub const solv_knownid_SOLVABLE_NAME: solv_knownid = 2;
pub const solv_knownid_SOLVABLE_ARCH: solv_knownid = 3;
pub const solv_knownid_SOLVABLE_EVR: solv_knownid = 4;
pub const solv_knownid_SOLVABLE_VENDOR: solv_knownid = 5;
pub const solv_knownid_SOLVABLE_PROVIDES: solv_knownid = 6;
pub const solv_knownid_SOLVABLE_OBSOLETES: solv_knownid = 7;
pub const solv_knownid_SOLVABLE_CONFLICTS: solv_knownid = 8;
pub const solv_knownid_SOLVABLE_REQUIRES: solv_knownid = 9;
pub const solv_knownid_SOLVABLE_RECOMMENDS: solv_knownid = 10;
pub const solv_knownid_SOLVABLE_SUGGESTS: solv_knownid = 11;
pub const solv_knownid_SOLVABLE_SUPPLEMENTS: solv_knownid = 12;
pub const solv_knownid_SOLVABLE_ENHANCES: solv_knownid = 13;
pub const solv_knownid_RPM_RPMDBID: solv_knownid = 14;
pub const solv_knownid_SOLVABLE_PREREQMARKER: solv_knownid = 15;
pub const solv_knownid_SOLVABLE_FILEMARKER: solv_knownid = 16;
pub const solv_knownid_NAMESPACE_INSTALLED: solv_knownid = 17;
pub const solv_knownid_NAMESPACE_MODALIAS: solv_knownid = 18;
pub const solv_knownid_NAMESPACE_SPLITPROVIDES: solv_knownid = 19;
pub const solv_knownid_NAMESPACE_LANGUAGE: solv_knownid = 20;
pub const solv_knownid_NAMESPACE_FILESYSTEM: solv_knownid = 21;
pub const solv_knownid_NAMESPACE_OTHERPROVIDERS: solv_knownid = 22;
pub const solv_knownid_SYSTEM_SYSTEM: solv_knownid = 23;
pub const solv_knownid_ARCH_SRC: solv_knownid = 24;
pub const solv_knownid_ARCH_NOSRC: solv_knownid = 25;
pub const solv_knownid_ARCH_NOARCH: solv_knownid = 26;
pub const solv_knownid_ARCH_ALL: solv_knownid = 27;
pub const solv_knownid_ARCH_ANY: solv_knownid = 28;
pub const solv_knownid_REPOSITORY_SOLVABLES: solv_knownid = 29;
pub const solv_knownid_REPOSITORY_DELTAINFO: solv_knownid = 30;
pub const solv_knownid_REPOSITORY_EXTERNAL: solv_knownid = 31;
pub const solv_knownid_REPOSITORY_KEYS: solv_knownid = 32;
pub const solv_knownid_REPOSITORY_LOCATION: solv_knownid = 33;
pub const solv_knownid_REPOKEY_TYPE_VOID: solv_knownid = 34;
pub const solv_knownid_REPOKEY_TYPE_CONSTANT: solv_knownid = 35;
pub const solv_knownid_REPOKEY_TYPE_CONSTANTID: solv_knownid = 36;
pub const solv_knownid_REPOKEY_TYPE_ID: solv_knownid = 37;
pub const solv_knownid_REPOKEY_TYPE_NUM: solv_knownid = 38;
pub const solv_knownid_REPOKEY_TYPE_DIR: solv_knownid = 39;
pub const solv_knownid_REPOKEY_TYPE_STR: solv_knownid = 40;
pub const solv_knownid_REPOKEY_TYPE_BINARY: solv_knownid = 41;
pub const solv_knownid_REPOKEY_TYPE_IDARRAY: solv_knownid = 42;
pub const solv_knownid_REPOKEY_TYPE_REL_IDARRAY: solv_knownid = 43;
pub const solv_knownid_REPOKEY_TYPE_DIRSTRARRAY: solv_knownid = 44;
pub const solv_knownid_REPOKEY_TYPE_DIRNUMNUMARRAY: solv_knownid = 45;
pub const solv_knownid_REPOKEY_TYPE_MD5: solv_knownid = 46;
pub const solv_knownid_REPOKEY_TYPE_SHA1: solv_knownid = 47;
pub const solv_knownid_REPOKEY_TYPE_SHA224: solv_knownid = 48;
pub const solv_knownid_REPOKEY_TYPE_SHA256: solv_knownid = 49;
pub const solv_knownid_REPOKEY_TYPE_SHA384: solv_knownid = 50;
pub const solv_knownid_REPOKEY_TYPE_SHA512: solv_knownid = 51;
pub const solv_knownid_REPOKEY_TYPE_FIXARRAY: solv_knownid = 52;
pub const solv_knownid_REPOKEY_TYPE_FLEXARRAY: solv_knownid = 53;
pub const solv_knownid_REPOKEY_TYPE_DELETED: solv_knownid = 54;
pub const solv_knownid_SOLVABLE_SUMMARY: solv_knownid = 55;
pub const solv_knownid_SOLVABLE_DESCRIPTION: solv_knownid = 56;
pub const solv_knownid_SOLVABLE_DISTRIBUTION: solv_knownid = 57;
pub const solv_knownid_SOLVABLE_AUTHORS: solv_knownid = 58;
pub const solv_knownid_SOLVABLE_PACKAGER: solv_knownid = 59;
pub const solv_knownid_SOLVABLE_GROUP: solv_knownid = 60;
pub const solv_knownid_SOLVABLE_URL: solv_knownid = 61;
pub const solv_knownid_SOLVABLE_KEYWORDS: solv_knownid = 62;
pub const solv_knownid_SOLVABLE_LICENSE: solv_knownid = 63;
pub const solv_knownid_SOLVABLE_BUILDTIME: solv_knownid = 64;
pub const solv_knownid_SOLVABLE_BUILDHOST: solv_knownid = 65;
pub const solv_knownid_SOLVABLE_EULA: solv_knownid = 66;
pub const solv_knownid_SOLVABLE_CPEID: solv_knownid = 67;
pub const solv_knownid_SOLVABLE_MESSAGEINS: solv_knownid = 68;
pub const solv_knownid_SOLVABLE_MESSAGEDEL: solv_knownid = 69;
pub const solv_knownid_SOLVABLE_INSTALLSIZE: solv_knownid = 70;
pub const solv_knownid_SOLVABLE_DISKUSAGE: solv_knownid = 71;
pub const solv_knownid_SOLVABLE_FILELIST: solv_knownid = 72;
pub const solv_knownid_SOLVABLE_INSTALLTIME: solv_knownid = 73;
pub const solv_knownid_SOLVABLE_MEDIADIR: solv_knownid = 74;
pub const solv_knownid_SOLVABLE_MEDIAFILE: solv_knownid = 75;
pub const solv_knownid_SOLVABLE_MEDIANR: solv_knownid = 76;
pub const solv_knownid_SOLVABLE_MEDIABASE: solv_knownid = 77;
pub const solv_knownid_SOLVABLE_DOWNLOADSIZE: solv_knownid = 78;
pub const solv_knownid_SOLVABLE_SOURCEARCH: solv_knownid = 79;
pub const solv_knownid_SOLVABLE_SOURCENAME: solv_knownid = 80;
pub const solv_knownid_SOLVABLE_SOURCEEVR: solv_knownid = 81;
pub const solv_knownid_SOLVABLE_ISVISIBLE: solv_knownid = 82;
pub const solv_knownid_SOLVABLE_TRIGGERS: solv_knownid = 83;
pub const solv_knownid_SOLVABLE_CHECKSUM: solv_knownid = 84;
pub const solv_knownid_SOLVABLE_PKGID: solv_knownid = 85;
pub const solv_knownid_SOLVABLE_HDRID: solv_knownid = 86;
pub const solv_knownid_SOLVABLE_LEADSIGID: solv_knownid = 87;
pub const solv_knownid_SOLVABLE_PATCHCATEGORY: solv_knownid = 88;
pub const solv_knownid_SOLVABLE_HEADEREND: solv_knownid = 89;
pub const solv_knownid_SOLVABLE_CHANGELOG: solv_knownid = 90;
pub const solv_knownid_SOLVABLE_CHANGELOG_AUTHOR: solv_knownid = 91;
pub const solv_knownid_SOLVABLE_CHANGELOG_TIME: solv_knownid = 92;
pub const solv_knownid_SOLVABLE_CHANGELOG_TEXT: solv_knownid = 93;
pub const solv_knownid_SOLVABLE_INSTALLSTATUS: solv_knownid = 94;
pub const solv_knownid_SOLVABLE_PREREQ_IGNOREINST: solv_knownid = 95;
pub const solv_knownid_SOLVABLE_CATEGORY: solv_knownid = 96;
pub const solv_knownid_SOLVABLE_INCLUDES: solv_knownid = 97;
pub const solv_knownid_SOLVABLE_EXTENDS: solv_knownid = 98;
pub const solv_knownid_SOLVABLE_ICON: solv_knownid = 99;
pub const solv_knownid_SOLVABLE_ORDER: solv_knownid = 100;
pub const solv_knownid_UPDATE_REBOOT: solv_knownid = 101;
pub const solv_knownid_UPDATE_RESTART: solv_knownid = 102;
pub const solv_knownid_UPDATE_RELOGIN: solv_knownid = 103;
pub const solv_knownid_UPDATE_MESSAGE: solv_knownid = 104;
pub const solv_knownid_UPDATE_SEVERITY: solv_knownid = 105;
pub const solv_knownid_UPDATE_RIGHTS: solv_knownid = 106;
pub const solv_knownid_UPDATE_COLLECTION: solv_knownid = 107;
pub const solv_knownid_UPDATE_COLLECTION_NAME: solv_knownid = 108;
pub const solv_knownid_UPDATE_COLLECTION_EVR: solv_knownid = 109;
pub const solv_knownid_UPDATE_COLLECTION_ARCH: solv_knownid = 110;
pub const solv_knownid_UPDATE_COLLECTION_FILENAME: solv_knownid = 111;
pub const solv_knownid_UPDATE_COLLECTION_FLAGS: solv_knownid = 112;
pub const solv_knownid_UPDATE_REFERENCE: solv_knownid = 113;
pub const solv_knownid_UPDATE_REFERENCE_TYPE: solv_knownid = 114;
pub const solv_knownid_UPDATE_REFERENCE_HREF: solv_knownid = 115;
pub const solv_knownid_UPDATE_REFERENCE_ID: solv_knownid = 116;
pub const solv_knownid_UPDATE_REFERENCE_TITLE: solv_knownid = 117;
pub const solv_knownid_PRODUCT_REFERENCEFILE: solv_knownid = 118;
pub const solv_knownid_PRODUCT_SHORTLABEL: solv_knownid = 119;
pub const solv_knownid_PRODUCT_DISTPRODUCT: solv_knownid = 120;
pub const solv_knownid_PRODUCT_DISTVERSION: solv_knownid = 121;
pub const solv_knownid_PRODUCT_TYPE: solv_knownid = 122;
pub const solv_knownid_PRODUCT_URL: solv_knownid = 123;
pub const solv_knownid_PRODUCT_URL_TYPE: solv_knownid = 124;
pub const solv_knownid_PRODUCT_FLAGS: solv_knownid = 125;
pub const solv_knownid_PRODUCT_PRODUCTLINE: solv_knownid = 126;
pub const solv_knownid_PRODUCT_REGISTER_TARGET: solv_knownid = 127;
pub const solv_knownid_PRODUCT_REGISTER_FLAVOR: solv_knownid = 128;
pub const solv_knownid_PRODUCT_REGISTER_RELEASE: solv_knownid = 129;
pub const solv_knownid_PRODUCT_UPDATES_REPOID: solv_knownid = 130;
pub const solv_knownid_PRODUCT_UPDATES: solv_knownid = 131;
pub const solv_knownid_PRODUCT_ENDOFLIFE: solv_knownid = 132;
pub const solv_knownid_SUSETAGS_DATADIR: solv_knownid = 133;
pub const solv_knownid_SUSETAGS_DESCRDIR: solv_knownid = 134;
pub const solv_knownid_SUSETAGS_DEFAULTVENDOR: solv_knownid = 135;
pub const solv_knownid_SUSETAGS_FILE: solv_knownid = 136;
pub const solv_knownid_SUSETAGS_FILE_NAME: solv_knownid = 137;
pub const solv_knownid_SUSETAGS_FILE_TYPE: solv_knownid = 138;
pub const solv_knownid_SUSETAGS_FILE_CHECKSUM: solv_knownid = 139;
pub const solv_knownid_SUSETAGS_SHARE_NAME: solv_knownid = 140;
pub const solv_knownid_SUSETAGS_SHARE_EVR: solv_knownid = 141;
pub const solv_knownid_SUSETAGS_SHARE_ARCH: solv_knownid = 142;
pub const solv_knownid_REPOSITORY_ADDEDFILEPROVIDES: solv_knownid = 143;
pub const solv_knownid_REPOSITORY_RPMDBCOOKIE: solv_knownid = 144;
pub const solv_knownid_REPOSITORY_FILTEREDFILELIST: solv_knownid = 145;
pub const solv_knownid_REPOSITORY_TIMESTAMP: solv_knownid = 146;
pub const solv_knownid_REPOSITORY_EXPIRE: solv_knownid = 147;
pub const solv_knownid_REPOSITORY_UPDATES: solv_knownid = 148;
pub const solv_knownid_REPOSITORY_DISTROS: solv_knownid = 149;
pub const solv_knownid_REPOSITORY_PRODUCT_LABEL: solv_knownid = 150;
pub const solv_knownid_REPOSITORY_PRODUCT_CPEID: solv_knownid = 151;
pub const solv_knownid_REPOSITORY_REPOID: solv_knownid = 152;
pub const solv_knownid_REPOSITORY_KEYWORDS: solv_knownid = 153;
pub const solv_knownid_REPOSITORY_REVISION: solv_knownid = 154;
pub const solv_knownid_REPOSITORY_TOOLVERSION: solv_knownid = 155;
pub const solv_knownid_DELTA_PACKAGE_NAME: solv_knownid = 156;
pub const solv_knownid_DELTA_PACKAGE_EVR: solv_knownid = 157;
pub const solv_knownid_DELTA_PACKAGE_ARCH: solv_knownid = 158;
pub const solv_knownid_DELTA_LOCATION_DIR: solv_knownid = 159;
pub const solv_knownid_DELTA_LOCATION_NAME: solv_knownid = 160;
pub const solv_knownid_DELTA_LOCATION_EVR: solv_knownid = 161;
pub const solv_knownid_DELTA_LOCATION_SUFFIX: solv_knownid = 162;
pub const solv_knownid_DELTA_DOWNLOADSIZE: solv_knownid = 163;
pub const solv_knownid_DELTA_CHECKSUM: solv_knownid = 164;
pub const solv_knownid_DELTA_BASE_EVR: solv_knownid = 165;
pub const solv_knownid_DELTA_SEQ_NAME: solv_knownid = 166;
pub const solv_knownid_DELTA_SEQ_EVR: solv_knownid = 167;
pub const solv_knownid_DELTA_SEQ_NUM: solv_knownid = 168;
pub const solv_knownid_DELTA_LOCATION_BASE: solv_knownid = 169;
pub const solv_knownid_REPOSITORY_REPOMD: solv_knownid = 170;
pub const solv_knownid_REPOSITORY_REPOMD_TYPE: solv_knownid = 171;
pub const solv_knownid_REPOSITORY_REPOMD_LOCATION: solv_knownid = 172;
pub const solv_knownid_REPOSITORY_REPOMD_TIMESTAMP: solv_knownid = 173;
pub const solv_knownid_REPOSITORY_REPOMD_CHECKSUM: solv_knownid = 174;
pub const solv_knownid_REPOSITORY_REPOMD_OPENCHECKSUM: solv_knownid = 175;
pub const solv_knownid_REPOSITORY_REPOMD_SIZE: solv_knownid = 176;
pub const solv_knownid_PUBKEY_KEYID: solv_knownid = 177;
pub const solv_knownid_PUBKEY_FINGERPRINT: solv_knownid = 178;
pub const solv_knownid_PUBKEY_EXPIRES: solv_knownid = 179;
pub const solv_knownid_PUBKEY_SIGNATURES: solv_knownid = 180;
pub const solv_knownid_PUBKEY_DATA: solv_knownid = 181;
pub const solv_knownid_PUBKEY_SUBKEYOF: solv_knownid = 182;
pub const solv_knownid_SIGNATURE_ISSUER: solv_knownid = 183;
pub const solv_knownid_SIGNATURE_TIME: solv_knownid = 184;
pub const solv_knownid_SIGNATURE_EXPIRES: solv_knownid = 185;
pub const solv_knownid_SIGNATURE_DATA: solv_knownid = 186;
pub const solv_knownid_UPDATE_MODULE: solv_knownid = 187;
pub const solv_knownid_UPDATE_MODULE_NAME: solv_knownid = 188;
pub const solv_knownid_UPDATE_MODULE_STREAM: solv_knownid = 189;
pub const solv_knownid_UPDATE_MODULE_VERSION: solv_knownid = 190;
pub const solv_knownid_UPDATE_MODULE_CONTEXT: solv_knownid = 191;
pub const solv_knownid_UPDATE_MODULE_ARCH: solv_knownid = 192;
pub const solv_knownid_SOLVABLE_BUILDVERSION: solv_knownid = 193;
pub const solv_knownid_SOLVABLE_BUILDFLAVOR: solv_knownid = 194;
pub const solv_knownid_UPDATE_STATUS: solv_knownid = 195;
pub const solv_knownid_LIBSOLV_SELF_DESTRUCT_PKG: solv_knownid = 196;
pub const solv_knownid_SOLVABLE_CONSTRAINS: solv_knownid = 197;
pub const solv_knownid_SOLVABLE_TRACK_FEATURES: solv_knownid = 198;
pub const solv_knownid_SOLVABLE_ISDEFAULT: solv_knownid = 199;
pub const solv_knownid_SOLVABLE_LANGONLY: solv_knownid = 200;
pub const solv_knownid_UPDATE_COLLECTIONLIST: solv_knownid = 201;
pub const solv_knownid_SOLVABLE_MULTIARCH: solv_knownid = 202;
pub const solv_knownid_SOLVABLE_SIGNATUREDATA: solv_knownid = 203;
pub const solv_knownid_ID_NUM_INTERNAL: solv_knownid = 204;
pub type solv_knownid = libc::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Datapos {
    pub repo: *mut s_Repo,
    pub solvid: Id,
    pub repodataid: Id,
    pub schema: Id,
    pub dp: Id,
}
pub type Datapos = s_Datapos;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Pool {
    pub appdata: *mut libc::c_void,
    pub ss: s_Stringpool,
    pub rels: *mut Reldep,
    pub nrels: libc::c_int,
    pub repos: *mut *mut s_Repo,
    pub nrepos: libc::c_int,
    pub urepos: libc::c_int,
    pub installed: *mut s_Repo,
    pub solvables: *mut Solvable,
    pub nsolvables: libc::c_int,
    pub languages: *mut *const libc::c_char,
    pub nlanguages: libc::c_int,
    pub disttype: libc::c_int,
    pub id2arch: *mut Id,
    pub id2color: *mut libc::c_uchar,
    pub lastarch: Id,
    pub vendormap: Queue,
    pub vendorclasses: *mut *const libc::c_char,
    pub whatprovides: *mut Offset,
    pub whatprovides_rel: *mut Offset,
    pub whatprovidesdata: *mut Id,
    pub whatprovidesdataoff: Offset,
    pub whatprovidesdataleft: libc::c_int,
    pub considered: *mut Map,
    pub nscallback: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut s_Pool, data: *mut libc::c_void, name: Id, evr: Id) -> Id,
    >,
    pub nscallbackdata: *mut libc::c_void,
    pub debugmask: libc::c_int,
    pub debugcallback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut s_Pool,
            data: *mut libc::c_void,
            type_: libc::c_int,
            str_: *const libc::c_char,
        ),
    >,
    pub debugcallbackdata: *mut libc::c_void,
    pub loadcallback: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut s_Pool,
            arg2: *mut s_Repodata,
            arg3: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub loadcallbackdata: *mut libc::c_void,
    pub pos: Datapos,
    pub pooljobs: Queue,
}
extern "C" {
    pub fn pool_create() -> *mut Pool;
}
extern "C" {
    pub fn pool_free(pool: *mut Pool);
}
extern "C" {
    pub fn pool_freeallrepos(pool: *mut Pool, reuseids: libc::c_int);
}
extern "C" {
    pub fn pool_setdebuglevel(pool: *mut Pool, level: libc::c_int);
}
extern "C" {
    pub fn pool_setdisttype(pool: *mut Pool, disttype: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pool_set_flag(pool: *mut Pool, flag: libc::c_int, value: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pool_get_flag(pool: *mut Pool, flag: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn pool_debug(pool: *mut Pool, type_: libc::c_int, format: *const libc::c_char, ...);
}
extern "C" {
    pub fn pool_setdebugcallback(
        pool: *mut Pool,
        debugcallback: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut s_Pool,
                data: *mut libc::c_void,
                type_: libc::c_int,
                str_: *const libc::c_char,
            ),
        >,
        debugcallbackdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn pool_setdebugmask(pool: *mut Pool, mask: libc::c_int);
}
extern "C" {
    pub fn pool_setloadcallback(
        pool: *mut Pool,
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut s_Pool,
                arg2: *mut s_Repodata,
                arg3: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        loadcbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn pool_setnamespacecallback(
        pool: *mut Pool,
        cb: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut s_Pool,
                arg2: *mut libc::c_void,
                arg3: Id,
                arg4: Id,
            ) -> Id,
        >,
        nscbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn pool_flush_namespaceproviders(pool: *mut Pool, ns: Id, evr: Id);
}
extern "C" {
    pub fn pool_set_custom_vendorcheck(
        pool: *mut Pool,
        vendorcheck: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut s_Pool,
                arg2: *mut Solvable,
                arg3: *mut Solvable,
            ) -> libc::c_int,
        >,
    );
}
extern "C" {
    pub fn pool_get_custom_vendorcheck(
        pool: *mut Pool,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            pool: *mut s_Pool,
            arg1: *mut Solvable,
            arg2: *mut Solvable,
        ) -> libc::c_int,
    >;
}
extern "C" {
    pub fn pool_alloctmpspace(pool: *mut Pool, len: libc::c_int) -> *mut libc::c_char;
}
extern "C" {
    pub fn pool_freetmpspace(pool: *mut Pool, space: *const libc::c_char);
}
extern "C" {
    pub fn pool_tmpjoin(
        pool: *mut Pool,
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn pool_tmpappend(
        pool: *mut Pool,
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn pool_bin2hex(
        pool: *mut Pool,
        buf: *const libc::c_uchar,
        len: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_set_installed(pool: *mut Pool, repo: *mut s_Repo);
}
extern "C" {
    pub fn pool_error(
        pool: *mut Pool,
        ret: libc::c_int,
        format: *const libc::c_char,
        ...
    ) -> libc::c_int;
}
extern "C" {
    pub fn pool_errstr(pool: *mut Pool) -> *mut libc::c_char;
}
extern "C" {
    pub fn pool_set_rootdir(pool: *mut Pool, rootdir: *const libc::c_char);
}
extern "C" {
    pub fn pool_get_rootdir(pool: *mut Pool) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_prepend_rootdir(pool: *mut Pool, dir: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn pool_prepend_rootdir_tmp(
        pool: *mut Pool,
        dir: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C" {
    #[doc = " Solvable management"]
    pub fn pool_add_solvable(pool: *mut Pool) -> Id;
}
extern "C" {
    pub fn pool_add_solvable_block(pool: *mut Pool, count: libc::c_int) -> Id;
}
extern "C" {
    pub fn pool_free_solvable_block(
        pool: *mut Pool,
        start: Id,
        count: libc::c_int,
        reuseids: libc::c_int,
    );
}
extern "C" {
    pub fn pool_solvable2str(pool: *mut Pool, s: *mut Solvable) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_set_languages(
        pool: *mut Pool,
        languages: *mut *const libc::c_char,
        nlanguages: libc::c_int,
    );
}
extern "C" {
    pub fn pool_id2langid(
        pool: *mut Pool,
        id: Id,
        lang: *const libc::c_char,
        create: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn pool_intersect_evrs(
        pool: *mut Pool,
        pflags: libc::c_int,
        pevr: Id,
        flags: libc::c_int,
        evr: Id,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pool_match_dep(pool: *mut Pool, d1: Id, d2: Id) -> libc::c_int;
}
extern "C" {
    pub fn pool_match_nevr_rel(pool: *mut Pool, s: *mut Solvable, d: Id) -> libc::c_int;
}
extern "C" {
    #[doc = " Prepares a pool for solving"]
    pub fn pool_createwhatprovides(pool: *mut Pool);
}
extern "C" {
    pub fn pool_addfileprovides(pool: *mut Pool);
}
extern "C" {
    pub fn pool_addfileprovides_queue(pool: *mut Pool, idq: *mut Queue, idqinst: *mut Queue);
}
extern "C" {
    pub fn pool_freewhatprovides(pool: *mut Pool);
}
extern "C" {
    pub fn pool_queuetowhatprovides(pool: *mut Pool, q: *mut Queue) -> Id;
}
extern "C" {
    pub fn pool_ids2whatprovides(pool: *mut Pool, ids: *mut Id, count: libc::c_int) -> Id;
}
extern "C" {
    pub fn pool_searchlazywhatprovidesq(pool: *mut Pool, d: Id) -> Id;
}
extern "C" {
    pub fn pool_addrelproviders(pool: *mut Pool, d: Id) -> Id;
}
extern "C" {
    pub fn pool_whatmatchesdep(
        pool: *mut Pool,
        keyname: Id,
        dep: Id,
        q: *mut Queue,
        marker: libc::c_int,
    );
}
extern "C" {
    pub fn pool_whatcontainsdep(
        pool: *mut Pool,
        keyname: Id,
        dep: Id,
        q: *mut Queue,
        marker: libc::c_int,
    );
}
extern "C" {
    pub fn pool_whatmatchessolvable(
        pool: *mut Pool,
        keyname: Id,
        solvid: Id,
        q: *mut Queue,
        marker: libc::c_int,
    );
}
extern "C" {
    pub fn pool_set_whatprovides(pool: *mut Pool, id: Id, providers: Id);
}
extern "C" {
    pub fn pool_search(
        pool: *mut Pool,
        p: Id,
        key: Id,
        match_: *const libc::c_char,
        flags: libc::c_int,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut s_Repodata,
                key: *mut s_Repokey,
                kv: *mut s_KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn pool_clear_pos(pool: *mut Pool);
}
extern "C" {
    pub fn pool_lookup_str(pool: *mut Pool, entry: Id, keyname: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_lookup_id(pool: *mut Pool, entry: Id, keyname: Id) -> Id;
}
extern "C" {
    pub fn pool_lookup_num(
        pool: *mut Pool,
        entry: Id,
        keyname: Id,
        notfound: libc::c_ulonglong,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn pool_lookup_void(pool: *mut Pool, entry: Id, keyname: Id) -> libc::c_int;
}
extern "C" {
    pub fn pool_lookup_bin_checksum(
        pool: *mut Pool,
        entry: Id,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_uchar;
}
extern "C" {
    pub fn pool_lookup_idarray(
        pool: *mut Pool,
        entry: Id,
        keyname: Id,
        q: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pool_lookup_checksum(
        pool: *mut Pool,
        entry: Id,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_lookup_deltalocation(
        pool: *mut Pool,
        entry: Id,
        medianrp: *mut libc::c_uint,
    ) -> *const libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_DUChanges {
    pub path: *const libc::c_char,
    pub kbytes: libc::c_longlong,
    pub files: libc::c_longlong,
    pub flags: libc::c_int,
}
pub type DUChanges = s_DUChanges;
extern "C" {
    pub fn pool_create_state_maps(
        pool: *mut Pool,
        installed: *mut Queue,
        installedmap: *mut Map,
        conflictsmap: *mut Map,
    );
}
extern "C" {
    pub fn pool_calc_duchanges(
        pool: *mut Pool,
        installedmap: *mut Map,
        mps: *mut DUChanges,
        nmps: libc::c_int,
    );
}
extern "C" {
    pub fn pool_calc_installsizechange(pool: *mut Pool, installedmap: *mut Map)
        -> libc::c_longlong;
}
extern "C" {
    pub fn pool_add_fileconflicts_deps(pool: *mut Pool, conflicts: *mut Queue);
}
extern "C" {
    pub fn pool_trivial_installable_multiversionmap(
        pool: *mut Pool,
        installedmap: *mut Map,
        pkgs: *mut Queue,
        res: *mut Queue,
        multiversionmap: *mut Map,
    );
}
extern "C" {
    pub fn pool_trivial_installable(
        pool: *mut Pool,
        installedmap: *mut Map,
        pkgs: *mut Queue,
        res: *mut Queue,
    );
}
extern "C" {
    pub fn pool_setarch(arg1: *mut Pool, arg2: *const libc::c_char);
}
extern "C" {
    pub fn pool_setarchpolicy(arg1: *mut Pool, arg2: *const libc::c_char);
}
extern "C" {
    pub fn pool_arch2color_slow(pool: *mut Pool, arch: Id) -> libc::c_uchar;
}
extern "C" {
    #[doc = " malloc\n exits with error message on error"]
    pub fn solv_malloc(arg1: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_malloc2(arg1: usize, arg2: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_calloc(arg1: usize, arg2: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_realloc(arg1: *mut libc::c_void, arg2: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_realloc2(arg1: *mut libc::c_void, arg2: usize, arg3: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_extend_realloc(
        arg1: *mut libc::c_void,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_free(arg1: *mut libc::c_void) -> *mut libc::c_void;
}
extern "C" {
    pub fn solv_strdup(arg1: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn solv_oom(arg1: usize, arg2: usize);
}
extern "C" {
    pub fn solv_timems(subtract: libc::c_uint) -> libc::c_uint;
}
extern "C" {
    pub fn solv_setcloexec(fd: libc::c_int, state: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn solv_sort(
        base: *mut libc::c_void,
        nmemb: usize,
        size: usize,
        compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const libc::c_void,
                arg2: *const libc::c_void,
                arg3: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        compard: *mut libc::c_void,
    );
}
extern "C" {
    pub fn solv_dupjoin(
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn solv_dupappend(
        str1: *const libc::c_char,
        str2: *const libc::c_char,
        str3: *const libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn solv_hex2bin(
        strp: *mut *const libc::c_char,
        buf: *mut libc::c_uchar,
        bufl: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solv_bin2hex(
        buf: *const libc::c_uchar,
        l: libc::c_int,
        str_: *mut libc::c_char,
    ) -> *mut libc::c_char;
}
extern "C" {
    pub fn solv_validutf8(buf: *const libc::c_char) -> usize;
}
extern "C" {
    pub fn solv_latin1toutf8(buf: *const libc::c_char) -> *mut libc::c_char;
}
extern "C" {
    pub fn solv_replacebadutf8(
        buf: *const libc::c_char,
        replchar: libc::c_int,
    ) -> *mut libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Dirpool {
    pub dirs: *mut Id,
    pub ndirs: libc::c_int,
    pub dirtraverse: *mut Id,
}
pub type Dirpool = s_Dirpool;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Repokey {
    pub name: Id,
    pub type_: Id,
    pub size: libc::c_uint,
    pub storage: libc::c_uint,
}
pub type Repokey = s_Repokey;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Repodata {
    pub repodataid: Id,
    pub repo: *mut s_Repo,
    pub state: libc::c_int,
    pub loadcallback: ::std::option::Option<unsafe extern "C" fn(arg1: *mut s_Repodata)>,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub keys: *mut Repokey,
    pub nkeys: libc::c_int,
    pub keybits: [libc::c_uchar; 32usize],
    pub schemata: *mut Id,
    pub nschemata: libc::c_int,
    pub schemadata: *mut Id,
    pub spool: Stringpool,
    pub localpool: libc::c_int,
    pub dirpool: Dirpool,
}
pub type Repodata = s_Repodata;
extern "C" {
    pub fn repodata_initdata(data: *mut Repodata, repo: *mut s_Repo, localpool: libc::c_int);
}
extern "C" {
    pub fn repodata_freedata(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_free(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_empty(data: *mut Repodata, localpool: libc::c_int);
}
extern "C" {
    pub fn repodata_load(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_key2id(data: *mut Repodata, key: *mut Repokey, create: libc::c_int) -> Id;
}
extern "C" {
    pub fn repodata_schema2id(data: *mut Repodata, schema: *mut Id, create: libc::c_int) -> Id;
}
extern "C" {
    pub fn repodata_free_schemahash(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_search(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        flags: libc::c_int,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut Repodata,
                key: *mut Repokey,
                kv: *mut s_KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repodata_search_keyskip(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        flags: libc::c_int,
        keyskip: *mut Id,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut Repodata,
                key: *mut Repokey,
                kv: *mut s_KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repodata_search_arrayelement(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        flags: libc::c_int,
        kv: *mut s_KeyValue,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut Repodata,
                key: *mut Repokey,
                kv: *mut s_KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repodata_stringify(
        pool: *mut Pool,
        data: *mut Repodata,
        key: *mut Repokey,
        kv: *mut s_KeyValue,
        flags: libc::c_int,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn repodata_set_filelisttype(data: *mut Repodata, filelisttype: libc::c_int);
}
extern "C" {
    pub fn repodata_filelistfilter_matches(
        data: *mut Repodata,
        str_: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repodata_free_filelistfilter(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_lookup_type(data: *mut Repodata, solvid: Id, keyname: Id) -> Id;
}
extern "C" {
    pub fn repodata_lookup_id(data: *mut Repodata, solvid: Id, keyname: Id) -> Id;
}
extern "C" {
    pub fn repodata_lookup_str(data: *mut Repodata, solvid: Id, keyname: Id)
        -> *const libc::c_char;
}
extern "C" {
    pub fn repodata_lookup_num(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        notfound: libc::c_ulonglong,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn repodata_lookup_void(data: *mut Repodata, solvid: Id, keyname: Id) -> libc::c_int;
}
extern "C" {
    pub fn repodata_lookup_bin_checksum(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_uchar;
}
extern "C" {
    pub fn repodata_lookup_idarray(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        q: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repodata_lookup_binary(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        lenp: *mut libc::c_int,
    ) -> *const libc::c_void;
}
extern "C" {
    pub fn repodata_lookup_count(data: *mut Repodata, solvid: Id, keyname: Id) -> libc::c_uint;
}
extern "C" {
    pub fn repodata_lookup_packed_dirstrarray(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
    ) -> *const libc::c_uchar;
}
extern "C" {
    pub fn repodata_fill_keyskip(data: *mut Repodata, solvid: Id, keyskip: *mut Id) -> *mut Id;
}
extern "C" {
    pub fn repodata_extend(data: *mut Repodata, p: Id);
}
extern "C" {
    pub fn repodata_extend_block(data: *mut Repodata, p: Id, num: libc::c_int);
}
extern "C" {
    pub fn repodata_shrink(data: *mut Repodata, end: libc::c_int);
}
extern "C" {
    pub fn repodata_internalize(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_new_handle(data: *mut Repodata) -> Id;
}
extern "C" {
    pub fn repodata_set_void(data: *mut Repodata, solvid: Id, keyname: Id);
}
extern "C" {
    pub fn repodata_set_num(data: *mut Repodata, solvid: Id, keyname: Id, num: libc::c_ulonglong);
}
extern "C" {
    pub fn repodata_set_id(data: *mut Repodata, solvid: Id, keyname: Id, id: Id);
}
extern "C" {
    pub fn repodata_set_str(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        str_: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_set_binary(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        buf: *mut libc::c_void,
        len: libc::c_int,
    );
}
extern "C" {
    pub fn repodata_set_poolstr(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        str_: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_set_constant(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        constant: libc::c_uint,
    );
}
extern "C" {
    pub fn repodata_set_constantid(data: *mut Repodata, solvid: Id, keyname: Id, id: Id);
}
extern "C" {
    pub fn repodata_set_bin_checksum(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        type_: Id,
        buf: *const libc::c_uchar,
    );
}
extern "C" {
    pub fn repodata_set_checksum(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        type_: Id,
        str_: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_set_idarray(data: *mut Repodata, solvid: Id, keyname: Id, q: *mut Queue);
}
extern "C" {
    pub fn repodata_add_dirnumnum(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        dir: Id,
        num: Id,
        num2: Id,
    );
}
extern "C" {
    pub fn repodata_add_dirstr(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        dir: Id,
        str_: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_free_dircache(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_add_idarray(data: *mut Repodata, solvid: Id, keyname: Id, id: Id);
}
extern "C" {
    pub fn repodata_add_poolstr_array(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        str_: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_add_fixarray(data: *mut Repodata, solvid: Id, keyname: Id, ghandle: Id);
}
extern "C" {
    pub fn repodata_add_flexarray(data: *mut Repodata, solvid: Id, keyname: Id, ghandle: Id);
}
extern "C" {
    pub fn repodata_set_kv(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        keytype: Id,
        kv: *mut s_KeyValue,
    );
}
extern "C" {
    pub fn repodata_unset(data: *mut Repodata, solvid: Id, keyname: Id);
}
extern "C" {
    pub fn repodata_unset_uninternalized(data: *mut Repodata, solvid: Id, keyname: Id);
}
extern "C" {
    pub fn repodata_merge_attrs(data: *mut Repodata, dest: Id, src: Id);
}
extern "C" {
    pub fn repodata_merge_some_attrs(
        data: *mut Repodata,
        dest: Id,
        src: Id,
        keyidmap: *mut Map,
        overwrite: libc::c_int,
    );
}
extern "C" {
    pub fn repodata_swap_attrs(data: *mut Repodata, dest: Id, src: Id);
}
extern "C" {
    pub fn repodata_create_stubs(data: *mut Repodata) -> *mut Repodata;
}
extern "C" {
    pub fn repodata_disable_paging(data: *mut Repodata);
}
extern "C" {
    pub fn repodata_globalize_id(data: *mut Repodata, id: Id, create: libc::c_int) -> Id;
}
extern "C" {
    pub fn repodata_localize_id(data: *mut Repodata, id: Id, create: libc::c_int) -> Id;
}
extern "C" {
    pub fn repodata_translate_id(
        data: *mut Repodata,
        fromdata: *mut Repodata,
        id: Id,
        create: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn repodata_translate_dir_slow(
        data: *mut Repodata,
        fromdata: *mut Repodata,
        dir: Id,
        create: libc::c_int,
        cache: *mut Id,
    ) -> Id;
}
extern "C" {
    pub fn repodata_str2dir(
        data: *mut Repodata,
        dir: *const libc::c_char,
        create: libc::c_int,
    ) -> Id;
}
extern "C" {
    pub fn repodata_dir2str(
        data: *mut Repodata,
        did: Id,
        suf: *const libc::c_char,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn repodata_chk2str(
        data: *mut Repodata,
        type_: Id,
        buf: *const libc::c_uchar,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn repodata_set_location(
        data: *mut Repodata,
        solvid: Id,
        medianr: libc::c_int,
        dir: *const libc::c_char,
        file: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_set_deltalocation(
        data: *mut Repodata,
        handle: Id,
        medianr: libc::c_int,
        dir: *const libc::c_char,
        file: *const libc::c_char,
    );
}
extern "C" {
    pub fn repodata_set_sourcepkg(data: *mut Repodata, solvid: Id, sourcepkg: *const libc::c_char);
}
extern "C" {
    pub fn repodata_lookup_kv_uninternalized(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        kv: *mut s_KeyValue,
    ) -> *mut Repokey;
}
extern "C" {
    pub fn repodata_search_uninternalized(
        data: *mut Repodata,
        solvid: Id,
        keyname: Id,
        flags: libc::c_int,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut Repodata,
                key: *mut Repokey,
                kv: *mut s_KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repodata_memused(data: *mut Repodata) -> libc::c_uint;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_KeyValue {
    pub id: Id,
    pub str_: *const libc::c_char,
    pub num: libc::c_uint,
    pub num2: libc::c_uint,
    pub entry: libc::c_int,
    pub eof: libc::c_int,
    pub parent: *mut s_KeyValue,
}
pub type KeyValue = s_KeyValue;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Datamatcher {
    pub flags: libc::c_int,
    pub match_: *const libc::c_char,
    pub matchdata: *mut libc::c_void,
    pub error: libc::c_int,
}
pub type Datamatcher = s_Datamatcher;
extern "C" {
    pub fn datamatcher_init(
        ma: *mut Datamatcher,
        match_: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn datamatcher_free(ma: *mut Datamatcher);
}
extern "C" {
    pub fn datamatcher_match(ma: *mut Datamatcher, str_: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    pub fn datamatcher_checkbasename(
        ma: *mut Datamatcher,
        str_: *const libc::c_char,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Dataiterator {
    pub state: libc::c_int,
    pub flags: libc::c_int,
    pub pool: *mut Pool,
    pub repo: *mut s_Repo,
    pub data: *mut s_Repodata,
    pub dp: *mut libc::c_uchar,
    pub ddp: *mut libc::c_uchar,
    pub idp: *mut Id,
    pub keyp: *mut Id,
    pub key: *mut s_Repokey,
    pub kv: KeyValue,
    pub matcher: Datamatcher,
    pub keyname: Id,
    pub repodataid: Id,
    pub solvid: Id,
    pub repoid: Id,
    pub keynames: [Id; 4usize],
    pub nkeynames: libc::c_int,
    pub rootlevel: libc::c_int,
    pub parents: [s_Dataiterator_di_parent; 3usize],
    pub nparents: libc::c_int,
    pub vert_ddp: *mut libc::c_uchar,
    pub vert_off: Id,
    pub vert_len: Id,
    pub vert_storestate: Id,
    pub dupstr: *mut libc::c_char,
    pub dupstrn: libc::c_int,
    pub keyskip: *mut Id,
    pub oldkeyskip: *mut Id,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Dataiterator_di_parent {
    pub kv: KeyValue,
    pub dp: *mut libc::c_uchar,
    pub keyp: *mut Id,
}
pub type Dataiterator = s_Dataiterator;
extern "C" {
    pub fn dataiterator_init(
        di: *mut Dataiterator,
        pool: *mut Pool,
        repo: *mut s_Repo,
        p: Id,
        keyname: Id,
        match_: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn dataiterator_init_clone(di: *mut Dataiterator, from: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_set_search(di: *mut Dataiterator, repo: *mut s_Repo, p: Id);
}
extern "C" {
    pub fn dataiterator_set_keyname(di: *mut Dataiterator, keyname: Id);
}
extern "C" {
    pub fn dataiterator_set_match(
        di: *mut Dataiterator,
        match_: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn dataiterator_prepend_keyname(di: *mut Dataiterator, keyname: Id);
}
extern "C" {
    pub fn dataiterator_free(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_step(di: *mut Dataiterator) -> libc::c_int;
}
extern "C" {
    pub fn dataiterator_setpos(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_setpos_parent(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_match(di: *mut Dataiterator, ma: *mut Datamatcher) -> libc::c_int;
}
extern "C" {
    pub fn dataiterator_skip_attribute(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_skip_solvable(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_skip_repo(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_jump_to_solvid(di: *mut Dataiterator, solvid: Id);
}
extern "C" {
    pub fn dataiterator_jump_to_repo(di: *mut Dataiterator, repo: *mut s_Repo);
}
extern "C" {
    pub fn dataiterator_entersub(di: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_clonepos(di: *mut Dataiterator, from: *mut Dataiterator);
}
extern "C" {
    pub fn dataiterator_seek(di: *mut Dataiterator, whence: libc::c_int);
}
extern "C" {
    pub fn dataiterator_strdup(di: *mut Dataiterator);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Repo {
    pub name: *const libc::c_char,
    pub repoid: Id,
    pub appdata: *mut libc::c_void,
    pub pool: *mut Pool,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub nsolvables: libc::c_int,
    pub disabled: libc::c_int,
    pub priority: libc::c_int,
    pub subpriority: libc::c_int,
    pub idarraydata: *mut Id,
    pub idarraysize: libc::c_int,
    pub nrepodata: libc::c_int,
    pub rpmdbid: *mut Id,
}
pub type Repo = s_Repo;
extern "C" {
    pub fn repo_create(pool: *mut Pool, name: *const libc::c_char) -> *mut Repo;
}
extern "C" {
    pub fn repo_free(repo: *mut Repo, reuseids: libc::c_int);
}
extern "C" {
    pub fn repo_empty(repo: *mut Repo, reuseids: libc::c_int);
}
extern "C" {
    pub fn repo_freedata(repo: *mut Repo);
}
extern "C" {
    pub fn repo_add_solvable(repo: *mut Repo) -> Id;
}
extern "C" {
    pub fn repo_add_solvable_block(repo: *mut Repo, count: libc::c_int) -> Id;
}
extern "C" {
    pub fn repo_free_solvable(repo: *mut Repo, p: Id, reuseids: libc::c_int);
}
extern "C" {
    pub fn repo_free_solvable_block(
        repo: *mut Repo,
        start: Id,
        count: libc::c_int,
        reuseids: libc::c_int,
    );
}
extern "C" {
    pub fn repo_sidedata_create(repo: *mut Repo, size: usize) -> *mut libc::c_void;
}
extern "C" {
    pub fn repo_sidedata_extend(
        repo: *mut Repo,
        b: *mut libc::c_void,
        size: usize,
        p: Id,
        count: libc::c_int,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn repo_add_solvable_block_before(
        repo: *mut Repo,
        count: libc::c_int,
        beforerepo: *mut Repo,
    ) -> Id;
}
extern "C" {
    pub fn repo_addid(repo: *mut Repo, olddeps: Offset, id: Id) -> Offset;
}
extern "C" {
    pub fn repo_addid_dep(repo: *mut Repo, olddeps: Offset, id: Id, marker: Id) -> Offset;
}
extern "C" {
    pub fn repo_reserve_ids(repo: *mut Repo, olddeps: Offset, num: libc::c_int) -> Offset;
}
extern "C" {
    pub fn repo_add_repodata(repo: *mut Repo, flags: libc::c_int) -> *mut Repodata;
}
extern "C" {
    pub fn repo_id2repodata(repo: *mut Repo, id: Id) -> *mut Repodata;
}
extern "C" {
    pub fn repo_last_repodata(repo: *mut Repo) -> *mut Repodata;
}
extern "C" {
    pub fn repo_search(
        repo: *mut Repo,
        p: Id,
        key: Id,
        match_: *const libc::c_char,
        flags: libc::c_int,
        callback: ::std::option::Option<
            unsafe extern "C" fn(
                cbdata: *mut libc::c_void,
                s: *mut Solvable,
                data: *mut Repodata,
                key: *mut Repokey,
                kv: *mut KeyValue,
            ) -> libc::c_int,
        >,
        cbdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repo_lookup_repodata(repo: *mut Repo, entry: Id, keyname: Id) -> *mut Repodata;
}
extern "C" {
    pub fn repo_lookup_repodata_opt(repo: *mut Repo, entry: Id, keyname: Id) -> *mut Repodata;
}
extern "C" {
    pub fn repo_lookup_filelist_repodata(
        repo: *mut Repo,
        entry: Id,
        matcher: *mut Datamatcher,
    ) -> *mut Repodata;
}
extern "C" {
    pub fn repo_lookup_type(repo: *mut Repo, entry: Id, keyname: Id) -> Id;
}
extern "C" {
    pub fn repo_lookup_str(repo: *mut Repo, entry: Id, keyname: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn repo_lookup_num(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        notfound: libc::c_ulonglong,
    ) -> libc::c_ulonglong;
}
extern "C" {
    pub fn repo_lookup_id(repo: *mut Repo, entry: Id, keyname: Id) -> Id;
}
extern "C" {
    pub fn repo_lookup_idarray(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        q: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repo_lookup_deparray(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        q: *mut Queue,
        marker: Id,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repo_lookup_void(repo: *mut Repo, entry: Id, keyname: Id) -> libc::c_int;
}
extern "C" {
    pub fn repo_lookup_checksum(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn repo_lookup_bin_checksum(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        typep: *mut Id,
    ) -> *const libc::c_uchar;
}
extern "C" {
    pub fn repo_lookup_binary(
        repo: *mut Repo,
        entry: Id,
        keyname: Id,
        lenp: *mut libc::c_int,
    ) -> *const libc::c_void;
}
extern "C" {
    pub fn repo_lookup_count(repo: *mut Repo, entry: Id, keyname: Id) -> libc::c_uint;
}
extern "C" {
    pub fn solv_depmarker(keyname: Id, marker: Id) -> Id;
}
extern "C" {
    pub fn repo_set_id(repo: *mut Repo, p: Id, keyname: Id, id: Id);
}
extern "C" {
    pub fn repo_set_num(repo: *mut Repo, p: Id, keyname: Id, num: libc::c_ulonglong);
}
extern "C" {
    pub fn repo_set_str(repo: *mut Repo, p: Id, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn repo_set_poolstr(repo: *mut Repo, p: Id, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn repo_add_poolstr_array(repo: *mut Repo, p: Id, keyname: Id, str_: *const libc::c_char);
}
extern "C" {
    pub fn repo_add_idarray(repo: *mut Repo, p: Id, keyname: Id, id: Id);
}
extern "C" {
    pub fn repo_add_deparray(repo: *mut Repo, p: Id, keyname: Id, dep: Id, marker: Id);
}
extern "C" {
    pub fn repo_set_idarray(repo: *mut Repo, p: Id, keyname: Id, q: *mut Queue);
}
extern "C" {
    pub fn repo_set_deparray(repo: *mut Repo, p: Id, keyname: Id, q: *mut Queue, marker: Id);
}
extern "C" {
    pub fn repo_unset(repo: *mut Repo, p: Id, keyname: Id);
}
extern "C" {
    pub fn repo_internalize(repo: *mut Repo);
}
extern "C" {
    pub fn repo_disable_paging(repo: *mut Repo);
}
extern "C" {
    pub fn repo_create_keyskip(repo: *mut Repo, entry: Id, oldkeyskip: *mut *mut Id) -> *mut Id;
}
extern "C" {
    pub fn repo_fix_supplements(
        repo: *mut Repo,
        provides: Offset,
        supplements: Offset,
        freshens: Offset,
    ) -> Offset;
}
extern "C" {
    pub fn repo_fix_conflicts(repo: *mut Repo, conflicts: Offset) -> Offset;
}
extern "C" {
    pub fn repo_rewrite_suse_deps(s: *mut Solvable, freshens: Offset);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Transaction {
    pub pool: *mut s_Pool,
    pub steps: Queue,
}
pub type Transaction = s_Transaction;
extern "C" {
    pub fn transaction_create(pool: *mut s_Pool) -> *mut Transaction;
}
extern "C" {
    pub fn transaction_create_decisionq(
        pool: *mut s_Pool,
        decisionq: *mut Queue,
        multiversionmap: *mut Map,
    ) -> *mut Transaction;
}
extern "C" {
    pub fn transaction_create_clone(srctrans: *mut Transaction) -> *mut Transaction;
}
extern "C" {
    pub fn transaction_free(trans: *mut Transaction);
}
extern "C" {
    pub fn transaction_obs_pkg(trans: *mut Transaction, p: Id) -> Id;
}
extern "C" {
    pub fn transaction_all_obs_pkgs(trans: *mut Transaction, p: Id, pkgs: *mut Queue);
}
extern "C" {
    pub fn transaction_type(trans: *mut Transaction, p: Id, mode: libc::c_int) -> Id;
}
extern "C" {
    pub fn transaction_classify(trans: *mut Transaction, mode: libc::c_int, classes: *mut Queue);
}
extern "C" {
    pub fn transaction_classify_pkgs(
        trans: *mut Transaction,
        mode: libc::c_int,
        type_: Id,
        from: Id,
        to: Id,
        pkgs: *mut Queue,
    );
}
extern "C" {
    pub fn transaction_installedresult(
        trans: *mut Transaction,
        installedq: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn transaction_calc_installsizechange(trans: *mut Transaction) -> libc::c_longlong;
}
extern "C" {
    pub fn transaction_calc_duchanges(
        trans: *mut Transaction,
        mps: *mut s_DUChanges,
        nmps: libc::c_int,
    );
}
extern "C" {
    pub fn transaction_order(trans: *mut Transaction, flags: libc::c_int);
}
extern "C" {
    pub fn transaction_order_add_choices(
        trans: *mut Transaction,
        chosen: Id,
        choices: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn transaction_add_obsoleted(trans: *mut Transaction);
}
extern "C" {
    pub fn transaction_check_order(trans: *mut Transaction);
}
extern "C" {
    pub fn transaction_order_get_cycleids(
        trans: *mut Transaction,
        q: *mut Queue,
        minseverity: libc::c_int,
    );
}
extern "C" {
    pub fn transaction_order_get_cycle(
        trans: *mut Transaction,
        cid: Id,
        q: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn transaction_order_get_edges(
        trans: *mut Transaction,
        p: Id,
        q: *mut Queue,
        unbroken: libc::c_int,
    );
}
extern "C" {
    pub fn transaction_free_orderdata(trans: *mut Transaction);
}
extern "C" {
    pub fn transaction_clone_orderdata(trans: *mut Transaction, srctrans: *mut Transaction);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Rule {
    pub p: Id,
    pub d: Id,
    pub w1: Id,
    pub w2: Id,
    pub n1: Id,
    pub n2: Id,
}
pub type Rule = s_Rule;
pub const SolverRuleinfo_SOLVER_RULE_UNKNOWN: SolverRuleinfo = 0;
pub const SolverRuleinfo_SOLVER_RULE_PKG: SolverRuleinfo = 256;
pub const SolverRuleinfo_SOLVER_RULE_PKG_NOT_INSTALLABLE: SolverRuleinfo = 257;
pub const SolverRuleinfo_SOLVER_RULE_PKG_NOTHING_PROVIDES_DEP: SolverRuleinfo = 258;
pub const SolverRuleinfo_SOLVER_RULE_PKG_REQUIRES: SolverRuleinfo = 259;
pub const SolverRuleinfo_SOLVER_RULE_PKG_SELF_CONFLICT: SolverRuleinfo = 260;
pub const SolverRuleinfo_SOLVER_RULE_PKG_CONFLICTS: SolverRuleinfo = 261;
pub const SolverRuleinfo_SOLVER_RULE_PKG_SAME_NAME: SolverRuleinfo = 262;
pub const SolverRuleinfo_SOLVER_RULE_PKG_OBSOLETES: SolverRuleinfo = 263;
pub const SolverRuleinfo_SOLVER_RULE_PKG_IMPLICIT_OBSOLETES: SolverRuleinfo = 264;
pub const SolverRuleinfo_SOLVER_RULE_PKG_INSTALLED_OBSOLETES: SolverRuleinfo = 265;
pub const SolverRuleinfo_SOLVER_RULE_PKG_RECOMMENDS: SolverRuleinfo = 266;
pub const SolverRuleinfo_SOLVER_RULE_PKG_CONSTRAINS: SolverRuleinfo = 267;
pub const SolverRuleinfo_SOLVER_RULE_UPDATE: SolverRuleinfo = 512;
pub const SolverRuleinfo_SOLVER_RULE_FEATURE: SolverRuleinfo = 768;
pub const SolverRuleinfo_SOLVER_RULE_JOB: SolverRuleinfo = 1024;
pub const SolverRuleinfo_SOLVER_RULE_JOB_NOTHING_PROVIDES_DEP: SolverRuleinfo = 1025;
pub const SolverRuleinfo_SOLVER_RULE_JOB_PROVIDED_BY_SYSTEM: SolverRuleinfo = 1026;
pub const SolverRuleinfo_SOLVER_RULE_JOB_UNKNOWN_PACKAGE: SolverRuleinfo = 1027;
pub const SolverRuleinfo_SOLVER_RULE_JOB_UNSUPPORTED: SolverRuleinfo = 1028;
pub const SolverRuleinfo_SOLVER_RULE_DISTUPGRADE: SolverRuleinfo = 1280;
pub const SolverRuleinfo_SOLVER_RULE_INFARCH: SolverRuleinfo = 1536;
pub const SolverRuleinfo_SOLVER_RULE_CHOICE: SolverRuleinfo = 1792;
pub const SolverRuleinfo_SOLVER_RULE_LEARNT: SolverRuleinfo = 2048;
pub const SolverRuleinfo_SOLVER_RULE_BEST: SolverRuleinfo = 2304;
pub const SolverRuleinfo_SOLVER_RULE_YUMOBS: SolverRuleinfo = 2560;
pub const SolverRuleinfo_SOLVER_RULE_RECOMMENDS: SolverRuleinfo = 2816;
pub const SolverRuleinfo_SOLVER_RULE_BLACK: SolverRuleinfo = 3072;
pub const SolverRuleinfo_SOLVER_RULE_STRICT_REPO_PRIORITY: SolverRuleinfo = 3328;
pub type SolverRuleinfo = libc::c_uint;
extern "C" {
    pub fn solver_addrule(solv: *mut s_Solver, p: Id, p2: Id, d: Id) -> *mut Rule;
}
extern "C" {
    pub fn solver_unifyrules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_rulecmp(solv: *mut s_Solver, r1: *mut Rule, r2: *mut Rule) -> libc::c_int;
}
extern "C" {
    pub fn solver_shrinkrules(solv: *mut s_Solver, nrules: libc::c_int);
}
extern "C" {
    pub fn solver_addpkgrulesforsolvable(solv: *mut s_Solver, s: *mut Solvable, m: *mut Map);
}
extern "C" {
    pub fn solver_addpkgrulesforweak(solv: *mut s_Solver, m: *mut Map);
}
extern "C" {
    pub fn solver_addpkgrulesforlinked(solv: *mut s_Solver, m: *mut Map);
}
extern "C" {
    pub fn solver_addpkgrulesforupdaters(
        solv: *mut s_Solver,
        s: *mut Solvable,
        m: *mut Map,
        allow_all: libc::c_int,
    );
}
extern "C" {
    pub fn solver_addfeaturerule(solv: *mut s_Solver, s: *mut Solvable);
}
extern "C" {
    pub fn solver_addupdaterule(solv: *mut s_Solver, s: *mut Solvable);
}
extern "C" {
    pub fn solver_addinfarchrules(solv: *mut s_Solver, addedmap: *mut Map);
}
extern "C" {
    pub fn solver_createdupmaps(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_freedupmaps(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_addduprules(solv: *mut s_Solver, addedmap: *mut Map);
}
extern "C" {
    pub fn solver_addchoicerules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_disablechoicerules(solv: *mut s_Solver, r: *mut Rule);
}
extern "C" {
    pub fn solver_addbestrules(
        solv: *mut s_Solver,
        havebestinstalljobs: libc::c_int,
        haslockjob: libc::c_int,
    );
}
extern "C" {
    pub fn solver_addyumobsrules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_addblackrules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_addrecommendsrules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_addstrictrepopriorules(solv: *mut s_Solver, addedmap: *mut Map);
}
extern "C" {
    pub fn solver_disablepolicyrules(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_reenablepolicyrules(solv: *mut s_Solver, jobidx: libc::c_int);
}
extern "C" {
    pub fn solver_reenablepolicyrules_cleandeps(solv: *mut s_Solver, pkg: Id);
}
extern "C" {
    pub fn solver_allruleinfos(solv: *mut s_Solver, rid: Id, rq: *mut Queue) -> libc::c_int;
}
extern "C" {
    pub fn solver_ruleinfo(
        solv: *mut s_Solver,
        rid: Id,
        fromp: *mut Id,
        top: *mut Id,
        depp: *mut Id,
    ) -> SolverRuleinfo;
}
extern "C" {
    pub fn solver_ruleclass(solv: *mut s_Solver, rid: Id) -> SolverRuleinfo;
}
extern "C" {
    pub fn solver_ruleliterals(solv: *mut s_Solver, rid: Id, q: *mut Queue);
}
extern "C" {
    pub fn solver_rule2jobidx(solv: *mut s_Solver, rid: Id) -> libc::c_int;
}
extern "C" {
    pub fn solver_rule2job(solv: *mut s_Solver, rid: Id, whatp: *mut Id) -> Id;
}
extern "C" {
    pub fn solver_rule2solvable(solv: *mut s_Solver, rid: Id) -> Id;
}
extern "C" {
    pub fn solver_rule2rules(solv: *mut s_Solver, rid: Id, q: *mut Queue, recursive: libc::c_int);
}
extern "C" {
    pub fn solver_rule2pkgrule(solv: *mut s_Solver, rid: Id) -> Id;
}
extern "C" {
    pub fn solver_breakorphans(solv: *mut s_Solver);
}
extern "C" {
    pub fn solver_check_brokenorphanrules(solv: *mut s_Solver, dq: *mut Queue);
}
extern "C" {
    pub fn solver_recordproblem(solv: *mut s_Solver, rid: Id);
}
extern "C" {
    pub fn solver_fixproblem(solv: *mut s_Solver, rid: Id);
}
extern "C" {
    pub fn solver_autouninstall(solv: *mut s_Solver, start: libc::c_int) -> Id;
}
extern "C" {
    pub fn solver_disableproblemset(solv: *mut s_Solver, start: libc::c_int);
}
extern "C" {
    pub fn solver_prepare_solutions(solv: *mut s_Solver) -> libc::c_int;
}
extern "C" {
    pub fn solver_problem_count(solv: *mut s_Solver) -> libc::c_uint;
}
extern "C" {
    pub fn solver_next_problem(solv: *mut s_Solver, problem: Id) -> Id;
}
extern "C" {
    pub fn solver_solution_count(solv: *mut s_Solver, problem: Id) -> libc::c_uint;
}
extern "C" {
    pub fn solver_next_solution(solv: *mut s_Solver, problem: Id, solution: Id) -> Id;
}
extern "C" {
    pub fn solver_solutionelement_count(
        solv: *mut s_Solver,
        problem: Id,
        solution: Id,
    ) -> libc::c_uint;
}
extern "C" {
    pub fn solver_solutionelement_internalid(solv: *mut s_Solver, problem: Id, solution: Id) -> Id;
}
extern "C" {
    pub fn solver_solutionelement_extrajobflags(
        solv: *mut s_Solver,
        problem: Id,
        solution: Id,
    ) -> Id;
}
extern "C" {
    pub fn solver_next_solutionelement(
        solv: *mut s_Solver,
        problem: Id,
        solution: Id,
        element: Id,
        p: *mut Id,
        rp: *mut Id,
    ) -> Id;
}
extern "C" {
    pub fn solver_take_solutionelement(
        solv: *mut s_Solver,
        p: Id,
        rp: Id,
        extrajobflags: Id,
        job: *mut Queue,
    );
}
extern "C" {
    pub fn solver_take_solution(solv: *mut s_Solver, problem: Id, solution: Id, job: *mut Queue);
}
extern "C" {
    pub fn solver_findproblemrule(solv: *mut s_Solver, problem: Id) -> Id;
}
extern "C" {
    pub fn solver_findallproblemrules(solv: *mut s_Solver, problem: Id, rules: *mut Queue);
}
extern "C" {
    pub fn solver_problemruleinfo2str(
        solv: *mut s_Solver,
        type_: SolverRuleinfo,
        source: Id,
        target: Id,
        dep: Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solver_problem2str(solv: *mut s_Solver, problem: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn solver_solutionelement2str(solv: *mut s_Solver, p: Id, rp: Id) -> *const libc::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Solver {
    pub pool: *mut Pool,
    pub job: Queue,
    pub solution_callback: ::std::option::Option<
        unsafe extern "C" fn(solv: *mut s_Solver, data: *mut libc::c_void) -> libc::c_int,
    >,
    pub solution_callback_data: *mut libc::c_void,
    pub pooljobcnt: libc::c_int,
}
pub type Solver = s_Solver;
extern "C" {
    pub fn solver_create(pool: *mut Pool) -> *mut Solver;
}
extern "C" {
    pub fn solver_free(solv: *mut Solver);
}
extern "C" {
    pub fn solver_solve(solv: *mut Solver, job: *mut Queue) -> libc::c_int;
}
extern "C" {
    pub fn solver_create_transaction(solv: *mut Solver) -> *mut Transaction;
}
extern "C" {
    pub fn solver_set_flag(solv: *mut Solver, flag: libc::c_int, value: libc::c_int)
        -> libc::c_int;
}
extern "C" {
    pub fn solver_get_flag(solv: *mut Solver, flag: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn solver_get_decisionlevel(solv: *mut Solver, p: Id) -> libc::c_int;
}
extern "C" {
    pub fn solver_get_decisionqueue(solv: *mut Solver, decisionq: *mut Queue);
}
extern "C" {
    pub fn solver_get_lastdecisionblocklevel(solv: *mut Solver) -> libc::c_int;
}
extern "C" {
    pub fn solver_get_decisionblock(solv: *mut Solver, level: libc::c_int, decisionq: *mut Queue);
}
extern "C" {
    pub fn solver_get_orphaned(solv: *mut Solver, orphanedq: *mut Queue);
}
extern "C" {
    pub fn solver_get_recommendations(
        solv: *mut Solver,
        recommendationsq: *mut Queue,
        suggestionsq: *mut Queue,
        noselected: libc::c_int,
    );
}
extern "C" {
    pub fn solver_get_unneeded(solv: *mut Solver, unneededq: *mut Queue, filtered: libc::c_int);
}
extern "C" {
    pub fn solver_get_userinstalled(solv: *mut Solver, q: *mut Queue, flags: libc::c_int);
}
extern "C" {
    pub fn pool_add_userinstalled_jobs(
        pool: *mut Pool,
        q: *mut Queue,
        job: *mut Queue,
        flags: libc::c_int,
    );
}
extern "C" {
    pub fn solver_get_cleandeps(solv: *mut Solver, cleandepsq: *mut Queue);
}
extern "C" {
    pub fn solver_describe_decision(solv: *mut Solver, p: Id, infop: *mut Id) -> libc::c_int;
}
extern "C" {
    pub fn solver_describe_weakdep_decision(solv: *mut Solver, p: Id, whyq: *mut Queue);
}
extern "C" {
    pub fn solver_alternatives_count(solv: *mut Solver) -> libc::c_int;
}
extern "C" {
    pub fn solver_get_alternative(
        solv: *mut Solver,
        alternative: Id,
        idp: *mut Id,
        fromp: *mut Id,
        chosenp: *mut Id,
        choices: *mut Queue,
        levelp: *mut libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solver_calculate_multiversionmap(
        pool: *mut Pool,
        job: *mut Queue,
        multiversionmap: *mut Map,
    );
}
extern "C" {
    pub fn solver_calculate_noobsmap(pool: *mut Pool, job: *mut Queue, multiversionmap: *mut Map);
}
extern "C" {
    pub fn solver_create_state_maps(
        solv: *mut Solver,
        installedmap: *mut Map,
        conflictsmap: *mut Map,
    );
}
extern "C" {
    pub fn solver_calc_duchanges(solv: *mut Solver, mps: *mut DUChanges, nmps: libc::c_int);
}
extern "C" {
    pub fn solver_calc_installsizechange(solv: *mut Solver) -> libc::c_int;
}
extern "C" {
    pub fn pool_job2solvables(pool: *mut Pool, pkgs: *mut Queue, how: Id, what: Id);
}
extern "C" {
    pub fn pool_isemptyupdatejob(pool: *mut Pool, how: Id, what: Id) -> libc::c_int;
}
extern "C" {
    pub fn solver_select2str(pool: *mut Pool, select: Id, what: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_job2str(pool: *mut Pool, how: Id, what: Id, flagmask: Id) -> *const libc::c_char;
}
extern "C" {
    pub fn solver_alternative2str(
        solv: *mut Solver,
        type_: libc::c_int,
        id: Id,
        from: Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn solver_trivial_installable(solv: *mut Solver, pkgs: *mut Queue, res: *mut Queue);
}
extern "C" {
    pub fn solver_printruleelement(solv: *mut Solver, type_: libc::c_int, r: *mut Rule, v: Id);
}
extern "C" {
    pub fn solver_printrule(solv: *mut Solver, type_: libc::c_int, r: *mut Rule);
}
extern "C" {
    pub fn solver_printruleclass(solv: *mut Solver, type_: libc::c_int, r: *mut Rule);
}
extern "C" {
    pub fn solver_printproblem(solv: *mut Solver, v: Id);
}
extern "C" {
    pub fn solver_printwatches(solv: *mut Solver, type_: libc::c_int);
}
extern "C" {
    pub fn solver_printdecisionq(solv: *mut Solver, type_: libc::c_int);
}
extern "C" {
    pub fn solver_printdecisions(solv: *mut Solver);
}
extern "C" {
    pub fn solver_printproblemruleinfo(solv: *mut Solver, rule: Id);
}
extern "C" {
    pub fn solver_printprobleminfo(solv: *mut Solver, problem: Id);
}
extern "C" {
    pub fn solver_printcompleteprobleminfo(solv: *mut Solver, problem: Id);
}
extern "C" {
    pub fn solver_printsolution(solv: *mut Solver, problem: Id, solution: Id);
}
extern "C" {
    pub fn solver_printallsolutions(solv: *mut Solver);
}
extern "C" {
    pub fn transaction_print(trans: *mut Transaction);
}
extern "C" {
    pub fn solver_printtrivial(solv: *mut Solver);
}
extern "C" {
    pub fn selection_make(
        pool: *mut Pool,
        selection: *mut Queue,
        name: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn selection_make_matchdeps(
        pool: *mut Pool,
        selection: *mut Queue,
        name: *const libc::c_char,
        flags: libc::c_int,
        keyname: libc::c_int,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn selection_make_matchdepid(
        pool: *mut Pool,
        selection: *mut Queue,
        dep: Id,
        flags: libc::c_int,
        keyname: libc::c_int,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn selection_make_matchsolvable(
        pool: *mut Pool,
        selection: *mut Queue,
        solvid: Id,
        flags: libc::c_int,
        keyname: libc::c_int,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn selection_make_matchsolvablelist(
        pool: *mut Pool,
        selection: *mut Queue,
        solvidq: *mut Queue,
        flags: libc::c_int,
        keyname: libc::c_int,
        marker: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn selection_filter(pool: *mut Pool, sel1: *mut Queue, sel2: *mut Queue);
}
extern "C" {
    pub fn selection_add(pool: *mut Pool, sel1: *mut Queue, sel2: *mut Queue);
}
extern "C" {
    pub fn selection_subtract(pool: *mut Pool, sel1: *mut Queue, sel2: *mut Queue);
}
extern "C" {
    pub fn selection_solvables(pool: *mut Pool, selection: *mut Queue, pkgs: *mut Queue);
}
extern "C" {
    pub fn pool_selection2str(
        pool: *mut Pool,
        selection: *mut Queue,
        flagmask: Id,
    ) -> *const libc::c_char;
}
extern "C" {
    pub fn pool_evrcmp_conda(
        pool: *const Pool,
        evr1: *const libc::c_char,
        evr2: *const libc::c_char,
        mode: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn solvable_conda_matchversion(
        s: *mut Solvable,
        version: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    pub fn pool_addrelproviders_conda(pool: *mut Pool, name: Id, evr: Id, plist: *mut Queue) -> Id;
}
extern "C" {
    pub fn pool_conda_matchspec(pool: *mut Pool, name: *const libc::c_char) -> Id;
}
extern "C" {
    pub fn repo_add_solv(repo: *mut Repo, fp: *mut FILE, flags: libc::c_int) -> libc::c_int;
}
extern "C" {
    pub fn solv_read_userdata(
        fp: *mut FILE,
        datap: *mut *mut libc::c_uchar,
        lenp: *mut libc::c_int,
    ) -> libc::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct s_Repowriter {
    pub repo: *mut Repo,
    pub flags: libc::c_int,
    pub repodatastart: libc::c_int,
    pub repodataend: libc::c_int,
    pub solvablestart: libc::c_int,
    pub solvableend: libc::c_int,
    pub keyfilter: ::std::option::Option<
        unsafe extern "C" fn(
            repo: *mut Repo,
            key: *mut Repokey,
            kfdata: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub kfdata: *mut libc::c_void,
    pub keyq: *mut Queue,
    pub userdata: *mut libc::c_void,
    pub userdatalen: libc::c_int,
}
pub type Repowriter = s_Repowriter;
extern "C" {
    pub fn repowriter_create(repo: *mut Repo) -> *mut Repowriter;
}
extern "C" {
    pub fn repowriter_free(writer: *mut Repowriter) -> *mut Repowriter;
}
extern "C" {
    pub fn repowriter_set_flags(writer: *mut Repowriter, flags: libc::c_int);
}
extern "C" {
    pub fn repowriter_set_keyfilter(
        writer: *mut Repowriter,
        keyfilter: ::std::option::Option<
            unsafe extern "C" fn(
                repo: *mut Repo,
                key: *mut Repokey,
                kfdata: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        kfdata: *mut libc::c_void,
    );
}
extern "C" {
    pub fn repowriter_set_keyqueue(writer: *mut Repowriter, keyq: *mut Queue);
}
extern "C" {
    pub fn repowriter_set_repodatarange(
        writer: *mut Repowriter,
        repodatastart: libc::c_int,
        repodataend: libc::c_int,
    );
}
extern "C" {
    pub fn repowriter_set_solvablerange(
        writer: *mut Repowriter,
        solvablestart: libc::c_int,
        solvableend: libc::c_int,
    );
}
extern "C" {
    pub fn repowriter_set_userdata(
        writer: *mut Repowriter,
        data: *const libc::c_void,
        len: libc::c_int,
    );
}
extern "C" {
    pub fn repowriter_write(writer: *mut Repowriter, fp: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn repo_write(repo: *mut Repo, fp: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn repodata_write(data: *mut Repodata, fp: *mut FILE) -> libc::c_int;
}
extern "C" {
    pub fn repo_write_stdkeyfilter(
        repo: *mut Repo,
        key: *mut Repokey,
        kfdata: *mut libc::c_void,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repo_write_filtered(
        repo: *mut Repo,
        fp: *mut FILE,
        keyfilter: ::std::option::Option<
            unsafe extern "C" fn(
                repo: *mut Repo,
                key: *mut Repokey,
                kfdata: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        kfdata: *mut libc::c_void,
        keyq: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repodata_write_filtered(
        data: *mut Repodata,
        fp: *mut FILE,
        keyfilter: ::std::option::Option<
            unsafe extern "C" fn(
                repo: *mut Repo,
                key: *mut Repokey,
                kfdata: *mut libc::c_void,
            ) -> libc::c_int,
        >,
        kfdata: *mut libc::c_void,
        keyq: *mut Queue,
    ) -> libc::c_int;
}
extern "C" {
    pub fn repo_add_conda(repo: *mut Repo, fp: *mut FILE, flags: libc::c_int) -> libc::c_int;
}
