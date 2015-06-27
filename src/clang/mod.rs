#![allow(dead_code)]

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::RwLock;
use libc;
use util::{BoolUtil, CheckedInto, ToCStr, TryFrom, TryInto};

pub mod ll;

// This is so the rc type can be switched out later.
fn rc<T>(v: T) -> Rc<T> {
    Rc::new(v)
}

#[derive(Debug)]
pub struct Index {
    ptr: ll::CXIndex,
}

impl Index {
    pub fn create(exclude_declarations_from_pch: bool, display_diagnostics: bool) -> Rc<Index> {
        let ptr = unsafe { ll::clang_createIndex(
            exclude_declarations_from_pch.as_either(1, 0),
            display_diagnostics.as_either(1, 0)) };
        assert!(!ptr.is_null());
        rc(Index {
            ptr: ptr,
        })
    }

    pub fn set_global_options(&self, options: libc::c_uint) {
        unsafe { ll::clang_CXIndex_setGlobalOptions(self.ptr, options) }
    }

    pub fn global_options(&self) -> libc::c_uint {
        unsafe { ll::clang_CXIndex_getGlobalOptions(self.ptr) }
    }
}

ext_impl! { Rc<Index> as RcIndexExt {
    fn create_translation_unit_from_source_file[S1: ToCStr, S2: ToCStr](&self, source_filename: S1, clang_command_line_args: &[S2], unsaved_files: &mut [UnsavedFile]) -> Result<Rc<TranslationUnit>, String> {
        let source_filename = source_filename.to_c_str();
        let clang_command_line_args = clang_command_line_args
            .iter().map(|s| s.to_c_str()).collect::<Vec<_>>()
            .iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        let mut unsaved_files = unsaved_files.iter_mut().map(|e| e.0).collect::<Vec<_>>();
        let ptr: ll::CXTranslationUnit = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ll::clang_createTranslationUnitFromSourceFile(
                self.ptr,
                source_filename.as_ptr(),
                clang_command_line_args.len().checked_into(),
                clang_command_line_args.as_ptr(),
                unsaved_files.len().checked_into(),
                unsaved_files.as_mut_ptr(),
            )
        };
        match !ptr.is_null() {
            true => Ok(TranslationUnit::from_ll(self.clone(), ptr)),
            false => Err("could not create translation unit".into())
        }
    }

    fn create_translation_unit[](&self, ast_filename: &str) -> Rc<TranslationUnit> {
        let ast_filename = ast_filename.to_c_str();
        TranslationUnit::from_ll(self.clone(), unsafe { ll::clang_createTranslationUnit(self.ptr, ast_filename.as_ptr()) })
    }

    fn parse_translation_unit[S1: ToCStr, S2: ToCStr](
        &self,
        source_filename: S1,
        command_line_args: &[S2],
        unsaved_files: &[UnsavedFile],
        options: TranslationUnitFlags
    ) -> Result<Rc<TranslationUnit>, ErrorCode> {
        unsafe {
            let source_filename = source_filename.to_c_str();
            let command_line_args = command_line_args
                .iter().map(|s| s.to_c_str()).collect::<Vec<_>>()
                .iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
            let mut unsaved_files = unsaved_files.iter().map(|e| e.0).collect::<Vec<_>>();
            let mut tu_ptr = ::std::ptr::null_mut();
            let err = ll::clang_parseTranslationUnit2(
                self.ptr,
                source_filename.as_ptr(),
                command_line_args.as_ptr(),
                command_line_args.len().checked_into(),
                unsaved_files.as_mut_ptr(),
                unsaved_files.len().checked_into(),
                options.into(),
                &mut tu_ptr
            ).try_into();
            match err {
                Some(err) => Err(err),
                None => Ok(TranslationUnit::from_ll(self.clone(), tu_ptr))
            }
        }
    }
}}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { ll::clang_disposeIndex(self.ptr) }
    }
}

thread_local! {
    static TU_PTR_MAP: RwLock<HashMap<ll::CXTranslationUnit, Weak<TranslationUnit>>> = {
        RwLock::new(HashMap::new())
    }
}

#[derive(Debug)]
pub struct TranslationUnit(Rc<Index>, pub ll::CXTranslationUnit);

impl TranslationUnit {
    fn from_ll(index: Rc<Index>, tu_ptr: ll::CXTranslationUnit) -> Rc<TranslationUnit> {
        use std::collections::hash_map::Entry::*;

        assert!(!tu_ptr.is_null());
        TU_PTR_MAP.with(|map| match map.write().unwrap().entry(tu_ptr) {
            Occupied(_) => panic!("tu {:?} already exists in TU_PTR_MAP!", tu_ptr),
            Vacant(e) => {
                let tu_rc = rc(TranslationUnit(index.clone(), tu_ptr));
                e.insert(tu_rc.downgrade());
                tu_rc
            }
        })
    }

    fn from_ll_cached(tu_ptr: ll::CXTranslationUnit) -> Rc<TranslationUnit> {
        assert!(!tu_ptr.is_null());
        TU_PTR_MAP.with(|map| match map.read().unwrap().get(&tu_ptr) {
            Some(tu_wrc) => match tu_wrc.upgrade() {
                Some(tu_rc) => tu_rc,
                None => panic!("tu {:?} in TU_PTR_MAP, but has been dropped!", tu_ptr)
            },
            None => panic!("tu {:?} not in TU_PTR_MAP!", tu_ptr)
        })
    }

    pub fn cursor(&self) -> Cursor {
        unsafe {
            match Cursor::from_ll(ll::clang_getTranslationUnitCursor(self.1)) {
                Some(c) => c,
                None => panic!("{:?} has no cursor", self)
            }
        }
    }
}

impl Drop for TranslationUnit {
    fn drop(&mut self) {
        unsafe { ll::clang_disposeTranslationUnit(self.1) }
    }
}

bitflags! {
    flags TranslationUnitFlags: ::libc::c_uint {
        const None = 0,
        const DetailedPreprocessingRecord = 1,
        const Incomplete = 2,
        const PrecompiledPreamble = 4,
        const CacheCompletionResults = 8,
        const ForSerialization = 16,
        const CXXChainedPCH = 32,
        const SkipFunctionBodies = 64,
        const IncludeBriefCommentsInCodeCompletion = 128,
    }
}

pub struct UnsavedFile(pub ll::Struct_CXUnsavedFile);

pub struct IndexAction(pub ll::CXIndexAction);

impl IndexAction {
    fn create(_cidx: &Index) -> IndexAction {
        panic!("nyi: have to handle lifetime management");
        // IndexAction(unsafe {
        //     ll::clang_IndexAction_create(cidx.0)
        // })
    }
}

impl Drop for IndexAction {
    fn drop(&mut self) {
        unsafe { ll::clang_IndexAction_dispose(self.0) }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    UnknownError,
    Failure,
    Crashed,
    InvalidArguments,
    AstReadError,
}

impl ::std::fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        use ::std::error::Error;
        write!(fmt, "{}", self.description())
    }
}

impl ::std::error::Error for ErrorCode {
    fn description(&self) -> &str {
        use self::ErrorCode::*;
        match *self {
            UnknownError => "unknown clang error",
            Failure => "clang failure",
            Crashed => "clang crashed",
            InvalidArguments => "clang invalid arguments",
            AstReadError => "clang ast read error",
        }
    }
}

impl TryFrom<libc::c_uint> for ErrorCode {
    fn try_from(v: libc::c_uint) -> Option<ErrorCode> {
        use self::ErrorCode::*;
        match v {
            ll::CXError_Success => None,
            ll::CXError_Failure => Some(Failure),
            ll::CXError_Crashed => Some(Crashed),
            ll::CXError_InvalidArguments => Some(InvalidArguments),
            ll::CXError_ASTReadError => Some(AstReadError),
            _ => Some(UnknownError)
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cursor(pub Rc<TranslationUnit>, pub ll::CXCursor);

impl Cursor {
    fn from_ll(cursor: ll::CXCursor) -> Option<Cursor> {
        unsafe {
            if ll::clang_Cursor_isNull(cursor) != 0 {
                None
            } else {
                let tu_ptr = ll::clang_Cursor_getTranslationUnit(cursor);
                Some(Cursor(TranslationUnit::from_ll_cached(tu_ptr), cursor))
            }
        }
    }

    pub fn children(&self) -> Vec<Cursor> {
        let mut v = vec![];
        self.visit_children(|decl, _parent| {
            v.push(decl);
            VisitAction::Continue
        });
        v
    }

    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation::from_ll(self.0.clone(), ll::clang_getCursorLocation(self.1))
        }
    }

    pub fn spelling(&self) -> String {
        unsafe {
            cxstring_to_string(ll::clang_getCursorSpelling(self.1))
        }
    }

    pub fn visit_children<F>(&self, mut f: F) -> VisitTermination
    where F: FnMut(Cursor, Option<Cursor>) -> VisitAction {
        extern "C" fn thunk<F>(
            cursor: ll::CXCursor,
            parent: ll::CXCursor,
            client_data: ll::CXClientData
        ) -> ll::Enum_CXChildVisitResult
        where F: FnMut(Cursor, Option<Cursor>) -> VisitAction {
            unsafe {
                let cursor = Cursor::from_ll(cursor).expect("non-null cursor for visitor");
                let parent = Cursor::from_ll(parent);
                let f: *mut F = ::std::mem::transmute(client_data);
                let action = (*f)(cursor, parent);
                action.into()
            }
        }

        let r = unsafe {
            let visitor = thunk::<F>;
            let client_data = &mut f as *mut F;
            ll::clang_visitChildren(self.1, Some(visitor), client_data as *mut _)
        };

        match r {
            0 => VisitTermination::Normal,
            _ => VisitTermination::Early
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VisitAction {
    Break = ll::CXChildVisit_Break,
    Continue = ll::CXChildVisit_Continue,
    Recurse = ll::CXChildVisit_Recurse,
}

impl From<VisitAction> for ll::Enum_CXChildVisitResult {
    fn from(v: VisitAction) -> ll::Enum_CXChildVisitResult {
        v as ll::Enum_CXChildVisitResult
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VisitTermination {
    Normal,
    Early,
}

pub struct SourceLocation(Rc<TranslationUnit>, ll::CXSourceLocation);

impl SourceLocation {
    fn from_ll(tu: Rc<TranslationUnit>, sl: ll::CXSourceLocation) -> SourceLocation {
        SourceLocation(tu, sl)
    }

    pub fn is_in_system_header(&self) -> bool {
        unsafe {
            ll::clang_Location_isInSystemHeader(self.1) != 0
        }
    }

    pub fn is_from_main_file(&self) -> bool {
        unsafe {
            ll::clang_Location_isFromMainFile(self.1) != 0
        }
    }

    // fn clang_getExpansionLocation(location: CXSourceLocation, file: *mut CXFile, line: *mut ::libc::c_uint, column: *mut ::libc::c_uint, offset: *mut ::libc::c_uint);
    
    // fn clang_getPresumedLocation(location: CXSourceLocation, filename: *mut CXString, line: *mut ::libc::c_uint, column: *mut ::libc::c_uint);
    
    // fn clang_getInstantiationLocation(location: CXSourceLocation, file: *mut CXFile, line: *mut ::libc::c_uint, column: *mut ::libc::c_uint, offset: *mut ::libc::c_uint);
    
    // fn clang_getSpellingLocation(location: CXSourceLocation, file: *mut CXFile, line: *mut ::libc::c_uint, column: *mut ::libc::c_uint, offset: *mut ::libc::c_uint);
    
    // fn clang_getFileLocation(location: CXSourceLocation, file: *mut CXFile, line: *mut ::libc::c_uint, column: *mut ::libc::c_uint, offset: *mut ::libc::c_uint);

    pub fn file_location(&self) -> (Option<File>, u32, u32, u32) {
        unsafe {
            let mut file = ::std::ptr::null_mut();
            let mut line = 0;
            let mut column = 0;
            let mut offset = 0;
            ll::clang_getFileLocation(self.1, &mut file, &mut line, &mut column, &mut offset);
            let file = if file.is_null() { None } else { Some(File::from_ll(self.0.clone(), file)) };
            (file, line, column, offset)
        }
    }
}

impl Eq for SourceLocation {}

impl PartialEq for SourceLocation {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            ll::clang_equalLocations(self.1, other.1) != 0
        }
    }
}

impl_Display! {
    for SourceLocation, (s,f) {
        let (file, l, c, _) = s.file_location();
        match file {
            Some(file) => write!(f, "{}:{}:{}", file, l, c),
            None => write!(f, "(unknown):{}:{}", l, c)
        }
    }
}

pub struct File(Rc<TranslationUnit>, ll::CXFile);

impl File {
    fn from_ll(tu: Rc<TranslationUnit>, file: ll::CXFile) -> File {
        File(tu, file)
    }

    fn file_name(&self) -> String {
        use ::std::path::PathBuf;
        use ::util::PathBufExt;
        let s = unsafe {
            cxstring_to_string(ll::clang_getFileName(self.1))
        };
        let mut p = PathBuf::from(s);
        p.normalize_path_sep();
        p.to_string_lossy().into_owned()
    }
}

impl_Display! { for File, (s, f) { write!(f, "{}", s.file_name()) } }

pub unsafe fn cxstring_to_string(cxs: ll::CXString) -> String {
    use std::ffi::CStr;
    let str = CStr::from_ptr(ll::clang_getCString(cxs)).to_string_lossy().into_owned();
    ll::clang_disposeString(cxs);
    str
}
