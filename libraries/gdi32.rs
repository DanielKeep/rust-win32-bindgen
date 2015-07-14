#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortDoc(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4912:23 */
    pub fn AbortPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4915:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AddFontResourceW as AddFontResource; /* wingdi.h:3606:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AddFontResourceA(_: ::winnt::LPCSTR) -> ::libc::c_int; /* wingdi.h:3603:22 */
    pub fn AddFontResourceW(_: ::winnt::LPCWSTR) -> ::libc::c_int; /* wingdi.h:3604:22 */
    pub fn AngleArc(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD, _: ::minwindef::FLOAT, _: ::minwindef::FLOAT) -> ::minwindef::BOOL; /* wingdi.h:4778:23 */
    pub fn AnimatePalette(_: ::windef::HPALETTE, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::wingdi::PALETTEENTRY) -> ::minwindef::BOOL; /* wingdi.h:3611:25 */
    pub fn Arc(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3612:25 */
    pub fn ArcTo(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4916:23 */
    pub fn BeginPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4917:23 */
    pub fn BitBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:3613:25 */
    pub fn CancelDC(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:3614:24 */
    pub fn ChoosePixelFormat(_: ::windef::HDC, _: *const ::wingdi::PIXELFORMATDESCRIPTOR) -> ::libc::c_int; /* wingdi.h:3616:24 */
    pub fn Chord(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3615:25 */
    pub fn CloseEnhMetaFile(_: ::windef::HDC) -> ::windef::HENHMETAFILE; /* wingdi.h:4664:31 */
    pub fn CloseFigure(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4918:23 */
    pub fn CloseMetaFile(_: ::windef::HDC) -> ::minwindef::HMETAFILE; /* wingdi.h:3617:29 */
    pub fn CombineRgn(_: ::minwindef::HRGN, _: ::minwindef::HRGN, _: ::minwindef::HRGN, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3618:26 */
    pub fn CombineTransform(_: ::wingdi::LPXFORM, _: *const ::wingdi::XFORM, _: *const ::wingdi::XFORM) -> ::minwindef::BOOL; /* wingdi.h:4783:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyEnhMetaFileW as CopyEnhMetaFile; /* wingdi.h:4668:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyEnhMetaFileA(_: ::windef::HENHMETAFILE, _: ::winnt::LPCSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4665:31 */
    pub fn CopyEnhMetaFileW(_: ::windef::HENHMETAFILE, _: ::winnt::LPCWSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4666:31 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyMetaFileW as CopyMetaFile; /* wingdi.h:3622:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyMetaFileA(_: ::minwindef::HMETAFILE, _: ::winnt::LPCSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3619:28 */
    pub fn CopyMetaFileW(_: ::minwindef::HMETAFILE, _: ::winnt::LPCWSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3620:28 */
    pub fn CreateBitmap(_: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_void) -> ::windef::HBITMAP; /* wingdi.h:3626:27 */
    pub fn CreateBitmapIndirect(_: *const ::wingdi::BITMAP) -> ::windef::HBITMAP; /* wingdi.h:3627:27 */
    pub fn CreateBrushIndirect(_: *const ::wingdi::LOGBRUSH) -> ::windef::HBRUSH; /* wingdi.h:3628:27 */
    pub fn CreateCompatibleBitmap(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::windef::HBITMAP; /* wingdi.h:3629:26 */
    pub fn CreateCompatibleDC(_: ::windef::HDC) -> ::windef::HDC; /* wingdi.h:3631:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDCW as CreateDC; /* wingdi.h:3635:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDCA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:3632:26 */
    pub fn CreateDCW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:3633:26 */
    pub fn CreateDIBPatternBrush(_: ::minwindef::HGLOBAL, _: ::minwindef::UINT) -> ::windef::HBRUSH; /* wingdi.h:3640:26 */
    pub fn CreateDIBPatternBrushPt(_: *const ::libc::c_void, _: ::minwindef::UINT) -> ::windef::HBRUSH; /* wingdi.h:3641:27 */
    pub fn CreateDIBSection(_: ::windef::HDC, _: *const ::wingdi::BITMAPINFO, _: ::minwindef::UINT, _: *mut *mut ::libc::c_void, _: ::winnt::HANDLE, _: ::minwindef::DWORD) -> ::windef::HBITMAP; /* wingdi.h:4790:52 */
    pub fn CreateDIBitmap(_: ::windef::HDC, _: *const ::wingdi::BITMAPINFOHEADER, _: ::minwindef::DWORD, _: *const ::libc::c_void, _: *const ::wingdi::BITMAPINFO, _: ::minwindef::UINT) -> ::windef::HBITMAP; /* wingdi.h:3639:26 */
    pub fn CreateDiscardableBitmap(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::windef::HBITMAP; /* wingdi.h:3630:26 */
    pub fn CreateEllipticRgn(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3642:26 */
    pub fn CreateEllipticRgnIndirect(_: *const ::windef::RECT) -> ::minwindef::HRGN; /* wingdi.h:3643:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateEnhMetaFileW as CreateEnhMetaFile; /* wingdi.h:4675:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateEnhMetaFileA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: *const ::windef::RECT, _: ::winnt::LPCSTR) -> ::windef::HDC; /* wingdi.h:4672:24 */
    pub fn CreateEnhMetaFileW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: *const ::windef::RECT, _: ::winnt::LPCWSTR) -> ::windef::HDC; /* wingdi.h:4673:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFontW as CreateFont; /* wingdi.h:3658:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFontA(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCSTR) -> ::windef::HFONT; /* wingdi.h:3651:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFontIndirectW as CreateFontIndirect; /* wingdi.h:3647:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFontIndirectA(_: *const ::wingdi::LOGFONTA) -> ::windef::HFONT; /* wingdi.h:3644:27 */
    pub fn CreateFontIndirectW(_: *const ::wingdi::LOGFONTW) -> ::windef::HFONT; /* wingdi.h:3645:27 */
    pub fn CreateFontW(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::LPCWSTR) -> ::windef::HFONT; /* wingdi.h:3654:26 */
    pub fn CreateHalftonePalette(_: ::windef::HDC) -> ::windef::HPALETTE; /* wingdi.h:4863:27 */
    pub fn CreateHatchBrush(_: ::libc::c_int, _: ::windef::COLORREF) -> ::windef::HBRUSH; /* wingdi.h:3663:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateICW as CreateIC; /* wingdi.h:3667:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateICA(_: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:3664:26 */
    pub fn CreateICW(_: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:3665:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMetaFileW as CreateMetaFile; /* wingdi.h:3674:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMetaFileA(_: ::winnt::LPCSTR) -> ::windef::HDC; /* wingdi.h:3671:26 */
    pub fn CreateMetaFileW(_: ::winnt::LPCWSTR) -> ::windef::HDC; /* wingdi.h:3672:26 */
    pub fn CreatePalette(_: *const ::wingdi::LOGPALETTE) -> ::windef::HPALETTE; /* wingdi.h:3678:28 */
    pub fn CreatePatternBrush(_: ::windef::HBITMAP) -> ::windef::HBRUSH; /* wingdi.h:3685:27 */
    pub fn CreatePen(_: ::libc::c_int, _: ::libc::c_int, _: ::windef::COLORREF) -> ::windef::HPEN; /* wingdi.h:3679:26 */
    pub fn CreatePenIndirect(_: *const ::wingdi::LOGPEN) -> ::windef::HPEN; /* wingdi.h:3680:27 */
    pub fn CreatePolyPolygonRgn(_: *const ::windef::POINT, _: *const ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3681:26 */
    pub fn CreatePolygonRgn(_: *const ::windef::POINT, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:4992:24 */
    pub fn CreateRectRgn(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3686:26 */
    pub fn CreateRectRgnIndirect(_: *const ::windef::RECT) -> ::minwindef::HRGN; /* wingdi.h:3687:26 */
    pub fn CreateRoundRectRgn(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3688:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateScalableFontResourceW as CreateScalableFontResource; /* wingdi.h:3692:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateScalableFontResourceA(_: ::minwindef::DWORD, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR, _: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wingdi.h:3689:26 */
    pub fn CreateScalableFontResourceW(_: ::minwindef::DWORD, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR, _: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wingdi.h:3690:26 */
    pub fn CreateSolidBrush(_: ::windef::COLORREF) -> ::windef::HBRUSH; /* wingdi.h:3696:26 */
    pub fn DPtoLP(_: ::windef::HDC, _: ::windef::LPPOINT, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4995:24 */
    pub fn DeleteDC(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:3698:23 */
    pub fn DeleteEnhMetaFile(_: ::windef::HENHMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:4679:24 */
    pub fn DeleteMetaFile(_: ::minwindef::HMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:3699:23 */
    pub fn DeleteObject(_: ::windef::HGDIOBJ) -> ::minwindef::BOOL; /* wingdi.h:3700:24 */
    pub fn DescribePixelFormat(_: ::windef::HDC, _: ::libc::c_int, _: ::minwindef::UINT, _: ::wingdi::LPPIXELFORMATDESCRIPTOR) -> ::libc::c_int; /* wingdi.h:3701:23 */
    pub fn DrawEscape(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCSTR) -> ::libc::c_int; /* wingdi.h:3823:23 */
    pub fn Ellipse(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3828:24 */
    pub fn EndDoc(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4909:23 */
    pub fn EndPage(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4911:23 */
    pub fn EndPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4919:23 */
    pub fn EnumEnhMetaFile(_: ::windef::HDC, _: ::windef::HENHMETAFILE, _: ::wingdi::ENHMFENUMPROC, _: ::minwindef::LPVOID, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4680:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumFontFamiliesW as EnumFontFamilies; /* wingdi.h:3843:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumFontFamiliesA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::wingdi::FONTENUMPROCA, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3840:23 */
    pub fn EnumFontFamiliesW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::wingdi::FONTENUMPROCW, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3841:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumFontsW as EnumFonts; /* wingdi.h:3850:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumFontsA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::wingdi::FONTENUMPROCA, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3847:23 */
    pub fn EnumFontsW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::wingdi::FONTENUMPROCW, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3848:23 */
    pub fn EnumMetaFile(_: ::windef::HDC, _: ::minwindef::HMETAFILE, _: ::wingdi::MFENUMPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* wingdi.h:4658:24 */
    pub fn EnumObjects(_: ::windef::HDC, _: ::libc::c_int, _: ::wingdi::GOBJENUMPROC, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3856:23 */
    pub fn EqualRgn(_: ::minwindef::HRGN, _: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:3862:23 */
    pub fn Escape(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCSTR, _: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:3863:24 */
    pub fn ExcludeClipRect(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3874:24 */
    pub fn ExtCreatePen(_: ::minwindef::DWORD, _: ::minwindef::DWORD, _: *const ::wingdi::LOGBRUSH, _: ::minwindef::DWORD, _: *const ::libc::c_ulong) -> ::windef::HPEN; /* wingdi.h:4931:23 */
    pub fn ExtCreateRegion(_: *const ::wingdi::XFORM, _: ::minwindef::DWORD, _: *const ::wingdi::RGNDATA) -> ::minwindef::HRGN; /* wingdi.h:3875:24 */
    pub fn ExtEscape(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::winnt::LPSTR) -> ::libc::c_int; /* wingdi.h:3868:23 */
    pub fn ExtFloodFill(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::COLORREF, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:3876:24 */
    pub fn ExtSelectClipRgn(_: ::windef::HDC, _: ::minwindef::HRGN, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4407:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExtTextOutW as ExtTextOut; /* wingdi.h:4980:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExtTextOutA(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: *const ::windef::RECT, _: ::winnt::LPCSTR, _: ::minwindef::UINT, _: *const ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4977:25 */
    pub fn ExtTextOutW(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: *const ::windef::RECT, _: ::winnt::LPCWSTR, _: ::minwindef::UINT, _: *const ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4978:25 */
    pub fn FillPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4920:23 */
    pub fn FillRgn(_: ::windef::HDC, _: ::minwindef::HRGN, _: ::windef::HBRUSH) -> ::minwindef::BOOL; /* wingdi.h:3877:24 */
    pub fn FixBrushOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5046:24 */
    pub fn FlattenPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4921:23 */
    pub fn FloodFill(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::COLORREF) -> ::minwindef::BOOL; /* wingdi.h:3878:24 */
    pub fn FrameRgn(_: ::windef::HDC, _: ::minwindef::HRGN, _: ::windef::HBRUSH, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3879:24 */
    pub fn GdiComment(_: ::windef::HDC, _: ::minwindef::UINT, _: *const ::libc::c_uchar) -> ::minwindef::BOOL; /* wingdi.h:4731:24 */
    pub fn GdiFlush() -> ::minwindef::BOOL; /* wingdi.h:5049:24 */
    pub fn GdiGetBatchLimit() -> ::minwindef::DWORD; /* wingdi.h:5051:24 */
    pub fn GdiSetBatchLimit(_: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:5050:24 */
    pub fn GetArcDirection(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4937:23 */
    pub fn GetAspectRatioFilterEx(_: ::windef::HDC, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:3881:23 */
    pub fn GetBitmapBits(_: ::windef::HBITMAP, _: ::winnt::LONG, _: ::minwindef::LPVOID) -> ::winnt::LONG; /* wingdi.h:3899:1 */
    pub fn GetBitmapDimensionEx(_: ::windef::HBITMAP, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:3905:24 */
    pub fn GetBkColor(_: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3882:27 */
    pub fn GetBkMode(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3892:1 */
    pub fn GetBoundsRect(_: ::windef::HDC, _: ::windef::LPRECT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:3906:24 */
    pub fn GetBrushOrgEx(_: ::windef::HDC, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:3908:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharABCWidthsW as GetCharABCWidths; /* wingdi.h:3941:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharABCWidthsA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:3932:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharABCWidthsFloatW as GetCharABCWidthsFloat; /* wingdi.h:3949:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharABCWidthsFloatA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPABCFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3946:26 */
    pub fn GetCharABCWidthsFloatW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPABCFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3947:26 */
    pub fn GetCharABCWidthsW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:3936:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidthW as GetCharWidth; /* wingdi.h:3913:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidth32W as GetCharWidth32; /* wingdi.h:3920:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharWidth32A(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3917:24 */
    pub fn GetCharWidth32W(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3918:24 */
    pub fn GetCharWidthA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3910:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidthFloatW as GetCharWidthFloat; /* wingdi.h:3927:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharWidthFloatA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3924:26 */
    pub fn GetCharWidthFloatW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3925:26 */
    pub fn GetCharWidthW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3911:24 */
    pub fn GetClipBox(_: ::windef::HDC, _: ::windef::LPRECT) -> ::libc::c_int; /* wingdi.h:3953:24 */
    pub fn GetClipRgn(_: ::windef::HDC, _: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:3954:24 */
    pub fn GetColorAdjustment(_: ::windef::HDC, _: ::wingdi::LPCOLORADJUSTMENT) -> ::minwindef::BOOL; /* wingdi.h:4862:23 */
    pub fn GetCurrentObject(_: ::windef::HDC, _: ::minwindef::UINT) -> ::windef::HGDIOBJ; /* wingdi.h:3956:26 */
    pub fn GetCurrentPositionEx(_: ::windef::HDC, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:3957:24 */
    pub fn GetDCOrgEx(_: ::windef::HDC, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5045:24 */
    pub fn GetDIBColorTable(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *mut ::wingdi::RGBQUAD) -> ::minwindef::UINT; /* wingdi.h:4802:23 */
    pub fn GetDIBits(_: ::windef::HDC, _: ::windef::HBITMAP, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPVOID, _: ::wingdi::LPBITMAPINFO, _: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:3959:24 */
    pub fn GetDeviceCaps(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3958:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnhMetaFileW as GetEnhMetaFile; /* wingdi.h:4685:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnhMetaFileA(_: ::winnt::LPCSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4682:32 */
    pub fn GetEnhMetaFileBits(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::minwindef::LPBYTE) -> ::minwindef::UINT; /* wingdi.h:4689:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnhMetaFileDescriptionW as GetEnhMetaFileDescription; /* wingdi.h:4699:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnhMetaFileDescriptionA(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::winnt::LPSTR) -> ::minwindef::UINT; /* wingdi.h:4692:24 */
    pub fn GetEnhMetaFileDescriptionW(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::winnt::LPWSTR) -> ::minwindef::UINT; /* wingdi.h:4695:24 */
    pub fn GetEnhMetaFileHeader(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::wingdi::LPENHMETAHEADER) -> ::minwindef::UINT; /* wingdi.h:4703:24 */
    pub fn GetEnhMetaFilePaletteEntries(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4706:24 */
    pub fn GetEnhMetaFilePixelFormat(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: *mut ::wingdi::PIXELFORMATDESCRIPTOR) -> ::minwindef::UINT; /* wingdi.h:4710:24 */
    pub fn GetEnhMetaFileW(_: ::winnt::LPCWSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4683:32 */
    pub fn GetFontData(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::winnt::PVOID, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:3963:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetGlyphOutlineW as GetGlyphOutline; /* wingdi.h:3987:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetGlyphOutlineA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPGLYPHMETRICS, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: *const ::wingdi::MAT2) -> ::minwindef::DWORD; /* wingdi.h:3970:24 */
    pub fn GetGlyphOutlineW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPGLYPHMETRICS, _: ::minwindef::DWORD, _: ::minwindef::LPVOID, _: *const ::wingdi::MAT2) -> ::minwindef::DWORD; /* wingdi.h:3978:24 */
    pub fn GetGraphicsMode(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3992:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKerningPairsW as GetKerningPairs; /* wingdi.h:5039:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKerningPairsA(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::wingdi::LPKERNINGPAIR) -> ::minwindef::DWORD; /* wingdi.h:5032:24 */
    pub fn GetKerningPairsW(_: ::windef::HDC, _: ::minwindef::DWORD, _: ::wingdi::LPKERNINGPAIR) -> ::minwindef::DWORD; /* wingdi.h:5035:24 */
    pub fn GetMapMode(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3993:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMetaFileW as GetMetaFile; /* wingdi.h:3998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMetaFileA(_: ::winnt::LPCSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3995:30 */
    pub fn GetMetaFileBitsEx(_: ::minwindef::HMETAFILE, _: ::minwindef::UINT, _: ::minwindef::LPVOID) -> ::minwindef::UINT; /* wingdi.h:3994:24 */
    pub fn GetMetaFileW(_: ::winnt::LPCWSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3996:30 */
    pub fn GetMetaRgn(_: ::windef::HDC, _: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:3955:24 */
    pub fn GetMiterLimit(_: ::windef::HDC, _: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:4936:23 */
    pub fn GetNearestColor(_: ::windef::HDC, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4002:27 */
    pub fn GetNearestPaletteIndex(_: ::windef::HPALETTE, _: ::windef::COLORREF) -> ::minwindef::UINT; /* wingdi.h:4003:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetObjectW as GetObject; /* wingdi.h:4942:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetObjectA(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:4939:24 */
    pub fn GetObjectType(_: ::windef::HGDIOBJ) -> ::minwindef::DWORD; /* wingdi.h:4004:24 */
    pub fn GetObjectW(_: ::winnt::HANDLE, _: ::libc::c_int, _: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:4940:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetOutlineTextMetricsW as GetOutlineTextMetrics; /* wingdi.h:4015:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetOutlineTextMetricsA(_: ::windef::HDC, _: ::minwindef::UINT, _: ::wingdi::LPOUTLINETEXTMETRICA) -> ::minwindef::UINT; /* wingdi.h:4008:25 */
    pub fn GetOutlineTextMetricsW(_: ::windef::HDC, _: ::minwindef::UINT, _: ::wingdi::LPOUTLINETEXTMETRICW) -> ::minwindef::UINT; /* wingdi.h:4011:25 */
    pub fn GetPaletteEntries(_: ::windef::HPALETTE, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4023:24 */
    pub fn GetPath(_: ::windef::HDC, _: ::windef::LPPOINT, _: ::minwindef::LPBYTE, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4922:23 */
    pub fn GetPixel(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::windef::COLORREF; /* wingdi.h:4027:27 */
    pub fn GetPixelFormat(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4028:24 */
    pub fn GetPolyFillMode(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4029:24 */
    pub fn GetROP2(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3880:23 */
    pub fn GetRandomRgn(_: ::windef::HDC, _: ::minwindef::HRGN, _: ::winnt::INT) -> ::libc::c_int; /* wingdi.h:4033:24 */
    pub fn GetRasterizerCaps(_: ::wingdi::LPRASTERIZER_STATUS, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4030:24 */
    pub fn GetRegionData(_: ::minwindef::HRGN, _: ::minwindef::DWORD, _: ::wingdi::LPRGNDATA) -> ::minwindef::DWORD; /* wingdi.h:4034:24 */
    pub fn GetRgnBox(_: ::minwindef::HRGN, _: ::windef::LPRECT) -> ::libc::c_int; /* wingdi.h:4037:24 */
    pub fn GetStockObject(_: ::libc::c_int) -> ::windef::HGDIOBJ; /* wingdi.h:4038:26 */
    pub fn GetStretchBltMode(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4039:24 */
    pub fn GetSystemPaletteEntries(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4043:1 */
    pub fn GetSystemPaletteUse(_: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4050:24 */
    pub fn GetTextAlign(_: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4052:24 */
    pub fn GetTextCharacterExtra(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4051:24 */
    pub fn GetTextColor(_: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:4053:27 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentExPointW as GetTextExtentExPoint; /* wingdi.h:4128:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextExtentExPointA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::minwindef::LPINT, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4106:1 */
    pub fn GetTextExtentExPointW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::minwindef::LPINT, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4118:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentPointW as GetTextExtentPoint; /* wingdi.h:4074:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentPoint32W as GetTextExtentPoint32; /* wingdi.h:4098:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextExtentPoint32A(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4082:1 */
    pub fn GetTextExtentPoint32W(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4091:1 */
    pub fn GetTextExtentPointA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4058:1 */
    pub fn GetTextExtentPointW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4067:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextFaceW as GetTextFace; /* wingdi.h:5019:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextFaceA(_: ::windef::HDC, _: ::libc::c_int, _: ::winnt::LPSTR) -> ::libc::c_int; /* wingdi.h:5016:24 */
    pub fn GetTextFaceW(_: ::windef::HDC, _: ::libc::c_int, _: ::winnt::LPWSTR) -> ::libc::c_int; /* wingdi.h:5017:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextMetricsW as GetTextMetrics; /* wingdi.h:4740:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextMetricsA(_: ::windef::HDC, _: ::wingdi::LPTEXTMETRICA) -> ::minwindef::BOOL; /* wingdi.h:4737:23 */
    pub fn GetTextMetricsW(_: ::windef::HDC, _: ::wingdi::LPTEXTMETRICW) -> ::minwindef::BOOL; /* wingdi.h:4738:23 */
    pub fn GetViewportExtEx(_: ::windef::HDC, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4360:24 */
    pub fn GetViewportOrgEx(_: ::windef::HDC, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4361:24 */
    pub fn GetWinMetaFileBits(_: ::windef::HENHMETAFILE, _: ::minwindef::UINT, _: ::minwindef::LPBYTE, _: ::winnt::INT, _: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4713:24 */
    pub fn GetWindowExtEx(_: ::windef::HDC, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4362:24 */
    pub fn GetWindowOrgEx(_: ::windef::HDC, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4363:24 */
    pub fn GetWorldTransform(_: ::windef::HDC, _: ::wingdi::LPXFORM) -> ::minwindef::BOOL; /* wingdi.h:4780:23 */
    pub fn IntersectClipRect(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4365:24 */
    pub fn InvertRgn(_: ::windef::HDC, _: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:4366:24 */
    pub fn LPtoDP(_: ::windef::HDC, _: ::windef::LPPOINT, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4996:24 */
    pub fn LineDDA(_: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::wingdi::LINEDDAPROC, _: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* wingdi.h:4367:23 */
    pub fn LineTo(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4368:24 */
    pub fn MaskBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HBITMAP, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4369:23 */
    pub fn ModifyWorldTransform(_: ::windef::HDC, _: *const ::wingdi::XFORM, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4782:23 */
    pub fn MoveToEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4969:25 */
    pub fn OffsetClipRgn(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4374:24 */
    pub fn OffsetRgn(_: ::minwindef::HRGN, _: ::libc::c_int, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4375:23 */
    pub fn OffsetViewportOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5009:25 */
    pub fn OffsetWindowOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5010:25 */
    pub fn PaintRgn(_: ::windef::HDC, _: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:4379:24 */
    pub fn PatBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4376:24 */
    pub fn PathToRegion(_: ::windef::HDC) -> ::minwindef::HRGN; /* wingdi.h:4923:23 */
    pub fn Pie(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4377:24 */
    pub fn PlayEnhMetaFile(_: ::windef::HDC, _: ::windef::HENHMETAFILE, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4718:24 */
    pub fn PlayEnhMetaFileRecord(_: ::windef::HDC, _: ::wingdi::LPHANDLETABLE, _: *const ::wingdi::ENHMETARECORD, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4719:24 */
    pub fn PlayMetaFile(_: ::windef::HDC, _: ::minwindef::HMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:4378:23 */
    pub fn PlayMetaFileRecord(_: ::windef::HDC, _: ::wingdi::LPHANDLETABLE, _: ::wingdi::LPMETARECORD, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4652:24 */
    pub fn PlgBlt(_: ::windef::HDC, _: *const ::windef::POINT, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HBITMAP, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4371:23 */
    pub fn PolyBezier(_: ::windef::HDC, _: *const ::windef::POINT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5000:24 */
    pub fn PolyBezierTo(_: ::windef::HDC, _: *const ::windef::POINT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5001:24 */
    pub fn PolyDraw(_: ::windef::HDC, _: *const ::windef::POINT, _: *const ::libc::c_uchar, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4924:23 */
    pub fn PolyPolygon(_: ::windef::HDC, _: *const ::windef::POINT, _: *const ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4380:24 */
    pub fn PolyPolyline(_: ::windef::HDC, _: *const ::windef::POINT, _: *const ::libc::c_ulong, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4779:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PolyTextOutW as PolyTextOut; /* wingdi.h:4987:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PolyTextOutA(_: ::windef::HDC, _: *const ::wingdi::POLYTEXTA, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4984:24 */
    pub fn PolyTextOutW(_: ::windef::HDC, _: *const ::wingdi::POLYTEXTW, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4985:24 */
    pub fn Polygon(_: ::windef::HDC, _: *const ::windef::POINT, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4997:25 */
    pub fn Polyline(_: ::windef::HDC, _: *const ::windef::POINT, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4998:25 */
    pub fn PolylineTo(_: ::windef::HDC, _: *const ::windef::POINT, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5002:24 */
    pub fn PtInRegion(_: ::minwindef::HRGN, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4381:23 */
    pub fn PtVisible(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4382:23 */
    pub fn RealizePalette(_: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4394:24 */
    pub fn RectInRegion(_: ::minwindef::HRGN, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4383:23 */
    pub fn RectVisible(_: ::windef::HDC, _: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4384:23 */
    pub fn Rectangle(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4385:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RemoveFontResourceW as RemoveFontResource; /* wingdi.h:4398:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RemoveFontResourceA(_: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wingdi.h:4395:23 */
    pub fn RemoveFontResourceW(_: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wingdi.h:4396:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ResetDCW as ResetDC; /* wingdi.h:4390:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ResetDCA(_: ::windef::HDC, _: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:4387:24 */
    pub fn ResetDCW(_: ::windef::HDC, _: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:4388:24 */
    pub fn ResizePalette(_: ::windef::HPALETTE, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4403:24 */
    pub fn RestoreDC(_: ::windef::HDC, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4386:24 */
    pub fn RoundRect(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4402:25 */
    pub fn SaveDC(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4405:24 */
    pub fn ScaleViewportExtEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5011:25 */
    pub fn ScaleWindowExtEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5012:25 */
    pub fn SelectClipPath(_: ::windef::HDC, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4925:23 */
    pub fn SelectClipRgn(_: ::windef::HDC, _: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:4406:24 */
    pub fn SelectObject(_: ::windef::HDC, _: ::windef::HGDIOBJ) -> ::windef::HGDIOBJ; /* wingdi.h:4409:27 */
    pub fn SelectPalette(_: ::windef::HDC, _: ::windef::HPALETTE, _: ::minwindef::BOOL) -> ::windef::HPALETTE; /* wingdi.h:4410:28 */
    pub fn SetAbortProc(_: ::windef::HDC, _: ::wingdi::ABORTPROC) -> ::libc::c_int; /* wingdi.h:4913:22 */
    pub fn SetArcDirection(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4926:23 */
    pub fn SetBitmapBits(_: ::windef::HBITMAP, _: ::minwindef::DWORD, _: *const ::libc::c_void) -> ::winnt::LONG; /* wingdi.h:4422:1 */
    pub fn SetBitmapDimensionEx(_: ::windef::HBITMAP, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5013:24 */
    pub fn SetBkColor(_: ::windef::HDC, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4411:28 */
    pub fn SetBkMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4418:25 */
    pub fn SetBoundsRect(_: ::windef::HDC, _: *const ::windef::RECT, _: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4427:24 */
    pub fn SetBrushOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5014:24 */
    pub fn SetColorAdjustment(_: ::windef::HDC, _: *const ::wingdi::COLORADJUSTMENT) -> ::minwindef::BOOL; /* wingdi.h:4861:23 */
    pub fn SetDIBColorTable(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::wingdi::RGBQUAD) -> ::minwindef::UINT; /* wingdi.h:4806:23 */
    pub fn SetDIBits(_: ::windef::HDC, _: ::windef::HBITMAP, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_void, _: *const ::wingdi::BITMAPINFO, _: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:4428:24 */
    pub fn SetDIBitsToDevice(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD, _: ::minwindef::DWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::libc::c_void, _: *const ::wingdi::BITMAPINFO, _: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:4429:25 */
    pub fn SetEnhMetaFileBits(_: ::minwindef::UINT, _: *const ::libc::c_uchar) -> ::windef::HENHMETAFILE; /* wingdi.h:4724:32 */
    pub fn SetGraphicsMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4432:24 */
    pub fn SetMapMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4433:25 */
    pub fn SetMapperFlags(_: ::windef::HDC, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4431:25 */
    pub fn SetMetaFileBitsEx(_: ::minwindef::UINT, _: *const ::libc::c_uchar) -> ::minwindef::HMETAFILE; /* wingdi.h:4440:30 */
    pub fn SetMetaRgn(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4408:23 */
    pub fn SetMiterLimit(_: ::windef::HDC, _: ::minwindef::FLOAT, _: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:4927:23 */
    pub fn SetPaletteEntries(_: ::windef::HPALETTE, _: ::minwindef::UINT, _: ::minwindef::UINT, _: *const ::wingdi::PALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4441:25 */
    pub fn SetPixel(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4445:28 */
    pub fn SetPixelFormat(_: ::windef::HDC, _: ::libc::c_int, _: *const ::wingdi::PIXELFORMATDESCRIPTOR) -> ::minwindef::BOOL; /* wingdi.h:4447:24 */
    pub fn SetPixelV(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::COLORREF) -> ::minwindef::BOOL; /* wingdi.h:4446:25 */
    pub fn SetPolyFillMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4448:25 */
    pub fn SetROP2(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4453:25 */
    pub fn SetRectRgn(_: ::minwindef::HRGN, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4450:25 */
    pub fn SetStretchBltMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4454:25 */
    pub fn SetSystemPaletteUse(_: ::windef::HDC, _: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4455:24 */
    pub fn SetTextAlign(_: ::windef::HDC, _: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4458:25 */
    pub fn SetTextCharacterExtra(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4456:25 */
    pub fn SetTextColor(_: ::windef::HDC, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4457:28 */
    pub fn SetTextJustification(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4459:25 */
    pub fn SetViewportExtEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5004:25 */
    pub fn SetViewportOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5005:25 */
    pub fn SetWinMetaFileBits(_: ::minwindef::UINT, _: *const ::libc::c_uchar, _: ::windef::HDC, _: *const ::wingdi::METAFILEPICT) -> ::windef::HENHMETAFILE; /* wingdi.h:4727:32 */
    pub fn SetWindowExtEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5006:25 */
    pub fn SetWindowOrgEx(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5007:25 */
    pub fn SetWorldTransform(_: ::windef::HDC, _: *const ::wingdi::XFORM) -> ::minwindef::BOOL; /* wingdi.h:4781:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartDocW as StartDoc; /* wingdi.h:4905:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartDocA(_: ::windef::HDC, _: *const ::wingdi::DOCINFOA) -> ::libc::c_int; /* wingdi.h:4902:23 */
    pub fn StartDocW(_: ::windef::HDC, _: *const ::wingdi::DOCINFOW) -> ::libc::c_int; /* wingdi.h:4903:23 */
    pub fn StartPage(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4910:23 */
    pub fn StretchBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4449:25 */
    pub fn StretchDIBits(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: *const ::libc::c_void, _: *const ::wingdi::BITMAPINFO, _: ::minwindef::UINT, _: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:4451:25 */
    pub fn StrokeAndFillPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4928:23 */
    pub fn StrokePath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4929:23 */
    pub fn SwapBuffers(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:6089:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TextOutW as TextOut; /* wingdi.h:4973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TextOutA(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCSTR, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4970:25 */
    pub fn TextOutW(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::winnt::LPCWSTR, _: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4971:25 */
    pub fn UnrealizeObject(_: ::windef::HGDIOBJ) -> ::minwindef::BOOL; /* wingdi.h:5047:24 */
    pub fn UpdateColors(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4460:24 */
    pub fn WidenPath(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4930:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CheckColorsInGamut(_: ::windef::HDC, _: ::wingdi::LPRGBTRIPLE, _: ::minwindef::LPVOID, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5069:30 */
    pub fn ColorMatchToTarget(_: ::windef::HDC, _: ::windef::HDC, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5117:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CreateColorSpaceW as CreateColorSpace; /* wingdi.h:5090:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CreateColorSpaceA(_: ::wingdi::LPLOGCOLORSPACEA) -> ::windef::HCOLORSPACE; /* wingdi.h:5087:30 */
    pub fn CreateColorSpaceW(_: ::wingdi::LPLOGCOLORSPACEW) -> ::windef::HCOLORSPACE; /* wingdi.h:5088:30 */
    pub fn DeleteColorSpace(_: ::windef::HCOLORSPACE) -> ::minwindef::BOOL; /* wingdi.h:5095:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::EnumFontFamiliesExW as EnumFontFamiliesEx; /* wingdi.h:3834:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn EnumFontFamiliesExA(_: ::windef::HDC, _: ::wingdi::LPLOGFONTA, _: ::wingdi::FONTENUMPROCA, _: ::minwindef::LPARAM, _: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:3831:23 */
    pub fn EnumFontFamiliesExW(_: ::windef::HDC, _: ::wingdi::LPLOGFONTW, _: ::wingdi::FONTENUMPROCW, _: ::minwindef::LPARAM, _: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:3832:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::EnumICMProfilesW as EnumICMProfiles; /* wingdi.h:5121:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn EnumICMProfilesA(_: ::windef::HDC, _: ::wingdi::ICMENUMPROCA, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:5118:30 */
    pub fn EnumICMProfilesW(_: ::windef::HDC, _: ::wingdi::ICMENUMPROCW, _: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:5119:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetCharacterPlacementW as GetCharacterPlacement; /* wingdi.h:4141:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetCharacterPlacementA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::wingdi::LPGCP_RESULTSA, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4138:24 */
    pub fn GetCharacterPlacementW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::libc::c_int, _: ::wingdi::LPGCP_RESULTSW, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4139:24 */
    pub fn GetColorSpace(_: ::windef::HDC) -> ::windef::HCOLORSPACE; /* wingdi.h:5074:30 */
    pub fn GetDeviceGammaRamp(_: ::windef::HDC, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* wingdi.h:5115:30 */
    pub fn GetFontLanguageInfo(_: ::windef::HDC) -> ::minwindef::DWORD; /* wingdi.h:4137:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetICMProfileW as GetICMProfile; /* wingdi.h:5103:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetICMProfileA(_: ::windef::HDC, _: ::minwindef::LPDWORD, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wingdi.h:5096:30 */
    pub fn GetICMProfileW(_: ::windef::HDC, _: ::minwindef::LPDWORD, _: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wingdi.h:5099:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetLogColorSpaceW as GetLogColorSpace; /* wingdi.h:5082:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetLogColorSpaceA(_: ::windef::HCOLORSPACE, _: ::wingdi::LPLOGCOLORSPACEA, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5075:30 */
    pub fn GetLogColorSpaceW(_: ::windef::HCOLORSPACE, _: ::wingdi::LPLOGCOLORSPACEW, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5078:30 */
    pub fn GetTextCharset(_: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4134:22 */
    pub fn GetTextCharsetInfo(_: ::windef::HDC, _: ::wingdi::LPFONTSIGNATURE, _: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:4135:22 */
    pub fn SetColorSpace(_: ::windef::HDC, _: ::windef::HCOLORSPACE) -> ::windef::HCOLORSPACE; /* wingdi.h:5094:30 */
    pub fn SetDeviceGammaRamp(_: ::windef::HDC, _: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* wingdi.h:5116:30 */
    pub fn SetICMMode(_: ::windef::HDC, _: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:5068:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::SetICMProfileW as SetICMProfile; /* wingdi.h:5111:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn SetICMProfileA(_: ::windef::HDC, _: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wingdi.h:5108:30 */
    pub fn SetICMProfileW(_: ::windef::HDC, _: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wingdi.h:5109:30 */
    pub fn TranslateCharsetInfo(_: *mut ::libc::c_ulong, _: ::wingdi::LPCHARSETINFO, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4136:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::UpdateICMRegKeyW as UpdateICMRegKey; /* wingdi.h:5130:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn UpdateICMRegKeyA(_: ::minwindef::DWORD, _: ::winnt::LPSTR, _: ::winnt::LPSTR, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:5126:30 */
    pub fn UpdateICMRegKeyW(_: ::minwindef::DWORD, _: ::winnt::LPWSTR, _: ::winnt::LPWSTR, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:5128:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddFontMemResourceEx(_: ::winnt::PVOID, _: ::minwindef::DWORD, _: ::winnt::PVOID, _: *mut ::libc::c_ulong) -> ::winnt::HANDLE; /* wingdi.h:4239:25 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::AddFontResourceExW as AddFontResourceEx; /* wingdi.h:4228:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddFontResourceExA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::libc::c_int; /* wingdi.h:4225:23 */
    pub fn AddFontResourceExW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::libc::c_int; /* wingdi.h:4226:23 */
    pub fn ColorCorrectPalette(_: ::windef::HDC, _: ::windef::HPALETTE, _: ::minwindef::DWORD, _: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5140:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateFontIndirectExW as CreateFontIndirectEx; /* wingdi.h:4325:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateFontIndirectExA(_: *const ::wingdi::ENUMLOGFONTEXDVA) -> ::windef::HFONT; /* wingdi.h:4322:25 */
    pub fn CreateFontIndirectExW(_: *const ::wingdi::ENUMLOGFONTEXDVW) -> ::windef::HFONT; /* wingdi.h:4323:25 */
    pub fn GdiAlphaBlend(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::wingdi::BLENDFUNCTION) -> ::minwindef::BOOL; /* wingdi.h:4634:24 */
    pub fn GdiGradientFill(_: ::windef::HDC, _: ::wingdi::PTRIVERTEX, _: ::minwindef::ULONG, _: ::winnt::PVOID, _: ::minwindef::ULONG, _: ::minwindef::ULONG) -> ::minwindef::BOOL; /* wingdi.h:4639:24 */
    pub fn GdiTransparentBlt(_: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::windef::HDC, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4636:24 */
    pub fn GetCharABCWidthsI(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPWORD, _: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:4204:24 */
    pub fn GetCharWidthI(_: ::windef::HDC, _: ::minwindef::UINT, _: ::minwindef::UINT, _: ::minwindef::LPWORD, _: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:4197:24 */
    pub fn GetDCBrushColor(_: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3885:27 */
    pub fn GetDCPenColor(_: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3886:27 */
    pub fn GetFontUnicodeRanges(_: ::windef::HDC, _: ::wingdi::LPGLYPHSET) -> ::minwindef::DWORD; /* wingdi.h:4179:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetGlyphIndicesW as GetGlyphIndices; /* wingdi.h:4183:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetGlyphIndicesA(_: ::windef::HDC, _: ::winnt::LPCSTR, _: ::libc::c_int, _: ::minwindef::LPWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4180:24 */
    pub fn GetGlyphIndicesW(_: ::windef::HDC, _: ::winnt::LPCWSTR, _: ::libc::c_int, _: ::minwindef::LPWORD, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4181:24 */
    pub fn GetLayout(_: ::windef::HDC) -> ::minwindef::DWORD; /* wingdi.h:4437:24 */
    pub fn GetTextExtentExPointI(_: ::windef::HDC, _: ::minwindef::LPWORD, _: ::libc::c_int, _: ::libc::c_int, _: ::minwindef::LPINT, _: ::minwindef::LPINT, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4188:24 */
    pub fn GetTextExtentPointI(_: ::windef::HDC, _: ::minwindef::LPWORD, _: ::libc::c_int, _: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4187:24 */
    pub fn RemoveFontMemResourceEx(_: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wingdi.h:4244:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RemoveFontResourceExW as RemoveFontResourceEx; /* wingdi.h:4235:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RemoveFontResourceExA(_: ::winnt::LPCSTR, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* wingdi.h:4232:23 */
    pub fn RemoveFontResourceExW(_: ::winnt::LPCWSTR, _: ::minwindef::DWORD, _: ::winnt::PVOID) -> ::minwindef::BOOL; /* wingdi.h:4233:23 */
    pub fn SetDCBrushColor(_: ::windef::HDC, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4414:27 */
    pub fn SetDCPenColor(_: ::windef::HDC, _: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4415:27 */
    pub fn SetLayout(_: ::windef::HDC, _: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4436:25 */
}
