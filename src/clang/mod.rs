#![allow(dead_code)]

use libc;
use util::{BoolUtil, CheckedInto, ToCStr, TryFrom};

pub mod ll;

pub struct Index(pub ll::CXIndex);

impl Index {
    pub fn create(exclude_declarations_from_pch: bool, display_diagnostics: bool) -> Index {
        let ptr = unsafe { ll::clang_createIndex(
            exclude_declarations_from_pch.as_either(1, 0),
            display_diagnostics.as_either(1, 0)) };
        assert!(!ptr.is_null());
        Index(ptr)
    }

    pub fn set_global_options(&self, options: libc::c_uint) {
        unsafe { ll::clang_CXIndex_setGlobalOptions(self.0, options) }
    }

    pub fn get_global_options(&self) -> libc::c_uint {
        unsafe { ll::clang_CXIndex_getGlobalOptions(self.0) }
    }

    pub fn create_translation_unit_from_source_file<S1: ToCStr, S2: ToCStr>(&self, source_filename: S1, clang_command_line_args: &[S2], unsaved_files: &mut [UnsavedFile]) -> Result<TranslationUnit, String> {
        let source_filename = source_filename.to_c_str();
        let clang_command_line_args = clang_command_line_args
            .iter().map(|s| s.to_c_str()).collect::<Vec<_>>()
            .iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        let mut unsaved_files = unsaved_files.iter_mut().map(|e| e.0).collect::<Vec<_>>();
        let ptr: ll::CXTranslationUnit = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ll::clang_createTranslationUnitFromSourceFile(
                self.0,
                source_filename.as_ptr(),
                clang_command_line_args.len().checked_into(),
                clang_command_line_args.as_ptr(),
                unsaved_files.len().checked_into(),
                unsaved_files.as_mut_ptr(),
            )
        };
        match !ptr.is_null() {
            true => Ok(TranslationUnit(self, ptr)),
            false => Err("could not create translation unit".into())
        }
    }

    pub fn create_translation_unit(&self, ast_filename: &str) -> TranslationUnit {
        let ast_filename = ast_filename.to_c_str();
        TranslationUnit(self, unsafe { ll::clang_createTranslationUnit(self.0, ast_filename.as_ptr()) })
    }

    pub fn parse_translation_unit<S1: ToCStr, S2: ToCStr>(&self, source_filename: S1, command_line_args: &[S2], unsaved_files: &[UnsavedFile], options: TranslationUnitFlags) -> TranslationUnit {
        let source_filename = source_filename.to_c_str();
        let command_line_args = command_line_args
            .iter().map(|s| s.to_c_str()).collect::<Vec<_>>()
            .iter().map(|s| s.as_ptr()).collect::<Vec<_>>();
        let mut unsaved_files = unsaved_files.iter().map(|e| e.0).collect::<Vec<_>>();
        TranslationUnit(
            self,
            unsafe {
                ll::clang_parseTranslationUnit(
                    self.0,
                    source_filename.as_ptr(),
                    command_line_args.as_ptr(),
                    command_line_args.len().checked_into(),
                    unsaved_files.as_mut_ptr(),
                    unsaved_files.len().checked_into(),
                    options.into(),
                )
            }
        )
    }
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { ll::clang_disposeIndex(self.0) }
    }
}

pub struct TranslationUnit<'a>(pub &'a Index, pub ll::CXTranslationUnit);

impl<'a> TranslationUnit<'a> {
    pub fn get_cursor(&self) -> Cursor {
        unsafe { ll::clang_getTranslationUnitCursor(self.1) }
    }
}

impl<'a> Drop for TranslationUnit<'a> {
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
    fn create(cidx: &Index) -> IndexAction {
        IndexAction(unsafe {
            ll::clang_IndexAction_create(cidx.0)
        })
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

pub struct Cursor<'a>(pub &'a Index, pub ll::CXTranslationUnit);

