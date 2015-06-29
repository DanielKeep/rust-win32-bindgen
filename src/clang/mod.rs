#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt;
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

ext_impl! { Rc<TranslationUnit> as RcTranslationUnitExt {
    fn tokenize_all_to_vec[](&self) -> Vec<Token> {
        unsafe {
            debug!("tokenize_all_to_vec({:?})", self);
            let cursor = self.cursor();
            let range = ll::clang_getCursorExtent(cursor.1);
            let mut toks_ptr = ::std::ptr::null_mut();
            let mut toks_len = 0;
            ll::clang_tokenize(self.1, range, &mut toks_ptr, &mut toks_len);
            let toks_slice = ::std::slice::from_raw_parts(toks_ptr, toks_len as usize);
            let toks: Vec<_> = toks_slice.iter().map(|tok| Token::from_ll(self.clone(), *tok)).collect();
            drop(toks_slice);
            ll::clang_disposeTokens(self.1, toks_ptr, toks_len);
            debug!(".. toks: [..; {}]", toks.len());
            toks
        }
    }
}}

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

    pub fn kind(&self) -> CursorKind {
        unsafe {
            ll::clang_getCursorKind(self.1).try_into().expect("valid kind for cursor")
        }
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

    pub fn type_(&self) -> Type {
        unsafe {
            Type::from_ll(self.0.clone(), ll::clang_getCursorType(self.1))
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

impl fmt::Display for Cursor {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(fmt, "{}: {:?} {}", self.location().display_short(), self.kind(), self.spelling())
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

c_enum! {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum CursorKind: ll::Enum_CXCursorKind {
        UnexposedDecl = 1,
        StructDecl = 2,
        UnionDecl = 3,
        ClassDecl = 4,
        EnumDecl = 5,
        FieldDecl = 6,
        EnumConstantDecl = 7,
        FunctionDecl = 8,
        VarDecl = 9,
        ParmDecl = 10,
        ObjCInterfaceDecl = 11,
        ObjCCategoryDecl = 12,
        ObjCProtocolDecl = 13,
        ObjCPropertyDecl = 14,
        ObjCIvarDecl = 15,
        ObjCInstanceMethodDecl = 16,
        ObjCClassMethodDecl = 17,
        ObjCImplementationDecl = 18,
        ObjCCategoryImplDecl = 19,
        TypedefDecl = 20,
        CXXMethod = 21,
        Namespace = 22,
        LinkageSpec = 23,
        Constructor = 24,
        Destructor = 25,
        ConversionFunction = 26,
        TemplateTypeParameter = 27,
        NonTypeTemplateParameter = 28,
        TemplateTemplateParameter = 29,
        FunctionTemplate = 30,
        ClassTemplate = 31,
        ClassTemplatePartialSpecialization = 32,
        NamespaceAlias = 33,
        UsingDirective = 34,
        UsingDeclaration = 35,
        TypeAliasDecl = 36,
        ObjCSynthesizeDecl = 37,
        ObjCDynamicDecl = 38,
        CXXAccessSpecifier = 39,
        ObjCSuperClassRef = 40,
        ObjCProtocolRef = 41,
        ObjCClassRef = 42,
        TypeRef = 43,
        CXXBaseSpecifier = 44,
        TemplateRef = 45,
        NamespaceRef = 46,
        MemberRef = 47,
        LabelRef = 48,
        OverloadedDeclRef = 49,
        VariableRef = 50,
        InvalidFile = 70,
        NoDeclFound = 71,
        NotImplemented = 72,
        InvalidCode = 73,
        UnexposedExpr = 100,
        DeclRefExpr = 101,
        MemberRefExpr = 102,
        CallExpr = 103,
        ObjCMessageExpr = 104,
        BlockExpr = 105,
        IntegerLiteral = 106,
        FloatingLiteral = 107,
        ImaginaryLiteral = 108,
        StringLiteral = 109,
        CharacterLiteral = 110,
        ParenExpr = 111,
        UnaryOperator = 112,
        ArraySubscriptExpr = 113,
        BinaryOperator = 114,
        CompoundAssignOperator = 115,
        ConditionalOperator = 116,
        CStyleCastExpr = 117,
        CompoundLiteralExpr = 118,
        InitListExpr = 119,
        AddrLabelExpr = 120,
        StmtExpr = 121,
        GenericSelectionExpr = 122,
        GNUNullExpr = 123,
        CXXStaticCastExpr = 124,
        CXXDynamicCastExpr = 125,
        CXXReinterpretCastExpr = 126,
        CXXConstCastExpr = 127,
        CXXFunctionalCastExpr = 128,
        CXXTypeidExpr = 129,
        CXXBoolLiteralExpr = 130,
        CXXNullPtrLiteralExpr = 131,
        CXXThisExpr = 132,
        CXXThrowExpr = 133,
        CXXNewExpr = 134,
        CXXDeleteExpr = 135,
        UnaryExpr = 136,
        ObjCStringLiteral = 137,
        ObjCEncodeExpr = 138,
        ObjCSelectorExpr = 139,
        ObjCProtocolExpr = 140,
        ObjCBridgedCastExpr = 141,
        PackExpansionExpr = 142,
        SizeOfPackExpr = 143,
        LambdaExpr = 144,
        ObjCBoolLiteralExpr = 145,
        ObjCSelfExpr = 146,
        UnexposedStmt = 200,
        LabelStmt = 201,
        CompoundStmt = 202,
        CaseStmt = 203,
        DefaultStmt = 204,
        IfStmt = 205,
        SwitchStmt = 206,
        WhileStmt = 207,
        DoStmt = 208,
        ForStmt = 209,
        GotoStmt = 210,
        IndirectGotoStmt = 211,
        ContinueStmt = 212,
        BreakStmt = 213,
        ReturnStmt = 214,
        // GCCAsmStmt = 215,
        AsmStmt = 215,
        ObjCAtTryStmt = 216,
        ObjCAtCatchStmt = 217,
        ObjCAtFinallyStmt = 218,
        ObjCAtThrowStmt = 219,
        ObjCAtSynchronizedStmt = 220,
        ObjCAutoreleasePoolStmt = 221,
        ObjCForCollectionStmt = 222,
        CXXCatchStmt = 223,
        CXXTryStmt = 224,
        CXXForRangeStmt = 225,
        SEHTryStmt = 226,
        SEHExceptStmt = 227,
        SEHFinallyStmt = 228,
        MSAsmStmt = 229,
        NullStmt = 230,
        DeclStmt = 231,
        OMPParallelDirective = 232,
        TranslationUnit = 300,
        UnexposedAttr = 400,
        IBActionAttr = 401,
        IBOutletAttr = 402,
        IBOutletCollectionAttr = 403,
        CXXFinalAttr = 404,
        CXXOverrideAttr = 405,
        AnnotateAttr = 406,
        AsmLabelAttr = 407,
        PackedAttr = 408,
        PreprocessingDirective = 500,
        MacroDefinition = 501,
        // MacroExpansion = 502,
        MacroInstantiation = 502,
        InclusionDirective = 503,
        ModuleImportDecl = 600,
    }
}

impl CursorKind {
    #[allow(non_upper_case_globals)] pub const FirstDecl: CursorKind = /* 1 */ CursorKind::UnexposedDecl;
    #[allow(non_upper_case_globals)] pub const LastDecl: CursorKind = /* 39 */ CursorKind::CXXAccessSpecifier;
    #[allow(non_upper_case_globals)] pub const FirstRef: CursorKind = /* 40 */ CursorKind::ObjCSuperClassRef;
    #[allow(non_upper_case_globals)] pub const LastRef: CursorKind = /* 50 */ CursorKind::VariableRef;
    #[allow(non_upper_case_globals)] pub const FirstInvalid: CursorKind = /* 70 */ CursorKind::InvalidFile;
    #[allow(non_upper_case_globals)] pub const LastInvalid: CursorKind = /* 73 */ CursorKind::InvalidCode;
    #[allow(non_upper_case_globals)] pub const FirstExpr: CursorKind = /* 100 */ CursorKind::UnexposedExpr;
    #[allow(non_upper_case_globals)] pub const LastExpr: CursorKind = /* 146 */ CursorKind::ObjCSelfExpr;
    #[allow(non_upper_case_globals)] pub const FirstStmt: CursorKind = /* 200 */ CursorKind::UnexposedStmt;
    #[allow(non_upper_case_globals)] pub const LastStmt: CursorKind = /* 232 */ CursorKind::OMPParallelDirective;
    #[allow(non_upper_case_globals)] pub const FirstAttr: CursorKind = /* 400 */ CursorKind::UnexposedAttr;
    #[allow(non_upper_case_globals)] pub const LastAttr: CursorKind = /* 408 */ CursorKind::PackedAttr;
    #[allow(non_upper_case_globals)] pub const FirstPreprocessing: CursorKind = /* 500 */ CursorKind::PreprocessingDirective;
    #[allow(non_upper_case_globals)] pub const LastPreprocessing: CursorKind = /* 503 */ CursorKind::InclusionDirective;
    #[allow(non_upper_case_globals)] pub const FirstExtraDecl: CursorKind = /* 600 */ CursorKind::ModuleImportDecl;
    #[allow(non_upper_case_globals)] pub const LastExtraDecl: CursorKind = /* 600 */ CursorKind::ModuleImportDecl;
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

    pub fn instantiation_location(&self) -> (Option<File>, u32, u32, u32) {
        unsafe {
            let mut file = ::std::ptr::null_mut();
            let mut line = 0;
            let mut column = 0;
            let mut offset = 0;
            ll::clang_getInstantiationLocation(self.1, &mut file, &mut line, &mut column, &mut offset);
            let file = if file.is_null() { None } else { Some(File::from_ll(self.0.clone(), file)) };
            (file, line, column, offset)
        }
    }

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

    pub fn display_short(&self) -> SourceLocationShortDisplay {
        SourceLocationShortDisplay(self)
    }

    pub fn file(&self) -> Option<File> {
        self.instantiation_location().0
    }

    pub fn line(&self) -> u32 {
        self.instantiation_location().1
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
        let (file, l, c, _) = s.instantiation_location();
        match file {
            Some(file) => write!(f, "{}:{}:{}", file, l, c),
            None => write!(f, "(unknown):{}:{}", l, c)
        }
    }
}

pub struct SourceLocationShortDisplay<'a>(&'a SourceLocation);

impl_Display! {
    <['a]> for SourceLocationShortDisplay<'a>, (s, f) {
        use std::path::PathBuf;
        let (file, l, c, _) = s.0.instantiation_location();
        match file {
            Some(file) => {
                let path = PathBuf::from(file.file_name());
                let path = path.file_name().expect("file_name from path");
                write!(f, "{}:{}:{}", path.to_str().unwrap(), l, c)
            },
            None => write!(f, "(unknown):{}:{}", l, c)
        }
    }
}

pub struct File(Rc<TranslationUnit>, ll::CXFile);

impl File {
    fn from_ll(tu: Rc<TranslationUnit>, file: ll::CXFile) -> File {
        File(tu, file)
    }

    pub fn file_name(&self) -> String {
        use ::std::path::PathBuf;
        use ::util::PathBufExt;
        let s = unsafe {
            cxstring_to_string(ll::clang_getFileName(self.1))
        };
        let mut p = PathBuf::from(s);
        p.normalize_path_sep();
        p.to_string_lossy().into_owned()
    }

    pub fn name(&self) -> String {
        use ::std::path::PathBuf;
        let path = PathBuf::from(self.file_name());
        path.file_stem().expect("valid file stem for File::name")
            .to_string_lossy().into_owned()
    }
}

impl_Display! { for File, (s, f) { write!(f, "{}", s.file_name()) } }

pub unsafe fn cxstring_to_string(cxs: ll::CXString) -> String {
    use std::ffi::CStr;
    let str = CStr::from_ptr(ll::clang_getCString(cxs)).to_string_lossy().into_owned();
    ll::clang_disposeString(cxs);
    str
}

pub struct Token(Rc<TranslationUnit>, ll::CXToken);

impl Token {
    fn from_ll(tu: Rc<TranslationUnit>, tok: ll::CXToken) -> Token {
        Token(tu, tok)
    }

    // pub fn clang_getTokenSpelling(arg1: CXTranslationUnit, arg2: CXToken) -> CXString;

    pub fn spelling(&self) -> String {
        unsafe {
            cxstring_to_string(ll::clang_getTokenSpelling((*self.0).1, self.1))
        }
    }
    
    // pub fn clang_getTokenLocation(arg1: CXTranslationUnit, arg2: CXToken) -> CXSourceLocation;
    
    // pub fn clang_getTokenExtent(arg1: CXTranslationUnit, arg2: CXToken) -> CXSourceRange;

    pub fn location(&self) -> SourceLocation {
        unsafe {
            SourceLocation::from_ll(self.0.clone(), ll::clang_getTokenLocation((*self.0).1, self.1))
        }
    }
}

pub struct Type(Rc<TranslationUnit>, ll::CXType);

impl Type {
    fn from_ll(tu: Rc<TranslationUnit>, type_: ll::CXType) -> Type {
        Type(tu, type_)
    }

    pub fn args(&self) -> Vec<Type> {
        unsafe {
            let len = ll::clang_getNumArgTypes(self.1);
            let mut args = Vec::with_capacity(len as usize);
            for i in 0..(len as u32) {
                args.push(Type::from_ll(self.0.clone(), ll::clang_getArgType(self.1, i)))
            }
            args
        }
    }

    pub fn array_element_type(&self) -> Type {
        unsafe {
            Type::from_ll(self.0.clone(), ll::clang_getArrayElementType(self.1))
        }
    }

    pub fn array_size(&self) -> u64 {
        unsafe {
            ll::clang_getArraySize(self.1) as u64 // TODO: checked
        }
    }

    pub fn calling_conv(&self) -> CallingConv {
        unsafe {
            ll::clang_getFunctionTypeCallingConv(self.1).try_into().expect("valid calling conv for type")
        }
    }

    pub fn declaration(&self) -> Cursor {
        unsafe {
            Cursor::from_ll(ll::clang_getTypeDeclaration(self.1)).expect("valid cursor for Type::declaration")
        }
    }

    pub fn is_const_qualified(&self) -> bool {
        unsafe {
            ll::clang_isConstQualifiedType(self.1) != 0
        }
    }

    pub fn kind(&self) -> TypeKind {
        self.1.kind.try_into().expect("valid type kind for type")
    }

    pub fn pointee(&self) -> Type {
        unsafe {
            Type::from_ll(self.0.clone(), ll::clang_getPointeeType(self.1))
        }
    }

    // pub fn clang_isFunctionTypeVariadic(T: CXType) -> ::libc::c_uint;

    pub fn result(&self) -> Type {
        unsafe {
            Type::from_ll(self.0.clone(), ll::clang_getResultType(self.1))
        }
    }

    pub fn spelling(&self) -> String {
        unsafe {
            cxstring_to_string(ll::clang_getTypeSpelling(self.1))
        }
    }
}

c_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum CallingConv: ll::Enum_CXCallingConv {
        Default = 0,
        C = 1,
        X86StdCall = 2,
        X86FastCall = 3,
        X86ThisCall = 4,
        X86Pascal = 5,
        AAPCS = 6,
        AAPCS_VFP = 7,
        PnaclCall = 8,
        IntelOclBicc = 9,
        X86_64Win64 = 10,
        X86_64SysV = 11,
        Invalid = 100,
        Unexposed = 200,
    }
}

c_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum TypeKind: ll::Enum_CXTypeKind {
        Invalid = 0,
        Unexposed = 1,
        Void = 2,
        Bool = 3,
        Char_U = 4,
        UChar = 5,
        Char16 = 6,
        Char32 = 7,
        UShort = 8,
        UInt = 9,
        ULong = 10,
        ULongLong = 11,
        UInt128 = 12,
        Char_S = 13,
        SChar = 14,
        WChar = 15,
        Short = 16,
        Int = 17,
        Long = 18,
        LongLong = 19,
        Int128 = 20,
        Float = 21,
        Double = 22,
        LongDouble = 23,
        NullPtr = 24,
        Overload = 25,
        Dependent = 26,
        ObjCId = 27,
        ObjCClass = 28,
        ObjCSel = 29,
        Complex = 100,
        Pointer = 101,
        BlockPointer = 102,
        LValueReference = 103,
        RValueReference = 104,
        Record = 105,
        Enum = 106,
        Typedef = 107,
        ObjCInterface = 108,
        ObjCObjectPointer = 109,
        FunctionNoProto = 110,
        FunctionProto = 111,
        ConstantArray = 112,
        Vector = 113,
        IncompleteArray = 114,
        VariableArray = 115,
        DependentSizedArray = 116,
        MemberPointer = 117,
    }
}

impl TypeKind {
    #[allow(non_upper_case_globals)] pub const FirstBuiltin: TypeKind = /* 2 */ TypeKind::Void;
    #[allow(non_upper_case_globals)] pub const LastBuiltin: TypeKind = /* 29 */ TypeKind::ObjCSel;
}
