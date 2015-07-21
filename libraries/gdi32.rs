#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AbortDoc(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4912:23 */
    pub fn AbortPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4915:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::AddFontResourceW as AddFontResource; /* wingdi.h:3606:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn AddFontResourceA(_: ::winnt::LPCSTR) -> ::libc::c_int; /* wingdi.h:3603:22 */
    pub fn AddFontResourceW(_: ::winnt::LPCWSTR) -> ::libc::c_int; /* wingdi.h:3604:22 */
    pub fn AngleArc(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, r: ::minwindef::DWORD, StartAngle: ::minwindef::FLOAT, SweepAngle: ::minwindef::FLOAT) -> ::minwindef::BOOL; /* wingdi.h:4778:23 */
    pub fn AnimatePalette(hPal: ::windef::HPALETTE, iStartIndex: ::minwindef::UINT, cEntries: ::minwindef::UINT, ppe: *const ::wingdi::PALETTEENTRY) -> ::minwindef::BOOL; /* wingdi.h:3611:25 */
    pub fn Arc(hdc: ::windef::HDC, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int, x3: ::libc::c_int, y3: ::libc::c_int, x4: ::libc::c_int, y4: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3612:25 */
    pub fn ArcTo(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int, xr1: ::libc::c_int, yr1: ::libc::c_int, xr2: ::libc::c_int, yr2: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4916:23 */
    pub fn BeginPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4917:23 */
    pub fn BitBlt(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, cx: ::libc::c_int, cy: ::libc::c_int, hdcSrc: ::windef::HDC, x1: ::libc::c_int, y1: ::libc::c_int, rop: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:3613:25 */
    pub fn CancelDC(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:3614:24 */
    pub fn ChoosePixelFormat(hdc: ::windef::HDC, ppfd: *const ::wingdi::PIXELFORMATDESCRIPTOR) -> ::libc::c_int; /* wingdi.h:3616:24 */
    pub fn Chord(hdc: ::windef::HDC, x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int, x3: ::libc::c_int, y3: ::libc::c_int, x4: ::libc::c_int, y4: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3615:25 */
    pub fn CloseEnhMetaFile(hdc: ::windef::HDC) -> ::windef::HENHMETAFILE; /* wingdi.h:4664:31 */
    pub fn CloseFigure(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4918:23 */
    pub fn CloseMetaFile(hdc: ::windef::HDC) -> ::minwindef::HMETAFILE; /* wingdi.h:3617:29 */
    pub fn CombineRgn(hrgnDst: ::minwindef::HRGN, hrgnSrc1: ::minwindef::HRGN, hrgnSrc2: ::minwindef::HRGN, iMode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3618:26 */
    pub fn CombineTransform(lpxfOut: ::wingdi::LPXFORM, lpxf1: *const ::wingdi::XFORM, lpxf2: *const ::wingdi::XFORM) -> ::minwindef::BOOL; /* wingdi.h:4783:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyEnhMetaFileW as CopyEnhMetaFile; /* wingdi.h:4668:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyEnhMetaFileA(hEnh: ::windef::HENHMETAFILE, lpFileName: ::winnt::LPCSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4665:31 */
    pub fn CopyEnhMetaFileW(hEnh: ::windef::HENHMETAFILE, lpFileName: ::winnt::LPCWSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4666:31 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CopyMetaFileW as CopyMetaFile; /* wingdi.h:3622:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CopyMetaFileA(_: ::minwindef::HMETAFILE, _: ::winnt::LPCSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3619:28 */
    pub fn CopyMetaFileW(_: ::minwindef::HMETAFILE, _: ::winnt::LPCWSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3620:28 */
    pub fn CreateBitmap(nWidth: ::libc::c_int, nHeight: ::libc::c_int, nPlanes: ::minwindef::UINT, nBitCount: ::minwindef::UINT, lpBits: *const ::libc::c_void) -> ::windef::HBITMAP; /* wingdi.h:3626:27 */
    pub fn CreateBitmapIndirect(pbm: *const ::wingdi::BITMAP) -> ::windef::HBITMAP; /* wingdi.h:3627:27 */
    pub fn CreateBrushIndirect(plbrush: *const ::wingdi::LOGBRUSH) -> ::windef::HBRUSH; /* wingdi.h:3628:27 */
    pub fn CreateCompatibleBitmap(hdc: ::windef::HDC, cx: ::libc::c_int, cy: ::libc::c_int) -> ::windef::HBITMAP; /* wingdi.h:3629:26 */
    pub fn CreateCompatibleDC(hdc: ::windef::HDC) -> ::windef::HDC; /* wingdi.h:3631:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateDCW as CreateDC; /* wingdi.h:3635:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateDCA(pwszDriver: ::winnt::LPCSTR, pwszDevice: ::winnt::LPCSTR, pszPort: ::winnt::LPCSTR, pdm: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:3632:26 */
    pub fn CreateDCW(pwszDriver: ::winnt::LPCWSTR, pwszDevice: ::winnt::LPCWSTR, pszPort: ::winnt::LPCWSTR, pdm: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:3633:26 */
    pub fn CreateDIBPatternBrush(h: ::minwindef::HGLOBAL, iUsage: ::minwindef::UINT) -> ::windef::HBRUSH; /* wingdi.h:3640:26 */
    pub fn CreateDIBPatternBrushPt(lpPackedDIB: *const ::libc::c_void, iUsage: ::minwindef::UINT) -> ::windef::HBRUSH; /* wingdi.h:3641:27 */
    pub fn CreateDIBSection(hdc: ::windef::HDC, pbmi: *const ::wingdi::BITMAPINFO, usage: ::minwindef::UINT, ppvBits: *mut *mut ::libc::c_void, hSection: ::winnt::HANDLE, offset: ::minwindef::DWORD) -> ::windef::HBITMAP; /* wingdi.h:4790:52 */
    pub fn CreateDIBitmap(hdc: ::windef::HDC, pbmih: *const ::wingdi::BITMAPINFOHEADER, flInit: ::minwindef::DWORD, pjBits: *const ::libc::c_void, pbmi: *const ::wingdi::BITMAPINFO, iUsage: ::minwindef::UINT) -> ::windef::HBITMAP; /* wingdi.h:3639:26 */
    pub fn CreateDiscardableBitmap(hdc: ::windef::HDC, cx: ::libc::c_int, cy: ::libc::c_int) -> ::windef::HBITMAP; /* wingdi.h:3630:26 */
    pub fn CreateEllipticRgn(x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3642:26 */
    pub fn CreateEllipticRgnIndirect(lprect: *const ::windef::RECT) -> ::minwindef::HRGN; /* wingdi.h:3643:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateEnhMetaFileW as CreateEnhMetaFile; /* wingdi.h:4675:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateEnhMetaFileA(hdc: ::windef::HDC, lpFilename: ::winnt::LPCSTR, lprc: *const ::windef::RECT, lpDesc: ::winnt::LPCSTR) -> ::windef::HDC; /* wingdi.h:4672:24 */
    pub fn CreateEnhMetaFileW(hdc: ::windef::HDC, lpFilename: ::winnt::LPCWSTR, lprc: *const ::windef::RECT, lpDesc: ::winnt::LPCWSTR) -> ::windef::HDC; /* wingdi.h:4673:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFontW as CreateFont; /* wingdi.h:3658:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFontA(cHeight: ::libc::c_int, cWidth: ::libc::c_int, cEscapement: ::libc::c_int, cOrientation: ::libc::c_int, cWeight: ::libc::c_int, bItalic: ::minwindef::DWORD, bUnderline: ::minwindef::DWORD, bStrikeOut: ::minwindef::DWORD, iCharSet: ::minwindef::DWORD, iOutPrecision: ::minwindef::DWORD, iClipPrecision: ::minwindef::DWORD, iQuality: ::minwindef::DWORD, iPitchAndFamily: ::minwindef::DWORD, pszFaceName: ::winnt::LPCSTR) -> ::windef::HFONT; /* wingdi.h:3651:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateFontIndirectW as CreateFontIndirect; /* wingdi.h:3647:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateFontIndirectA(lplf: *const ::wingdi::LOGFONTA) -> ::windef::HFONT; /* wingdi.h:3644:27 */
    pub fn CreateFontIndirectW(lplf: *const ::wingdi::LOGFONTW) -> ::windef::HFONT; /* wingdi.h:3645:27 */
    pub fn CreateFontW(cHeight: ::libc::c_int, cWidth: ::libc::c_int, cEscapement: ::libc::c_int, cOrientation: ::libc::c_int, cWeight: ::libc::c_int, bItalic: ::minwindef::DWORD, bUnderline: ::minwindef::DWORD, bStrikeOut: ::minwindef::DWORD, iCharSet: ::minwindef::DWORD, iOutPrecision: ::minwindef::DWORD, iClipPrecision: ::minwindef::DWORD, iQuality: ::minwindef::DWORD, iPitchAndFamily: ::minwindef::DWORD, pszFaceName: ::winnt::LPCWSTR) -> ::windef::HFONT; /* wingdi.h:3654:26 */
    pub fn CreateHalftonePalette(hdc: ::windef::HDC) -> ::windef::HPALETTE; /* wingdi.h:4863:27 */
    pub fn CreateHatchBrush(iHatch: ::libc::c_int, color: ::windef::COLORREF) -> ::windef::HBRUSH; /* wingdi.h:3663:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateICW as CreateIC; /* wingdi.h:3667:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateICA(pszDriver: ::winnt::LPCSTR, pszDevice: ::winnt::LPCSTR, pszPort: ::winnt::LPCSTR, pdm: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:3664:26 */
    pub fn CreateICW(pszDriver: ::winnt::LPCWSTR, pszDevice: ::winnt::LPCWSTR, pszPort: ::winnt::LPCWSTR, pdm: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:3665:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateMetaFileW as CreateMetaFile; /* wingdi.h:3674:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateMetaFileA(pszFile: ::winnt::LPCSTR) -> ::windef::HDC; /* wingdi.h:3671:26 */
    pub fn CreateMetaFileW(pszFile: ::winnt::LPCWSTR) -> ::windef::HDC; /* wingdi.h:3672:26 */
    pub fn CreatePalette(plpal: *const ::wingdi::LOGPALETTE) -> ::windef::HPALETTE; /* wingdi.h:3678:28 */
    pub fn CreatePatternBrush(hbm: ::windef::HBITMAP) -> ::windef::HBRUSH; /* wingdi.h:3685:27 */
    pub fn CreatePen(iStyle: ::libc::c_int, cWidth: ::libc::c_int, color: ::windef::COLORREF) -> ::windef::HPEN; /* wingdi.h:3679:26 */
    pub fn CreatePenIndirect(plpen: *const ::wingdi::LOGPEN) -> ::windef::HPEN; /* wingdi.h:3680:27 */
    pub fn CreatePolyPolygonRgn(pptl: *const ::windef::POINT, pc: *const ::libc::c_int, cPoly: ::libc::c_int, iMode: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3681:26 */
    pub fn CreatePolygonRgn(pptl: *const ::windef::POINT, cPoint: ::libc::c_int, iMode: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:4992:24 */
    pub fn CreateRectRgn(x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3686:26 */
    pub fn CreateRectRgnIndirect(lprect: *const ::windef::RECT) -> ::minwindef::HRGN; /* wingdi.h:3687:26 */
    pub fn CreateRoundRectRgn(x1: ::libc::c_int, y1: ::libc::c_int, x2: ::libc::c_int, y2: ::libc::c_int, w: ::libc::c_int, h: ::libc::c_int) -> ::minwindef::HRGN; /* wingdi.h:3688:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::CreateScalableFontResourceW as CreateScalableFontResource; /* wingdi.h:3692:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn CreateScalableFontResourceA(fdwHidden: ::minwindef::DWORD, lpszFont: ::winnt::LPCSTR, lpszFile: ::winnt::LPCSTR, lpszPath: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wingdi.h:3689:26 */
    pub fn CreateScalableFontResourceW(fdwHidden: ::minwindef::DWORD, lpszFont: ::winnt::LPCWSTR, lpszFile: ::winnt::LPCWSTR, lpszPath: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wingdi.h:3690:26 */
    pub fn CreateSolidBrush(color: ::windef::COLORREF) -> ::windef::HBRUSH; /* wingdi.h:3696:26 */
    pub fn DPtoLP(hdc: ::windef::HDC, lppt: ::windef::LPPOINT, c: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4995:24 */
    pub fn DeleteDC(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:3698:23 */
    pub fn DeleteEnhMetaFile(hmf: ::windef::HENHMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:4679:24 */
    pub fn DeleteMetaFile(hmf: ::minwindef::HMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:3699:23 */
    pub fn DeleteObject(ho: ::windef::HGDIOBJ) -> ::minwindef::BOOL; /* wingdi.h:3700:24 */
    pub fn DescribePixelFormat(hdc: ::windef::HDC, iPixelFormat: ::libc::c_int, nBytes: ::minwindef::UINT, ppfd: ::wingdi::LPPIXELFORMATDESCRIPTOR) -> ::libc::c_int; /* wingdi.h:3701:23 */
    pub fn DrawEscape(hdc: ::windef::HDC, iEscape: ::libc::c_int, cjIn: ::libc::c_int, lpIn: ::winnt::LPCSTR) -> ::libc::c_int; /* wingdi.h:3823:23 */
    pub fn Ellipse(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3828:24 */
    pub fn EndDoc(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4909:23 */
    pub fn EndPage(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4911:23 */
    pub fn EndPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4919:23 */
    pub fn EnumEnhMetaFile(hdc: ::windef::HDC, hmf: ::windef::HENHMETAFILE, proc_: ::wingdi::ENHMFENUMPROC, param: ::minwindef::LPVOID, lpRect: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4680:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumFontFamiliesW as EnumFontFamilies; /* wingdi.h:3843:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumFontFamiliesA(hdc: ::windef::HDC, lpLogfont: ::winnt::LPCSTR, lpProc: ::wingdi::FONTENUMPROCA, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3840:23 */
    pub fn EnumFontFamiliesW(hdc: ::windef::HDC, lpLogfont: ::winnt::LPCWSTR, lpProc: ::wingdi::FONTENUMPROCW, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3841:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::EnumFontsW as EnumFonts; /* wingdi.h:3850:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn EnumFontsA(hdc: ::windef::HDC, lpLogfont: ::winnt::LPCSTR, lpProc: ::wingdi::FONTENUMPROCA, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3847:23 */
    pub fn EnumFontsW(hdc: ::windef::HDC, lpLogfont: ::winnt::LPCWSTR, lpProc: ::wingdi::FONTENUMPROCW, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3848:23 */
    pub fn EnumMetaFile(hdc: ::windef::HDC, hmf: ::minwindef::HMETAFILE, proc_: ::wingdi::MFENUMPROC, param: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* wingdi.h:4658:24 */
    pub fn EnumObjects(hdc: ::windef::HDC, nType: ::libc::c_int, lpFunc: ::wingdi::GOBJENUMPROC, lParam: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:3856:23 */
    pub fn EqualRgn(hrgn1: ::minwindef::HRGN, hrgn2: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:3862:23 */
    pub fn Escape(hdc: ::windef::HDC, iEscape: ::libc::c_int, cjIn: ::libc::c_int, pvIn: ::winnt::LPCSTR, pvOut: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:3863:24 */
    pub fn ExcludeClipRect(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3874:24 */
    pub fn ExtCreatePen(iPenStyle: ::minwindef::DWORD, cWidth: ::minwindef::DWORD, plbrush: *const ::wingdi::LOGBRUSH, cStyle: ::minwindef::DWORD, pstyle: *const ::libc::c_ulong) -> ::windef::HPEN; /* wingdi.h:4931:23 */
    pub fn ExtCreateRegion(lpx: *const ::wingdi::XFORM, nCount: ::minwindef::DWORD, lpData: *const ::wingdi::RGNDATA) -> ::minwindef::HRGN; /* wingdi.h:3875:24 */
    pub fn ExtEscape(hdc: ::windef::HDC, iEscape: ::libc::c_int, cjInput: ::libc::c_int, lpInData: ::winnt::LPCSTR, cjOutput: ::libc::c_int, lpOutData: ::winnt::LPSTR) -> ::libc::c_int; /* wingdi.h:3868:23 */
    pub fn ExtFloodFill(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, color: ::windef::COLORREF, type_: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:3876:24 */
    pub fn ExtSelectClipRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN, mode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4407:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ExtTextOutW as ExtTextOut; /* wingdi.h:4980:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ExtTextOutA(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, options: ::minwindef::UINT, lprect: *const ::windef::RECT, lpString: ::winnt::LPCSTR, c: ::minwindef::UINT, lpDx: *const ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4977:25 */
    pub fn ExtTextOutW(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, options: ::minwindef::UINT, lprect: *const ::windef::RECT, lpString: ::winnt::LPCWSTR, c: ::minwindef::UINT, lpDx: *const ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4978:25 */
    pub fn FillPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4920:23 */
    pub fn FillRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN, hbr: ::windef::HBRUSH) -> ::minwindef::BOOL; /* wingdi.h:3877:24 */
    pub fn FixBrushOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, ptl: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5046:24 */
    pub fn FlattenPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4921:23 */
    pub fn FloodFill(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, color: ::windef::COLORREF) -> ::minwindef::BOOL; /* wingdi.h:3878:24 */
    pub fn FrameRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN, hbr: ::windef::HBRUSH, w: ::libc::c_int, h: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:3879:24 */
    pub fn GdiComment(hdc: ::windef::HDC, nSize: ::minwindef::UINT, lpData: *const ::libc::c_uchar) -> ::minwindef::BOOL; /* wingdi.h:4731:24 */
    pub fn GdiFlush() -> ::minwindef::BOOL; /* wingdi.h:5049:24 */
    pub fn GdiGetBatchLimit() -> ::minwindef::DWORD; /* wingdi.h:5051:24 */
    pub fn GdiSetBatchLimit(dw: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:5050:24 */
    pub fn GetArcDirection(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4937:23 */
    pub fn GetAspectRatioFilterEx(hdc: ::windef::HDC, lpsize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:3881:23 */
    pub fn GetBitmapBits(hbit: ::windef::HBITMAP, cb: ::winnt::LONG, lpvBits: ::minwindef::LPVOID) -> ::winnt::LONG; /* wingdi.h:3899:1 */
    pub fn GetBitmapDimensionEx(hbit: ::windef::HBITMAP, lpsize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:3905:24 */
    pub fn GetBkColor(hdc: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3882:27 */
    pub fn GetBkMode(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3892:1 */
    pub fn GetBoundsRect(hdc: ::windef::HDC, lprect: ::windef::LPRECT, flags: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:3906:24 */
    pub fn GetBrushOrgEx(hdc: ::windef::HDC, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:3908:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharABCWidthsW as GetCharABCWidths; /* wingdi.h:3941:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharABCWidthsA(hdc: ::windef::HDC, wFirst: ::minwindef::UINT, wLast: ::minwindef::UINT, lpABC: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:3932:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharABCWidthsFloatW as GetCharABCWidthsFloat; /* wingdi.h:3949:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharABCWidthsFloatA(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpABC: ::wingdi::LPABCFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3946:26 */
    pub fn GetCharABCWidthsFloatW(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpABC: ::wingdi::LPABCFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3947:26 */
    pub fn GetCharABCWidthsW(hdc: ::windef::HDC, wFirst: ::minwindef::UINT, wLast: ::minwindef::UINT, lpABC: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:3936:26 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidthW as GetCharWidth; /* wingdi.h:3913:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidth32W as GetCharWidth32; /* wingdi.h:3920:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharWidth32A(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3917:24 */
    pub fn GetCharWidth32W(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3918:24 */
    pub fn GetCharWidthA(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3910:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetCharWidthFloatW as GetCharWidthFloat; /* wingdi.h:3927:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetCharWidthFloatA(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3924:26 */
    pub fn GetCharWidthFloatW(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:3925:26 */
    pub fn GetCharWidthW(hdc: ::windef::HDC, iFirst: ::minwindef::UINT, iLast: ::minwindef::UINT, lpBuffer: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:3911:24 */
    pub fn GetClipBox(hdc: ::windef::HDC, lprect: ::windef::LPRECT) -> ::libc::c_int; /* wingdi.h:3953:24 */
    pub fn GetClipRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:3954:24 */
    pub fn GetColorAdjustment(hdc: ::windef::HDC, lpca: ::wingdi::LPCOLORADJUSTMENT) -> ::minwindef::BOOL; /* wingdi.h:4862:23 */
    pub fn GetCurrentObject(hdc: ::windef::HDC, type_: ::minwindef::UINT) -> ::windef::HGDIOBJ; /* wingdi.h:3956:26 */
    pub fn GetCurrentPositionEx(hdc: ::windef::HDC, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:3957:24 */
    pub fn GetDCOrgEx(hdc: ::windef::HDC, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5045:24 */
    pub fn GetDIBColorTable(hdc: ::windef::HDC, iStart: ::minwindef::UINT, cEntries: ::minwindef::UINT, prgbq: *mut ::wingdi::RGBQUAD) -> ::minwindef::UINT; /* wingdi.h:4802:23 */
    pub fn GetDIBits(hdc: ::windef::HDC, hbm: ::windef::HBITMAP, start: ::minwindef::UINT, cLines: ::minwindef::UINT, lpvBits: ::minwindef::LPVOID, lpbmi: ::wingdi::LPBITMAPINFO, usage: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:3959:24 */
    pub fn GetDeviceCaps(hdc: ::windef::HDC, index: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:3958:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnhMetaFileW as GetEnhMetaFile; /* wingdi.h:4685:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnhMetaFileA(lpName: ::winnt::LPCSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4682:32 */
    pub fn GetEnhMetaFileBits(hEMF: ::windef::HENHMETAFILE, nSize: ::minwindef::UINT, lpData: ::minwindef::LPBYTE) -> ::minwindef::UINT; /* wingdi.h:4689:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetEnhMetaFileDescriptionW as GetEnhMetaFileDescription; /* wingdi.h:4699:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetEnhMetaFileDescriptionA(hemf: ::windef::HENHMETAFILE, cchBuffer: ::minwindef::UINT, lpDescription: ::winnt::LPSTR) -> ::minwindef::UINT; /* wingdi.h:4692:24 */
    pub fn GetEnhMetaFileDescriptionW(hemf: ::windef::HENHMETAFILE, cchBuffer: ::minwindef::UINT, lpDescription: ::winnt::LPWSTR) -> ::minwindef::UINT; /* wingdi.h:4695:24 */
    pub fn GetEnhMetaFileHeader(hemf: ::windef::HENHMETAFILE, nSize: ::minwindef::UINT, lpEnhMetaHeader: ::wingdi::LPENHMETAHEADER) -> ::minwindef::UINT; /* wingdi.h:4703:24 */
    pub fn GetEnhMetaFilePaletteEntries(hemf: ::windef::HENHMETAFILE, nNumEntries: ::minwindef::UINT, lpPaletteEntries: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4706:24 */
    pub fn GetEnhMetaFilePixelFormat(hemf: ::windef::HENHMETAFILE, cbBuffer: ::minwindef::UINT, ppfd: *mut ::wingdi::PIXELFORMATDESCRIPTOR) -> ::minwindef::UINT; /* wingdi.h:4710:24 */
    pub fn GetEnhMetaFileW(lpName: ::winnt::LPCWSTR) -> ::windef::HENHMETAFILE; /* wingdi.h:4683:32 */
    pub fn GetFontData(hdc: ::windef::HDC, dwTable: ::minwindef::DWORD, dwOffset: ::minwindef::DWORD, pvBuffer: ::winnt::PVOID, cjBuffer: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:3963:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetGlyphOutlineW as GetGlyphOutline; /* wingdi.h:3987:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetGlyphOutlineA(hdc: ::windef::HDC, uChar: ::minwindef::UINT, fuFormat: ::minwindef::UINT, lpgm: ::wingdi::LPGLYPHMETRICS, cjBuffer: ::minwindef::DWORD, pvBuffer: ::minwindef::LPVOID, lpmat2: *const ::wingdi::MAT2) -> ::minwindef::DWORD; /* wingdi.h:3970:24 */
    pub fn GetGlyphOutlineW(hdc: ::windef::HDC, uChar: ::minwindef::UINT, fuFormat: ::minwindef::UINT, lpgm: ::wingdi::LPGLYPHMETRICS, cjBuffer: ::minwindef::DWORD, pvBuffer: ::minwindef::LPVOID, lpmat2: *const ::wingdi::MAT2) -> ::minwindef::DWORD; /* wingdi.h:3978:24 */
    pub fn GetGraphicsMode(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3992:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetKerningPairsW as GetKerningPairs; /* wingdi.h:5039:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetKerningPairsA(hdc: ::windef::HDC, nPairs: ::minwindef::DWORD, lpKernPair: ::wingdi::LPKERNINGPAIR) -> ::minwindef::DWORD; /* wingdi.h:5032:24 */
    pub fn GetKerningPairsW(hdc: ::windef::HDC, nPairs: ::minwindef::DWORD, lpKernPair: ::wingdi::LPKERNINGPAIR) -> ::minwindef::DWORD; /* wingdi.h:5035:24 */
    pub fn GetMapMode(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3993:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetMetaFileW as GetMetaFile; /* wingdi.h:3998:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetMetaFileA(lpName: ::winnt::LPCSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3995:30 */
    pub fn GetMetaFileBitsEx(hMF: ::minwindef::HMETAFILE, cbBuffer: ::minwindef::UINT, lpData: ::minwindef::LPVOID) -> ::minwindef::UINT; /* wingdi.h:3994:24 */
    pub fn GetMetaFileW(lpName: ::winnt::LPCWSTR) -> ::minwindef::HMETAFILE; /* wingdi.h:3996:30 */
    pub fn GetMetaRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:3955:24 */
    pub fn GetMiterLimit(hdc: ::windef::HDC, plimit: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:4936:23 */
    pub fn GetNearestColor(hdc: ::windef::HDC, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4002:27 */
    pub fn GetNearestPaletteIndex(h: ::windef::HPALETTE, color: ::windef::COLORREF) -> ::minwindef::UINT; /* wingdi.h:4003:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetObjectW as GetObject; /* wingdi.h:4942:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetObjectA(h: ::winnt::HANDLE, c: ::libc::c_int, pv: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:4939:24 */
    pub fn GetObjectType(h: ::windef::HGDIOBJ) -> ::minwindef::DWORD; /* wingdi.h:4004:24 */
    pub fn GetObjectW(h: ::winnt::HANDLE, c: ::libc::c_int, pv: ::minwindef::LPVOID) -> ::libc::c_int; /* wingdi.h:4940:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetOutlineTextMetricsW as GetOutlineTextMetrics; /* wingdi.h:4015:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetOutlineTextMetricsA(hdc: ::windef::HDC, cjCopy: ::minwindef::UINT, potm: ::wingdi::LPOUTLINETEXTMETRICA) -> ::minwindef::UINT; /* wingdi.h:4008:25 */
    pub fn GetOutlineTextMetricsW(hdc: ::windef::HDC, cjCopy: ::minwindef::UINT, potm: ::wingdi::LPOUTLINETEXTMETRICW) -> ::minwindef::UINT; /* wingdi.h:4011:25 */
    pub fn GetPaletteEntries(hpal: ::windef::HPALETTE, iStart: ::minwindef::UINT, cEntries: ::minwindef::UINT, pPalEntries: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4023:24 */
    pub fn GetPath(hdc: ::windef::HDC, apt: ::windef::LPPOINT, aj: ::minwindef::LPBYTE, cpt: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4922:23 */
    pub fn GetPixel(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int) -> ::windef::COLORREF; /* wingdi.h:4027:27 */
    pub fn GetPixelFormat(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4028:24 */
    pub fn GetPolyFillMode(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4029:24 */
    pub fn GetROP2(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:3880:23 */
    pub fn GetRandomRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN, i: ::winnt::INT) -> ::libc::c_int; /* wingdi.h:4033:24 */
    pub fn GetRasterizerCaps(lpraststat: ::wingdi::LPRASTERIZER_STATUS, cjBytes: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4030:24 */
    pub fn GetRegionData(hrgn: ::minwindef::HRGN, nCount: ::minwindef::DWORD, lpRgnData: ::wingdi::LPRGNDATA) -> ::minwindef::DWORD; /* wingdi.h:4034:24 */
    pub fn GetRgnBox(hrgn: ::minwindef::HRGN, lprc: ::windef::LPRECT) -> ::libc::c_int; /* wingdi.h:4037:24 */
    pub fn GetStockObject(i: ::libc::c_int) -> ::windef::HGDIOBJ; /* wingdi.h:4038:26 */
    pub fn GetStretchBltMode(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4039:24 */
    pub fn GetSystemPaletteEntries(hdc: ::windef::HDC, iStart: ::minwindef::UINT, cEntries: ::minwindef::UINT, pPalEntries: ::wingdi::LPPALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4043:1 */
    pub fn GetSystemPaletteUse(hdc: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4050:24 */
    pub fn GetTextAlign(hdc: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4052:24 */
    pub fn GetTextCharacterExtra(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4051:24 */
    pub fn GetTextColor(hdc: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:4053:27 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentExPointW as GetTextExtentExPoint; /* wingdi.h:4128:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextExtentExPointA(hdc: ::windef::HDC, lpszString: ::winnt::LPCSTR, cchString: ::libc::c_int, nMaxExtent: ::libc::c_int, lpnFit: ::minwindef::LPINT, lpnDx: ::minwindef::LPINT, lpSize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4106:1 */
    pub fn GetTextExtentExPointW(hdc: ::windef::HDC, lpszString: ::winnt::LPCWSTR, cchString: ::libc::c_int, nMaxExtent: ::libc::c_int, lpnFit: ::minwindef::LPINT, lpnDx: ::minwindef::LPINT, lpSize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4118:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentPointW as GetTextExtentPoint; /* wingdi.h:4074:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextExtentPoint32W as GetTextExtentPoint32; /* wingdi.h:4098:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextExtentPoint32A(hdc: ::windef::HDC, lpString: ::winnt::LPCSTR, c: ::libc::c_int, psizl: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4082:1 */
    pub fn GetTextExtentPoint32W(hdc: ::windef::HDC, lpString: ::winnt::LPCWSTR, c: ::libc::c_int, psizl: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4091:1 */
    pub fn GetTextExtentPointA(hdc: ::windef::HDC, lpString: ::winnt::LPCSTR, c: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4058:1 */
    pub fn GetTextExtentPointW(hdc: ::windef::HDC, lpString: ::winnt::LPCWSTR, c: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4067:1 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextFaceW as GetTextFace; /* wingdi.h:5019:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextFaceA(hdc: ::windef::HDC, c: ::libc::c_int, lpName: ::winnt::LPSTR) -> ::libc::c_int; /* wingdi.h:5016:24 */
    pub fn GetTextFaceW(hdc: ::windef::HDC, c: ::libc::c_int, lpName: ::winnt::LPWSTR) -> ::libc::c_int; /* wingdi.h:5017:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::GetTextMetricsW as GetTextMetrics; /* wingdi.h:4740:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn GetTextMetricsA(hdc: ::windef::HDC, lptm: ::wingdi::LPTEXTMETRICA) -> ::minwindef::BOOL; /* wingdi.h:4737:23 */
    pub fn GetTextMetricsW(hdc: ::windef::HDC, lptm: ::wingdi::LPTEXTMETRICW) -> ::minwindef::BOOL; /* wingdi.h:4738:23 */
    pub fn GetViewportExtEx(hdc: ::windef::HDC, lpsize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4360:24 */
    pub fn GetViewportOrgEx(hdc: ::windef::HDC, lppoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4361:24 */
    pub fn GetWinMetaFileBits(hemf: ::windef::HENHMETAFILE, cbData16: ::minwindef::UINT, pData16: ::minwindef::LPBYTE, iMapMode: ::winnt::INT, hdcRef: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4713:24 */
    pub fn GetWindowExtEx(hdc: ::windef::HDC, lpsize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4362:24 */
    pub fn GetWindowOrgEx(hdc: ::windef::HDC, lppoint: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4363:24 */
    pub fn GetWorldTransform(hdc: ::windef::HDC, lpxf: ::wingdi::LPXFORM) -> ::minwindef::BOOL; /* wingdi.h:4780:23 */
    pub fn IntersectClipRect(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4365:24 */
    pub fn InvertRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:4366:24 */
    pub fn LPtoDP(hdc: ::windef::HDC, lppt: ::windef::LPPOINT, c: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4996:24 */
    pub fn LineDDA(xStart: ::libc::c_int, yStart: ::libc::c_int, xEnd: ::libc::c_int, yEnd: ::libc::c_int, lpProc: ::wingdi::LINEDDAPROC, data: ::minwindef::LPARAM) -> ::minwindef::BOOL; /* wingdi.h:4367:23 */
    pub fn LineTo(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4368:24 */
    pub fn MaskBlt(hdcDest: ::windef::HDC, xDest: ::libc::c_int, yDest: ::libc::c_int, width: ::libc::c_int, height: ::libc::c_int, hdcSrc: ::windef::HDC, xSrc: ::libc::c_int, ySrc: ::libc::c_int, hbmMask: ::windef::HBITMAP, xMask: ::libc::c_int, yMask: ::libc::c_int, rop: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4369:23 */
    pub fn ModifyWorldTransform(hdc: ::windef::HDC, lpxf: *const ::wingdi::XFORM, mode: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4782:23 */
    pub fn MoveToEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:4969:25 */
    pub fn OffsetClipRgn(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4374:24 */
    pub fn OffsetRgn(hrgn: ::minwindef::HRGN, x: ::libc::c_int, y: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4375:23 */
    pub fn OffsetViewportOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5009:25 */
    pub fn OffsetWindowOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5010:25 */
    pub fn PaintRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN) -> ::minwindef::BOOL; /* wingdi.h:4379:24 */
    pub fn PatBlt(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, w: ::libc::c_int, h: ::libc::c_int, rop: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4376:24 */
    pub fn PathToRegion(hdc: ::windef::HDC) -> ::minwindef::HRGN; /* wingdi.h:4923:23 */
    pub fn Pie(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int, xr1: ::libc::c_int, yr1: ::libc::c_int, xr2: ::libc::c_int, yr2: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4377:24 */
    pub fn PlayEnhMetaFile(hdc: ::windef::HDC, hmf: ::windef::HENHMETAFILE, lprect: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4718:24 */
    pub fn PlayEnhMetaFileRecord(hdc: ::windef::HDC, pht: ::wingdi::LPHANDLETABLE, pmr: *const ::wingdi::ENHMETARECORD, cht: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4719:24 */
    pub fn PlayMetaFile(hdc: ::windef::HDC, hmf: ::minwindef::HMETAFILE) -> ::minwindef::BOOL; /* wingdi.h:4378:23 */
    pub fn PlayMetaFileRecord(hdc: ::windef::HDC, lpHandleTable: ::wingdi::LPHANDLETABLE, lpMR: ::wingdi::LPMETARECORD, noObjs: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4652:24 */
    pub fn PlgBlt(hdcDest: ::windef::HDC, lpPoint: *const ::windef::POINT, hdcSrc: ::windef::HDC, xSrc: ::libc::c_int, ySrc: ::libc::c_int, width: ::libc::c_int, height: ::libc::c_int, hbmMask: ::windef::HBITMAP, xMask: ::libc::c_int, yMask: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4371:23 */
    pub fn PolyBezier(hdc: ::windef::HDC, apt: *const ::windef::POINT, cpt: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5000:24 */
    pub fn PolyBezierTo(hdc: ::windef::HDC, apt: *const ::windef::POINT, cpt: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5001:24 */
    pub fn PolyDraw(hdc: ::windef::HDC, apt: *const ::windef::POINT, aj: *const ::libc::c_uchar, cpt: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4924:23 */
    pub fn PolyPolygon(hdc: ::windef::HDC, apt: *const ::windef::POINT, asz: *const ::libc::c_int, csz: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4380:24 */
    pub fn PolyPolyline(hdc: ::windef::HDC, apt: *const ::windef::POINT, asz: *const ::libc::c_ulong, csz: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4779:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::PolyTextOutW as PolyTextOut; /* wingdi.h:4987:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn PolyTextOutA(hdc: ::windef::HDC, ppt: *const ::wingdi::POLYTEXTA, nstrings: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4984:24 */
    pub fn PolyTextOutW(hdc: ::windef::HDC, ppt: *const ::wingdi::POLYTEXTW, nstrings: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4985:24 */
    pub fn Polygon(hdc: ::windef::HDC, apt: *const ::windef::POINT, cpt: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4997:25 */
    pub fn Polyline(hdc: ::windef::HDC, apt: *const ::windef::POINT, cpt: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4998:25 */
    pub fn PolylineTo(hdc: ::windef::HDC, apt: *const ::windef::POINT, cpt: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5002:24 */
    pub fn PtInRegion(hrgn: ::minwindef::HRGN, x: ::libc::c_int, y: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4381:23 */
    pub fn PtVisible(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4382:23 */
    pub fn RealizePalette(hdc: ::windef::HDC) -> ::minwindef::UINT; /* wingdi.h:4394:24 */
    pub fn RectInRegion(hrgn: ::minwindef::HRGN, lprect: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4383:23 */
    pub fn RectVisible(hdc: ::windef::HDC, lprect: *const ::windef::RECT) -> ::minwindef::BOOL; /* wingdi.h:4384:23 */
    pub fn Rectangle(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4385:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::RemoveFontResourceW as RemoveFontResource; /* wingdi.h:4398:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn RemoveFontResourceA(lpFileName: ::winnt::LPCSTR) -> ::minwindef::BOOL; /* wingdi.h:4395:23 */
    pub fn RemoveFontResourceW(lpFileName: ::winnt::LPCWSTR) -> ::minwindef::BOOL; /* wingdi.h:4396:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::ResetDCW as ResetDC; /* wingdi.h:4390:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn ResetDCA(hdc: ::windef::HDC, lpdm: *const ::wingdi::DEVMODEA) -> ::windef::HDC; /* wingdi.h:4387:24 */
    pub fn ResetDCW(hdc: ::windef::HDC, lpdm: *const ::wingdi::DEVMODEW) -> ::windef::HDC; /* wingdi.h:4388:24 */
    pub fn ResizePalette(hpal: ::windef::HPALETTE, n: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4403:24 */
    pub fn RestoreDC(hdc: ::windef::HDC, nSavedDC: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4386:24 */
    pub fn RoundRect(hdc: ::windef::HDC, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int, width: ::libc::c_int, height: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4402:25 */
    pub fn SaveDC(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4405:24 */
    pub fn ScaleViewportExtEx(hdc: ::windef::HDC, xn: ::libc::c_int, dx: ::libc::c_int, yn: ::libc::c_int, yd: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5011:25 */
    pub fn ScaleWindowExtEx(hdc: ::windef::HDC, xn: ::libc::c_int, xd: ::libc::c_int, yn: ::libc::c_int, yd: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5012:25 */
    pub fn SelectClipPath(hdc: ::windef::HDC, mode: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4925:23 */
    pub fn SelectClipRgn(hdc: ::windef::HDC, hrgn: ::minwindef::HRGN) -> ::libc::c_int; /* wingdi.h:4406:24 */
    pub fn SelectObject(hdc: ::windef::HDC, h: ::windef::HGDIOBJ) -> ::windef::HGDIOBJ; /* wingdi.h:4409:27 */
    pub fn SelectPalette(hdc: ::windef::HDC, hPal: ::windef::HPALETTE, bForceBkgd: ::minwindef::BOOL) -> ::windef::HPALETTE; /* wingdi.h:4410:28 */
    pub fn SetAbortProc(hdc: ::windef::HDC, proc_: ::wingdi::ABORTPROC) -> ::libc::c_int; /* wingdi.h:4913:22 */
    pub fn SetArcDirection(hdc: ::windef::HDC, dir: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4926:23 */
    pub fn SetBitmapBits(hbm: ::windef::HBITMAP, cb: ::minwindef::DWORD, pvBits: *const ::libc::c_void) -> ::winnt::LONG; /* wingdi.h:4422:1 */
    pub fn SetBitmapDimensionEx(hbm: ::windef::HBITMAP, w: ::libc::c_int, h: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5013:24 */
    pub fn SetBkColor(hdc: ::windef::HDC, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4411:28 */
    pub fn SetBkMode(hdc: ::windef::HDC, mode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4418:25 */
    pub fn SetBoundsRect(hdc: ::windef::HDC, lprect: *const ::windef::RECT, flags: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4427:24 */
    pub fn SetBrushOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5014:24 */
    pub fn SetColorAdjustment(hdc: ::windef::HDC, lpca: *const ::wingdi::COLORADJUSTMENT) -> ::minwindef::BOOL; /* wingdi.h:4861:23 */
    pub fn SetDIBColorTable(hdc: ::windef::HDC, iStart: ::minwindef::UINT, cEntries: ::minwindef::UINT, prgbq: *const ::wingdi::RGBQUAD) -> ::minwindef::UINT; /* wingdi.h:4806:23 */
    pub fn SetDIBits(hdc: ::windef::HDC, hbm: ::windef::HBITMAP, start: ::minwindef::UINT, cLines: ::minwindef::UINT, lpBits: *const ::libc::c_void, lpbmi: *const ::wingdi::BITMAPINFO, ColorUse: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:4428:24 */
    pub fn SetDIBitsToDevice(hdc: ::windef::HDC, xDest: ::libc::c_int, yDest: ::libc::c_int, w: ::minwindef::DWORD, h: ::minwindef::DWORD, xSrc: ::libc::c_int, ySrc: ::libc::c_int, StartScan: ::minwindef::UINT, cLines: ::minwindef::UINT, lpvBits: *const ::libc::c_void, lpbmi: *const ::wingdi::BITMAPINFO, ColorUse: ::minwindef::UINT) -> ::libc::c_int; /* wingdi.h:4429:25 */
    pub fn SetEnhMetaFileBits(nSize: ::minwindef::UINT, pb: *const ::libc::c_uchar) -> ::windef::HENHMETAFILE; /* wingdi.h:4724:32 */
    pub fn SetGraphicsMode(hdc: ::windef::HDC, iMode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4432:24 */
    pub fn SetMapMode(hdc: ::windef::HDC, iMode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4433:25 */
    pub fn SetMapperFlags(hdc: ::windef::HDC, flags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4431:25 */
    pub fn SetMetaFileBitsEx(cbBuffer: ::minwindef::UINT, lpData: *const ::libc::c_uchar) -> ::minwindef::HMETAFILE; /* wingdi.h:4440:30 */
    pub fn SetMetaRgn(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4408:23 */
    pub fn SetMiterLimit(hdc: ::windef::HDC, limit: ::minwindef::FLOAT, old: ::minwindef::PFLOAT) -> ::minwindef::BOOL; /* wingdi.h:4927:23 */
    pub fn SetPaletteEntries(hpal: ::windef::HPALETTE, iStart: ::minwindef::UINT, cEntries: ::minwindef::UINT, pPalEntries: *const ::wingdi::PALETTEENTRY) -> ::minwindef::UINT; /* wingdi.h:4441:25 */
    pub fn SetPixel(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4445:28 */
    pub fn SetPixelFormat(hdc: ::windef::HDC, format: ::libc::c_int, ppfd: *const ::wingdi::PIXELFORMATDESCRIPTOR) -> ::minwindef::BOOL; /* wingdi.h:4447:24 */
    pub fn SetPixelV(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, color: ::windef::COLORREF) -> ::minwindef::BOOL; /* wingdi.h:4446:25 */
    pub fn SetPolyFillMode(hdc: ::windef::HDC, mode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4448:25 */
    pub fn SetROP2(hdc: ::windef::HDC, rop2: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4453:25 */
    pub fn SetRectRgn(hrgn: ::minwindef::HRGN, left: ::libc::c_int, top: ::libc::c_int, right: ::libc::c_int, bottom: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4450:25 */
    pub fn SetStretchBltMode(hdc: ::windef::HDC, mode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4454:25 */
    pub fn SetSystemPaletteUse(hdc: ::windef::HDC, use_: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4455:24 */
    pub fn SetTextAlign(hdc: ::windef::HDC, align: ::minwindef::UINT) -> ::minwindef::UINT; /* wingdi.h:4458:25 */
    pub fn SetTextCharacterExtra(hdc: ::windef::HDC, extra: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4456:25 */
    pub fn SetTextColor(hdc: ::windef::HDC, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4457:28 */
    pub fn SetTextJustification(hdc: ::windef::HDC, extra: ::libc::c_int, count: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4459:25 */
    pub fn SetViewportExtEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5004:25 */
    pub fn SetViewportOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5005:25 */
    pub fn SetWinMetaFileBits(nSize: ::minwindef::UINT, lpMeta16Data: *const ::libc::c_uchar, hdcRef: ::windef::HDC, lpMFP: *const ::wingdi::METAFILEPICT) -> ::windef::HENHMETAFILE; /* wingdi.h:4727:32 */
    pub fn SetWindowExtEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpsz: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:5006:25 */
    pub fn SetWindowOrgEx(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lppt: ::windef::LPPOINT) -> ::minwindef::BOOL; /* wingdi.h:5007:25 */
    pub fn SetWorldTransform(hdc: ::windef::HDC, lpxf: *const ::wingdi::XFORM) -> ::minwindef::BOOL; /* wingdi.h:4781:23 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::StartDocW as StartDoc; /* wingdi.h:4905:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn StartDocA(hdc: ::windef::HDC, lpdi: *const ::wingdi::DOCINFOA) -> ::libc::c_int; /* wingdi.h:4902:23 */
    pub fn StartDocW(hdc: ::windef::HDC, lpdi: *const ::wingdi::DOCINFOW) -> ::libc::c_int; /* wingdi.h:4903:23 */
    pub fn StartPage(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4910:23 */
    pub fn StretchBlt(hdcDest: ::windef::HDC, xDest: ::libc::c_int, yDest: ::libc::c_int, wDest: ::libc::c_int, hDest: ::libc::c_int, hdcSrc: ::windef::HDC, xSrc: ::libc::c_int, ySrc: ::libc::c_int, wSrc: ::libc::c_int, hSrc: ::libc::c_int, rop: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4449:25 */
    pub fn StretchDIBits(hdc: ::windef::HDC, xDest: ::libc::c_int, yDest: ::libc::c_int, DestWidth: ::libc::c_int, DestHeight: ::libc::c_int, xSrc: ::libc::c_int, ySrc: ::libc::c_int, SrcWidth: ::libc::c_int, SrcHeight: ::libc::c_int, lpBits: *const ::libc::c_void, lpbmi: *const ::wingdi::BITMAPINFO, iUsage: ::minwindef::UINT, rop: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:4451:25 */
    pub fn StrokeAndFillPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4928:23 */
    pub fn StrokePath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4929:23 */
    pub fn SwapBuffers(_: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:6089:24 */
}
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use self::TextOutW as TextOut; /* wingdi.h:4973:9 */
#[cfg(feature="winapi_desktop")] 
extern "system" {
    pub fn TextOutA(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpString: ::winnt::LPCSTR, c: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4970:25 */
    pub fn TextOutW(hdc: ::windef::HDC, x: ::libc::c_int, y: ::libc::c_int, lpString: ::winnt::LPCWSTR, c: ::libc::c_int) -> ::minwindef::BOOL; /* wingdi.h:4971:25 */
    pub fn UnrealizeObject(h: ::windef::HGDIOBJ) -> ::minwindef::BOOL; /* wingdi.h:5047:24 */
    pub fn UpdateColors(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4460:24 */
    pub fn WidenPath(hdc: ::windef::HDC) -> ::minwindef::BOOL; /* wingdi.h:4930:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CheckColorsInGamut(hdc: ::windef::HDC, lpRGBTriple: ::wingdi::LPRGBTRIPLE, dlpBuffer: ::minwindef::LPVOID, nCount: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5069:30 */
    pub fn ColorMatchToTarget(hdc: ::windef::HDC, hdcTarget: ::windef::HDC, action: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5117:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::CreateColorSpaceW as CreateColorSpace; /* wingdi.h:5090:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn CreateColorSpaceA(lplcs: ::wingdi::LPLOGCOLORSPACEA) -> ::windef::HCOLORSPACE; /* wingdi.h:5087:30 */
    pub fn CreateColorSpaceW(lplcs: ::wingdi::LPLOGCOLORSPACEW) -> ::windef::HCOLORSPACE; /* wingdi.h:5088:30 */
    pub fn DeleteColorSpace(hcs: ::windef::HCOLORSPACE) -> ::minwindef::BOOL; /* wingdi.h:5095:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::EnumFontFamiliesExW as EnumFontFamiliesEx; /* wingdi.h:3834:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn EnumFontFamiliesExA(hdc: ::windef::HDC, lpLogfont: ::wingdi::LPLOGFONTA, lpProc: ::wingdi::FONTENUMPROCA, lParam: ::minwindef::LPARAM, dwFlags: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:3831:23 */
    pub fn EnumFontFamiliesExW(hdc: ::windef::HDC, lpLogfont: ::wingdi::LPLOGFONTW, lpProc: ::wingdi::FONTENUMPROCW, lParam: ::minwindef::LPARAM, dwFlags: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:3832:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::EnumICMProfilesW as EnumICMProfiles; /* wingdi.h:5121:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn EnumICMProfilesA(hdc: ::windef::HDC, proc_: ::wingdi::ICMENUMPROCA, param: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:5118:30 */
    pub fn EnumICMProfilesW(hdc: ::windef::HDC, proc_: ::wingdi::ICMENUMPROCW, param: ::minwindef::LPARAM) -> ::libc::c_int; /* wingdi.h:5119:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetCharacterPlacementW as GetCharacterPlacement; /* wingdi.h:4141:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetCharacterPlacementA(hdc: ::windef::HDC, lpString: ::winnt::LPCSTR, nCount: ::libc::c_int, nMexExtent: ::libc::c_int, lpResults: ::wingdi::LPGCP_RESULTSA, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4138:24 */
    pub fn GetCharacterPlacementW(hdc: ::windef::HDC, lpString: ::winnt::LPCWSTR, nCount: ::libc::c_int, nMexExtent: ::libc::c_int, lpResults: ::wingdi::LPGCP_RESULTSW, dwFlags: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4139:24 */
    pub fn GetColorSpace(hdc: ::windef::HDC) -> ::windef::HCOLORSPACE; /* wingdi.h:5074:30 */
    pub fn GetDeviceGammaRamp(hdc: ::windef::HDC, lpRamp: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* wingdi.h:5115:30 */
    pub fn GetFontLanguageInfo(hdc: ::windef::HDC) -> ::minwindef::DWORD; /* wingdi.h:4137:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetICMProfileW as GetICMProfile; /* wingdi.h:5103:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetICMProfileA(hdc: ::windef::HDC, pBufSize: ::minwindef::LPDWORD, pszFilename: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wingdi.h:5096:30 */
    pub fn GetICMProfileW(hdc: ::windef::HDC, pBufSize: ::minwindef::LPDWORD, pszFilename: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wingdi.h:5099:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::GetLogColorSpaceW as GetLogColorSpace; /* wingdi.h:5082:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn GetLogColorSpaceA(hColorSpace: ::windef::HCOLORSPACE, lpBuffer: ::wingdi::LPLOGCOLORSPACEA, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5075:30 */
    pub fn GetLogColorSpaceW(hColorSpace: ::windef::HCOLORSPACE, lpBuffer: ::wingdi::LPLOGCOLORSPACEW, nSize: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5078:30 */
    pub fn GetTextCharset(hdc: ::windef::HDC) -> ::libc::c_int; /* wingdi.h:4134:22 */
    pub fn GetTextCharsetInfo(hdc: ::windef::HDC, lpSig: ::wingdi::LPFONTSIGNATURE, dwFlags: ::minwindef::DWORD) -> ::libc::c_int; /* wingdi.h:4135:22 */
    pub fn SetColorSpace(hdc: ::windef::HDC, hcs: ::windef::HCOLORSPACE) -> ::windef::HCOLORSPACE; /* wingdi.h:5094:30 */
    pub fn SetDeviceGammaRamp(hdc: ::windef::HDC, lpRamp: ::minwindef::LPVOID) -> ::minwindef::BOOL; /* wingdi.h:5116:30 */
    pub fn SetICMMode(hdc: ::windef::HDC, mode: ::libc::c_int) -> ::libc::c_int; /* wingdi.h:5068:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::SetICMProfileW as SetICMProfile; /* wingdi.h:5111:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn SetICMProfileA(hdc: ::windef::HDC, lpFileName: ::winnt::LPSTR) -> ::minwindef::BOOL; /* wingdi.h:5108:30 */
    pub fn SetICMProfileW(hdc: ::windef::HDC, lpFileName: ::winnt::LPWSTR) -> ::minwindef::BOOL; /* wingdi.h:5109:30 */
    pub fn TranslateCharsetInfo(lpSrc: *mut ::libc::c_ulong, lpCs: ::wingdi::LPCHARSETINFO, dwFlags: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:4136:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use self::UpdateICMRegKeyW as UpdateICMRegKey; /* wingdi.h:5130:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] 
extern "system" {
    pub fn UpdateICMRegKeyA(reserved: ::minwindef::DWORD, lpszCMID: ::winnt::LPSTR, lpszFileName: ::winnt::LPSTR, command: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:5126:30 */
    pub fn UpdateICMRegKeyW(reserved: ::minwindef::DWORD, lpszCMID: ::winnt::LPWSTR, lpszFileName: ::winnt::LPWSTR, command: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:5128:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddFontMemResourceEx(pFileView: ::winnt::PVOID, cjSize: ::minwindef::DWORD, pvResrved: ::winnt::PVOID, pNumFonts: *mut ::libc::c_ulong) -> ::winnt::HANDLE; /* wingdi.h:4239:25 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::AddFontResourceExW as AddFontResourceEx; /* wingdi.h:4228:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn AddFontResourceExA(name: ::winnt::LPCSTR, fl: ::minwindef::DWORD, res: ::winnt::PVOID) -> ::libc::c_int; /* wingdi.h:4225:23 */
    pub fn AddFontResourceExW(name: ::winnt::LPCWSTR, fl: ::minwindef::DWORD, res: ::winnt::PVOID) -> ::libc::c_int; /* wingdi.h:4226:23 */
    pub fn ColorCorrectPalette(hdc: ::windef::HDC, hPal: ::windef::HPALETTE, deFirst: ::minwindef::DWORD, num: ::minwindef::DWORD) -> ::minwindef::BOOL; /* wingdi.h:5140:30 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::CreateFontIndirectExW as CreateFontIndirectEx; /* wingdi.h:4325:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn CreateFontIndirectExA(_: *const ::wingdi::ENUMLOGFONTEXDVA) -> ::windef::HFONT; /* wingdi.h:4322:25 */
    pub fn CreateFontIndirectExW(_: *const ::wingdi::ENUMLOGFONTEXDVW) -> ::windef::HFONT; /* wingdi.h:4323:25 */
    pub fn GdiAlphaBlend(hdcDest: ::windef::HDC, xoriginDest: ::libc::c_int, yoriginDest: ::libc::c_int, wDest: ::libc::c_int, hDest: ::libc::c_int, hdcSrc: ::windef::HDC, xoriginSrc: ::libc::c_int, yoriginSrc: ::libc::c_int, wSrc: ::libc::c_int, hSrc: ::libc::c_int, ftn: ::wingdi::BLENDFUNCTION) -> ::minwindef::BOOL; /* wingdi.h:4634:24 */
    pub fn GdiGradientFill(hdc: ::windef::HDC, pVertex: ::wingdi::PTRIVERTEX, nVertex: ::minwindef::ULONG, pMesh: ::winnt::PVOID, nCount: ::minwindef::ULONG, ulMode: ::minwindef::ULONG) -> ::minwindef::BOOL; /* wingdi.h:4639:24 */
    pub fn GdiTransparentBlt(hdcDest: ::windef::HDC, xoriginDest: ::libc::c_int, yoriginDest: ::libc::c_int, wDest: ::libc::c_int, hDest: ::libc::c_int, hdcSrc: ::windef::HDC, xoriginSrc: ::libc::c_int, yoriginSrc: ::libc::c_int, wSrc: ::libc::c_int, hSrc: ::libc::c_int, crTransparent: ::minwindef::UINT) -> ::minwindef::BOOL; /* wingdi.h:4636:24 */
    pub fn GetCharABCWidthsI(hdc: ::windef::HDC, giFirst: ::minwindef::UINT, cgi: ::minwindef::UINT, pgi: ::minwindef::LPWORD, pabc: ::wingdi::LPABC) -> ::minwindef::BOOL; /* wingdi.h:4204:24 */
    pub fn GetCharWidthI(hdc: ::windef::HDC, giFirst: ::minwindef::UINT, cgi: ::minwindef::UINT, pgi: ::minwindef::LPWORD, piWidths: ::minwindef::LPINT) -> ::minwindef::BOOL; /* wingdi.h:4197:24 */
    pub fn GetDCBrushColor(hdc: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3885:27 */
    pub fn GetDCPenColor(hdc: ::windef::HDC) -> ::windef::COLORREF; /* wingdi.h:3886:27 */
    pub fn GetFontUnicodeRanges(hdc: ::windef::HDC, lpgs: ::wingdi::LPGLYPHSET) -> ::minwindef::DWORD; /* wingdi.h:4179:24 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::GetGlyphIndicesW as GetGlyphIndices; /* wingdi.h:4183:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn GetGlyphIndicesA(hdc: ::windef::HDC, lpstr: ::winnt::LPCSTR, c: ::libc::c_int, pgi: ::minwindef::LPWORD, fl: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4180:24 */
    pub fn GetGlyphIndicesW(hdc: ::windef::HDC, lpstr: ::winnt::LPCWSTR, c: ::libc::c_int, pgi: ::minwindef::LPWORD, fl: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4181:24 */
    pub fn GetLayout(hdc: ::windef::HDC) -> ::minwindef::DWORD; /* wingdi.h:4437:24 */
    pub fn GetTextExtentExPointI(hdc: ::windef::HDC, lpwszString: ::minwindef::LPWORD, cwchString: ::libc::c_int, nMaxExtent: ::libc::c_int, lpnFit: ::minwindef::LPINT, lpnDx: ::minwindef::LPINT, lpSize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4188:24 */
    pub fn GetTextExtentPointI(hdc: ::windef::HDC, pgiIn: ::minwindef::LPWORD, cgi: ::libc::c_int, psize: ::windef::LPSIZE) -> ::minwindef::BOOL; /* wingdi.h:4187:24 */
    pub fn RemoveFontMemResourceEx(h: ::winnt::HANDLE) -> ::minwindef::BOOL; /* wingdi.h:4244:23 */
}
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use self::RemoveFontResourceExW as RemoveFontResourceEx; /* wingdi.h:4235:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] 
extern "system" {
    pub fn RemoveFontResourceExA(name: ::winnt::LPCSTR, fl: ::minwindef::DWORD, pdv: ::winnt::PVOID) -> ::minwindef::BOOL; /* wingdi.h:4232:23 */
    pub fn RemoveFontResourceExW(name: ::winnt::LPCWSTR, fl: ::minwindef::DWORD, pdv: ::winnt::PVOID) -> ::minwindef::BOOL; /* wingdi.h:4233:23 */
    pub fn SetDCBrushColor(hdc: ::windef::HDC, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4414:27 */
    pub fn SetDCPenColor(hdc: ::windef::HDC, color: ::windef::COLORREF) -> ::windef::COLORREF; /* wingdi.h:4415:27 */
    pub fn SetLayout(hdc: ::windef::HDC, l: ::minwindef::DWORD) -> ::minwindef::DWORD; /* wingdi.h:4436:25 */
}
