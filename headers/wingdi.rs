#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct DRAWPATRECT { ptPosition: ::windef::POINT, ptSize: ::windef::POINT, wStyle: ::minwindef::WORD, wPattern: ::minwindef::WORD } /* wingdi.h:289:16, wingdi.h:289:16, wingdi.h:289:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PDRAWPATRECT = *mut ::wingdi::DRAWPATRECT; /* wingdi.h:294:17, wingdi.h:294:17, wingdi.h:294:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct PSINJECTDATA { DataBytes: ::minwindef::DWORD, InjectionPoint: ::minwindef::WORD, PageNumber: ::minwindef::WORD } /* wingdi.h:421:16, wingdi.h:421:16, wingdi.h:421:16 */
#[cfg(feature="winapi_desktop")] pub type PPSINJECTDATA = *mut ::wingdi::PSINJECTDATA; /* wingdi.h:429:18, wingdi.h:429:18, wingdi.h:429:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct PSFEATURE_OUTPUT { bPageIndependent: ::minwindef::BOOL, bSetPageDevice: ::minwindef::BOOL } /* wingdi.h:509:16, wingdi.h:509:16, wingdi.h:509:16 */
#[cfg(feature="winapi_desktop")] pub type PPSFEATURE_OUTPUT = *mut ::wingdi::PSFEATURE_OUTPUT; /* wingdi.h:514:22, wingdi.h:514:22, wingdi.h:514:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct PSFEATURE_CUSTPAPER { lOrientation: ::winnt::LONG, lWidth: ::winnt::LONG, lHeight: ::winnt::LONG, lWidthOffset: ::winnt::LONG, lHeightOffset: ::winnt::LONG } /* wingdi.h:520:16, wingdi.h:520:16, wingdi.h:520:16 */
#[cfg(feature="winapi_desktop")] pub type PPSFEATURE_CUSTPAPER = *mut ::wingdi::PSFEATURE_CUSTPAPER; /* wingdi.h:528:25, wingdi.h:528:25, wingdi.h:528:25 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct XFORM { eM11: ::minwindef::FLOAT, eM12: ::minwindef::FLOAT, eM21: ::minwindef::FLOAT, eM22: ::minwindef::FLOAT, eDx: ::minwindef::FLOAT, eDy: ::minwindef::FLOAT } /* wingdi.h:586:17, wingdi.h:586:17, wingdi.h:586:17 */
#[cfg(feature="winapi_app")] pub type PXFORM = *mut ::wingdi::XFORM; /* wingdi.h:594:13, wingdi.h:594:13, wingdi.h:594:13 */
#[cfg(feature="winapi_app")] pub type LPXFORM = *mut ::wingdi::XFORM; /* wingdi.h:594:26, wingdi.h:594:26, wingdi.h:594:26 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct BITMAP { bmType: ::winnt::LONG, bmWidth: ::winnt::LONG, bmHeight: ::winnt::LONG, bmWidthBytes: ::winnt::LONG, bmPlanes: ::minwindef::WORD, bmBitsPixel: ::minwindef::WORD, bmBits: ::minwindef::LPVOID } /* wingdi.h:597:16, wingdi.h:597:16, wingdi.h:597:16 */
#[cfg(feature="winapi_app")] pub type PBITMAP = *mut ::wingdi::BITMAP; /* wingdi.h:606:14, wingdi.h:606:14, wingdi.h:606:14 */
#[cfg(feature="winapi_app")] pub type NPBITMAP = *mut ::wingdi::BITMAP; /* wingdi.h:606:29, wingdi.h:606:29, wingdi.h:606:29 */
#[cfg(feature="winapi_app")] pub type LPBITMAP = *mut ::wingdi::BITMAP; /* wingdi.h:606:44, wingdi.h:606:44, wingdi.h:606:44 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RGBTRIPLE { rgbtBlue: ::minwindef::BYTE, rgbtGreen: ::minwindef::BYTE, rgbtRed: ::minwindef::BYTE } /* wingdi.h:616:16, wingdi.h:616:16, wingdi.h:616:16 */
#[cfg(feature="winapi_app")] pub type PRGBTRIPLE = *mut ::wingdi::RGBTRIPLE; /* wingdi.h:620:15, wingdi.h:620:15, wingdi.h:620:15 */
#[cfg(feature="winapi_app")] pub type NPRGBTRIPLE = *mut ::wingdi::RGBTRIPLE; /* wingdi.h:620:33, wingdi.h:620:33, wingdi.h:620:33 */
#[cfg(feature="winapi_app")] pub type LPRGBTRIPLE = *mut ::wingdi::RGBTRIPLE; /* wingdi.h:620:51, wingdi.h:620:51, wingdi.h:620:51 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RGBQUAD { rgbBlue: ::minwindef::BYTE, rgbGreen: ::minwindef::BYTE, rgbRed: ::minwindef::BYTE, rgbReserved: ::minwindef::BYTE } /* wingdi.h:630:16, wingdi.h:630:16, wingdi.h:630:16 */
#[cfg(feature="winapi_desktop")] pub type LPRGBQUAD = *mut ::wingdi::RGBQUAD; /* wingdi.h:643:22, wingdi.h:643:22, wingdi.h:643:22 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LCSCSTYPE = ::winnt::LONG; /* wingdi.h:668:16, wingdi.h:668:16, wingdi.h:668:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LCSGAMUTMATCH = ::winnt::LONG; /* wingdi.h:672:17, wingdi.h:672:17, wingdi.h:672:17 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type FXPT16DOT16 = ::libc::c_long; /* wingdi.h:700:25, wingdi.h:700:25, wingdi.h:700:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPFXPT16DOT16 = *mut ::libc::c_long; /* wingdi.h:700:43, wingdi.h:700:43, wingdi.h:700:43 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type FXPT2DOT30 = ::libc::c_long; /* wingdi.h:701:25, wingdi.h:701:25, wingdi.h:701:25 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPFXPT2DOT30 = *mut ::libc::c_long; /* wingdi.h:701:42, wingdi.h:701:42, wingdi.h:701:42 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CIEXYZ { ciexyzX: ::wingdi::FXPT2DOT30, ciexyzY: ::wingdi::FXPT2DOT30, ciexyzZ: ::wingdi::FXPT2DOT30 } /* wingdi.h:706:16, wingdi.h:706:16, wingdi.h:706:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCIEXYZ = *mut ::wingdi::CIEXYZ; /* wingdi.h:719:22, wingdi.h:719:22, wingdi.h:719:22 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CIEXYZTRIPLE { ciexyzRed: ::wingdi::CIEXYZ, ciexyzGreen: ::wingdi::CIEXYZ, ciexyzBlue: ::wingdi::CIEXYZ } /* wingdi.h:727:16, wingdi.h:727:16, wingdi.h:727:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCIEXYZTRIPLE = *mut ::wingdi::CIEXYZTRIPLE; /* wingdi.h:740:30, wingdi.h:740:30, wingdi.h:740:30 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct LOGCOLORSPACEA { lcsSignature: ::minwindef::DWORD, lcsVersion: ::minwindef::DWORD, lcsSize: ::minwindef::DWORD, lcsCSType: ::wingdi::LCSCSTYPE, lcsIntent: ::wingdi::LCSGAMUTMATCH, lcsEndpoints: ::wingdi::CIEXYZTRIPLE, lcsGammaRed: ::minwindef::DWORD, lcsGammaGreen: ::minwindef::DWORD, lcsGammaBlue: ::minwindef::DWORD, lcsFilename: *mut [::winnt::CHAR; 260] } /* wingdi.h:753:16, wingdi.h:753:16, wingdi.h:753:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPLOGCOLORSPACEA = *mut ::wingdi::LOGCOLORSPACEA; /* wingdi.h:764:20, wingdi.h:764:20, wingdi.h:764:20 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct LOGCOLORSPACEW { lcsSignature: ::minwindef::DWORD, lcsVersion: ::minwindef::DWORD, lcsSize: ::minwindef::DWORD, lcsCSType: ::wingdi::LCSCSTYPE, lcsIntent: ::wingdi::LCSGAMUTMATCH, lcsEndpoints: ::wingdi::CIEXYZTRIPLE, lcsGammaRed: ::minwindef::DWORD, lcsGammaGreen: ::minwindef::DWORD, lcsGammaBlue: ::minwindef::DWORD, lcsFilename: *mut [::winnt::WCHAR; 260] } /* wingdi.h:765:16, wingdi.h:765:16, wingdi.h:765:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPLOGCOLORSPACEW = *mut ::wingdi::LOGCOLORSPACEW; /* wingdi.h:776:20, wingdi.h:776:20, wingdi.h:776:20 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LOGCOLORSPACE = ::wingdi::LOGCOLORSPACEW; /* wingdi.h:778:24, wingdi.h:778:24, wingdi.h:778:24 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPLOGCOLORSPACE = ::wingdi::LPLOGCOLORSPACEW; /* wingdi.h:779:26, wingdi.h:779:26, wingdi.h:779:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct BITMAPCOREHEADER { bcSize: ::minwindef::DWORD, bcWidth: ::minwindef::WORD, bcHeight: ::minwindef::WORD, bcPlanes: ::minwindef::WORD, bcBitCount: ::minwindef::WORD } /* wingdi.h:794:16, wingdi.h:794:16, wingdi.h:794:16 */
#[cfg(feature="winapi_desktop")] pub type LPBITMAPCOREHEADER = *mut ::wingdi::BITMAPCOREHEADER; /* wingdi.h:800:26, wingdi.h:800:26, wingdi.h:800:26 */
#[cfg(feature="winapi_desktop")] pub type PBITMAPCOREHEADER = *mut ::wingdi::BITMAPCOREHEADER; /* wingdi.h:800:47, wingdi.h:800:47, wingdi.h:800:47 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct BITMAPINFOHEADER { biSize: ::minwindef::DWORD, biWidth: ::winnt::LONG, biHeight: ::winnt::LONG, biPlanes: ::minwindef::WORD, biBitCount: ::minwindef::WORD, biCompression: ::minwindef::DWORD, biSizeImage: ::minwindef::DWORD, biXPelsPerMeter: ::winnt::LONG, biYPelsPerMeter: ::winnt::LONG, biClrUsed: ::minwindef::DWORD, biClrImportant: ::minwindef::DWORD } /* wingdi.h:808:16, wingdi.h:808:16, wingdi.h:808:16 */
#[cfg(feature="winapi_app")] pub type LPBITMAPINFOHEADER = *mut ::wingdi::BITMAPINFOHEADER; /* wingdi.h:820:26, wingdi.h:820:26, wingdi.h:820:26 */
#[cfg(feature="winapi_app")] pub type PBITMAPINFOHEADER = *mut ::wingdi::BITMAPINFOHEADER; /* wingdi.h:820:47, wingdi.h:820:47, wingdi.h:820:47 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct BITMAPV4HEADER { bV4Size: ::minwindef::DWORD, bV4Width: ::winnt::LONG, bV4Height: ::winnt::LONG, bV4Planes: ::minwindef::WORD, bV4BitCount: ::minwindef::WORD, bV4V4Compression: ::minwindef::DWORD, bV4SizeImage: ::minwindef::DWORD, bV4XPelsPerMeter: ::winnt::LONG, bV4YPelsPerMeter: ::winnt::LONG, bV4ClrUsed: ::minwindef::DWORD, bV4ClrImportant: ::minwindef::DWORD, bV4RedMask: ::minwindef::DWORD, bV4GreenMask: ::minwindef::DWORD, bV4BlueMask: ::minwindef::DWORD, bV4AlphaMask: ::minwindef::DWORD, bV4CSType: ::minwindef::DWORD, bV4Endpoints: ::wingdi::CIEXYZTRIPLE, bV4GammaRed: ::minwindef::DWORD, bV4GammaGreen: ::minwindef::DWORD, bV4GammaBlue: ::minwindef::DWORD } /* wingdi.h:830:9, wingdi.h:830:9, wingdi.h:830:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPBITMAPV4HEADER = *mut ::wingdi::BITMAPV4HEADER; /* wingdi.h:851:24, wingdi.h:851:24, wingdi.h:851:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PBITMAPV4HEADER = *mut ::wingdi::BITMAPV4HEADER; /* wingdi.h:851:43, wingdi.h:851:43, wingdi.h:851:43 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct BITMAPV5HEADER { bV5Size: ::minwindef::DWORD, bV5Width: ::winnt::LONG, bV5Height: ::winnt::LONG, bV5Planes: ::minwindef::WORD, bV5BitCount: ::minwindef::WORD, bV5Compression: ::minwindef::DWORD, bV5SizeImage: ::minwindef::DWORD, bV5XPelsPerMeter: ::winnt::LONG, bV5YPelsPerMeter: ::winnt::LONG, bV5ClrUsed: ::minwindef::DWORD, bV5ClrImportant: ::minwindef::DWORD, bV5RedMask: ::minwindef::DWORD, bV5GreenMask: ::minwindef::DWORD, bV5BlueMask: ::minwindef::DWORD, bV5AlphaMask: ::minwindef::DWORD, bV5CSType: ::minwindef::DWORD, bV5Endpoints: ::wingdi::CIEXYZTRIPLE, bV5GammaRed: ::minwindef::DWORD, bV5GammaGreen: ::minwindef::DWORD, bV5GammaBlue: ::minwindef::DWORD, bV5Intent: ::minwindef::DWORD, bV5ProfileData: ::minwindef::DWORD, bV5ProfileSize: ::minwindef::DWORD, bV5Reserved: ::minwindef::DWORD } /* wingdi.h:861:9, wingdi.h:861:9, wingdi.h:861:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPBITMAPV5HEADER = *mut ::wingdi::BITMAPV5HEADER; /* wingdi.h:886:24, wingdi.h:886:24, wingdi.h:886:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PBITMAPV5HEADER = *mut ::wingdi::BITMAPV5HEADER; /* wingdi.h:886:43, wingdi.h:886:43, wingdi.h:886:43 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct BITMAPINFO { bmiHeader: ::wingdi::BITMAPINFOHEADER, bmiColors: *mut [::wingdi::RGBQUAD; 1] } /* wingdi.h:909:16, wingdi.h:909:16, wingdi.h:909:16 */
#[cfg(feature="winapi_app")] pub type LPBITMAPINFO = *mut ::wingdi::BITMAPINFO; /* wingdi.h:912:20, wingdi.h:912:20, wingdi.h:912:20 */
#[cfg(feature="winapi_app")] pub type PBITMAPINFO = *mut ::wingdi::BITMAPINFO; /* wingdi.h:912:35, wingdi.h:912:35, wingdi.h:912:35 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct BITMAPCOREINFO { bmciHeader: ::wingdi::BITMAPCOREHEADER, bmciColors: *mut [::wingdi::RGBTRIPLE; 1] } /* wingdi.h:920:16, wingdi.h:920:16, wingdi.h:920:16 */
#[cfg(feature="winapi_desktop")] pub type LPBITMAPCOREINFO = *mut ::wingdi::BITMAPCOREINFO; /* wingdi.h:923:24, wingdi.h:923:24, wingdi.h:923:24 */
#[cfg(feature="winapi_desktop")] pub type PBITMAPCOREINFO = *mut ::wingdi::BITMAPCOREINFO; /* wingdi.h:923:43, wingdi.h:923:43, wingdi.h:923:43 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct BITMAPFILEHEADER { bfType: ::minwindef::WORD, bfSize: ::minwindef::DWORD, bfReserved1: ::minwindef::WORD, bfReserved2: ::minwindef::WORD, bfOffBits: ::minwindef::DWORD } /* wingdi.h:933:16, wingdi.h:933:16, wingdi.h:933:16 */
#[cfg(feature="winapi_desktop")] pub type LPBITMAPFILEHEADER = *mut ::wingdi::BITMAPFILEHEADER; /* wingdi.h:939:26, wingdi.h:939:26, wingdi.h:939:26 */
#[cfg(feature="winapi_desktop")] pub type PBITMAPFILEHEADER = *mut ::wingdi::BITMAPFILEHEADER; /* wingdi.h:939:47, wingdi.h:939:47, wingdi.h:939:47 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct FONTSIGNATURE { fsUsb: *mut [::minwindef::DWORD; 4], fsCsb: *mut [::minwindef::DWORD; 2] } /* wingdi.h:954:16, wingdi.h:954:16, wingdi.h:954:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PFONTSIGNATURE = *mut ::wingdi::FONTSIGNATURE; /* wingdi.h:958:19, wingdi.h:958:19, wingdi.h:958:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPFONTSIGNATURE = *mut ::wingdi::FONTSIGNATURE; /* wingdi.h:958:39, wingdi.h:958:39, wingdi.h:958:39 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct CHARSETINFO { ciCharset: ::minwindef::UINT, ciACP: ::minwindef::UINT, fs: ::wingdi::FONTSIGNATURE } /* wingdi.h:960:16, wingdi.h:960:16, wingdi.h:960:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PCHARSETINFO = *mut ::wingdi::CHARSETINFO; /* wingdi.h:965:17, wingdi.h:965:17, wingdi.h:965:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NPCHARSETINFO = *mut ::wingdi::CHARSETINFO; /* wingdi.h:965:37, wingdi.h:965:37, wingdi.h:965:37 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPCHARSETINFO = *mut ::wingdi::CHARSETINFO; /* wingdi.h:965:57, wingdi.h:965:57, wingdi.h:965:57 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct LOCALESIGNATURE { lsUsb: *mut [::minwindef::DWORD; 4], lsCsbDefault: *mut [::minwindef::DWORD; 2], lsCsbSupported: *mut [::minwindef::DWORD; 2] } /* wingdi.h:980:16, wingdi.h:980:16, wingdi.h:980:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PLOCALESIGNATURE = *mut ::wingdi::LOCALESIGNATURE; /* wingdi.h:985:21, wingdi.h:985:21, wingdi.h:985:21 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPLOCALESIGNATURE = *mut ::wingdi::LOCALESIGNATURE; /* wingdi.h:985:43, wingdi.h:985:43, wingdi.h:985:43 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct HANDLETABLE { objectHandle: *mut [::windef::HGDIOBJ; 1] } /* wingdi.h:1000:16, wingdi.h:1000:16, wingdi.h:1000:16 */
#[cfg(feature="winapi_app")] pub type PHANDLETABLE = *mut ::wingdi::HANDLETABLE; /* wingdi.h:1003:19, wingdi.h:1003:19, wingdi.h:1003:19 */
#[cfg(feature="winapi_app")] pub type LPHANDLETABLE = *mut ::wingdi::HANDLETABLE; /* wingdi.h:1003:38, wingdi.h:1003:38, wingdi.h:1003:38 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct METARECORD { rdSize: ::minwindef::DWORD, rdFunction: ::minwindef::WORD, rdParm: *mut [::minwindef::WORD; 1] } /* wingdi.h:1005:16, wingdi.h:1005:16, wingdi.h:1005:16 */
#[cfg(feature="winapi_desktop")] pub type PMETARECORD = *mut ::wingdi::METARECORD; /* wingdi.h:1018:41, wingdi.h:1018:41, wingdi.h:1018:41 */
#[cfg(feature="winapi_app")] pub type LPMETARECORD = *mut ::wingdi::METARECORD; /* wingdi.h:1026:45, wingdi.h:1026:45, wingdi.h:1026:45 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct METAFILEPICT { mm: ::winnt::LONG, xExt: ::winnt::LONG, yExt: ::winnt::LONG, hMF: ::minwindef::HMETAFILE } /* wingdi.h:1028:16, wingdi.h:1028:16, wingdi.h:1028:16 */
#[cfg(feature="winapi_app")] pub type LPMETAFILEPICT = *mut ::wingdi::METAFILEPICT; /* wingdi.h:1034:24, wingdi.h:1034:24, wingdi.h:1034:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct METAHEADER { mtType: ::minwindef::WORD, mtHeaderSize: ::minwindef::WORD, mtVersion: ::minwindef::WORD, mtSize: ::minwindef::DWORD, mtNoObjects: ::minwindef::WORD, mtMaxRecord: ::minwindef::DWORD, mtNoParameters: ::minwindef::WORD } /* wingdi.h:1044:16, wingdi.h:1044:16, wingdi.h:1044:16 */
#[cfg(feature="winapi_desktop")] pub type PMETAHEADER = *mut ::wingdi::METAHEADER; /* wingdi.h:1054:41, wingdi.h:1054:41, wingdi.h:1054:41 */
#[cfg(feature="winapi_desktop")] pub type LPMETAHEADER = *mut ::wingdi::METAHEADER; /* wingdi.h:1055:45, wingdi.h:1055:45, wingdi.h:1055:45 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct ENHMETARECORD { iType: ::minwindef::DWORD, nSize: ::minwindef::DWORD, dParm: *mut [::minwindef::DWORD; 1] } /* wingdi.h:1066:16, wingdi.h:1066:16, wingdi.h:1066:16 */
#[cfg(feature="winapi_app")] pub type PENHMETARECORD = *mut ::wingdi::ENHMETARECORD; /* wingdi.h:1071:19, wingdi.h:1071:19, wingdi.h:1071:19 */
#[cfg(feature="winapi_app")] pub type LPENHMETARECORD = *mut ::wingdi::ENHMETARECORD; /* wingdi.h:1071:36, wingdi.h:1071:36, wingdi.h:1071:36 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct ENHMETAHEADER { iType: ::minwindef::DWORD, nSize: ::minwindef::DWORD, rclBounds: ::windef::RECTL, rclFrame: ::windef::RECTL, dSignature: ::minwindef::DWORD, nVersion: ::minwindef::DWORD, nBytes: ::minwindef::DWORD, nRecords: ::minwindef::DWORD, nHandles: ::minwindef::WORD, sReserved: ::minwindef::WORD, nDescription: ::minwindef::DWORD, offDescription: ::minwindef::DWORD, nPalEntries: ::minwindef::DWORD, szlDevice: ::windef::SIZEL, szlMillimeters: ::windef::SIZEL, cbPixelFormat: ::minwindef::DWORD, offPixelFormat: ::minwindef::DWORD, bOpenGL: ::minwindef::DWORD, szlMicrometers: ::windef::SIZEL } /* wingdi.h:1073:16, wingdi.h:1073:16, wingdi.h:1073:16 */
#[cfg(feature="winapi_app")] pub type PENHMETAHEADER = *mut ::wingdi::ENHMETAHEADER; /* wingdi.h:1106:19, wingdi.h:1106:19, wingdi.h:1106:19 */
#[cfg(feature="winapi_app")] pub type LPENHMETAHEADER = *mut ::wingdi::ENHMETAHEADER; /* wingdi.h:1106:36, wingdi.h:1106:36, wingdi.h:1106:36 */
#[cfg(feature="winapi_desktop")] pub type BCHAR = ::winnt::WCHAR; /* wingdi.h:1128:19, wingdi.h:1128:19, wingdi.h:1128:19 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct TEXTMETRICA { tmHeight: ::winnt::LONG, tmAscent: ::winnt::LONG, tmDescent: ::winnt::LONG, tmInternalLeading: ::winnt::LONG, tmExternalLeading: ::winnt::LONG, tmAveCharWidth: ::winnt::LONG, tmMaxCharWidth: ::winnt::LONG, tmWeight: ::winnt::LONG, tmOverhang: ::winnt::LONG, tmDigitizedAspectX: ::winnt::LONG, tmDigitizedAspectY: ::winnt::LONG, tmFirstChar: ::minwindef::BYTE, tmLastChar: ::minwindef::BYTE, tmDefaultChar: ::minwindef::BYTE, tmBreakChar: ::minwindef::BYTE, tmItalic: ::minwindef::BYTE, tmUnderlined: ::minwindef::BYTE, tmStruckOut: ::minwindef::BYTE, tmPitchAndFamily: ::minwindef::BYTE, tmCharSet: ::minwindef::BYTE } /* wingdi.h:1143:16, wingdi.h:1143:16, wingdi.h:1143:16 */
#[cfg(feature="winapi_app")] pub type PTEXTMETRICA = *mut ::wingdi::TEXTMETRICA; /* wingdi.h:1165:17, wingdi.h:1165:17, wingdi.h:1165:17 */
#[cfg(feature="winapi_app")] pub type NPTEXTMETRICA = *mut ::wingdi::TEXTMETRICA; /* wingdi.h:1165:37, wingdi.h:1165:37, wingdi.h:1165:37 */
#[cfg(feature="winapi_app")] pub type LPTEXTMETRICA = *mut ::wingdi::TEXTMETRICA; /* wingdi.h:1165:57, wingdi.h:1165:57, wingdi.h:1165:57 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct TEXTMETRICW { tmHeight: ::winnt::LONG, tmAscent: ::winnt::LONG, tmDescent: ::winnt::LONG, tmInternalLeading: ::winnt::LONG, tmExternalLeading: ::winnt::LONG, tmAveCharWidth: ::winnt::LONG, tmMaxCharWidth: ::winnt::LONG, tmWeight: ::winnt::LONG, tmOverhang: ::winnt::LONG, tmDigitizedAspectX: ::winnt::LONG, tmDigitizedAspectY: ::winnt::LONG, tmFirstChar: ::winnt::WCHAR, tmLastChar: ::winnt::WCHAR, tmDefaultChar: ::winnt::WCHAR, tmBreakChar: ::winnt::WCHAR, tmItalic: ::minwindef::BYTE, tmUnderlined: ::minwindef::BYTE, tmStruckOut: ::minwindef::BYTE, tmPitchAndFamily: ::minwindef::BYTE, tmCharSet: ::minwindef::BYTE } /* wingdi.h:1166:16, wingdi.h:1166:16, wingdi.h:1166:16 */
#[cfg(feature="winapi_app")] pub type PTEXTMETRICW = *mut ::wingdi::TEXTMETRICW; /* wingdi.h:1188:17, wingdi.h:1188:17, wingdi.h:1188:17 */
#[cfg(feature="winapi_app")] pub type NPTEXTMETRICW = *mut ::wingdi::TEXTMETRICW; /* wingdi.h:1188:37, wingdi.h:1188:37, wingdi.h:1188:37 */
#[cfg(feature="winapi_app")] pub type LPTEXTMETRICW = *mut ::wingdi::TEXTMETRICW; /* wingdi.h:1188:57, wingdi.h:1188:57, wingdi.h:1188:57 */
#[cfg(feature="winapi_app")] pub type TEXTMETRIC = ::wingdi::TEXTMETRICW; /* wingdi.h:1190:21, wingdi.h:1190:21, wingdi.h:1190:21 */
#[cfg(feature="winapi_app")] pub type PTEXTMETRIC = ::wingdi::PTEXTMETRICW; /* wingdi.h:1191:22, wingdi.h:1191:22, wingdi.h:1191:22 */
#[cfg(feature="winapi_app")] pub type NPTEXTMETRIC = ::wingdi::NPTEXTMETRICW; /* wingdi.h:1192:23, wingdi.h:1192:23, wingdi.h:1192:23 */
#[cfg(feature="winapi_app")] pub type LPTEXTMETRIC = ::wingdi::LPTEXTMETRICW; /* wingdi.h:1193:23, wingdi.h:1193:23, wingdi.h:1193:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NEWTEXTMETRICA { tmHeight: ::winnt::LONG, tmAscent: ::winnt::LONG, tmDescent: ::winnt::LONG, tmInternalLeading: ::winnt::LONG, tmExternalLeading: ::winnt::LONG, tmAveCharWidth: ::winnt::LONG, tmMaxCharWidth: ::winnt::LONG, tmWeight: ::winnt::LONG, tmOverhang: ::winnt::LONG, tmDigitizedAspectX: ::winnt::LONG, tmDigitizedAspectY: ::winnt::LONG, tmFirstChar: ::minwindef::BYTE, tmLastChar: ::minwindef::BYTE, tmDefaultChar: ::minwindef::BYTE, tmBreakChar: ::minwindef::BYTE, tmItalic: ::minwindef::BYTE, tmUnderlined: ::minwindef::BYTE, tmStruckOut: ::minwindef::BYTE, tmPitchAndFamily: ::minwindef::BYTE, tmCharSet: ::minwindef::BYTE, ntmFlags: ::minwindef::DWORD, ntmSizeEM: ::minwindef::UINT, ntmCellHeight: ::minwindef::UINT, ntmAvgWidth: ::minwindef::UINT } /* wingdi.h:1226:16, wingdi.h:1226:16, wingdi.h:1226:16 */
#[cfg(feature="winapi_desktop")] pub type PNEWTEXTMETRICA = *mut ::wingdi::NEWTEXTMETRICA; /* wingdi.h:1252:20, wingdi.h:1252:20, wingdi.h:1252:20 */
#[cfg(feature="winapi_desktop")] pub type NPNEWTEXTMETRICA = *mut ::wingdi::NEWTEXTMETRICA; /* wingdi.h:1252:43, wingdi.h:1252:43, wingdi.h:1252:43 */
#[cfg(feature="winapi_desktop")] pub type LPNEWTEXTMETRICA = *mut ::wingdi::NEWTEXTMETRICA; /* wingdi.h:1252:66, wingdi.h:1252:66, wingdi.h:1252:66 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct NEWTEXTMETRICW { tmHeight: ::winnt::LONG, tmAscent: ::winnt::LONG, tmDescent: ::winnt::LONG, tmInternalLeading: ::winnt::LONG, tmExternalLeading: ::winnt::LONG, tmAveCharWidth: ::winnt::LONG, tmMaxCharWidth: ::winnt::LONG, tmWeight: ::winnt::LONG, tmOverhang: ::winnt::LONG, tmDigitizedAspectX: ::winnt::LONG, tmDigitizedAspectY: ::winnt::LONG, tmFirstChar: ::winnt::WCHAR, tmLastChar: ::winnt::WCHAR, tmDefaultChar: ::winnt::WCHAR, tmBreakChar: ::winnt::WCHAR, tmItalic: ::minwindef::BYTE, tmUnderlined: ::minwindef::BYTE, tmStruckOut: ::minwindef::BYTE, tmPitchAndFamily: ::minwindef::BYTE, tmCharSet: ::minwindef::BYTE, ntmFlags: ::minwindef::DWORD, ntmSizeEM: ::minwindef::UINT, ntmCellHeight: ::minwindef::UINT, ntmAvgWidth: ::minwindef::UINT } /* wingdi.h:1253:16, wingdi.h:1253:16, wingdi.h:1253:16 */
#[cfg(feature="winapi_desktop")] pub type PNEWTEXTMETRICW = *mut ::wingdi::NEWTEXTMETRICW; /* wingdi.h:1279:20, wingdi.h:1279:20, wingdi.h:1279:20 */
#[cfg(feature="winapi_desktop")] pub type NPNEWTEXTMETRICW = *mut ::wingdi::NEWTEXTMETRICW; /* wingdi.h:1279:43, wingdi.h:1279:43, wingdi.h:1279:43 */
#[cfg(feature="winapi_desktop")] pub type LPNEWTEXTMETRICW = *mut ::wingdi::NEWTEXTMETRICW; /* wingdi.h:1279:66, wingdi.h:1279:66, wingdi.h:1279:66 */
#[cfg(feature="winapi_desktop")] pub type NEWTEXTMETRIC = ::wingdi::NEWTEXTMETRICW; /* wingdi.h:1281:24, wingdi.h:1281:24, wingdi.h:1281:24 */
#[cfg(feature="winapi_desktop")] pub type PNEWTEXTMETRIC = ::wingdi::PNEWTEXTMETRICW; /* wingdi.h:1282:25, wingdi.h:1282:25, wingdi.h:1282:25 */
#[cfg(feature="winapi_desktop")] pub type NPNEWTEXTMETRIC = ::wingdi::NPNEWTEXTMETRICW; /* wingdi.h:1283:26, wingdi.h:1283:26, wingdi.h:1283:26 */
#[cfg(feature="winapi_desktop")] pub type LPNEWTEXTMETRIC = ::wingdi::LPNEWTEXTMETRICW; /* wingdi.h:1284:26, wingdi.h:1284:26, wingdi.h:1284:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct NEWTEXTMETRICEXA { ntmTm: ::wingdi::NEWTEXTMETRICA, ntmFontSig: ::wingdi::FONTSIGNATURE } /* wingdi.h:1302:16, wingdi.h:1302:16, wingdi.h:1302:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct NEWTEXTMETRICEXW { ntmTm: ::wingdi::NEWTEXTMETRICW, ntmFontSig: ::wingdi::FONTSIGNATURE } /* wingdi.h:1307:16, wingdi.h:1307:16, wingdi.h:1307:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type NEWTEXTMETRICEX = ::wingdi::NEWTEXTMETRICEXW; /* wingdi.h:1313:26, wingdi.h:1313:26, wingdi.h:1313:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct PELARRAY { paXCount: ::winnt::LONG, paYCount: ::winnt::LONG, paXExt: ::winnt::LONG, paYExt: ::winnt::LONG, paRGBs: ::minwindef::BYTE } /* wingdi.h:1329:16, wingdi.h:1329:16, wingdi.h:1329:16 */
#[cfg(feature="winapi_desktop")] pub type PPELARRAY = *mut ::wingdi::PELARRAY; /* wingdi.h:1336:16, wingdi.h:1336:16, wingdi.h:1336:16 */
#[cfg(feature="winapi_desktop")] pub type NPPELARRAY = *mut ::wingdi::PELARRAY; /* wingdi.h:1336:33, wingdi.h:1336:33, wingdi.h:1336:33 */
#[cfg(feature="winapi_desktop")] pub type LPPELARRAY = *mut ::wingdi::PELARRAY; /* wingdi.h:1336:50, wingdi.h:1336:50, wingdi.h:1336:50 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGBRUSH { lbStyle: ::minwindef::UINT, lbColor: ::windef::COLORREF, lbHatch: ::basetsd::ULONG_PTR } /* wingdi.h:1345:16, wingdi.h:1345:16, wingdi.h:1345:16 */
#[cfg(feature="winapi_app")] pub type PLOGBRUSH = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1350:16, wingdi.h:1350:16, wingdi.h:1350:16 */
#[cfg(feature="winapi_app")] pub type NPLOGBRUSH = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1350:33, wingdi.h:1350:33, wingdi.h:1350:33 */
#[cfg(feature="winapi_app")] pub type LPLOGBRUSH = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1350:50, wingdi.h:1350:50, wingdi.h:1350:50 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGBRUSH32 { lbStyle: ::minwindef::UINT, lbColor: ::windef::COLORREF, lbHatch: ::minwindef::ULONG } /* wingdi.h:1352:16, wingdi.h:1352:16, wingdi.h:1352:16 */
#[cfg(feature="winapi_app")] pub type PLOGBRUSH32 = *mut ::wingdi::LOGBRUSH32; /* wingdi.h:1357:18, wingdi.h:1357:18, wingdi.h:1357:18 */
#[cfg(feature="winapi_app")] pub type NPLOGBRUSH32 = *mut ::wingdi::LOGBRUSH32; /* wingdi.h:1357:37, wingdi.h:1357:37, wingdi.h:1357:37 */
#[cfg(feature="winapi_app")] pub type LPLOGBRUSH32 = *mut ::wingdi::LOGBRUSH32; /* wingdi.h:1357:56, wingdi.h:1357:56, wingdi.h:1357:56 */
#[cfg(feature="winapi_desktop")] pub type PATTERN = ::wingdi::LOGBRUSH; /* wingdi.h:1365:29, wingdi.h:1365:29, wingdi.h:1365:29 */
#[cfg(feature="winapi_desktop")] pub type PPATTERN = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1366:30, wingdi.h:1366:30, wingdi.h:1366:30 */
#[cfg(feature="winapi_desktop")] pub type NPPATTERN = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1367:30, wingdi.h:1367:30, wingdi.h:1367:30 */
#[cfg(feature="winapi_desktop")] pub type LPPATTERN = *mut ::wingdi::LOGBRUSH; /* wingdi.h:1368:30, wingdi.h:1368:30, wingdi.h:1368:30 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGPEN { lopnStyle: ::minwindef::UINT, lopnWidth: ::windef::POINT, lopnColor: ::windef::COLORREF } /* wingdi.h:1377:16, wingdi.h:1377:16, wingdi.h:1377:16 */
#[cfg(feature="winapi_app")] pub type PLOGPEN = *mut ::wingdi::LOGPEN; /* wingdi.h:1382:14, wingdi.h:1382:14, wingdi.h:1382:14 */
#[cfg(feature="winapi_app")] pub type NPLOGPEN = *mut ::wingdi::LOGPEN; /* wingdi.h:1382:29, wingdi.h:1382:29, wingdi.h:1382:29 */
#[cfg(feature="winapi_app")] pub type LPLOGPEN = *mut ::wingdi::LOGPEN; /* wingdi.h:1382:44, wingdi.h:1382:44, wingdi.h:1382:44 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EXTLOGPEN { elpPenStyle: ::minwindef::DWORD, elpWidth: ::minwindef::DWORD, elpBrushStyle: ::minwindef::UINT, elpColor: ::windef::COLORREF, elpHatch: ::basetsd::ULONG_PTR, elpNumEntries: ::minwindef::DWORD, elpStyleEntry: *mut [::minwindef::DWORD; 1] } /* wingdi.h:1390:16, wingdi.h:1390:16, wingdi.h:1390:16 */
#[cfg(feature="winapi_desktop")] pub type PEXTLOGPEN = *mut ::wingdi::EXTLOGPEN; /* wingdi.h:1398:15, wingdi.h:1398:15, wingdi.h:1398:15 */
#[cfg(feature="winapi_desktop")] pub type NPEXTLOGPEN = *mut ::wingdi::EXTLOGPEN; /* wingdi.h:1398:33, wingdi.h:1398:33, wingdi.h:1398:33 */
#[cfg(feature="winapi_desktop")] pub type LPEXTLOGPEN = *mut ::wingdi::EXTLOGPEN; /* wingdi.h:1398:51, wingdi.h:1398:51, wingdi.h:1398:51 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct EXTLOGPEN32 { elpPenStyle: ::minwindef::DWORD, elpWidth: ::minwindef::DWORD, elpBrushStyle: ::minwindef::UINT, elpColor: ::windef::COLORREF, elpHatch: ::minwindef::ULONG, elpNumEntries: ::minwindef::DWORD, elpStyleEntry: *mut [::minwindef::DWORD; 1] } /* wingdi.h:1406:16, wingdi.h:1406:16, wingdi.h:1406:16 */
#[cfg(feature="winapi_app")] pub type PEXTLOGPEN32 = *mut ::wingdi::EXTLOGPEN32; /* wingdi.h:1414:17, wingdi.h:1414:17, wingdi.h:1414:17 */
#[cfg(feature="winapi_app")] pub type NPEXTLOGPEN32 = *mut ::wingdi::EXTLOGPEN32; /* wingdi.h:1414:37, wingdi.h:1414:37, wingdi.h:1414:37 */
#[cfg(feature="winapi_app")] pub type LPEXTLOGPEN32 = *mut ::wingdi::EXTLOGPEN32; /* wingdi.h:1414:57, wingdi.h:1414:57, wingdi.h:1414:57 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct PALETTEENTRY { peRed: ::minwindef::BYTE, peGreen: ::minwindef::BYTE, peBlue: ::minwindef::BYTE, peFlags: ::minwindef::BYTE } /* wingdi.h:1418:16, wingdi.h:1418:16, wingdi.h:1418:16 */
#[cfg(feature="winapi_app")] pub type PPALETTEENTRY = *mut ::wingdi::PALETTEENTRY; /* wingdi.h:1423:18, wingdi.h:1423:18, wingdi.h:1423:18 */
#[cfg(feature="winapi_app")] pub type LPPALETTEENTRY = *mut ::wingdi::PALETTEENTRY; /* wingdi.h:1423:38, wingdi.h:1423:38, wingdi.h:1423:38 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGPALETTE { palVersion: ::minwindef::WORD, palNumEntries: ::minwindef::WORD, palPalEntry: *mut [::wingdi::PALETTEENTRY; 1] } /* wingdi.h:1429:16, wingdi.h:1429:16, wingdi.h:1429:16 */
#[cfg(feature="winapi_app")] pub type PLOGPALETTE = *mut ::wingdi::LOGPALETTE; /* wingdi.h:1433:16, wingdi.h:1433:16, wingdi.h:1433:16 */
#[cfg(feature="winapi_app")] pub type NPLOGPALETTE = *mut ::wingdi::LOGPALETTE; /* wingdi.h:1433:35, wingdi.h:1433:35, wingdi.h:1433:35 */
#[cfg(feature="winapi_app")] pub type LPLOGPALETTE = *mut ::wingdi::LOGPALETTE; /* wingdi.h:1433:54, wingdi.h:1433:54, wingdi.h:1433:54 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGFONTA { lfHeight: ::winnt::LONG, lfWidth: ::winnt::LONG, lfEscapement: ::winnt::LONG, lfOrientation: ::winnt::LONG, lfWeight: ::winnt::LONG, lfItalic: ::minwindef::BYTE, lfUnderline: ::minwindef::BYTE, lfStrikeOut: ::minwindef::BYTE, lfCharSet: ::minwindef::BYTE, lfOutPrecision: ::minwindef::BYTE, lfClipPrecision: ::minwindef::BYTE, lfQuality: ::minwindef::BYTE, lfPitchAndFamily: ::minwindef::BYTE, lfFaceName: *mut [::winnt::CHAR; 32] } /* wingdi.h:1440:16, wingdi.h:1440:16, wingdi.h:1440:16 */
#[cfg(feature="winapi_app")] pub type PLOGFONTA = *mut ::wingdi::LOGFONTA; /* wingdi.h:1456:14, wingdi.h:1456:14, wingdi.h:1456:14 */
#[cfg(feature="winapi_app")] pub type NPLOGFONTA = *mut ::wingdi::LOGFONTA; /* wingdi.h:1456:31, wingdi.h:1456:31, wingdi.h:1456:31 */
#[cfg(feature="winapi_app")] pub type LPLOGFONTA = *mut ::wingdi::LOGFONTA; /* wingdi.h:1456:48, wingdi.h:1456:48, wingdi.h:1456:48 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct LOGFONTW { lfHeight: ::winnt::LONG, lfWidth: ::winnt::LONG, lfEscapement: ::winnt::LONG, lfOrientation: ::winnt::LONG, lfWeight: ::winnt::LONG, lfItalic: ::minwindef::BYTE, lfUnderline: ::minwindef::BYTE, lfStrikeOut: ::minwindef::BYTE, lfCharSet: ::minwindef::BYTE, lfOutPrecision: ::minwindef::BYTE, lfClipPrecision: ::minwindef::BYTE, lfQuality: ::minwindef::BYTE, lfPitchAndFamily: ::minwindef::BYTE, lfFaceName: *mut [::winnt::WCHAR; 32] } /* wingdi.h:1457:16, wingdi.h:1457:16, wingdi.h:1457:16 */
#[cfg(feature="winapi_app")] pub type PLOGFONTW = *mut ::wingdi::LOGFONTW; /* wingdi.h:1473:14, wingdi.h:1473:14, wingdi.h:1473:14 */
#[cfg(feature="winapi_app")] pub type NPLOGFONTW = *mut ::wingdi::LOGFONTW; /* wingdi.h:1473:31, wingdi.h:1473:31, wingdi.h:1473:31 */
#[cfg(feature="winapi_app")] pub type LPLOGFONTW = *mut ::wingdi::LOGFONTW; /* wingdi.h:1473:48, wingdi.h:1473:48, wingdi.h:1473:48 */
#[cfg(feature="winapi_app")] pub type LOGFONT = ::wingdi::LOGFONTW; /* wingdi.h:1475:18, wingdi.h:1475:18, wingdi.h:1475:18 */
#[cfg(feature="winapi_app")] pub type PLOGFONT = ::wingdi::PLOGFONTW; /* wingdi.h:1476:19, wingdi.h:1476:19, wingdi.h:1476:19 */
#[cfg(feature="winapi_app")] pub type NPLOGFONT = ::wingdi::NPLOGFONTW; /* wingdi.h:1477:20, wingdi.h:1477:20, wingdi.h:1477:20 */
#[cfg(feature="winapi_app")] pub type LPLOGFONT = ::wingdi::LPLOGFONTW; /* wingdi.h:1478:20, wingdi.h:1478:20, wingdi.h:1478:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUMLOGFONTA { elfLogFont: ::wingdi::LOGFONTA, elfFullName: *mut [::minwindef::BYTE; 64], elfStyle: *mut [::minwindef::BYTE; 32] } /* wingdi.h:1495:16, wingdi.h:1495:16, wingdi.h:1495:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUMLOGFONTA = *mut ::wingdi::ENUMLOGFONTA; /* wingdi.h:1500:22, wingdi.h:1500:22, wingdi.h:1500:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ENUMLOGFONTW { elfLogFont: ::wingdi::LOGFONTW, elfFullName: *mut [::winnt::WCHAR; 64], elfStyle: *mut [::winnt::WCHAR; 32] } /* wingdi.h:1502:16, wingdi.h:1502:16, wingdi.h:1502:16 */
#[cfg(feature="winapi_desktop")] pub type LPENUMLOGFONTW = *mut ::wingdi::ENUMLOGFONTW; /* wingdi.h:1507:22, wingdi.h:1507:22, wingdi.h:1507:22 */
#[cfg(feature="winapi_desktop")] pub type ENUMLOGFONT = ::wingdi::ENUMLOGFONTW; /* wingdi.h:1509:22, wingdi.h:1509:22, wingdi.h:1509:22 */
#[cfg(feature="winapi_desktop")] pub type LPENUMLOGFONT = ::wingdi::LPENUMLOGFONTW; /* wingdi.h:1510:24, wingdi.h:1510:24, wingdi.h:1510:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct ENUMLOGFONTEXA { elfLogFont: ::wingdi::LOGFONTA, elfFullName: *mut [::minwindef::BYTE; 64], elfStyle: *mut [::minwindef::BYTE; 32], elfScript: *mut [::minwindef::BYTE; 32] } /* wingdi.h:1517:16, wingdi.h:1517:16, wingdi.h:1517:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPENUMLOGFONTEXA = *mut ::wingdi::ENUMLOGFONTEXA; /* wingdi.h:1523:24, wingdi.h:1523:24, wingdi.h:1523:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct ENUMLOGFONTEXW { elfLogFont: ::wingdi::LOGFONTW, elfFullName: *mut [::winnt::WCHAR; 64], elfStyle: *mut [::winnt::WCHAR; 32], elfScript: *mut [::winnt::WCHAR; 32] } /* wingdi.h:1524:16, wingdi.h:1524:16, wingdi.h:1524:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPENUMLOGFONTEXW = *mut ::wingdi::ENUMLOGFONTEXW; /* wingdi.h:1530:24, wingdi.h:1530:24, wingdi.h:1530:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type ENUMLOGFONTEX = ::wingdi::ENUMLOGFONTEXW; /* wingdi.h:1532:24, wingdi.h:1532:24, wingdi.h:1532:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPENUMLOGFONTEX = ::wingdi::LPENUMLOGFONTEXW; /* wingdi.h:1533:26, wingdi.h:1533:26, wingdi.h:1533:26 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct PANOSE { bFamilyType: ::minwindef::BYTE, bSerifStyle: ::minwindef::BYTE, bWeight: ::minwindef::BYTE, bProportion: ::minwindef::BYTE, bContrast: ::minwindef::BYTE, bStrokeVariation: ::minwindef::BYTE, bArmStyle: ::minwindef::BYTE, bLetterform: ::minwindef::BYTE, bMidline: ::minwindef::BYTE, bXHeight: ::minwindef::BYTE } /* wingdi.h:1673:16, wingdi.h:1673:16, wingdi.h:1673:16 */
#[cfg(feature="winapi_app")] pub type LPPANOSE = *mut ::wingdi::PANOSE; /* wingdi.h:1685:13, wingdi.h:1685:13, wingdi.h:1685:13 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct EXTLOGFONTA { elfLogFont: ::wingdi::LOGFONTA, elfFullName: *mut [::minwindef::BYTE; 64], elfStyle: *mut [::minwindef::BYTE; 32], elfVersion: ::minwindef::DWORD, elfStyleSize: ::minwindef::DWORD, elfMatch: ::minwindef::DWORD, elfReserved: ::minwindef::DWORD, elfVendorId: *mut [::minwindef::BYTE; 4], elfCulture: ::minwindef::DWORD, elfPanose: ::wingdi::PANOSE } /* wingdi.h:1799:16, wingdi.h:1799:16, wingdi.h:1799:16 */
#[cfg(feature="winapi_app")] pub type PEXTLOGFONTA = *mut ::wingdi::EXTLOGFONTA; /* wingdi.h:1810:17, wingdi.h:1810:17, wingdi.h:1810:17 */
#[cfg(feature="winapi_app")] pub type NPEXTLOGFONTA = *mut ::wingdi::EXTLOGFONTA; /* wingdi.h:1810:37, wingdi.h:1810:37, wingdi.h:1810:37 */
#[cfg(feature="winapi_app")] pub type LPEXTLOGFONTA = *mut ::wingdi::EXTLOGFONTA; /* wingdi.h:1810:57, wingdi.h:1810:57, wingdi.h:1810:57 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct EXTLOGFONTW { elfLogFont: ::wingdi::LOGFONTW, elfFullName: *mut [::winnt::WCHAR; 64], elfStyle: *mut [::winnt::WCHAR; 32], elfVersion: ::minwindef::DWORD, elfStyleSize: ::minwindef::DWORD, elfMatch: ::minwindef::DWORD, elfReserved: ::minwindef::DWORD, elfVendorId: *mut [::minwindef::BYTE; 4], elfCulture: ::minwindef::DWORD, elfPanose: ::wingdi::PANOSE } /* wingdi.h:1811:16, wingdi.h:1811:16, wingdi.h:1811:16 */
#[cfg(feature="winapi_app")] pub type PEXTLOGFONTW = *mut ::wingdi::EXTLOGFONTW; /* wingdi.h:1822:17, wingdi.h:1822:17, wingdi.h:1822:17 */
#[cfg(feature="winapi_app")] pub type NPEXTLOGFONTW = *mut ::wingdi::EXTLOGFONTW; /* wingdi.h:1822:37, wingdi.h:1822:37, wingdi.h:1822:37 */
#[cfg(feature="winapi_app")] pub type LPEXTLOGFONTW = *mut ::wingdi::EXTLOGFONTW; /* wingdi.h:1822:57, wingdi.h:1822:57, wingdi.h:1822:57 */
#[cfg(feature="winapi_app")] pub type EXTLOGFONT = ::wingdi::EXTLOGFONTW; /* wingdi.h:1824:21, wingdi.h:1824:21, wingdi.h:1824:21 */
#[cfg(feature="winapi_app")] pub type PEXTLOGFONT = ::wingdi::PEXTLOGFONTW; /* wingdi.h:1825:22, wingdi.h:1825:22, wingdi.h:1825:22 */
#[cfg(feature="winapi_app")] pub type NPEXTLOGFONT = ::wingdi::NPEXTLOGFONTW; /* wingdi.h:1826:23, wingdi.h:1826:23, wingdi.h:1826:23 */
#[cfg(feature="winapi_app")] pub type LPEXTLOGFONT = ::wingdi::LPEXTLOGFONTW; /* wingdi.h:1827:23, wingdi.h:1827:23, wingdi.h:1827:23 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct DEVMODEA { dmDeviceName: *mut [::minwindef::BYTE; 32], dmSpecVersion: ::minwindef::WORD, dmDriverVersion: ::minwindef::WORD, dmSize: ::minwindef::WORD, dmDriverExtra: ::minwindef::WORD, dmFields: ::minwindef::DWORD, dmColor: ::libc::c_short, dmDuplex: ::libc::c_short, dmYResolution: ::libc::c_short, dmTTOption: ::libc::c_short, dmCollate: ::libc::c_short, dmFormName: *mut [::minwindef::BYTE; 32], dmLogPixels: ::minwindef::WORD, dmBitsPerPel: ::minwindef::DWORD, dmPelsWidth: ::minwindef::DWORD, dmPelsHeight: ::minwindef::DWORD, dmDisplayFrequency: ::minwindef::DWORD, dmICMMethod: ::minwindef::DWORD, dmICMIntent: ::minwindef::DWORD, dmMediaType: ::minwindef::DWORD, dmDitherType: ::minwindef::DWORD, dmReserved1: ::minwindef::DWORD, dmReserved2: ::minwindef::DWORD, dmPanningWidth: ::minwindef::DWORD, dmPanningHeight: ::minwindef::DWORD } /* wingdi.h:2185:16, wingdi.h:2185:16, wingdi.h:2185:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct DEVMODEA_Child_6; /* STUB! */ /* wingdi.h:2192:5, wingdi.h:2192:5, wingdi.h:2192:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct DEVMODEA_Child_17; /* STUB! */ /* wingdi.h:2221:5, wingdi.h:2221:5, wingdi.h:2221:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PDEVMODEA = *mut ::wingdi::DEVMODEA; /* wingdi.h:2238:14, wingdi.h:2238:14, wingdi.h:2238:14 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type NPDEVMODEA = *mut ::wingdi::DEVMODEA; /* wingdi.h:2238:26, wingdi.h:2238:26, wingdi.h:2238:26 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPDEVMODEA = *mut ::wingdi::DEVMODEA; /* wingdi.h:2238:39, wingdi.h:2238:39, wingdi.h:2238:39 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub struct DEVMODEW { dmDeviceName: *mut [::winnt::WCHAR; 32], dmSpecVersion: ::minwindef::WORD, dmDriverVersion: ::minwindef::WORD, dmSize: ::minwindef::WORD, dmDriverExtra: ::minwindef::WORD, dmFields: ::minwindef::DWORD, dmColor: ::libc::c_short, dmDuplex: ::libc::c_short, dmYResolution: ::libc::c_short, dmTTOption: ::libc::c_short, dmCollate: ::libc::c_short, dmFormName: *mut [::winnt::WCHAR; 32], dmLogPixels: ::minwindef::WORD, dmBitsPerPel: ::minwindef::DWORD, dmPelsWidth: ::minwindef::DWORD, dmPelsHeight: ::minwindef::DWORD, dmDisplayFrequency: ::minwindef::DWORD, dmICMMethod: ::minwindef::DWORD, dmICMIntent: ::minwindef::DWORD, dmMediaType: ::minwindef::DWORD, dmDitherType: ::minwindef::DWORD, dmReserved1: ::minwindef::DWORD, dmReserved2: ::minwindef::DWORD, dmPanningWidth: ::minwindef::DWORD, dmPanningHeight: ::minwindef::DWORD } /* wingdi.h:2239:16, wingdi.h:2239:16, wingdi.h:2239:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct DEVMODEW_Child_6; /* STUB! */ /* wingdi.h:2246:5, wingdi.h:2246:5, wingdi.h:2246:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] #[repr(C)] pub /*union*/ struct DEVMODEW_Child_17; /* STUB! */ /* wingdi.h:2275:5, wingdi.h:2275:5, wingdi.h:2275:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PDEVMODEW = *mut ::wingdi::DEVMODEW; /* wingdi.h:2292:14, wingdi.h:2292:14, wingdi.h:2292:14 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type NPDEVMODEW = *mut ::wingdi::DEVMODEW; /* wingdi.h:2292:26, wingdi.h:2292:26, wingdi.h:2292:26 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPDEVMODEW = *mut ::wingdi::DEVMODEW; /* wingdi.h:2292:39, wingdi.h:2292:39, wingdi.h:2292:39 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type DEVMODE = ::wingdi::DEVMODEW; /* wingdi.h:2294:18, wingdi.h:2294:18, wingdi.h:2294:18 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type PDEVMODE = ::wingdi::PDEVMODEW; /* wingdi.h:2295:19, wingdi.h:2295:19, wingdi.h:2295:19 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type NPDEVMODE = ::wingdi::NPDEVMODEW; /* wingdi.h:2296:20, wingdi.h:2296:20, wingdi.h:2296:20 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_05010000"))] pub type LPDEVMODE = ::wingdi::LPDEVMODEW; /* wingdi.h:2297:20, wingdi.h:2297:20, wingdi.h:2297:20 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct DISPLAY_DEVICEA { cb: ::minwindef::DWORD, DeviceName: *mut [::winnt::CHAR; 32], DeviceString: *mut [::winnt::CHAR; 128], StateFlags: ::minwindef::DWORD, DeviceID: *mut [::winnt::CHAR; 128], DeviceKey: *mut [::winnt::CHAR; 128] } /* wingdi.h:2720:16, wingdi.h:2720:16, wingdi.h:2720:16 */
#[cfg(feature="winapi_app")] pub type PDISPLAY_DEVICEA = *mut ::wingdi::DISPLAY_DEVICEA; /* wingdi.h:2727:21, wingdi.h:2727:21, wingdi.h:2727:21 */
#[cfg(feature="winapi_app")] pub type LPDISPLAY_DEVICEA = *mut ::wingdi::DISPLAY_DEVICEA; /* wingdi.h:2727:40, wingdi.h:2727:40, wingdi.h:2727:40 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct DISPLAY_DEVICEW { cb: ::minwindef::DWORD, DeviceName: *mut [::winnt::WCHAR; 32], DeviceString: *mut [::winnt::WCHAR; 128], StateFlags: ::minwindef::DWORD, DeviceID: *mut [::winnt::WCHAR; 128], DeviceKey: *mut [::winnt::WCHAR; 128] } /* wingdi.h:2728:16, wingdi.h:2728:16, wingdi.h:2728:16 */
#[cfg(feature="winapi_app")] pub type PDISPLAY_DEVICEW = *mut ::wingdi::DISPLAY_DEVICEW; /* wingdi.h:2735:21, wingdi.h:2735:21, wingdi.h:2735:21 */
#[cfg(feature="winapi_app")] pub type LPDISPLAY_DEVICEW = *mut ::wingdi::DISPLAY_DEVICEW; /* wingdi.h:2735:40, wingdi.h:2735:40, wingdi.h:2735:40 */
#[cfg(feature="winapi_app")] pub type DISPLAY_DEVICE = ::wingdi::DISPLAY_DEVICEW; /* wingdi.h:2737:25, wingdi.h:2737:25, wingdi.h:2737:25 */
#[cfg(feature="winapi_app")] pub type PDISPLAY_DEVICE = ::wingdi::PDISPLAY_DEVICEW; /* wingdi.h:2738:26, wingdi.h:2738:26, wingdi.h:2738:26 */
#[cfg(feature="winapi_app")] pub type LPDISPLAY_DEVICE = ::wingdi::LPDISPLAY_DEVICEW; /* wingdi.h:2739:27, wingdi.h:2739:27, wingdi.h:2739:27 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_RATIONAL { Numerator: ::basetsd::UINT32, Denominator: ::basetsd::UINT32 } /* wingdi.h:2785:16, wingdi.h:2785:16, wingdi.h:2785:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY {DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER = -1, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15 = 0, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO = 1, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO = 2, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO = 3, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI = 4, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI = 5, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS = 6, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN = 8, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI = 9, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL = 10, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED = 11, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL = 12, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED = 13, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE = 14, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST = 15, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL = -2147483648} pub use self::DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::{DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HD15, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SVIDEO, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPOSITE_VIDEO, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_COMPONENT_VIDEO, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DVI, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_HDMI, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_LVDS, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_D_JPN, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDI, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EXTERNAL, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_DISPLAYPORT_EMBEDDED, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EXTERNAL, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_UDI_EMBEDDED, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_SDTVDONGLE, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_MIRACAST, DISPLAYCONFIG_OUTPUT_TECHNOLOGY_INTERNAL}; pub const DISPLAYCONFIG_OUTPUT_TECHNOLOGY_FORCE_UINT32: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY = DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::DISPLAYCONFIG_OUTPUT_TECHNOLOGY_OTHER; /* wingdi.h:2791:9, wingdi.h:2791:9, wingdi.h:2791:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_SCANLINE_ORDERING {DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED = 0, DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE = 1, DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED = 2, DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST = 3, DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_SCANLINE_ORDERING::{DISPLAYCONFIG_SCANLINE_ORDERING_UNSPECIFIED, DISPLAYCONFIG_SCANLINE_ORDERING_PROGRESSIVE, DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED, DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_LOWERFIELDFIRST, DISPLAYCONFIG_SCANLINE_ORDERING_FORCE_UINT32}; pub const DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED_UPPERFIELDFIRST: DISPLAYCONFIG_SCANLINE_ORDERING = DISPLAYCONFIG_SCANLINE_ORDERING::DISPLAYCONFIG_SCANLINE_ORDERING_INTERLACED; /* wingdi.h:2813:9, wingdi.h:2813:9, wingdi.h:2813:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_2DREGION { cx: ::basetsd::UINT32, cy: ::basetsd::UINT32 } /* wingdi.h:2823:16, wingdi.h:2823:16, wingdi.h:2823:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO { pixelRate: ::basetsd::UINT64, hSyncFreq: ::wingdi::DISPLAYCONFIG_RATIONAL, vSyncFreq: ::wingdi::DISPLAYCONFIG_RATIONAL, activeSize: ::wingdi::DISPLAYCONFIG_2DREGION, totalSize: ::wingdi::DISPLAYCONFIG_2DREGION, scanLineOrdering: ::wingdi::DISPLAYCONFIG_SCANLINE_ORDERING } /* wingdi.h:2829:16, wingdi.h:2829:16, wingdi.h:2829:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub /*union*/ struct DISPLAYCONFIG_VIDEO_SIGNAL_INFO_Child_5; /* STUB! */ /* wingdi.h:2837:5, wingdi.h:2837:5, wingdi.h:2837:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_SCALING {DISPLAYCONFIG_SCALING_IDENTITY = 1, DISPLAYCONFIG_SCALING_CENTERED = 2, DISPLAYCONFIG_SCALING_STRETCHED = 3, DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX = 4, DISPLAYCONFIG_SCALING_CUSTOM = 5, DISPLAYCONFIG_SCALING_PREFERRED = 128, DISPLAYCONFIG_SCALING_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_SCALING::{DISPLAYCONFIG_SCALING_IDENTITY, DISPLAYCONFIG_SCALING_CENTERED, DISPLAYCONFIG_SCALING_STRETCHED, DISPLAYCONFIG_SCALING_ASPECTRATIOCENTEREDMAX, DISPLAYCONFIG_SCALING_CUSTOM, DISPLAYCONFIG_SCALING_PREFERRED, DISPLAYCONFIG_SCALING_FORCE_UINT32}; /* wingdi.h:2856:9, wingdi.h:2856:9, wingdi.h:2856:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_ROTATION {DISPLAYCONFIG_ROTATION_IDENTITY = 1, DISPLAYCONFIG_ROTATION_ROTATE90 = 2, DISPLAYCONFIG_ROTATION_ROTATE180 = 3, DISPLAYCONFIG_ROTATION_ROTATE270 = 4, DISPLAYCONFIG_ROTATION_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_ROTATION::{DISPLAYCONFIG_ROTATION_IDENTITY, DISPLAYCONFIG_ROTATION_ROTATE90, DISPLAYCONFIG_ROTATION_ROTATE180, DISPLAYCONFIG_ROTATION_ROTATE270, DISPLAYCONFIG_ROTATION_FORCE_UINT32}; /* wingdi.h:2867:9, wingdi.h:2867:9, wingdi.h:2867:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_MODE_INFO_TYPE {DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE = 1, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET = 2, DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_MODE_INFO_TYPE::{DISPLAYCONFIG_MODE_INFO_TYPE_SOURCE, DISPLAYCONFIG_MODE_INFO_TYPE_TARGET, DISPLAYCONFIG_MODE_INFO_TYPE_FORCE_UINT32}; /* wingdi.h:2876:9, wingdi.h:2876:9, wingdi.h:2876:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_PIXELFORMAT {DISPLAYCONFIG_PIXELFORMAT_8BPP = 1, DISPLAYCONFIG_PIXELFORMAT_16BPP = 2, DISPLAYCONFIG_PIXELFORMAT_24BPP = 3, DISPLAYCONFIG_PIXELFORMAT_32BPP = 4, DISPLAYCONFIG_PIXELFORMAT_NONGDI = 5, DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_PIXELFORMAT::{DISPLAYCONFIG_PIXELFORMAT_8BPP, DISPLAYCONFIG_PIXELFORMAT_16BPP, DISPLAYCONFIG_PIXELFORMAT_24BPP, DISPLAYCONFIG_PIXELFORMAT_32BPP, DISPLAYCONFIG_PIXELFORMAT_NONGDI, DISPLAYCONFIG_PIXELFORMAT_FORCE_UINT32}; /* wingdi.h:2883:9, wingdi.h:2883:9, wingdi.h:2883:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_SOURCE_MODE { width: ::basetsd::UINT32, height: ::basetsd::UINT32, pixelFormat: ::wingdi::DISPLAYCONFIG_PIXELFORMAT, position: ::windef::POINTL } /* wingdi.h:2893:16, wingdi.h:2893:16, wingdi.h:2893:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_TARGET_MODE { targetVideoSignalInfo: ::wingdi::DISPLAYCONFIG_VIDEO_SIGNAL_INFO } /* wingdi.h:2901:16, wingdi.h:2901:16, wingdi.h:2901:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_MODE_INFO { infoType: ::wingdi::DISPLAYCONFIG_MODE_INFO_TYPE, id: ::basetsd::UINT32, adapterId: ::winnt::LUID } /* wingdi.h:2906:16, wingdi.h:2906:16, wingdi.h:2906:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub /*union*/ struct DISPLAYCONFIG_MODE_INFO_Child_3; /* STUB! */ /* wingdi.h:2911:5, wingdi.h:2911:5, wingdi.h:2911:5 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_PATH_SOURCE_INFO { adapterId: ::winnt::LUID, id: ::basetsd::UINT32, modeInfoIdx: ::basetsd::UINT32, statusFlags: ::basetsd::UINT32 } /* wingdi.h:2920:16, wingdi.h:2920:16, wingdi.h:2920:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_PATH_TARGET_INFO { adapterId: ::winnt::LUID, id: ::basetsd::UINT32, modeInfoIdx: ::basetsd::UINT32, outputTechnology: ::wingdi::DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, rotation: ::wingdi::DISPLAYCONFIG_ROTATION, scaling: ::wingdi::DISPLAYCONFIG_SCALING, refreshRate: ::wingdi::DISPLAYCONFIG_RATIONAL, scanLineOrdering: ::wingdi::DISPLAYCONFIG_SCANLINE_ORDERING, targetAvailable: ::minwindef::BOOL, statusFlags: ::basetsd::UINT32 } /* wingdi.h:2934:16, wingdi.h:2934:16, wingdi.h:2934:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_PATH_INFO { sourceInfo: ::wingdi::DISPLAYCONFIG_PATH_SOURCE_INFO, targetInfo: ::wingdi::DISPLAYCONFIG_PATH_TARGET_INFO, flags: ::basetsd::UINT32 } /* wingdi.h:2957:16, wingdi.h:2957:16, wingdi.h:2957:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_TOPOLOGY_ID {DISPLAYCONFIG_TOPOLOGY_INTERNAL = 1, DISPLAYCONFIG_TOPOLOGY_CLONE = 2, DISPLAYCONFIG_TOPOLOGY_EXTEND = 4, DISPLAYCONFIG_TOPOLOGY_EXTERNAL = 8, DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_TOPOLOGY_ID::{DISPLAYCONFIG_TOPOLOGY_INTERNAL, DISPLAYCONFIG_TOPOLOGY_CLONE, DISPLAYCONFIG_TOPOLOGY_EXTEND, DISPLAYCONFIG_TOPOLOGY_EXTERNAL, DISPLAYCONFIG_TOPOLOGY_FORCE_UINT32}; /* wingdi.h:2970:9, wingdi.h:2970:9, wingdi.h:2970:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub enum DISPLAYCONFIG_DEVICE_INFO_TYPE {DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME = 1, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME = 2, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE = 3, DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME = 4, DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE = 5, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE = 6, DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32 = -1} pub use self::DISPLAYCONFIG_DEVICE_INFO_TYPE::{DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE, DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME, DISPLAYCONFIG_DEVICE_INFO_SET_TARGET_PERSISTENCE, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE, DISPLAYCONFIG_DEVICE_INFO_FORCE_UINT32}; /* wingdi.h:2980:9, wingdi.h:2980:9, wingdi.h:2980:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER { type_: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_TYPE, size: ::basetsd::UINT32, adapterId: ::winnt::LUID, id: ::basetsd::UINT32 } /* wingdi.h:2998:16, wingdi.h:2998:16, wingdi.h:2998:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER, viewGdiDeviceName: *mut [::winnt::WCHAR; 32] } /* wingdi.h:3012:16, wingdi.h:3012:16, wingdi.h:3012:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS; /* wingdi.h:3018:16, wingdi.h:3018:16, wingdi.h:3018:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub /*union*/ struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS_Child_0; /* STUB! */ /* wingdi.h:3020:5, wingdi.h:3020:5, wingdi.h:3020:5 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER, flags: ::wingdi::DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS, outputTechnology: ::wingdi::DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, edidManufactureId: ::basetsd::UINT16, edidProductCodeId: ::basetsd::UINT16, connectorInstance: ::basetsd::UINT32, monitorFriendlyDeviceName: *mut [::winnt::WCHAR; 64], monitorDevicePath: *mut [::winnt::WCHAR; 128] } /* wingdi.h:3033:16, wingdi.h:3033:16, wingdi.h:3033:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER, width: ::basetsd::UINT32, height: ::basetsd::UINT32, targetMode: ::wingdi::DISPLAYCONFIG_TARGET_MODE } /* wingdi.h:3045:16, wingdi.h:3045:16, wingdi.h:3045:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_ADAPTER_NAME { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER, adapterDevicePath: *mut [::winnt::WCHAR; 128] } /* wingdi.h:3053:16, wingdi.h:3053:16, wingdi.h:3053:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_TARGET_BASE_TYPE { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER, baseOutputTechnology: ::wingdi::DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY } /* wingdi.h:3059:16, wingdi.h:3059:16, wingdi.h:3059:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE { header: ::wingdi::DISPLAYCONFIG_DEVICE_INFO_HEADER } /* wingdi.h:3064:16, wingdi.h:3064:16, wingdi.h:3064:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_06010000"))] #[repr(C)] pub /*union*/ struct DISPLAYCONFIG_SET_TARGET_PERSISTENCE_Child_1; /* STUB! */ /* wingdi.h:3067:5, wingdi.h:3067:5, wingdi.h:3067:5 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RGNDATAHEADER { dwSize: ::minwindef::DWORD, iType: ::minwindef::DWORD, nCount: ::minwindef::DWORD, nRgnSize: ::minwindef::DWORD, rcBound: ::windef::RECT } /* wingdi.h:3121:16, wingdi.h:3121:16, wingdi.h:3121:16 */
#[cfg(feature="winapi_app")] pub type PRGNDATAHEADER = *mut ::wingdi::RGNDATAHEADER; /* wingdi.h:3127:19, wingdi.h:3127:19, wingdi.h:3127:19 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct RGNDATA { rdh: ::wingdi::RGNDATAHEADER, Buffer: *mut [::libc::c_schar; 1] } /* wingdi.h:3129:16, wingdi.h:3129:16, wingdi.h:3129:16 */
#[cfg(feature="winapi_app")] pub type PRGNDATA = *mut ::wingdi::RGNDATA; /* wingdi.h:3132:13, wingdi.h:3132:13, wingdi.h:3132:13 */
#[cfg(feature="winapi_app")] pub type NPRGNDATA = *mut ::wingdi::RGNDATA; /* wingdi.h:3132:29, wingdi.h:3132:29, wingdi.h:3132:29 */
#[cfg(feature="winapi_app")] pub type LPRGNDATA = *mut ::wingdi::RGNDATA; /* wingdi.h:3132:45, wingdi.h:3132:45, wingdi.h:3132:45 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ABC { abcA: ::libc::c_int, abcB: ::minwindef::UINT, abcC: ::libc::c_int } /* wingdi.h:3145:16, wingdi.h:3145:16, wingdi.h:3145:16 */
#[cfg(feature="winapi_desktop")] pub type PABC = *mut ::wingdi::ABC; /* wingdi.h:3149:9, wingdi.h:3149:9, wingdi.h:3149:9 */
#[cfg(feature="winapi_desktop")] pub type NPABC = *mut ::wingdi::ABC; /* wingdi.h:3149:21, wingdi.h:3149:21, wingdi.h:3149:21 */
#[cfg(feature="winapi_desktop")] pub type LPABC = *mut ::wingdi::ABC; /* wingdi.h:3149:33, wingdi.h:3149:33, wingdi.h:3149:33 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct ABCFLOAT { abcfA: ::minwindef::FLOAT, abcfB: ::minwindef::FLOAT, abcfC: ::minwindef::FLOAT } /* wingdi.h:3151:16, wingdi.h:3151:16, wingdi.h:3151:16 */
#[cfg(feature="winapi_desktop")] pub type PABCFLOAT = *mut ::wingdi::ABCFLOAT; /* wingdi.h:3155:14, wingdi.h:3155:14, wingdi.h:3155:14 */
#[cfg(feature="winapi_desktop")] pub type NPABCFLOAT = *mut ::wingdi::ABCFLOAT; /* wingdi.h:3155:31, wingdi.h:3155:31, wingdi.h:3155:31 */
#[cfg(feature="winapi_desktop")] pub type LPABCFLOAT = *mut ::wingdi::ABCFLOAT; /* wingdi.h:3155:48, wingdi.h:3155:48, wingdi.h:3155:48 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct OUTLINETEXTMETRICA { otmSize: ::minwindef::UINT, otmTextMetrics: ::wingdi::TEXTMETRICA, otmFiller: ::minwindef::BYTE, otmPanoseNumber: ::wingdi::PANOSE, otmfsSelection: ::minwindef::UINT, otmfsType: ::minwindef::UINT, otmsCharSlopeRise: ::libc::c_int, otmsCharSlopeRun: ::libc::c_int, otmItalicAngle: ::libc::c_int, otmEMSquare: ::minwindef::UINT, otmAscent: ::libc::c_int, otmDescent: ::libc::c_int, otmLineGap: ::minwindef::UINT, otmsCapEmHeight: ::minwindef::UINT, otmsXHeight: ::minwindef::UINT, otmrcFontBox: ::windef::RECT, otmMacAscent: ::libc::c_int, otmMacDescent: ::libc::c_int, otmMacLineGap: ::minwindef::UINT, otmusMinimumPPEM: ::minwindef::UINT, otmptSubscriptSize: ::windef::POINT, otmptSubscriptOffset: ::windef::POINT, otmptSuperscriptSize: ::windef::POINT, otmptSuperscriptOffset: ::windef::POINT, otmsStrikeoutSize: ::minwindef::UINT, otmsStrikeoutPosition: ::libc::c_int, otmsUnderscoreSize: ::libc::c_int, otmsUnderscorePosition: ::libc::c_int, otmpFamilyName: ::winnt::PSTR, otmpFaceName: ::winnt::PSTR, otmpStyleName: ::winnt::PSTR, otmpFullName: ::winnt::PSTR } /* wingdi.h:3169:16, wingdi.h:3169:16, wingdi.h:3169:16 */
#[cfg(feature="winapi_desktop")] pub type POUTLINETEXTMETRICA = *mut ::wingdi::OUTLINETEXTMETRICA; /* wingdi.h:3202:24, wingdi.h:3202:24, wingdi.h:3202:24 */
#[cfg(feature="winapi_desktop")] pub type NPOUTLINETEXTMETRICA = *mut ::wingdi::OUTLINETEXTMETRICA; /* wingdi.h:3202:51, wingdi.h:3202:51, wingdi.h:3202:51 */
#[cfg(feature="winapi_desktop")] pub type LPOUTLINETEXTMETRICA = *mut ::wingdi::OUTLINETEXTMETRICA; /* wingdi.h:3202:78, wingdi.h:3202:78, wingdi.h:3202:78 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct OUTLINETEXTMETRICW { otmSize: ::minwindef::UINT, otmTextMetrics: ::wingdi::TEXTMETRICW, otmFiller: ::minwindef::BYTE, otmPanoseNumber: ::wingdi::PANOSE, otmfsSelection: ::minwindef::UINT, otmfsType: ::minwindef::UINT, otmsCharSlopeRise: ::libc::c_int, otmsCharSlopeRun: ::libc::c_int, otmItalicAngle: ::libc::c_int, otmEMSquare: ::minwindef::UINT, otmAscent: ::libc::c_int, otmDescent: ::libc::c_int, otmLineGap: ::minwindef::UINT, otmsCapEmHeight: ::minwindef::UINT, otmsXHeight: ::minwindef::UINT, otmrcFontBox: ::windef::RECT, otmMacAscent: ::libc::c_int, otmMacDescent: ::libc::c_int, otmMacLineGap: ::minwindef::UINT, otmusMinimumPPEM: ::minwindef::UINT, otmptSubscriptSize: ::windef::POINT, otmptSubscriptOffset: ::windef::POINT, otmptSuperscriptSize: ::windef::POINT, otmptSuperscriptOffset: ::windef::POINT, otmsStrikeoutSize: ::minwindef::UINT, otmsStrikeoutPosition: ::libc::c_int, otmsUnderscoreSize: ::libc::c_int, otmsUnderscorePosition: ::libc::c_int, otmpFamilyName: ::winnt::PSTR, otmpFaceName: ::winnt::PSTR, otmpStyleName: ::winnt::PSTR, otmpFullName: ::winnt::PSTR } /* wingdi.h:3203:16, wingdi.h:3203:16, wingdi.h:3203:16 */
#[cfg(feature="winapi_desktop")] pub type POUTLINETEXTMETRICW = *mut ::wingdi::OUTLINETEXTMETRICW; /* wingdi.h:3236:24, wingdi.h:3236:24, wingdi.h:3236:24 */
#[cfg(feature="winapi_desktop")] pub type NPOUTLINETEXTMETRICW = *mut ::wingdi::OUTLINETEXTMETRICW; /* wingdi.h:3236:51, wingdi.h:3236:51, wingdi.h:3236:51 */
#[cfg(feature="winapi_desktop")] pub type LPOUTLINETEXTMETRICW = *mut ::wingdi::OUTLINETEXTMETRICW; /* wingdi.h:3236:78, wingdi.h:3236:78, wingdi.h:3236:78 */
#[cfg(feature="winapi_desktop")] pub type OUTLINETEXTMETRIC = ::wingdi::OUTLINETEXTMETRICW; /* wingdi.h:3238:28, wingdi.h:3238:28, wingdi.h:3238:28 */
#[cfg(feature="winapi_desktop")] pub type POUTLINETEXTMETRIC = ::wingdi::POUTLINETEXTMETRICW; /* wingdi.h:3239:29, wingdi.h:3239:29, wingdi.h:3239:29 */
#[cfg(feature="winapi_desktop")] pub type NPOUTLINETEXTMETRIC = ::wingdi::NPOUTLINETEXTMETRICW; /* wingdi.h:3240:30, wingdi.h:3240:30, wingdi.h:3240:30 */
#[cfg(feature="winapi_desktop")] pub type LPOUTLINETEXTMETRIC = ::wingdi::LPOUTLINETEXTMETRICW; /* wingdi.h:3241:30, wingdi.h:3241:30, wingdi.h:3241:30 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct POLYTEXTA { x: ::libc::c_int, y: ::libc::c_int, n: ::minwindef::UINT, lpstr: ::winnt::LPCSTR, uiFlags: ::minwindef::UINT, rcl: ::windef::RECT, pdx: *mut ::libc::c_int } /* wingdi.h:3261:16, wingdi.h:3261:16, wingdi.h:3261:16 */
#[cfg(feature="winapi_app")] pub type PPOLYTEXTA = *mut ::wingdi::POLYTEXTA; /* wingdi.h:3270:15, wingdi.h:3270:15, wingdi.h:3270:15 */
#[cfg(feature="winapi_app")] pub type NPPOLYTEXTA = *mut ::wingdi::POLYTEXTA; /* wingdi.h:3270:33, wingdi.h:3270:33, wingdi.h:3270:33 */
#[cfg(feature="winapi_app")] pub type LPPOLYTEXTA = *mut ::wingdi::POLYTEXTA; /* wingdi.h:3270:51, wingdi.h:3270:51, wingdi.h:3270:51 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct POLYTEXTW { x: ::libc::c_int, y: ::libc::c_int, n: ::minwindef::UINT, lpstr: ::winnt::LPCWSTR, uiFlags: ::minwindef::UINT, rcl: ::windef::RECT, pdx: *mut ::libc::c_int } /* wingdi.h:3271:16, wingdi.h:3271:16, wingdi.h:3271:16 */
#[cfg(feature="winapi_app")] pub type PPOLYTEXTW = *mut ::wingdi::POLYTEXTW; /* wingdi.h:3280:15, wingdi.h:3280:15, wingdi.h:3280:15 */
#[cfg(feature="winapi_app")] pub type NPPOLYTEXTW = *mut ::wingdi::POLYTEXTW; /* wingdi.h:3280:33, wingdi.h:3280:33, wingdi.h:3280:33 */
#[cfg(feature="winapi_app")] pub type LPPOLYTEXTW = *mut ::wingdi::POLYTEXTW; /* wingdi.h:3280:51, wingdi.h:3280:51, wingdi.h:3280:51 */
#[cfg(feature="winapi_app")] pub type POLYTEXT = ::wingdi::POLYTEXTW; /* wingdi.h:3282:19, wingdi.h:3282:19, wingdi.h:3282:19 */
#[cfg(feature="winapi_app")] pub type PPOLYTEXT = ::wingdi::PPOLYTEXTW; /* wingdi.h:3283:20, wingdi.h:3283:20, wingdi.h:3283:20 */
#[cfg(feature="winapi_app")] pub type NPPOLYTEXT = ::wingdi::NPPOLYTEXTW; /* wingdi.h:3284:21, wingdi.h:3284:21, wingdi.h:3284:21 */
#[cfg(feature="winapi_app")] pub type LPPOLYTEXT = ::wingdi::LPPOLYTEXTW; /* wingdi.h:3285:21, wingdi.h:3285:21, wingdi.h:3285:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct FIXED { fract: ::minwindef::WORD, value: ::libc::c_short } /* wingdi.h:3299:16, wingdi.h:3299:16, wingdi.h:3299:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct MAT2 { eM11: ::wingdi::FIXED, eM12: ::wingdi::FIXED, eM21: ::wingdi::FIXED, eM22: ::wingdi::FIXED } /* wingdi.h:3310:16, wingdi.h:3310:16, wingdi.h:3310:16 */
#[cfg(feature="winapi_desktop")] pub type LPMAT2 = *mut ::wingdi::MAT2; /* wingdi.h:3315:14, wingdi.h:3315:14, wingdi.h:3315:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct GLYPHMETRICS { gmBlackBoxX: ::minwindef::UINT, gmBlackBoxY: ::minwindef::UINT, gmptGlyphOrigin: ::windef::POINT, gmCellIncX: ::libc::c_short, gmCellIncY: ::libc::c_short } /* wingdi.h:3319:16, wingdi.h:3319:16, wingdi.h:3319:16 */
#[cfg(feature="winapi_desktop")] pub type LPGLYPHMETRICS = *mut ::wingdi::GLYPHMETRICS; /* wingdi.h:3325:22, wingdi.h:3325:22, wingdi.h:3325:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct POINTFX { x: ::wingdi::FIXED, y: ::wingdi::FIXED } /* wingdi.h:3357:16, wingdi.h:3357:16, wingdi.h:3357:16 */
#[cfg(feature="winapi_desktop")] pub type LPPOINTFX = *mut ::wingdi::POINTFX; /* wingdi.h:3361:17, wingdi.h:3361:17, wingdi.h:3361:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct TTPOLYCURVE { wType: ::minwindef::WORD, cpfx: ::minwindef::WORD, apfx: *mut [::wingdi::POINTFX; 1] } /* wingdi.h:3363:16, wingdi.h:3363:16, wingdi.h:3363:16 */
#[cfg(feature="winapi_desktop")] pub type LPTTPOLYCURVE = *mut ::wingdi::TTPOLYCURVE; /* wingdi.h:3368:21, wingdi.h:3368:21, wingdi.h:3368:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct TTPOLYGONHEADER { cb: ::minwindef::DWORD, dwType: ::minwindef::DWORD, pfxStart: ::wingdi::POINTFX } /* wingdi.h:3370:16, wingdi.h:3370:16, wingdi.h:3370:16 */
#[cfg(feature="winapi_desktop")] pub type LPTTPOLYGONHEADER = *mut ::wingdi::TTPOLYGONHEADER; /* wingdi.h:3375:25, wingdi.h:3375:25, wingdi.h:3375:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct GCP_RESULTSA { lStructSize: ::minwindef::DWORD, lpOutString: ::winnt::LPSTR, lpOrder: *mut ::libc::c_uint, lpDx: *mut ::libc::c_int, lpCaretPos: *mut ::libc::c_int, lpClass: ::winnt::LPSTR, lpGlyphs: ::winnt::LPWSTR, nGlyphs: ::minwindef::UINT, nMaxFit: ::libc::c_int } /* wingdi.h:3427:16, wingdi.h:3427:16, wingdi.h:3427:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPGCP_RESULTSA = *mut ::wingdi::GCP_RESULTSA; /* wingdi.h:3438:26, wingdi.h:3438:26, wingdi.h:3438:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct GCP_RESULTSW { lStructSize: ::minwindef::DWORD, lpOutString: ::winnt::LPWSTR, lpOrder: *mut ::libc::c_uint, lpDx: *mut ::libc::c_int, lpCaretPos: *mut ::libc::c_int, lpClass: ::winnt::LPSTR, lpGlyphs: ::winnt::LPWSTR, nGlyphs: ::minwindef::UINT, nMaxFit: ::libc::c_int } /* wingdi.h:3439:16, wingdi.h:3439:16, wingdi.h:3439:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPGCP_RESULTSW = *mut ::wingdi::GCP_RESULTSW; /* wingdi.h:3450:26, wingdi.h:3450:26, wingdi.h:3450:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type GCP_RESULTS = ::wingdi::GCP_RESULTSW; /* wingdi.h:3452:22, wingdi.h:3452:22, wingdi.h:3452:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPGCP_RESULTS = ::wingdi::LPGCP_RESULTSW; /* wingdi.h:3453:24, wingdi.h:3453:24, wingdi.h:3453:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct RASTERIZER_STATUS { nSize: ::libc::c_short, wFlags: ::libc::c_short, nLanguageID: ::libc::c_short } /* wingdi.h:3466:16, wingdi.h:3466:16, wingdi.h:3466:16 */
#[cfg(feature="winapi_desktop")] pub type LPRASTERIZER_STATUS = *mut ::wingdi::RASTERIZER_STATUS; /* wingdi.h:3470:27, wingdi.h:3470:27, wingdi.h:3470:27 */
#[cfg(feature="winapi_app")] #[repr(C)] pub struct PIXELFORMATDESCRIPTOR { nSize: ::minwindef::WORD, nVersion: ::minwindef::WORD, dwFlags: ::minwindef::DWORD, iPixelType: ::minwindef::BYTE, cColorBits: ::minwindef::BYTE, cRedBits: ::minwindef::BYTE, cRedShift: ::minwindef::BYTE, cGreenBits: ::minwindef::BYTE, cGreenShift: ::minwindef::BYTE, cBlueBits: ::minwindef::BYTE, cBlueShift: ::minwindef::BYTE, cAlphaBits: ::minwindef::BYTE, cAlphaShift: ::minwindef::BYTE, cAccumBits: ::minwindef::BYTE, cAccumRedBits: ::minwindef::BYTE, cAccumGreenBits: ::minwindef::BYTE, cAccumBlueBits: ::minwindef::BYTE, cAccumAlphaBits: ::minwindef::BYTE, cDepthBits: ::minwindef::BYTE, cStencilBits: ::minwindef::BYTE, cAuxBuffers: ::minwindef::BYTE, iLayerType: ::minwindef::BYTE, bReserved: ::minwindef::BYTE, dwLayerMask: ::minwindef::DWORD, dwVisibleMask: ::minwindef::DWORD, dwDamageMask: ::minwindef::DWORD } /* wingdi.h:3483:16, wingdi.h:3483:16, wingdi.h:3483:16 */
#[cfg(feature="winapi_app")] pub type PPIXELFORMATDESCRIPTOR = *mut ::wingdi::PIXELFORMATDESCRIPTOR; /* wingdi.h:3511:27, wingdi.h:3511:27, wingdi.h:3511:27 */
#[cfg(feature="winapi_app")] pub type LPPIXELFORMATDESCRIPTOR = *mut ::wingdi::PIXELFORMATDESCRIPTOR; /* wingdi.h:3511:56, wingdi.h:3511:56, wingdi.h:3511:56 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type OLDFONTENUMPROCA = extern "system" fn(*const ::wingdi::LOGFONTA, *const ::wingdi::TEXTMETRICA, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:3555:24, wingdi.h:3555:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type OLDFONTENUMPROCW = extern "system" fn(*const ::wingdi::LOGFONTW, *const ::wingdi::TEXTMETRICW, ::libc::c_ulong, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:3556:24, wingdi.h:3556:24 */
#[cfg(feature="winapi_desktop")] pub type FONTENUMPROCA = ::wingdi::OLDFONTENUMPROCA; /* wingdi.h:3572:29, wingdi.h:3572:29, wingdi.h:3572:29 */
#[cfg(feature="winapi_desktop")] pub type FONTENUMPROCW = ::wingdi::OLDFONTENUMPROCW; /* wingdi.h:3573:29, wingdi.h:3573:29, wingdi.h:3573:29 */
#[cfg(feature="winapi_desktop")] pub type FONTENUMPROC = ::wingdi::FONTENUMPROCW; /* wingdi.h:3575:23, wingdi.h:3575:23, wingdi.h:3575:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type GOBJENUMPROC = extern "system" fn(*mut ::libc::c_void, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:3580:24, wingdi.h:3580:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type LINEDDAPROC = extern "system" fn(::libc::c_int, ::libc::c_int, ::libc::c_long); /* wingdi.h:3581:25, wingdi.h:3581:25 */
#[cfg(feature="winapi_desktop")] pub type LPFNDEVMODE = extern "system" fn(*mut ::windef::HWND__, *mut ::minwindef::HINSTANCE__, *mut ::wingdi::DEVMODEW, *mut ::libc::c_schar, *mut ::libc::c_schar, *mut ::wingdi::DEVMODEW, *mut ::libc::c_schar, ::libc::c_uint) -> ::libc::c_uint; /* wingdi.h:3710:27, wingdi.h:3710:27, wingdi.h:3710:27 */
#[cfg(feature="winapi_desktop")] pub type LPFNDEVCAPS = extern "system" fn(*mut ::libc::c_schar, *mut ::libc::c_schar, ::libc::c_uint, *mut ::libc::c_schar, *mut ::wingdi::DEVMODEW) -> ::libc::c_ulong; /* wingdi.h:3712:27, wingdi.h:3712:27, wingdi.h:3712:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct WCRANGE { wcLow: ::winnt::WCHAR, cGlyphs: ::minwindef::USHORT } /* wingdi.h:4155:16, wingdi.h:4155:16, wingdi.h:4155:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PWCRANGE = *mut ::wingdi::WCRANGE; /* wingdi.h:4159:13, wingdi.h:4159:13, wingdi.h:4159:13 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPWCRANGE = *mut ::wingdi::WCRANGE; /* wingdi.h:4159:27, wingdi.h:4159:27, wingdi.h:4159:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct GLYPHSET { cbThis: ::minwindef::DWORD, flAccel: ::minwindef::DWORD, cGlyphsSupported: ::minwindef::DWORD, cRanges: ::minwindef::DWORD, ranges: *mut [::wingdi::WCRANGE; 1] } /* wingdi.h:4162:16, wingdi.h:4162:16, wingdi.h:4162:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PGLYPHSET = *mut ::wingdi::GLYPHSET; /* wingdi.h:4169:14, wingdi.h:4169:14, wingdi.h:4169:14 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPGLYPHSET = *mut ::wingdi::GLYPHSET; /* wingdi.h:4169:30, wingdi.h:4169:30, wingdi.h:4169:30 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct DESIGNVECTOR { dvReserved: ::minwindef::DWORD, dvNumAxes: ::minwindef::DWORD, dvValues: *mut [::winnt::LONG; 16] } /* wingdi.h:4218:16, wingdi.h:4218:16, wingdi.h:4218:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PDESIGNVECTOR = *mut ::wingdi::DESIGNVECTOR; /* wingdi.h:4223:18, wingdi.h:4223:18, wingdi.h:4223:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPDESIGNVECTOR = *mut ::wingdi::DESIGNVECTOR; /* wingdi.h:4223:38, wingdi.h:4223:38, wingdi.h:4223:38 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct AXISINFOA { axMinValue: ::winnt::LONG, axMaxValue: ::winnt::LONG, axAxisName: *mut [::minwindef::BYTE; 16] } /* wingdi.h:4254:16, wingdi.h:4254:16, wingdi.h:4254:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXISINFOA = *mut ::wingdi::AXISINFOA; /* wingdi.h:4259:15, wingdi.h:4259:15, wingdi.h:4259:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXISINFOA = *mut ::wingdi::AXISINFOA; /* wingdi.h:4259:32, wingdi.h:4259:32, wingdi.h:4259:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct AXISINFOW { axMinValue: ::winnt::LONG, axMaxValue: ::winnt::LONG, axAxisName: *mut [::winnt::WCHAR; 16] } /* wingdi.h:4260:16, wingdi.h:4260:16, wingdi.h:4260:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXISINFOW = *mut ::wingdi::AXISINFOW; /* wingdi.h:4265:15, wingdi.h:4265:15, wingdi.h:4265:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXISINFOW = *mut ::wingdi::AXISINFOW; /* wingdi.h:4265:32, wingdi.h:4265:32, wingdi.h:4265:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type AXISINFO = ::wingdi::AXISINFOW; /* wingdi.h:4267:19, wingdi.h:4267:19, wingdi.h:4267:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXISINFO = ::wingdi::PAXISINFOW; /* wingdi.h:4268:20, wingdi.h:4268:20, wingdi.h:4268:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXISINFO = ::wingdi::LPAXISINFOW; /* wingdi.h:4269:21, wingdi.h:4269:21, wingdi.h:4269:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct AXESLISTA { axlReserved: ::minwindef::DWORD, axlNumAxes: ::minwindef::DWORD, axlAxisInfo: *mut [::wingdi::AXISINFOA; 16] } /* wingdi.h:4276:16, wingdi.h:4276:16, wingdi.h:4276:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXESLISTA = *mut ::wingdi::AXESLISTA; /* wingdi.h:4281:15, wingdi.h:4281:15, wingdi.h:4281:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXESLISTA = *mut ::wingdi::AXESLISTA; /* wingdi.h:4281:32, wingdi.h:4281:32, wingdi.h:4281:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct AXESLISTW { axlReserved: ::minwindef::DWORD, axlNumAxes: ::minwindef::DWORD, axlAxisInfo: *mut [::wingdi::AXISINFOW; 16] } /* wingdi.h:4282:16, wingdi.h:4282:16, wingdi.h:4282:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXESLISTW = *mut ::wingdi::AXESLISTW; /* wingdi.h:4287:15, wingdi.h:4287:15, wingdi.h:4287:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXESLISTW = *mut ::wingdi::AXESLISTW; /* wingdi.h:4287:32, wingdi.h:4287:32, wingdi.h:4287:32 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type AXESLIST = ::wingdi::AXESLISTW; /* wingdi.h:4289:19, wingdi.h:4289:19, wingdi.h:4289:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PAXESLIST = ::wingdi::PAXESLISTW; /* wingdi.h:4290:20, wingdi.h:4290:20, wingdi.h:4290:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPAXESLIST = ::wingdi::LPAXESLISTW; /* wingdi.h:4291:21, wingdi.h:4291:21, wingdi.h:4291:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct ENUMLOGFONTEXDVA { elfEnumLogfontEx: ::wingdi::ENUMLOGFONTEXA, elfDesignVector: ::wingdi::DESIGNVECTOR } /* wingdi.h:4302:16, wingdi.h:4302:16, wingdi.h:4302:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMLOGFONTEXDVA = *mut ::wingdi::ENUMLOGFONTEXDVA; /* wingdi.h:4306:22, wingdi.h:4306:22, wingdi.h:4306:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMLOGFONTEXDVA = *mut ::wingdi::ENUMLOGFONTEXDVA; /* wingdi.h:4306:46, wingdi.h:4306:46, wingdi.h:4306:46 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct ENUMLOGFONTEXDVW { elfEnumLogfontEx: ::wingdi::ENUMLOGFONTEXW, elfDesignVector: ::wingdi::DESIGNVECTOR } /* wingdi.h:4307:16, wingdi.h:4307:16, wingdi.h:4307:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMLOGFONTEXDVW = *mut ::wingdi::ENUMLOGFONTEXDVW; /* wingdi.h:4311:22, wingdi.h:4311:22, wingdi.h:4311:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMLOGFONTEXDVW = *mut ::wingdi::ENUMLOGFONTEXDVW; /* wingdi.h:4311:46, wingdi.h:4311:46, wingdi.h:4311:46 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type ENUMLOGFONTEXDV = ::wingdi::ENUMLOGFONTEXDVW; /* wingdi.h:4313:26, wingdi.h:4313:26, wingdi.h:4313:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMLOGFONTEXDV = ::wingdi::PENUMLOGFONTEXDVW; /* wingdi.h:4314:27, wingdi.h:4314:27, wingdi.h:4314:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMLOGFONTEXDV = ::wingdi::LPENUMLOGFONTEXDVW; /* wingdi.h:4315:28, wingdi.h:4315:28, wingdi.h:4315:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct ENUMTEXTMETRICA { etmNewTextMetricEx: ::wingdi::NEWTEXTMETRICEXA, etmAxesList: ::wingdi::AXESLISTA } /* wingdi.h:4331:16, wingdi.h:4331:16, wingdi.h:4331:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMTEXTMETRICA = *mut ::wingdi::ENUMTEXTMETRICA; /* wingdi.h:4335:21, wingdi.h:4335:21, wingdi.h:4335:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMTEXTMETRICA = *mut ::wingdi::ENUMTEXTMETRICA; /* wingdi.h:4335:44, wingdi.h:4335:44, wingdi.h:4335:44 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct ENUMTEXTMETRICW { etmNewTextMetricEx: ::wingdi::NEWTEXTMETRICEXW, etmAxesList: ::wingdi::AXESLISTW } /* wingdi.h:4336:16, wingdi.h:4336:16, wingdi.h:4336:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMTEXTMETRICW = *mut ::wingdi::ENUMTEXTMETRICW; /* wingdi.h:4340:21, wingdi.h:4340:21, wingdi.h:4340:21 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMTEXTMETRICW = *mut ::wingdi::ENUMTEXTMETRICW; /* wingdi.h:4340:44, wingdi.h:4340:44, wingdi.h:4340:44 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type ENUMTEXTMETRIC = ::wingdi::ENUMTEXTMETRICW; /* wingdi.h:4342:25, wingdi.h:4342:25, wingdi.h:4342:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PENUMTEXTMETRIC = ::wingdi::PENUMTEXTMETRICW; /* wingdi.h:4343:26, wingdi.h:4343:26, wingdi.h:4343:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPENUMTEXTMETRIC = ::wingdi::LPENUMTEXTMETRICW; /* wingdi.h:4344:27, wingdi.h:4344:27, wingdi.h:4344:27 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type COLOR16 = ::minwindef::USHORT; /* wingdi.h:4511:16, wingdi.h:4511:16, wingdi.h:4511:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct TRIVERTEX { x: ::winnt::LONG, y: ::winnt::LONG, Red: ::wingdi::COLOR16, Green: ::wingdi::COLOR16, Blue: ::wingdi::COLOR16, Alpha: ::wingdi::COLOR16 } /* wingdi.h:4513:16, wingdi.h:4513:16, wingdi.h:4513:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PTRIVERTEX = *mut ::wingdi::TRIVERTEX; /* wingdi.h:4521:13, wingdi.h:4521:13, wingdi.h:4521:13 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPTRIVERTEX = *mut ::wingdi::TRIVERTEX; /* wingdi.h:4521:25, wingdi.h:4521:25, wingdi.h:4521:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct GRADIENT_TRIANGLE { Vertex1: ::minwindef::ULONG, Vertex2: ::minwindef::ULONG, Vertex3: ::minwindef::ULONG } /* wingdi.h:4529:16, wingdi.h:4529:16, wingdi.h:4529:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PGRADIENT_TRIANGLE = *mut ::wingdi::GRADIENT_TRIANGLE; /* wingdi.h:4534:22, wingdi.h:4534:22, wingdi.h:4534:22 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPGRADIENT_TRIANGLE = *mut ::wingdi::GRADIENT_TRIANGLE; /* wingdi.h:4534:42, wingdi.h:4534:42, wingdi.h:4534:42 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct GRADIENT_RECT { UpperLeft: ::minwindef::ULONG, LowerRight: ::minwindef::ULONG } /* wingdi.h:4536:16, wingdi.h:4536:16, wingdi.h:4536:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PGRADIENT_RECT = *mut ::wingdi::GRADIENT_RECT; /* wingdi.h:4540:17, wingdi.h:4540:17, wingdi.h:4540:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type LPGRADIENT_RECT = *mut ::wingdi::GRADIENT_RECT; /* wingdi.h:4540:33, wingdi.h:4540:33, wingdi.h:4540:33 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct BLENDFUNCTION { BlendOp: ::minwindef::BYTE, BlendFlags: ::minwindef::BYTE, SourceConstantAlpha: ::minwindef::BYTE, AlphaFormat: ::minwindef::BYTE } /* wingdi.h:4548:16, wingdi.h:4548:16, wingdi.h:4548:16 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PBLENDFUNCTION = *mut ::wingdi::BLENDFUNCTION; /* wingdi.h:4554:17, wingdi.h:4554:17, wingdi.h:4554:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type MFENUMPROC = extern "system" fn(*mut ::windef::HDC__, *mut ::wingdi::HANDLETABLE, *mut ::wingdi::METARECORD, ::libc::c_int, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:4657:24, wingdi.h:4657:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ENHMFENUMPROC = extern "system" fn(*mut ::windef::HDC__, *mut ::wingdi::HANDLETABLE, *const ::wingdi::ENHMETARECORD, ::libc::c_int, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:4660:24, wingdi.h:4660:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DIBSECTION { dsBm: ::wingdi::BITMAP, dsBmih: ::wingdi::BITMAPINFOHEADER, dsBitfields: *mut [::minwindef::DWORD; 3], dshSection: ::winnt::HANDLE, dsOffset: ::minwindef::DWORD } /* wingdi.h:4769:16, wingdi.h:4769:16, wingdi.h:4769:16 */
#[cfg(feature="winapi_desktop")] pub type LPDIBSECTION = *mut ::wingdi::DIBSECTION; /* wingdi.h:4775:20, wingdi.h:4775:20, wingdi.h:4775:20 */
#[cfg(feature="winapi_desktop")] pub type PDIBSECTION = *mut ::wingdi::DIBSECTION; /* wingdi.h:4775:35, wingdi.h:4775:35, wingdi.h:4775:35 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct COLORADJUSTMENT { caSize: ::minwindef::WORD, caFlags: ::minwindef::WORD, caIlluminantIndex: ::minwindef::WORD, caRedGamma: ::minwindef::WORD, caGreenGamma: ::minwindef::WORD, caBlueGamma: ::minwindef::WORD, caReferenceBlack: ::minwindef::WORD, caReferenceWhite: ::minwindef::WORD, caContrast: ::winnt::SHORT, caBrightness: ::winnt::SHORT, caColorfulness: ::winnt::SHORT, caRedGreenTint: ::winnt::SHORT } /* wingdi.h:4846:17, wingdi.h:4846:17, wingdi.h:4846:17 */
#[cfg(feature="winapi_desktop")] pub type PCOLORADJUSTMENT = *mut ::wingdi::COLORADJUSTMENT; /* wingdi.h:4859:21, wingdi.h:4859:21, wingdi.h:4859:21 */
#[cfg(feature="winapi_desktop")] pub type LPCOLORADJUSTMENT = *mut ::wingdi::COLORADJUSTMENT; /* wingdi.h:4859:44, wingdi.h:4859:44, wingdi.h:4859:44 */
#[cfg(feature="winapi_desktop")] pub type ABORTPROC = extern "system" fn(*mut ::windef::HDC__, ::libc::c_int) -> ::libc::c_int; /* wingdi.h:4866:25, wingdi.h:4866:25, wingdi.h:4866:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DOCINFOA { cbSize: ::libc::c_int, lpszDocName: ::winnt::LPCSTR, lpszOutput: ::winnt::LPCSTR, lpszDatatype: ::winnt::LPCSTR, fwType: ::minwindef::DWORD } /* wingdi.h:4871:16, wingdi.h:4871:16, wingdi.h:4871:16 */
#[cfg(feature="winapi_desktop")] pub type LPDOCINFOA = *mut ::wingdi::DOCINFOA; /* wingdi.h:4879:14, wingdi.h:4879:14, wingdi.h:4879:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct DOCINFOW { cbSize: ::libc::c_int, lpszDocName: ::winnt::LPCWSTR, lpszOutput: ::winnt::LPCWSTR, lpszDatatype: ::winnt::LPCWSTR, fwType: ::minwindef::DWORD } /* wingdi.h:4880:16, wingdi.h:4880:16, wingdi.h:4880:16 */
#[cfg(feature="winapi_desktop")] pub type LPDOCINFOW = *mut ::wingdi::DOCINFOW; /* wingdi.h:4888:14, wingdi.h:4888:14, wingdi.h:4888:14 */
#[cfg(feature="winapi_desktop")] pub type DOCINFO = ::wingdi::DOCINFOW; /* wingdi.h:4890:18, wingdi.h:4890:18, wingdi.h:4890:18 */
#[cfg(feature="winapi_desktop")] pub type LPDOCINFO = ::wingdi::LPDOCINFOW; /* wingdi.h:4891:20, wingdi.h:4891:20, wingdi.h:4891:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct KERNINGPAIR { wFirst: ::minwindef::WORD, wSecond: ::minwindef::WORD, iKernAmount: ::libc::c_int } /* wingdi.h:5026:16, wingdi.h:5026:16, wingdi.h:5026:16 */
#[cfg(feature="winapi_desktop")] pub type LPKERNINGPAIR = *mut ::wingdi::KERNINGPAIR; /* wingdi.h:5030:17, wingdi.h:5030:17, wingdi.h:5030:17 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ICMENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:5060:24, wingdi.h:5060:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86", target_arch="arm"))] pub type ICMENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_long) -> ::libc::c_int; /* wingdi.h:5061:24, wingdi.h:5061:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMR { iType: ::minwindef::DWORD, nSize: ::minwindef::DWORD } /* wingdi.h:5305:16, wingdi.h:5305:16, wingdi.h:5305:16 */
#[cfg(feature="winapi_desktop")] pub type PEMR = *mut ::wingdi::EMR; /* wingdi.h:5310:9, wingdi.h:5310:9, wingdi.h:5310:9 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRTEXT { ptlReference: ::windef::POINTL, nChars: ::minwindef::DWORD, offString: ::minwindef::DWORD, fOptions: ::minwindef::DWORD, rcl: ::windef::RECTL, offDx: ::minwindef::DWORD } /* wingdi.h:5314:16, wingdi.h:5314:16, wingdi.h:5314:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRTEXT = *mut ::wingdi::EMRTEXT; /* wingdi.h:5323:13, wingdi.h:5323:13, wingdi.h:5323:13 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRABORTPATH { emr: ::wingdi::EMR } /* wingdi.h:5327:16, wingdi.h:5327:16, wingdi.h:5327:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRABORTPATH = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5330:23, wingdi.h:5330:23, wingdi.h:5330:23 */
#[cfg(feature="winapi_desktop")] pub type EMRBEGINPATH = ::wingdi::EMRABORTPATH; /* wingdi.h:5331:3, wingdi.h:5331:3, wingdi.h:5331:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRBEGINPATH = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5331:23, wingdi.h:5331:23, wingdi.h:5331:23 */
#[cfg(feature="winapi_desktop")] pub type EMRENDPATH = ::wingdi::EMRABORTPATH; /* wingdi.h:5332:3, wingdi.h:5332:3, wingdi.h:5332:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRENDPATH = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5332:23, wingdi.h:5332:23, wingdi.h:5332:23 */
#[cfg(feature="winapi_desktop")] pub type EMRCLOSEFIGURE = ::wingdi::EMRABORTPATH; /* wingdi.h:5333:3, wingdi.h:5333:3, wingdi.h:5333:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRCLOSEFIGURE = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5333:23, wingdi.h:5333:23, wingdi.h:5333:23 */
#[cfg(feature="winapi_desktop")] pub type EMRFLATTENPATH = ::wingdi::EMRABORTPATH; /* wingdi.h:5334:3, wingdi.h:5334:3, wingdi.h:5334:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRFLATTENPATH = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5334:23, wingdi.h:5334:23, wingdi.h:5334:23 */
#[cfg(feature="winapi_desktop")] pub type EMRWIDENPATH = ::wingdi::EMRABORTPATH; /* wingdi.h:5335:3, wingdi.h:5335:3, wingdi.h:5335:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRWIDENPATH = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5335:23, wingdi.h:5335:23, wingdi.h:5335:23 */
#[cfg(feature="winapi_desktop")] pub type EMRSETMETARGN = ::wingdi::EMRABORTPATH; /* wingdi.h:5336:3, wingdi.h:5336:3, wingdi.h:5336:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETMETARGN = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5336:23, wingdi.h:5336:23, wingdi.h:5336:23 */
#[cfg(feature="winapi_desktop")] pub type EMRSAVEDC = ::wingdi::EMRABORTPATH; /* wingdi.h:5337:3, wingdi.h:5337:3, wingdi.h:5337:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSAVEDC = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5337:23, wingdi.h:5337:23, wingdi.h:5337:23 */
#[cfg(feature="winapi_desktop")] pub type EMRREALIZEPALETTE = ::wingdi::EMRABORTPATH; /* wingdi.h:5338:3, wingdi.h:5338:3, wingdi.h:5338:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRREALIZEPALETTE = *mut ::wingdi::EMRABORTPATH; /* wingdi.h:5338:23, wingdi.h:5338:23, wingdi.h:5338:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSELECTCLIPPATH { emr: ::wingdi::EMR, iMode: ::minwindef::DWORD } /* wingdi.h:5340:16, wingdi.h:5340:16, wingdi.h:5340:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSELECTCLIPPATH = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5344:26, wingdi.h:5344:26, wingdi.h:5344:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETBKMODE = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5345:3, wingdi.h:5345:3, wingdi.h:5345:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETBKMODE = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5345:26, wingdi.h:5345:26, wingdi.h:5345:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETMAPMODE = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5346:3, wingdi.h:5346:3, wingdi.h:5346:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETMAPMODE = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5346:26, wingdi.h:5346:26, wingdi.h:5346:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type EMRSETLAYOUT = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5348:3, wingdi.h:5348:3, wingdi.h:5348:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRSETLAYOUT = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5348:26, wingdi.h:5348:26, wingdi.h:5348:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETPOLYFILLMODE = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5350:3, wingdi.h:5350:3, wingdi.h:5350:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETPOLYFILLMODE = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5350:26, wingdi.h:5350:26, wingdi.h:5350:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETROP2 = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5351:3, wingdi.h:5351:3, wingdi.h:5351:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETROP2 = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5351:26, wingdi.h:5351:26, wingdi.h:5351:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETSTRETCHBLTMODE = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5352:3, wingdi.h:5352:3, wingdi.h:5352:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETSTRETCHBLTMODE = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5352:26, wingdi.h:5352:26, wingdi.h:5352:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETICMMODE = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5353:3, wingdi.h:5353:3, wingdi.h:5353:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETICMMODE = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5353:26, wingdi.h:5353:26, wingdi.h:5353:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSETTEXTALIGN = ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5354:3, wingdi.h:5354:3, wingdi.h:5354:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETTEXTALIGN = *mut ::wingdi::EMRSELECTCLIPPATH; /* wingdi.h:5354:26, wingdi.h:5354:26, wingdi.h:5354:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETMITERLIMIT { emr: ::wingdi::EMR, eMiterLimit: ::minwindef::FLOAT } /* wingdi.h:5356:16, wingdi.h:5356:16, wingdi.h:5356:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETMITERLIMIT = *mut ::wingdi::EMRSETMITERLIMIT; /* wingdi.h:5360:22, wingdi.h:5360:22, wingdi.h:5360:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRRESTOREDC { emr: ::wingdi::EMR, iRelative: ::winnt::LONG } /* wingdi.h:5362:16, wingdi.h:5362:16, wingdi.h:5362:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRRESTOREDC = *mut ::wingdi::EMRRESTOREDC; /* wingdi.h:5366:18, wingdi.h:5366:18, wingdi.h:5366:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETARCDIRECTION { emr: ::wingdi::EMR, iArcDirection: ::minwindef::DWORD } /* wingdi.h:5368:16, wingdi.h:5368:16, wingdi.h:5368:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETARCDIRECTION = *mut ::wingdi::EMRSETARCDIRECTION; /* wingdi.h:5373:24, wingdi.h:5373:24, wingdi.h:5373:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETMAPPERFLAGS { emr: ::wingdi::EMR, dwFlags: ::minwindef::DWORD } /* wingdi.h:5375:16, wingdi.h:5375:16, wingdi.h:5375:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETMAPPERFLAGS = *mut ::wingdi::EMRSETMAPPERFLAGS; /* wingdi.h:5379:23, wingdi.h:5379:23, wingdi.h:5379:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETBKCOLOR { emr: ::wingdi::EMR, crColor: ::windef::COLORREF } /* wingdi.h:5381:16, wingdi.h:5381:16, wingdi.h:5381:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETBKCOLOR = *mut ::wingdi::EMRSETBKCOLOR; /* wingdi.h:5385:21, wingdi.h:5385:21, wingdi.h:5385:21 */
#[cfg(feature="winapi_desktop")] pub type EMRSETTEXTCOLOR = ::wingdi::EMRSETBKCOLOR; /* wingdi.h:5386:3, wingdi.h:5386:3, wingdi.h:5386:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETTEXTCOLOR = *mut ::wingdi::EMRSETBKCOLOR; /* wingdi.h:5386:21, wingdi.h:5386:21, wingdi.h:5386:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSELECTOBJECT { emr: ::wingdi::EMR, ihObject: ::minwindef::DWORD } /* wingdi.h:5388:16, wingdi.h:5388:16, wingdi.h:5388:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSELECTOBJECT = *mut ::wingdi::EMRSELECTOBJECT; /* wingdi.h:5392:21, wingdi.h:5392:21, wingdi.h:5392:21 */
#[cfg(feature="winapi_desktop")] pub type EMRDELETEOBJECT = ::wingdi::EMRSELECTOBJECT; /* wingdi.h:5393:3, wingdi.h:5393:3, wingdi.h:5393:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRDELETEOBJECT = *mut ::wingdi::EMRSELECTOBJECT; /* wingdi.h:5393:21, wingdi.h:5393:21, wingdi.h:5393:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSELECTPALETTE { emr: ::wingdi::EMR, ihPal: ::minwindef::DWORD } /* wingdi.h:5395:16, wingdi.h:5395:16, wingdi.h:5395:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSELECTPALETTE = *mut ::wingdi::EMRSELECTPALETTE; /* wingdi.h:5399:22, wingdi.h:5399:22, wingdi.h:5399:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRRESIZEPALETTE { emr: ::wingdi::EMR, ihPal: ::minwindef::DWORD, cEntries: ::minwindef::DWORD } /* wingdi.h:5401:16, wingdi.h:5401:16, wingdi.h:5401:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRRESIZEPALETTE = *mut ::wingdi::EMRRESIZEPALETTE; /* wingdi.h:5406:22, wingdi.h:5406:22, wingdi.h:5406:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETPALETTEENTRIES { emr: ::wingdi::EMR, ihPal: ::minwindef::DWORD, iStart: ::minwindef::DWORD, cEntries: ::minwindef::DWORD, aPalEntries: *mut [::wingdi::PALETTEENTRY; 1] } /* wingdi.h:5408:16, wingdi.h:5408:16, wingdi.h:5408:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETPALETTEENTRIES = *mut ::wingdi::EMRSETPALETTEENTRIES; /* wingdi.h:5415:26, wingdi.h:5415:26, wingdi.h:5415:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETCOLORADJUSTMENT { emr: ::wingdi::EMR, ColorAdjustment: ::wingdi::COLORADJUSTMENT } /* wingdi.h:5417:16, wingdi.h:5417:16, wingdi.h:5417:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETCOLORADJUSTMENT = *mut ::wingdi::EMRSETCOLORADJUSTMENT; /* wingdi.h:5421:27, wingdi.h:5421:27, wingdi.h:5421:27 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRGDICOMMENT { emr: ::wingdi::EMR, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5423:16, wingdi.h:5423:16, wingdi.h:5423:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRGDICOMMENT = *mut ::wingdi::EMRGDICOMMENT; /* wingdi.h:5428:19, wingdi.h:5428:19, wingdi.h:5428:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREOF { emr: ::wingdi::EMR, nPalEntries: ::minwindef::DWORD, offPalEntries: ::minwindef::DWORD, nSizeLast: ::minwindef::DWORD } /* wingdi.h:5430:16, wingdi.h:5430:16, wingdi.h:5430:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREOF = *mut ::wingdi::EMREOF; /* wingdi.h:5438:12, wingdi.h:5438:12, wingdi.h:5438:12 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRLINETO { emr: ::wingdi::EMR, ptl: ::windef::POINTL } /* wingdi.h:5440:16, wingdi.h:5440:16, wingdi.h:5440:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRLINETO = *mut ::wingdi::EMRLINETO; /* wingdi.h:5444:17, wingdi.h:5444:17, wingdi.h:5444:17 */
#[cfg(feature="winapi_desktop")] pub type EMRMOVETOEX = ::wingdi::EMRLINETO; /* wingdi.h:5445:3, wingdi.h:5445:3, wingdi.h:5445:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRMOVETOEX = *mut ::wingdi::EMRLINETO; /* wingdi.h:5445:17, wingdi.h:5445:17, wingdi.h:5445:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMROFFSETCLIPRGN { emr: ::wingdi::EMR, ptlOffset: ::windef::POINTL } /* wingdi.h:5447:16, wingdi.h:5447:16, wingdi.h:5447:16 */
#[cfg(feature="winapi_desktop")] pub type PEMROFFSETCLIPRGN = *mut ::wingdi::EMROFFSETCLIPRGN; /* wingdi.h:5451:22, wingdi.h:5451:22, wingdi.h:5451:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRFILLPATH { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL } /* wingdi.h:5453:16, wingdi.h:5453:16, wingdi.h:5453:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRFILLPATH = *mut ::wingdi::EMRFILLPATH; /* wingdi.h:5457:26, wingdi.h:5457:26, wingdi.h:5457:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSTROKEANDFILLPATH = ::wingdi::EMRFILLPATH; /* wingdi.h:5458:3, wingdi.h:5458:3, wingdi.h:5458:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSTROKEANDFILLPATH = *mut ::wingdi::EMRFILLPATH; /* wingdi.h:5458:26, wingdi.h:5458:26, wingdi.h:5458:26 */
#[cfg(feature="winapi_desktop")] pub type EMRSTROKEPATH = ::wingdi::EMRFILLPATH; /* wingdi.h:5459:3, wingdi.h:5459:3, wingdi.h:5459:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSTROKEPATH = *mut ::wingdi::EMRFILLPATH; /* wingdi.h:5459:26, wingdi.h:5459:26, wingdi.h:5459:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXCLUDECLIPRECT { emr: ::wingdi::EMR, rclClip: ::windef::RECTL } /* wingdi.h:5461:16, wingdi.h:5461:16, wingdi.h:5461:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXCLUDECLIPRECT = *mut ::wingdi::EMREXCLUDECLIPRECT; /* wingdi.h:5465:26, wingdi.h:5465:26, wingdi.h:5465:26 */
#[cfg(feature="winapi_desktop")] pub type EMRINTERSECTCLIPRECT = ::wingdi::EMREXCLUDECLIPRECT; /* wingdi.h:5466:3, wingdi.h:5466:3, wingdi.h:5466:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRINTERSECTCLIPRECT = *mut ::wingdi::EMREXCLUDECLIPRECT; /* wingdi.h:5466:26, wingdi.h:5466:26, wingdi.h:5466:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETVIEWPORTORGEX { emr: ::wingdi::EMR, ptlOrigin: ::windef::POINTL } /* wingdi.h:5468:16, wingdi.h:5468:16, wingdi.h:5468:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETVIEWPORTORGEX = *mut ::wingdi::EMRSETVIEWPORTORGEX; /* wingdi.h:5472:25, wingdi.h:5472:25, wingdi.h:5472:25 */
#[cfg(feature="winapi_desktop")] pub type EMRSETWINDOWORGEX = ::wingdi::EMRSETVIEWPORTORGEX; /* wingdi.h:5473:3, wingdi.h:5473:3, wingdi.h:5473:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETWINDOWORGEX = *mut ::wingdi::EMRSETVIEWPORTORGEX; /* wingdi.h:5473:25, wingdi.h:5473:25, wingdi.h:5473:25 */
#[cfg(feature="winapi_desktop")] pub type EMRSETBRUSHORGEX = ::wingdi::EMRSETVIEWPORTORGEX; /* wingdi.h:5474:3, wingdi.h:5474:3, wingdi.h:5474:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETBRUSHORGEX = *mut ::wingdi::EMRSETVIEWPORTORGEX; /* wingdi.h:5474:25, wingdi.h:5474:25, wingdi.h:5474:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETVIEWPORTEXTEX { emr: ::wingdi::EMR, szlExtent: ::windef::SIZEL } /* wingdi.h:5476:16, wingdi.h:5476:16, wingdi.h:5476:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETVIEWPORTEXTEX = *mut ::wingdi::EMRSETVIEWPORTEXTEX; /* wingdi.h:5480:25, wingdi.h:5480:25, wingdi.h:5480:25 */
#[cfg(feature="winapi_desktop")] pub type EMRSETWINDOWEXTEX = ::wingdi::EMRSETVIEWPORTEXTEX; /* wingdi.h:5481:3, wingdi.h:5481:3, wingdi.h:5481:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETWINDOWEXTEX = *mut ::wingdi::EMRSETVIEWPORTEXTEX; /* wingdi.h:5481:25, wingdi.h:5481:25, wingdi.h:5481:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSCALEVIEWPORTEXTEX { emr: ::wingdi::EMR, xNum: ::winnt::LONG, xDenom: ::winnt::LONG, yNum: ::winnt::LONG, yDenom: ::winnt::LONG } /* wingdi.h:5483:16, wingdi.h:5483:16, wingdi.h:5483:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSCALEVIEWPORTEXTEX = *mut ::wingdi::EMRSCALEVIEWPORTEXTEX; /* wingdi.h:5490:27, wingdi.h:5490:27, wingdi.h:5490:27 */
#[cfg(feature="winapi_desktop")] pub type EMRSCALEWINDOWEXTEX = ::wingdi::EMRSCALEVIEWPORTEXTEX; /* wingdi.h:5491:3, wingdi.h:5491:3, wingdi.h:5491:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRSCALEWINDOWEXTEX = *mut ::wingdi::EMRSCALEVIEWPORTEXTEX; /* wingdi.h:5491:27, wingdi.h:5491:27, wingdi.h:5491:27 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETWORLDTRANSFORM { emr: ::wingdi::EMR, xform: ::wingdi::XFORM } /* wingdi.h:5493:16, wingdi.h:5493:16, wingdi.h:5493:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETWORLDTRANSFORM = *mut ::wingdi::EMRSETWORLDTRANSFORM; /* wingdi.h:5497:26, wingdi.h:5497:26, wingdi.h:5497:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRMODIFYWORLDTRANSFORM { emr: ::wingdi::EMR, xform: ::wingdi::XFORM, iMode: ::minwindef::DWORD } /* wingdi.h:5499:16, wingdi.h:5499:16, wingdi.h:5499:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRMODIFYWORLDTRANSFORM = *mut ::wingdi::EMRMODIFYWORLDTRANSFORM; /* wingdi.h:5504:29, wingdi.h:5504:29, wingdi.h:5504:29 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETPIXELV { emr: ::wingdi::EMR, ptlPixel: ::windef::POINTL, crColor: ::windef::COLORREF } /* wingdi.h:5506:16, wingdi.h:5506:16, wingdi.h:5506:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETPIXELV = *mut ::wingdi::EMRSETPIXELV; /* wingdi.h:5511:18, wingdi.h:5511:18, wingdi.h:5511:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXTFLOODFILL { emr: ::wingdi::EMR, ptlStart: ::windef::POINTL, crColor: ::windef::COLORREF, iMode: ::minwindef::DWORD } /* wingdi.h:5513:16, wingdi.h:5513:16, wingdi.h:5513:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTFLOODFILL = *mut ::wingdi::EMREXTFLOODFILL; /* wingdi.h:5519:21, wingdi.h:5519:21, wingdi.h:5519:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRELLIPSE { emr: ::wingdi::EMR, rclBox: ::windef::RECTL } /* wingdi.h:5521:16, wingdi.h:5521:16, wingdi.h:5521:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRELLIPSE = *mut ::wingdi::EMRELLIPSE; /* wingdi.h:5525:17, wingdi.h:5525:17, wingdi.h:5525:17 */
#[cfg(feature="winapi_desktop")] pub type EMRRECTANGLE = ::wingdi::EMRELLIPSE; /* wingdi.h:5526:3, wingdi.h:5526:3, wingdi.h:5526:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRRECTANGLE = *mut ::wingdi::EMRELLIPSE; /* wingdi.h:5526:18, wingdi.h:5526:18, wingdi.h:5526:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRROUNDRECT { emr: ::wingdi::EMR, rclBox: ::windef::RECTL, szlCorner: ::windef::SIZEL } /* wingdi.h:5529:16, wingdi.h:5529:16, wingdi.h:5529:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRROUNDRECT = *mut ::wingdi::EMRROUNDRECT; /* wingdi.h:5534:18, wingdi.h:5534:18, wingdi.h:5534:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRARC { emr: ::wingdi::EMR, rclBox: ::windef::RECTL, ptlStart: ::windef::POINTL, ptlEnd: ::windef::POINTL } /* wingdi.h:5536:16, wingdi.h:5536:16, wingdi.h:5536:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRARC = *mut ::wingdi::EMRARC; /* wingdi.h:5542:14, wingdi.h:5542:14, wingdi.h:5542:14 */
#[cfg(feature="winapi_desktop")] pub type EMRARCTO = ::wingdi::EMRARC; /* wingdi.h:5543:3, wingdi.h:5543:3, wingdi.h:5543:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRARCTO = *mut ::wingdi::EMRARC; /* wingdi.h:5543:14, wingdi.h:5543:14, wingdi.h:5543:14 */
#[cfg(feature="winapi_desktop")] pub type EMRCHORD = ::wingdi::EMRARC; /* wingdi.h:5544:3, wingdi.h:5544:3, wingdi.h:5544:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRCHORD = *mut ::wingdi::EMRARC; /* wingdi.h:5544:14, wingdi.h:5544:14, wingdi.h:5544:14 */
#[cfg(feature="winapi_desktop")] pub type EMRPIE = ::wingdi::EMRARC; /* wingdi.h:5545:3, wingdi.h:5545:3, wingdi.h:5545:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPIE = *mut ::wingdi::EMRARC; /* wingdi.h:5545:14, wingdi.h:5545:14, wingdi.h:5545:14 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRANGLEARC { emr: ::wingdi::EMR, ptlCenter: ::windef::POINTL, nRadius: ::minwindef::DWORD, eStartAngle: ::minwindef::FLOAT, eSweepAngle: ::minwindef::FLOAT } /* wingdi.h:5547:16, wingdi.h:5547:16, wingdi.h:5547:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRANGLEARC = *mut ::wingdi::EMRANGLEARC; /* wingdi.h:5554:17, wingdi.h:5554:17, wingdi.h:5554:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYLINE { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cptl: ::minwindef::DWORD, aptl: *mut [::windef::POINTL; 1] } /* wingdi.h:5556:16, wingdi.h:5556:16, wingdi.h:5556:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYLINE = *mut ::wingdi::EMRPOLYLINE; /* wingdi.h:5562:21, wingdi.h:5562:21, wingdi.h:5562:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYBEZIER = ::wingdi::EMRPOLYLINE; /* wingdi.h:5563:3, wingdi.h:5563:3, wingdi.h:5563:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYBEZIER = *mut ::wingdi::EMRPOLYLINE; /* wingdi.h:5563:21, wingdi.h:5563:21, wingdi.h:5563:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYGON = ::wingdi::EMRPOLYLINE; /* wingdi.h:5564:3, wingdi.h:5564:3, wingdi.h:5564:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYGON = *mut ::wingdi::EMRPOLYLINE; /* wingdi.h:5564:21, wingdi.h:5564:21, wingdi.h:5564:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYBEZIERTO = ::wingdi::EMRPOLYLINE; /* wingdi.h:5565:3, wingdi.h:5565:3, wingdi.h:5565:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYBEZIERTO = *mut ::wingdi::EMRPOLYLINE; /* wingdi.h:5565:21, wingdi.h:5565:21, wingdi.h:5565:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYLINETO = ::wingdi::EMRPOLYLINE; /* wingdi.h:5566:3, wingdi.h:5566:3, wingdi.h:5566:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYLINETO = *mut ::wingdi::EMRPOLYLINE; /* wingdi.h:5566:21, wingdi.h:5566:21, wingdi.h:5566:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYLINE16 { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cpts: ::minwindef::DWORD, apts: *mut [::windef::POINTS; 1] } /* wingdi.h:5568:16, wingdi.h:5568:16, wingdi.h:5568:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYLINE16 = *mut ::wingdi::EMRPOLYLINE16; /* wingdi.h:5574:23, wingdi.h:5574:23, wingdi.h:5574:23 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYBEZIER16 = ::wingdi::EMRPOLYLINE16; /* wingdi.h:5575:3, wingdi.h:5575:3, wingdi.h:5575:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYBEZIER16 = *mut ::wingdi::EMRPOLYLINE16; /* wingdi.h:5575:23, wingdi.h:5575:23, wingdi.h:5575:23 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYGON16 = ::wingdi::EMRPOLYLINE16; /* wingdi.h:5576:3, wingdi.h:5576:3, wingdi.h:5576:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYGON16 = *mut ::wingdi::EMRPOLYLINE16; /* wingdi.h:5576:23, wingdi.h:5576:23, wingdi.h:5576:23 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYBEZIERTO16 = ::wingdi::EMRPOLYLINE16; /* wingdi.h:5577:3, wingdi.h:5577:3, wingdi.h:5577:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYBEZIERTO16 = *mut ::wingdi::EMRPOLYLINE16; /* wingdi.h:5577:23, wingdi.h:5577:23, wingdi.h:5577:23 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYLINETO16 = ::wingdi::EMRPOLYLINE16; /* wingdi.h:5578:3, wingdi.h:5578:3, wingdi.h:5578:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYLINETO16 = *mut ::wingdi::EMRPOLYLINE16; /* wingdi.h:5578:23, wingdi.h:5578:23, wingdi.h:5578:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYDRAW { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cptl: ::minwindef::DWORD, aptl: *mut [::windef::POINTL; 1], abTypes: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5580:16, wingdi.h:5580:16, wingdi.h:5580:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYDRAW = *mut ::wingdi::EMRPOLYDRAW; /* wingdi.h:5587:17, wingdi.h:5587:17, wingdi.h:5587:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYDRAW16 { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cpts: ::minwindef::DWORD, apts: *mut [::windef::POINTS; 1], abTypes: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5589:16, wingdi.h:5589:16, wingdi.h:5589:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYDRAW16 = *mut ::wingdi::EMRPOLYDRAW16; /* wingdi.h:5596:19, wingdi.h:5596:19, wingdi.h:5596:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYPOLYLINE { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, nPolys: ::minwindef::DWORD, cptl: ::minwindef::DWORD, aPolyCounts: *mut [::minwindef::DWORD; 1], aptl: *mut [::windef::POINTL; 1] } /* wingdi.h:5598:16, wingdi.h:5598:16, wingdi.h:5598:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYPOLYLINE = *mut ::wingdi::EMRPOLYPOLYLINE; /* wingdi.h:5606:21, wingdi.h:5606:21, wingdi.h:5606:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYPOLYGON = ::wingdi::EMRPOLYPOLYLINE; /* wingdi.h:5607:3, wingdi.h:5607:3, wingdi.h:5607:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYPOLYGON = *mut ::wingdi::EMRPOLYPOLYLINE; /* wingdi.h:5607:21, wingdi.h:5607:21, wingdi.h:5607:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYPOLYLINE16 { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, nPolys: ::minwindef::DWORD, cpts: ::minwindef::DWORD, aPolyCounts: *mut [::minwindef::DWORD; 1], apts: *mut [::windef::POINTS; 1] } /* wingdi.h:5609:16, wingdi.h:5609:16, wingdi.h:5609:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYPOLYLINE16 = *mut ::wingdi::EMRPOLYPOLYLINE16; /* wingdi.h:5617:23, wingdi.h:5617:23, wingdi.h:5617:23 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYPOLYGON16 = ::wingdi::EMRPOLYPOLYLINE16; /* wingdi.h:5618:3, wingdi.h:5618:3, wingdi.h:5618:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYPOLYGON16 = *mut ::wingdi::EMRPOLYPOLYLINE16; /* wingdi.h:5618:23, wingdi.h:5618:23, wingdi.h:5618:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRINVERTRGN { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cbRgnData: ::minwindef::DWORD, RgnData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5620:16, wingdi.h:5620:16, wingdi.h:5620:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRINVERTRGN = *mut ::wingdi::EMRINVERTRGN; /* wingdi.h:5626:18, wingdi.h:5626:18, wingdi.h:5626:18 */
#[cfg(feature="winapi_desktop")] pub type EMRPAINTRGN = ::wingdi::EMRINVERTRGN; /* wingdi.h:5627:3, wingdi.h:5627:3, wingdi.h:5627:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPAINTRGN = *mut ::wingdi::EMRINVERTRGN; /* wingdi.h:5627:18, wingdi.h:5627:18, wingdi.h:5627:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRFILLRGN { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cbRgnData: ::minwindef::DWORD, ihBrush: ::minwindef::DWORD, RgnData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5629:16, wingdi.h:5629:16, wingdi.h:5629:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRFILLRGN = *mut ::wingdi::EMRFILLRGN; /* wingdi.h:5636:16, wingdi.h:5636:16, wingdi.h:5636:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRFRAMERGN { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cbRgnData: ::minwindef::DWORD, ihBrush: ::minwindef::DWORD, szlStroke: ::windef::SIZEL, RgnData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5638:16, wingdi.h:5638:16, wingdi.h:5638:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRFRAMERGN = *mut ::wingdi::EMRFRAMERGN; /* wingdi.h:5646:17, wingdi.h:5646:17, wingdi.h:5646:17 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXTSELECTCLIPRGN { emr: ::wingdi::EMR, cbRgnData: ::minwindef::DWORD, iMode: ::minwindef::DWORD, RgnData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5648:16, wingdi.h:5648:16, wingdi.h:5648:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTSELECTCLIPRGN = *mut ::wingdi::EMREXTSELECTCLIPRGN; /* wingdi.h:5654:25, wingdi.h:5654:25, wingdi.h:5654:25 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXTTEXTOUTA { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, iGraphicsMode: ::minwindef::DWORD, exScale: ::minwindef::FLOAT, eyScale: ::minwindef::FLOAT, emrtext: ::wingdi::EMRTEXT } /* wingdi.h:5656:16, wingdi.h:5656:16, wingdi.h:5656:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTTEXTOUTA = *mut ::wingdi::EMREXTTEXTOUTA; /* wingdi.h:5665:20, wingdi.h:5665:20, wingdi.h:5665:20 */
#[cfg(feature="winapi_desktop")] pub type EMREXTTEXTOUTW = ::wingdi::EMREXTTEXTOUTA; /* wingdi.h:5666:3, wingdi.h:5666:3, wingdi.h:5666:3 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTTEXTOUTW = *mut ::wingdi::EMREXTTEXTOUTA; /* wingdi.h:5666:20, wingdi.h:5666:20, wingdi.h:5666:20 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPOLYTEXTOUTA { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, iGraphicsMode: ::minwindef::DWORD, exScale: ::minwindef::FLOAT, eyScale: ::minwindef::FLOAT, cStrings: ::winnt::LONG, aemrtext: *mut [::wingdi::EMRTEXT; 1] } /* wingdi.h:5668:16, wingdi.h:5668:16, wingdi.h:5668:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYTEXTOUTA = *mut ::wingdi::EMRPOLYTEXTOUTA; /* wingdi.h:5678:21, wingdi.h:5678:21, wingdi.h:5678:21 */
#[cfg(feature="winapi_desktop")] pub type EMRPOLYTEXTOUTW = ::wingdi::EMRPOLYTEXTOUTA; /* wingdi.h:5679:3, wingdi.h:5679:3, wingdi.h:5679:3 */
#[cfg(feature="winapi_desktop")] pub type PEMRPOLYTEXTOUTW = *mut ::wingdi::EMRPOLYTEXTOUTA; /* wingdi.h:5679:21, wingdi.h:5679:21, wingdi.h:5679:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRBITBLT { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG, dwRop: ::minwindef::DWORD, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD } /* wingdi.h:5681:16, wingdi.h:5681:16, wingdi.h:5681:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRBITBLT = *mut ::wingdi::EMRBITBLT; /* wingdi.h:5700:15, wingdi.h:5700:15, wingdi.h:5700:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSTRETCHBLT { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG, dwRop: ::minwindef::DWORD, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG } /* wingdi.h:5702:16, wingdi.h:5702:16, wingdi.h:5702:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSTRETCHBLT = *mut ::wingdi::EMRSTRETCHBLT; /* wingdi.h:5723:19, wingdi.h:5723:19, wingdi.h:5723:19 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRMASKBLT { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG, dwRop: ::minwindef::DWORD, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, xMask: ::winnt::LONG, yMask: ::winnt::LONG, iUsageMask: ::minwindef::DWORD, offBmiMask: ::minwindef::DWORD, cbBmiMask: ::minwindef::DWORD, offBitsMask: ::minwindef::DWORD, cbBitsMask: ::minwindef::DWORD } /* wingdi.h:5725:16, wingdi.h:5725:16, wingdi.h:5725:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRMASKBLT = *mut ::wingdi::EMRMASKBLT; /* wingdi.h:5751:16, wingdi.h:5751:16, wingdi.h:5751:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRPLGBLT { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, aptlDest: *mut [::windef::POINTL; 3], xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, xMask: ::winnt::LONG, yMask: ::winnt::LONG, iUsageMask: ::minwindef::DWORD, offBmiMask: ::minwindef::DWORD, cbBmiMask: ::minwindef::DWORD, offBitsMask: ::minwindef::DWORD, cbBitsMask: ::minwindef::DWORD } /* wingdi.h:5753:16, wingdi.h:5753:16, wingdi.h:5753:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRPLGBLT = *mut ::wingdi::EMRPLGBLT; /* wingdi.h:5777:15, wingdi.h:5777:15, wingdi.h:5777:15 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSETDIBITSTODEVICE { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, iUsageSrc: ::minwindef::DWORD, iStartScan: ::minwindef::DWORD, cScans: ::minwindef::DWORD } /* wingdi.h:5779:16, wingdi.h:5779:16, wingdi.h:5779:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSETDIBITSTODEVICE = *mut ::wingdi::EMRSETDIBITSTODEVICE; /* wingdi.h:5796:26, wingdi.h:5796:26, wingdi.h:5796:26 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRSTRETCHDIBITS { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, iUsageSrc: ::minwindef::DWORD, dwRop: ::minwindef::DWORD, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG } /* wingdi.h:5798:16, wingdi.h:5798:16, wingdi.h:5798:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRSTRETCHDIBITS = *mut ::wingdi::EMRSTRETCHDIBITS; /* wingdi.h:5816:22, wingdi.h:5816:22, wingdi.h:5816:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXTCREATEFONTINDIRECTW { emr: ::wingdi::EMR, ihFont: ::minwindef::DWORD, elfw: ::wingdi::EXTLOGFONTW } /* wingdi.h:5818:16, wingdi.h:5818:16, wingdi.h:5818:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTCREATEFONTINDIRECTW = *mut ::wingdi::EMREXTCREATEFONTINDIRECTW; /* wingdi.h:5823:31, wingdi.h:5823:31, wingdi.h:5823:31 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRCREATEPALETTE { emr: ::wingdi::EMR, ihPal: ::minwindef::DWORD, lgpl: ::wingdi::LOGPALETTE } /* wingdi.h:5825:16, wingdi.h:5825:16, wingdi.h:5825:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRCREATEPALETTE = *mut ::wingdi::EMRCREATEPALETTE; /* wingdi.h:5831:22, wingdi.h:5831:22, wingdi.h:5831:22 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRCREATEPEN { emr: ::wingdi::EMR, ihPen: ::minwindef::DWORD, lopn: ::wingdi::LOGPEN } /* wingdi.h:5833:16, wingdi.h:5833:16, wingdi.h:5833:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRCREATEPEN = *mut ::wingdi::EMRCREATEPEN; /* wingdi.h:5838:18, wingdi.h:5838:18, wingdi.h:5838:18 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMREXTCREATEPEN { emr: ::wingdi::EMR, ihPen: ::minwindef::DWORD, offBmi: ::minwindef::DWORD, cbBmi: ::minwindef::DWORD, offBits: ::minwindef::DWORD, cbBits: ::minwindef::DWORD, elp: ::wingdi::EXTLOGPEN32 } /* wingdi.h:5840:16, wingdi.h:5840:16, wingdi.h:5840:16 */
#[cfg(feature="winapi_desktop")] pub type PEMREXTCREATEPEN = *mut ::wingdi::EMREXTCREATEPEN; /* wingdi.h:5851:21, wingdi.h:5851:21, wingdi.h:5851:21 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRCREATEBRUSHINDIRECT { emr: ::wingdi::EMR, ihBrush: ::minwindef::DWORD, lb: ::wingdi::LOGBRUSH32 } /* wingdi.h:5853:16, wingdi.h:5853:16, wingdi.h:5853:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRCREATEBRUSHINDIRECT = *mut ::wingdi::EMRCREATEBRUSHINDIRECT; /* wingdi.h:5859:28, wingdi.h:5859:28, wingdi.h:5859:28 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRCREATEMONOBRUSH { emr: ::wingdi::EMR, ihBrush: ::minwindef::DWORD, iUsage: ::minwindef::DWORD, offBmi: ::minwindef::DWORD, cbBmi: ::minwindef::DWORD, offBits: ::minwindef::DWORD, cbBits: ::minwindef::DWORD } /* wingdi.h:5861:16, wingdi.h:5861:16, wingdi.h:5861:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRCREATEMONOBRUSH = *mut ::wingdi::EMRCREATEMONOBRUSH; /* wingdi.h:5870:24, wingdi.h:5870:24, wingdi.h:5870:24 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRCREATEDIBPATTERNBRUSHPT { emr: ::wingdi::EMR, ihBrush: ::minwindef::DWORD, iUsage: ::minwindef::DWORD, offBmi: ::minwindef::DWORD, cbBmi: ::minwindef::DWORD, offBits: ::minwindef::DWORD, cbBits: ::minwindef::DWORD } /* wingdi.h:5872:16, wingdi.h:5872:16, wingdi.h:5872:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRCREATEDIBPATTERNBRUSHPT = *mut ::wingdi::EMRCREATEDIBPATTERNBRUSHPT; /* wingdi.h:5883:32, wingdi.h:5883:32, wingdi.h:5883:32 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct EMRFORMAT { dSignature: ::minwindef::DWORD, nVersion: ::minwindef::DWORD, cbData: ::minwindef::DWORD, offData: ::minwindef::DWORD } /* wingdi.h:5885:16, wingdi.h:5885:16, wingdi.h:5885:16 */
#[cfg(feature="winapi_desktop")] pub type PEMRFORMAT = *mut ::wingdi::EMRFORMAT; /* wingdi.h:5892:15, wingdi.h:5892:15, wingdi.h:5892:15 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct EMRGLSRECORD { emr: ::wingdi::EMR, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5896:16, wingdi.h:5896:16, wingdi.h:5896:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRGLSRECORD = *mut ::wingdi::EMRGLSRECORD; /* wingdi.h:5901:18, wingdi.h:5901:18, wingdi.h:5901:18 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct EMRGLSBOUNDEDRECORD { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5903:16, wingdi.h:5903:16, wingdi.h:5903:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRGLSBOUNDEDRECORD = *mut ::wingdi::EMRGLSBOUNDEDRECORD; /* wingdi.h:5909:25, wingdi.h:5909:25, wingdi.h:5909:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct EMRPIXELFORMAT { emr: ::wingdi::EMR, pfd: ::wingdi::PIXELFORMATDESCRIPTOR } /* wingdi.h:5911:16, wingdi.h:5911:16, wingdi.h:5911:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRPIXELFORMAT = *mut ::wingdi::EMRPIXELFORMAT; /* wingdi.h:5915:20, wingdi.h:5915:20, wingdi.h:5915:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct EMRCREATECOLORSPACE { emr: ::wingdi::EMR, ihCS: ::minwindef::DWORD, lcs: ::wingdi::LOGCOLORSPACEA } /* wingdi.h:5917:16, wingdi.h:5917:16, wingdi.h:5917:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRCREATECOLORSPACE = *mut ::wingdi::EMRCREATECOLORSPACE; /* wingdi.h:5922:25, wingdi.h:5922:25, wingdi.h:5922:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[repr(C)] pub struct EMRSETCOLORSPACE { emr: ::wingdi::EMR, ihCS: ::minwindef::DWORD } /* wingdi.h:5924:16, wingdi.h:5924:16, wingdi.h:5924:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRSETCOLORSPACE = *mut ::wingdi::EMRSETCOLORSPACE; /* wingdi.h:5928:25, wingdi.h:5928:25, wingdi.h:5928:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type EMRSELECTCOLORSPACE = ::wingdi::EMRSETCOLORSPACE; /* wingdi.h:5929:3, wingdi.h:5929:3, wingdi.h:5929:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRSELECTCOLORSPACE = *mut ::wingdi::EMRSETCOLORSPACE; /* wingdi.h:5929:25, wingdi.h:5929:25, wingdi.h:5929:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type EMRDELETECOLORSPACE = ::wingdi::EMRSETCOLORSPACE; /* wingdi.h:5930:3, wingdi.h:5930:3, wingdi.h:5930:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub type PEMRDELETECOLORSPACE = *mut ::wingdi::EMRSETCOLORSPACE; /* wingdi.h:5930:25, wingdi.h:5930:25, wingdi.h:5930:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMREXTESCAPE { emr: ::wingdi::EMR, iEscape: ::winnt::INT, cbEscData: ::winnt::INT, EscData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5936:16, wingdi.h:5936:16, wingdi.h:5936:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMREXTESCAPE = *mut ::wingdi::EMREXTESCAPE; /* wingdi.h:5942:19, wingdi.h:5942:19, wingdi.h:5942:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type EMRDRAWESCAPE = ::wingdi::EMREXTESCAPE; /* wingdi.h:5943:3, wingdi.h:5943:3, wingdi.h:5943:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRDRAWESCAPE = *mut ::wingdi::EMREXTESCAPE; /* wingdi.h:5943:19, wingdi.h:5943:19, wingdi.h:5943:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRNAMEDESCAPE { emr: ::wingdi::EMR, iEscape: ::winnt::INT, cbDriver: ::winnt::INT, cbEscData: ::winnt::INT, EscData: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5945:16, wingdi.h:5945:16, wingdi.h:5945:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRNAMEDESCAPE = *mut ::wingdi::EMRNAMEDESCAPE; /* wingdi.h:5952:20, wingdi.h:5952:20, wingdi.h:5952:20 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRSETICMPROFILE { emr: ::wingdi::EMR, dwFlags: ::minwindef::DWORD, cbName: ::minwindef::DWORD, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5956:16, wingdi.h:5956:16, wingdi.h:5956:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRSETICMPROFILE = *mut ::wingdi::EMRSETICMPROFILE; /* wingdi.h:5963:23, wingdi.h:5963:23, wingdi.h:5963:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type EMRSETICMPROFILEA = ::wingdi::EMRSETICMPROFILE; /* wingdi.h:5964:3, wingdi.h:5964:3, wingdi.h:5964:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRSETICMPROFILEA = *mut ::wingdi::EMRSETICMPROFILE; /* wingdi.h:5964:23, wingdi.h:5964:23, wingdi.h:5964:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type EMRSETICMPROFILEW = ::wingdi::EMRSETICMPROFILE; /* wingdi.h:5965:3, wingdi.h:5965:3, wingdi.h:5965:3 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRSETICMPROFILEW = *mut ::wingdi::EMRSETICMPROFILE; /* wingdi.h:5965:23, wingdi.h:5965:23, wingdi.h:5965:23 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRCREATECOLORSPACEW { emr: ::wingdi::EMR, ihCS: ::minwindef::DWORD, lcs: ::wingdi::LOGCOLORSPACEW, dwFlags: ::minwindef::DWORD, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5969:16, wingdi.h:5969:16, wingdi.h:5969:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRCREATECOLORSPACEW = *mut ::wingdi::EMRCREATECOLORSPACEW; /* wingdi.h:5977:26, wingdi.h:5977:26, wingdi.h:5977:26 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRCOLORMATCHTOTARGET { emr: ::wingdi::EMR, dwAction: ::minwindef::DWORD, dwFlags: ::minwindef::DWORD, cbName: ::minwindef::DWORD, cbData: ::minwindef::DWORD, Data: *mut [::minwindef::BYTE; 1] } /* wingdi.h:5981:16, wingdi.h:5981:16, wingdi.h:5981:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRCOLORMATCHTOTARGET = *mut ::wingdi::EMRCOLORMATCHTOTARGET; /* wingdi.h:5989:27, wingdi.h:5989:27, wingdi.h:5989:27 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRCOLORCORRECTPALETTE { emr: ::wingdi::EMR, ihPalette: ::minwindef::DWORD, nFirstEntry: ::minwindef::DWORD, nPalEntries: ::minwindef::DWORD, nReserved: ::minwindef::DWORD } /* wingdi.h:5991:16, wingdi.h:5991:16, wingdi.h:5991:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRCOLORCORRECTPALETTE = *mut ::wingdi::EMRCOLORCORRECTPALETTE; /* wingdi.h:5998:28, wingdi.h:5998:28, wingdi.h:5998:28 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRALPHABLEND { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG, dwRop: ::minwindef::DWORD, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG } /* wingdi.h:6000:16, wingdi.h:6000:16, wingdi.h:6000:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRALPHABLEND = *mut ::wingdi::EMRALPHABLEND; /* wingdi.h:6021:19, wingdi.h:6021:19, wingdi.h:6021:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRGRADIENTFILL { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, nVer: ::minwindef::DWORD, nTri: ::minwindef::DWORD, ulMode: ::minwindef::ULONG, Ver: *mut [::wingdi::TRIVERTEX; 1] } /* wingdi.h:6023:16, wingdi.h:6023:16, wingdi.h:6023:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRGRADIENTFILL = *mut ::wingdi::EMRGRADIENTFILL; /* wingdi.h:6031:19, wingdi.h:6031:19, wingdi.h:6031:19 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct EMRTRANSPARENTBLT { emr: ::wingdi::EMR, rclBounds: ::windef::RECTL, xDest: ::winnt::LONG, yDest: ::winnt::LONG, cxDest: ::winnt::LONG, cyDest: ::winnt::LONG, dwRop: ::minwindef::DWORD, xSrc: ::winnt::LONG, ySrc: ::winnt::LONG, xformSrc: ::wingdi::XFORM, crBkColorSrc: ::windef::COLORREF, iUsageSrc: ::minwindef::DWORD, offBmiSrc: ::minwindef::DWORD, cbBmiSrc: ::minwindef::DWORD, offBitsSrc: ::minwindef::DWORD, cbBitsSrc: ::minwindef::DWORD, cxSrc: ::winnt::LONG, cySrc: ::winnt::LONG } /* wingdi.h:6033:16, wingdi.h:6033:16, wingdi.h:6033:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PEMRTRANSPARENTBLT = *mut ::wingdi::EMRTRANSPARENTBLT; /* wingdi.h:6054:23, wingdi.h:6054:23, wingdi.h:6054:23 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct POINTFLOAT { x: ::minwindef::FLOAT, y: ::minwindef::FLOAT } /* wingdi.h:6091:16, wingdi.h:6091:16, wingdi.h:6091:16 */
#[cfg(feature="winapi_desktop")] pub type PPOINTFLOAT = *mut ::wingdi::POINTFLOAT; /* wingdi.h:6094:16, wingdi.h:6094:16, wingdi.h:6094:16 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct GLYPHMETRICSFLOAT { gmfBlackBoxX: ::minwindef::FLOAT, gmfBlackBoxY: ::minwindef::FLOAT, gmfptGlyphOrigin: ::wingdi::POINTFLOAT, gmfCellIncX: ::minwindef::FLOAT, gmfCellIncY: ::minwindef::FLOAT } /* wingdi.h:6096:16, wingdi.h:6096:16, wingdi.h:6096:16 */
#[cfg(feature="winapi_desktop")] pub type PGLYPHMETRICSFLOAT = *mut ::wingdi::GLYPHMETRICSFLOAT; /* wingdi.h:6102:23, wingdi.h:6102:23, wingdi.h:6102:23 */
#[cfg(feature="winapi_desktop")] pub type LPGLYPHMETRICSFLOAT = *mut ::wingdi::GLYPHMETRICSFLOAT; /* wingdi.h:6102:48, wingdi.h:6102:48, wingdi.h:6102:48 */
#[cfg(feature="winapi_desktop")] #[repr(C)] pub struct LAYERPLANEDESCRIPTOR { nSize: ::minwindef::WORD, nVersion: ::minwindef::WORD, dwFlags: ::minwindef::DWORD, iPixelType: ::minwindef::BYTE, cColorBits: ::minwindef::BYTE, cRedBits: ::minwindef::BYTE, cRedShift: ::minwindef::BYTE, cGreenBits: ::minwindef::BYTE, cGreenShift: ::minwindef::BYTE, cBlueBits: ::minwindef::BYTE, cBlueShift: ::minwindef::BYTE, cAlphaBits: ::minwindef::BYTE, cAlphaShift: ::minwindef::BYTE, cAccumBits: ::minwindef::BYTE, cAccumRedBits: ::minwindef::BYTE, cAccumGreenBits: ::minwindef::BYTE, cAccumBlueBits: ::minwindef::BYTE, cAccumAlphaBits: ::minwindef::BYTE, cDepthBits: ::minwindef::BYTE, cStencilBits: ::minwindef::BYTE, cAuxBuffers: ::minwindef::BYTE, iLayerPlane: ::minwindef::BYTE, bReserved: ::minwindef::BYTE, crTransparent: ::windef::COLORREF } /* wingdi.h:6117:16, wingdi.h:6117:16, wingdi.h:6117:16 */
#[cfg(feature="winapi_desktop")] pub type PLAYERPLANEDESCRIPTOR = *mut ::wingdi::LAYERPLANEDESCRIPTOR; /* wingdi.h:6142:26, wingdi.h:6142:26, wingdi.h:6142:26 */
#[cfg(feature="winapi_desktop")] pub type LPLAYERPLANEDESCRIPTOR = *mut ::wingdi::LAYERPLANEDESCRIPTOR; /* wingdi.h:6142:54, wingdi.h:6142:54, wingdi.h:6142:54 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] #[repr(C)] pub struct WGLSWAP { hdc: ::windef::HDC, uiFlags: ::minwindef::UINT } /* wingdi.h:6203:16, wingdi.h:6203:16, wingdi.h:6203:16 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type PWGLSWAP = *mut ::wingdi::WGLSWAP; /* wingdi.h:6207:13, wingdi.h:6207:13, wingdi.h:6207:13 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub type LPWGLSWAP = *mut ::wingdi::WGLSWAP; /* wingdi.h:6207:28, wingdi.h:6207:28, wingdi.h:6207:28 */
pub const R2_BLACK: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:60:9, wingdi.h:60:9, wingdi.h:60:9 */
pub const R2_NOTMERGEPEN: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:61:9, wingdi.h:61:9, wingdi.h:61:9 */
pub const R2_MASKNOTPEN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:62:9, wingdi.h:62:9, wingdi.h:62:9 */
pub const R2_NOTCOPYPEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:63:9, wingdi.h:63:9, wingdi.h:63:9 */
pub const R2_MASKPENNOT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:64:9, wingdi.h:64:9, wingdi.h:64:9 */
pub const R2_NOT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:65:9, wingdi.h:65:9, wingdi.h:65:9 */
pub const R2_XORPEN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:66:9, wingdi.h:66:9, wingdi.h:66:9 */
pub const R2_NOTMASKPEN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:67:9, wingdi.h:67:9, wingdi.h:67:9 */
pub const R2_MASKPEN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:68:9, wingdi.h:68:9, wingdi.h:68:9 */
pub const R2_NOTXORPEN: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:69:9, wingdi.h:69:9, wingdi.h:69:9 */
pub const R2_NOP: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:70:9, wingdi.h:70:9, wingdi.h:70:9 */
pub const R2_MERGENOTPEN: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:71:9, wingdi.h:71:9, wingdi.h:71:9 */
pub const R2_COPYPEN: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:72:9, wingdi.h:72:9, wingdi.h:72:9 */
pub const R2_MERGEPENNOT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:73:9, wingdi.h:73:9, wingdi.h:73:9 */
pub const R2_MERGEPEN: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:74:9, wingdi.h:74:9, wingdi.h:74:9 */
pub const R2_WHITE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:75:9, wingdi.h:75:9, wingdi.h:75:9 */
pub const R2_LAST: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:76:9, wingdi.h:76:9, wingdi.h:76:9 */
pub const SRCCOPY: ::minwindef::DWORD = 0xcc0020i32 as ::minwindef::DWORD; /* Cast { value: Integer(13369376, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:79:9, wingdi.h:79:9, wingdi.h:79:9 */
pub const SRCPAINT: ::minwindef::DWORD = 0xee0086i32 as ::minwindef::DWORD; /* Cast { value: Integer(15597702, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:80:9, wingdi.h:80:9, wingdi.h:80:9 */
pub const SRCAND: ::minwindef::DWORD = 0x8800c6i32 as ::minwindef::DWORD; /* Cast { value: Integer(8913094, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:81:9, wingdi.h:81:9, wingdi.h:81:9 */
pub const SRCINVERT: ::minwindef::DWORD = 0x660046i32 as ::minwindef::DWORD; /* Cast { value: Integer(6684742, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:82:9, wingdi.h:82:9, wingdi.h:82:9 */
pub const SRCERASE: ::minwindef::DWORD = 0x440328i32 as ::minwindef::DWORD; /* Cast { value: Integer(4457256, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:83:9, wingdi.h:83:9, wingdi.h:83:9 */
pub const NOTSRCCOPY: ::minwindef::DWORD = 0x330008i32 as ::minwindef::DWORD; /* Cast { value: Integer(3342344, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:84:9, wingdi.h:84:9, wingdi.h:84:9 */
pub const NOTSRCERASE: ::minwindef::DWORD = 0x1100a6i32 as ::minwindef::DWORD; /* Cast { value: Integer(1114278, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:85:9, wingdi.h:85:9, wingdi.h:85:9 */
pub const MERGECOPY: ::minwindef::DWORD = 0xc000cai32 as ::minwindef::DWORD; /* Cast { value: Integer(12583114, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:86:9, wingdi.h:86:9, wingdi.h:86:9 */
pub const MERGEPAINT: ::minwindef::DWORD = 0xbb0226i32 as ::minwindef::DWORD; /* Cast { value: Integer(12255782, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:87:9, wingdi.h:87:9, wingdi.h:87:9 */
pub const PATCOPY: ::minwindef::DWORD = 0xf00021i32 as ::minwindef::DWORD; /* Cast { value: Integer(15728673, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:88:9, wingdi.h:88:9, wingdi.h:88:9 */
pub const PATPAINT: ::minwindef::DWORD = 0xfb0a09i32 as ::minwindef::DWORD; /* Cast { value: Integer(16452105, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:89:9, wingdi.h:89:9, wingdi.h:89:9 */
pub const PATINVERT: ::minwindef::DWORD = 0x5a0049i32 as ::minwindef::DWORD; /* Cast { value: Integer(5898313, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:90:9, wingdi.h:90:9, wingdi.h:90:9 */
pub const DSTINVERT: ::minwindef::DWORD = 0x550009i32 as ::minwindef::DWORD; /* Cast { value: Integer(5570569, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:91:9, wingdi.h:91:9, wingdi.h:91:9 */
pub const BLACKNESS: ::minwindef::DWORD = 0x42i32 as ::minwindef::DWORD; /* Cast { value: Integer(66, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:92:9, wingdi.h:92:9, wingdi.h:92:9 */
pub const WHITENESS: ::minwindef::DWORD = 0xff0062i32 as ::minwindef::DWORD; /* Cast { value: Integer(16711778, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:93:9, wingdi.h:93:9, wingdi.h:93:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const NOMIRRORBITMAP: ::minwindef::DWORD = 0x80000000i32 as ::minwindef::DWORD; /* Cast { value: Integer(2147483648, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:96:9, wingdi.h:96:9, wingdi.h:96:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CAPTUREBLT: ::minwindef::DWORD = 0x40000000i32 as ::minwindef::DWORD; /* Cast { value: Integer(1073741824, Yes, Unknown), ty: Type("DWORD", false) } */ /* wingdi.h:97:9, wingdi.h:97:9, wingdi.h:97:9 */
pub const ERROR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:114:9, wingdi.h:114:9, wingdi.h:114:9 */
pub const NULLREGION: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:115:9, wingdi.h:115:9, wingdi.h:115:9 */
pub const SIMPLEREGION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:116:9, wingdi.h:116:9, wingdi.h:116:9 */
pub const COMPLEXREGION: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:117:9, wingdi.h:117:9, wingdi.h:117:9 */
#[doc(inline)] pub use ::wingdi::ERROR as RGN_ERROR; /* wingdi.h:118:9, wingdi.h:118:9, wingdi.h:118:9 */
pub const RGN_AND: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:121:9, wingdi.h:121:9, wingdi.h:121:9 */
pub const RGN_OR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:122:9, wingdi.h:122:9, wingdi.h:122:9 */
pub const RGN_XOR: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:123:9, wingdi.h:123:9, wingdi.h:123:9 */
pub const RGN_DIFF: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:124:9, wingdi.h:124:9, wingdi.h:124:9 */
pub const RGN_COPY: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:125:9, wingdi.h:125:9, wingdi.h:125:9 */
#[doc(inline)] pub use ::wingdi::RGN_AND as RGN_MIN; /* wingdi.h:126:9, wingdi.h:126:9, wingdi.h:126:9 */
#[doc(inline)] pub use ::wingdi::RGN_COPY as RGN_MAX; /* wingdi.h:127:9, wingdi.h:127:9, wingdi.h:127:9 */
pub const BLACKONWHITE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:130:9, wingdi.h:130:9, wingdi.h:130:9 */
pub const WHITEONBLACK: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:131:9, wingdi.h:131:9, wingdi.h:131:9 */
pub const COLORONCOLOR: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:132:9, wingdi.h:132:9, wingdi.h:132:9 */
pub const HALFTONE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:133:9, wingdi.h:133:9, wingdi.h:133:9 */
pub const MAXSTRETCHBLTMODE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:134:9, wingdi.h:134:9, wingdi.h:134:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::wingdi::BLACKONWHITE as STRETCH_ANDSCANS; /* wingdi.h:138:9, wingdi.h:138:9, wingdi.h:138:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::wingdi::WHITEONBLACK as STRETCH_ORSCANS; /* wingdi.h:139:9, wingdi.h:139:9, wingdi.h:139:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::wingdi::COLORONCOLOR as STRETCH_DELETESCANS; /* wingdi.h:140:9, wingdi.h:140:9, wingdi.h:140:9 */
#[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::wingdi::HALFTONE as STRETCH_HALFTONE; /* wingdi.h:141:9, wingdi.h:141:9, wingdi.h:141:9 */
pub const ALTERNATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:145:9, wingdi.h:145:9, wingdi.h:145:9 */
pub const WINDING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:146:9, wingdi.h:146:9, wingdi.h:146:9 */
pub const POLYFILL_LAST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:147:9, wingdi.h:147:9, wingdi.h:147:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LAYOUT_RTL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:151:9, wingdi.h:151:9, wingdi.h:151:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LAYOUT_BTT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:152:9, wingdi.h:152:9, wingdi.h:152:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LAYOUT_VBH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:153:9, wingdi.h:153:9, wingdi.h:153:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const LAYOUT_BITMAPORIENTATIONPRESERVED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:155:9, wingdi.h:155:9, wingdi.h:155:9 */
pub const TA_NOUPDATECP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:159:9, wingdi.h:159:9, wingdi.h:159:9 */
pub const TA_UPDATECP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:160:9, wingdi.h:160:9, wingdi.h:160:9 */
pub const TA_LEFT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:162:9, wingdi.h:162:9, wingdi.h:162:9 */
pub const TA_RIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:163:9, wingdi.h:163:9, wingdi.h:163:9 */
pub const TA_CENTER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:164:9, wingdi.h:164:9, wingdi.h:164:9 */
pub const TA_TOP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:166:9, wingdi.h:166:9, wingdi.h:166:9 */
pub const TA_BOTTOM: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:167:9, wingdi.h:167:9, wingdi.h:167:9 */
pub const TA_BASELINE: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:168:9, wingdi.h:168:9, wingdi.h:168:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TA_RTLREADING: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:170:9, wingdi.h:170:9, wingdi.h:170:9 */
#[doc(inline)] pub use ::wingdi::TA_BASELINE as VTA_BASELINE; /* wingdi.h:176:9, wingdi.h:176:9, wingdi.h:176:9 */
#[doc(inline)] pub use ::wingdi::TA_BOTTOM as VTA_LEFT; /* wingdi.h:177:9, wingdi.h:177:9, wingdi.h:177:9 */
#[doc(inline)] pub use ::wingdi::TA_TOP as VTA_RIGHT; /* wingdi.h:178:9, wingdi.h:178:9, wingdi.h:178:9 */
#[doc(inline)] pub use ::wingdi::TA_CENTER as VTA_CENTER; /* wingdi.h:179:9, wingdi.h:179:9, wingdi.h:179:9 */
#[doc(inline)] pub use ::wingdi::TA_RIGHT as VTA_BOTTOM; /* wingdi.h:180:9, wingdi.h:180:9, wingdi.h:180:9 */
#[doc(inline)] pub use ::wingdi::TA_LEFT as VTA_TOP; /* wingdi.h:181:9, wingdi.h:181:9, wingdi.h:181:9 */
pub const ETO_OPAQUE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:183:9, wingdi.h:183:9, wingdi.h:183:9 */
pub const ETO_CLIPPED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:184:9, wingdi.h:184:9, wingdi.h:184:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ETO_GLYPH_INDEX: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:186:9, wingdi.h:186:9, wingdi.h:186:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ETO_RTLREADING: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:187:9, wingdi.h:187:9, wingdi.h:187:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ETO_NUMERICSLOCAL: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:188:9, wingdi.h:188:9, wingdi.h:188:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ETO_NUMERICSLATIN: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:189:9, wingdi.h:189:9, wingdi.h:189:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ETO_IGNORELANGUAGE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:190:9, wingdi.h:190:9, wingdi.h:190:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const ETO_PDY: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:193:9, wingdi.h:193:9, wingdi.h:193:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const ETO_REVERSE_INDEX_MAP: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wingdi.h:196:9, wingdi.h:196:9, wingdi.h:196:9 */
pub const ASPECT_FILTERING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:199:9, wingdi.h:199:9, wingdi.h:199:9 */
pub const DCB_RESET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:203:9, wingdi.h:203:9, wingdi.h:203:9 */
pub const DCB_ACCUMULATE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:204:9, wingdi.h:204:9, wingdi.h:204:9 */
#[doc(inline)] pub use ::wingdi::DCB_ACCUMULATE as DCB_DIRTY; /* wingdi.h:205:9, wingdi.h:205:9, wingdi.h:205:9 */
pub const DCB_ENABLE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:207:9, wingdi.h:207:9, wingdi.h:207:9 */
pub const DCB_DISABLE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:208:9, wingdi.h:208:9, wingdi.h:208:9 */
pub const META_SETBKCOLOR: i32 = 0x201i32; /* Integer(513, Yes, Unknown) */ /* wingdi.h:213:9, wingdi.h:213:9, wingdi.h:213:9 */
pub const META_SETBKMODE: i32 = 0x102i32; /* Integer(258, Yes, Unknown) */ /* wingdi.h:214:9, wingdi.h:214:9, wingdi.h:214:9 */
pub const META_SETMAPMODE: i32 = 0x103i32; /* Integer(259, Yes, Unknown) */ /* wingdi.h:215:9, wingdi.h:215:9, wingdi.h:215:9 */
pub const META_SETROP2: i32 = 0x104i32; /* Integer(260, Yes, Unknown) */ /* wingdi.h:216:9, wingdi.h:216:9, wingdi.h:216:9 */
pub const META_SETRELABS: i32 = 0x105i32; /* Integer(261, Yes, Unknown) */ /* wingdi.h:217:9, wingdi.h:217:9, wingdi.h:217:9 */
pub const META_SETPOLYFILLMODE: i32 = 0x106i32; /* Integer(262, Yes, Unknown) */ /* wingdi.h:218:9, wingdi.h:218:9, wingdi.h:218:9 */
pub const META_SETSTRETCHBLTMODE: i32 = 0x107i32; /* Integer(263, Yes, Unknown) */ /* wingdi.h:219:9, wingdi.h:219:9, wingdi.h:219:9 */
pub const META_SETTEXTCHAREXTRA: i32 = 0x108i32; /* Integer(264, Yes, Unknown) */ /* wingdi.h:220:9, wingdi.h:220:9, wingdi.h:220:9 */
pub const META_SETTEXTCOLOR: i32 = 0x209i32; /* Integer(521, Yes, Unknown) */ /* wingdi.h:221:9, wingdi.h:221:9, wingdi.h:221:9 */
pub const META_SETTEXTJUSTIFICATION: i32 = 0x20ai32; /* Integer(522, Yes, Unknown) */ /* wingdi.h:222:9, wingdi.h:222:9, wingdi.h:222:9 */
pub const META_SETWINDOWORG: i32 = 0x20bi32; /* Integer(523, Yes, Unknown) */ /* wingdi.h:223:9, wingdi.h:223:9, wingdi.h:223:9 */
pub const META_SETWINDOWEXT: i32 = 0x20ci32; /* Integer(524, Yes, Unknown) */ /* wingdi.h:224:9, wingdi.h:224:9, wingdi.h:224:9 */
pub const META_SETVIEWPORTORG: i32 = 0x20di32; /* Integer(525, Yes, Unknown) */ /* wingdi.h:225:9, wingdi.h:225:9, wingdi.h:225:9 */
pub const META_SETVIEWPORTEXT: i32 = 0x20ei32; /* Integer(526, Yes, Unknown) */ /* wingdi.h:226:9, wingdi.h:226:9, wingdi.h:226:9 */
pub const META_OFFSETWINDOWORG: i32 = 0x20fi32; /* Integer(527, Yes, Unknown) */ /* wingdi.h:227:9, wingdi.h:227:9, wingdi.h:227:9 */
pub const META_SCALEWINDOWEXT: i32 = 0x410i32; /* Integer(1040, Yes, Unknown) */ /* wingdi.h:228:9, wingdi.h:228:9, wingdi.h:228:9 */
pub const META_OFFSETVIEWPORTORG: i32 = 0x211i32; /* Integer(529, Yes, Unknown) */ /* wingdi.h:229:9, wingdi.h:229:9, wingdi.h:229:9 */
pub const META_SCALEVIEWPORTEXT: i32 = 0x412i32; /* Integer(1042, Yes, Unknown) */ /* wingdi.h:230:9, wingdi.h:230:9, wingdi.h:230:9 */
pub const META_LINETO: i32 = 0x213i32; /* Integer(531, Yes, Unknown) */ /* wingdi.h:231:9, wingdi.h:231:9, wingdi.h:231:9 */
pub const META_MOVETO: i32 = 0x214i32; /* Integer(532, Yes, Unknown) */ /* wingdi.h:232:9, wingdi.h:232:9, wingdi.h:232:9 */
pub const META_EXCLUDECLIPRECT: i32 = 0x415i32; /* Integer(1045, Yes, Unknown) */ /* wingdi.h:233:9, wingdi.h:233:9, wingdi.h:233:9 */
pub const META_INTERSECTCLIPRECT: i32 = 0x416i32; /* Integer(1046, Yes, Unknown) */ /* wingdi.h:234:9, wingdi.h:234:9, wingdi.h:234:9 */
pub const META_ARC: i32 = 0x817i32; /* Integer(2071, Yes, Unknown) */ /* wingdi.h:235:9, wingdi.h:235:9, wingdi.h:235:9 */
pub const META_ELLIPSE: i32 = 0x418i32; /* Integer(1048, Yes, Unknown) */ /* wingdi.h:236:9, wingdi.h:236:9, wingdi.h:236:9 */
pub const META_FLOODFILL: i32 = 0x419i32; /* Integer(1049, Yes, Unknown) */ /* wingdi.h:237:9, wingdi.h:237:9, wingdi.h:237:9 */
pub const META_PIE: i32 = 0x81ai32; /* Integer(2074, Yes, Unknown) */ /* wingdi.h:238:9, wingdi.h:238:9, wingdi.h:238:9 */
pub const META_RECTANGLE: i32 = 0x41bi32; /* Integer(1051, Yes, Unknown) */ /* wingdi.h:239:9, wingdi.h:239:9, wingdi.h:239:9 */
pub const META_ROUNDRECT: i32 = 0x61ci32; /* Integer(1564, Yes, Unknown) */ /* wingdi.h:240:9, wingdi.h:240:9, wingdi.h:240:9 */
pub const META_PATBLT: i32 = 0x61di32; /* Integer(1565, Yes, Unknown) */ /* wingdi.h:241:9, wingdi.h:241:9, wingdi.h:241:9 */
pub const META_SAVEDC: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:242:9, wingdi.h:242:9, wingdi.h:242:9 */
pub const META_SETPIXEL: i32 = 0x41fi32; /* Integer(1055, Yes, Unknown) */ /* wingdi.h:243:9, wingdi.h:243:9, wingdi.h:243:9 */
pub const META_OFFSETCLIPRGN: i32 = 0x220i32; /* Integer(544, Yes, Unknown) */ /* wingdi.h:244:9, wingdi.h:244:9, wingdi.h:244:9 */
pub const META_TEXTOUT: i32 = 0x521i32; /* Integer(1313, Yes, Unknown) */ /* wingdi.h:245:9, wingdi.h:245:9, wingdi.h:245:9 */
pub const META_BITBLT: i32 = 0x922i32; /* Integer(2338, Yes, Unknown) */ /* wingdi.h:246:9, wingdi.h:246:9, wingdi.h:246:9 */
pub const META_STRETCHBLT: i32 = 0xb23i32; /* Integer(2851, Yes, Unknown) */ /* wingdi.h:247:9, wingdi.h:247:9, wingdi.h:247:9 */
pub const META_POLYGON: i32 = 0x324i32; /* Integer(804, Yes, Unknown) */ /* wingdi.h:248:9, wingdi.h:248:9, wingdi.h:248:9 */
pub const META_POLYLINE: i32 = 0x325i32; /* Integer(805, Yes, Unknown) */ /* wingdi.h:249:9, wingdi.h:249:9, wingdi.h:249:9 */
pub const META_ESCAPE: i32 = 0x626i32; /* Integer(1574, Yes, Unknown) */ /* wingdi.h:250:9, wingdi.h:250:9, wingdi.h:250:9 */
pub const META_RESTOREDC: i32 = 0x127i32; /* Integer(295, Yes, Unknown) */ /* wingdi.h:251:9, wingdi.h:251:9, wingdi.h:251:9 */
pub const META_FILLREGION: i32 = 0x228i32; /* Integer(552, Yes, Unknown) */ /* wingdi.h:252:9, wingdi.h:252:9, wingdi.h:252:9 */
pub const META_FRAMEREGION: i32 = 0x429i32; /* Integer(1065, Yes, Unknown) */ /* wingdi.h:253:9, wingdi.h:253:9, wingdi.h:253:9 */
pub const META_INVERTREGION: i32 = 0x12ai32; /* Integer(298, Yes, Unknown) */ /* wingdi.h:254:9, wingdi.h:254:9, wingdi.h:254:9 */
pub const META_PAINTREGION: i32 = 0x12bi32; /* Integer(299, Yes, Unknown) */ /* wingdi.h:255:9, wingdi.h:255:9, wingdi.h:255:9 */
pub const META_SELECTCLIPREGION: i32 = 0x12ci32; /* Integer(300, Yes, Unknown) */ /* wingdi.h:256:9, wingdi.h:256:9, wingdi.h:256:9 */
pub const META_SELECTOBJECT: i32 = 0x12di32; /* Integer(301, Yes, Unknown) */ /* wingdi.h:257:9, wingdi.h:257:9, wingdi.h:257:9 */
pub const META_SETTEXTALIGN: i32 = 0x12ei32; /* Integer(302, Yes, Unknown) */ /* wingdi.h:258:9, wingdi.h:258:9, wingdi.h:258:9 */
pub const META_CHORD: i32 = 0x830i32; /* Integer(2096, Yes, Unknown) */ /* wingdi.h:259:9, wingdi.h:259:9, wingdi.h:259:9 */
pub const META_SETMAPPERFLAGS: i32 = 0x231i32; /* Integer(561, Yes, Unknown) */ /* wingdi.h:260:9, wingdi.h:260:9, wingdi.h:260:9 */
pub const META_EXTTEXTOUT: i32 = 0xa32i32; /* Integer(2610, Yes, Unknown) */ /* wingdi.h:261:9, wingdi.h:261:9, wingdi.h:261:9 */
pub const META_SETDIBTODEV: i32 = 0xd33i32; /* Integer(3379, Yes, Unknown) */ /* wingdi.h:262:9, wingdi.h:262:9, wingdi.h:262:9 */
pub const META_SELECTPALETTE: i32 = 0x234i32; /* Integer(564, Yes, Unknown) */ /* wingdi.h:263:9, wingdi.h:263:9, wingdi.h:263:9 */
pub const META_REALIZEPALETTE: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* wingdi.h:264:9, wingdi.h:264:9, wingdi.h:264:9 */
pub const META_ANIMATEPALETTE: i32 = 0x436i32; /* Integer(1078, Yes, Unknown) */ /* wingdi.h:265:9, wingdi.h:265:9, wingdi.h:265:9 */
pub const META_SETPALENTRIES: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* wingdi.h:266:9, wingdi.h:266:9, wingdi.h:266:9 */
pub const META_POLYPOLYGON: i32 = 0x538i32; /* Integer(1336, Yes, Unknown) */ /* wingdi.h:267:9, wingdi.h:267:9, wingdi.h:267:9 */
pub const META_RESIZEPALETTE: i32 = 0x139i32; /* Integer(313, Yes, Unknown) */ /* wingdi.h:268:9, wingdi.h:268:9, wingdi.h:268:9 */
pub const META_DIBBITBLT: i32 = 0x940i32; /* Integer(2368, Yes, Unknown) */ /* wingdi.h:269:9, wingdi.h:269:9, wingdi.h:269:9 */
pub const META_DIBSTRETCHBLT: i32 = 0xb41i32; /* Integer(2881, Yes, Unknown) */ /* wingdi.h:270:9, wingdi.h:270:9, wingdi.h:270:9 */
pub const META_DIBCREATEPATTERNBRUSH: i32 = 0x142i32; /* Integer(322, Yes, Unknown) */ /* wingdi.h:271:9, wingdi.h:271:9, wingdi.h:271:9 */
pub const META_STRETCHDIB: i32 = 0xf43i32; /* Integer(3907, Yes, Unknown) */ /* wingdi.h:272:9, wingdi.h:272:9, wingdi.h:272:9 */
pub const META_EXTFLOODFILL: i32 = 0x548i32; /* Integer(1352, Yes, Unknown) */ /* wingdi.h:273:9, wingdi.h:273:9, wingdi.h:273:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const META_SETLAYOUT: i32 = 0x149i32; /* Integer(329, Yes, Unknown) */ /* wingdi.h:275:9, wingdi.h:275:9, wingdi.h:275:9 */
pub const META_DELETEOBJECT: i32 = 0x1f0i32; /* Integer(496, Yes, Unknown) */ /* wingdi.h:277:9, wingdi.h:277:9, wingdi.h:277:9 */
pub const META_CREATEPALETTE: i32 = 0xf7i32; /* Integer(247, Yes, Unknown) */ /* wingdi.h:278:9, wingdi.h:278:9, wingdi.h:278:9 */
pub const META_CREATEPATTERNBRUSH: i32 = 0x1f9i32; /* Integer(505, Yes, Unknown) */ /* wingdi.h:279:9, wingdi.h:279:9, wingdi.h:279:9 */
pub const META_CREATEPENINDIRECT: i32 = 0x2fai32; /* Integer(762, Yes, Unknown) */ /* wingdi.h:280:9, wingdi.h:280:9, wingdi.h:280:9 */
pub const META_CREATEFONTINDIRECT: i32 = 0x2fbi32; /* Integer(763, Yes, Unknown) */ /* wingdi.h:281:9, wingdi.h:281:9, wingdi.h:281:9 */
pub const META_CREATEBRUSHINDIRECT: i32 = 0x2fci32; /* Integer(764, Yes, Unknown) */ /* wingdi.h:282:9, wingdi.h:282:9, wingdi.h:282:9 */
pub const META_CREATEREGION: i32 = 0x6ffi32; /* Integer(1791, Yes, Unknown) */ /* wingdi.h:283:9, wingdi.h:283:9, wingdi.h:283:9 */
pub const NEWFRAME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:303:9, wingdi.h:303:9, wingdi.h:303:9 */
pub const ABORTDOC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:304:9, wingdi.h:304:9, wingdi.h:304:9 */
pub const NEXTBAND: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:305:9, wingdi.h:305:9, wingdi.h:305:9 */
pub const SETCOLORTABLE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:306:9, wingdi.h:306:9, wingdi.h:306:9 */
pub const GETCOLORTABLE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:307:9, wingdi.h:307:9, wingdi.h:307:9 */
pub const FLUSHOUTPUT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:308:9, wingdi.h:308:9, wingdi.h:308:9 */
pub const DRAFTMODE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:309:9, wingdi.h:309:9, wingdi.h:309:9 */
pub const QUERYESCSUPPORT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:310:9, wingdi.h:310:9, wingdi.h:310:9 */
pub const SETABORTPROC: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:311:9, wingdi.h:311:9, wingdi.h:311:9 */
pub const STARTDOC: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:312:9, wingdi.h:312:9, wingdi.h:312:9 */
pub const ENDDOC: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:313:9, wingdi.h:313:9, wingdi.h:313:9 */
pub const GETPHYSPAGESIZE: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:314:9, wingdi.h:314:9, wingdi.h:314:9 */
pub const GETPRINTINGOFFSET: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:315:9, wingdi.h:315:9, wingdi.h:315:9 */
pub const GETSCALINGFACTOR: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:316:9, wingdi.h:316:9, wingdi.h:316:9 */
pub const MFCOMMENT: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:317:9, wingdi.h:317:9, wingdi.h:317:9 */
pub const GETPENWIDTH: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:318:9, wingdi.h:318:9, wingdi.h:318:9 */
pub const SETCOPYCOUNT: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:319:9, wingdi.h:319:9, wingdi.h:319:9 */
pub const SELECTPAPERSOURCE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:320:9, wingdi.h:320:9, wingdi.h:320:9 */
pub const DEVICEDATA: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:321:9, wingdi.h:321:9, wingdi.h:321:9 */
pub const PASSTHROUGH: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:322:9, wingdi.h:322:9, wingdi.h:322:9 */
pub const GETTECHNOLGY: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:323:9, wingdi.h:323:9, wingdi.h:323:9 */
pub const GETTECHNOLOGY: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:324:9, wingdi.h:324:9, wingdi.h:324:9 */
pub const SETLINECAP: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* wingdi.h:325:9, wingdi.h:325:9, wingdi.h:325:9 */
pub const SETLINEJOIN: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* wingdi.h:326:9, wingdi.h:326:9, wingdi.h:326:9 */
pub const SETMITERLIMIT: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* wingdi.h:327:9, wingdi.h:327:9, wingdi.h:327:9 */
pub const BANDINFO: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:328:9, wingdi.h:328:9, wingdi.h:328:9 */
pub const DRAWPATTERNRECT: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* wingdi.h:329:9, wingdi.h:329:9, wingdi.h:329:9 */
pub const GETVECTORPENSIZE: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* wingdi.h:330:9, wingdi.h:330:9, wingdi.h:330:9 */
pub const GETVECTORBRUSHSIZE: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* wingdi.h:331:9, wingdi.h:331:9, wingdi.h:331:9 */
pub const ENABLEDUPLEX: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* wingdi.h:332:9, wingdi.h:332:9, wingdi.h:332:9 */
pub const GETSETPAPERBINS: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* wingdi.h:333:9, wingdi.h:333:9, wingdi.h:333:9 */
pub const GETSETPRINTORIENT: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:334:9, wingdi.h:334:9, wingdi.h:334:9 */
pub const ENUMPAPERBINS: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* wingdi.h:335:9, wingdi.h:335:9, wingdi.h:335:9 */
pub const SETDIBSCALING: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:336:9, wingdi.h:336:9, wingdi.h:336:9 */
pub const EPSPRINTING: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* wingdi.h:337:9, wingdi.h:337:9, wingdi.h:337:9 */
pub const ENUMPAPERMETRICS: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* wingdi.h:338:9, wingdi.h:338:9, wingdi.h:338:9 */
pub const GETSETPAPERMETRICS: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* wingdi.h:339:9, wingdi.h:339:9, wingdi.h:339:9 */
pub const POSTSCRIPT_DATA: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* wingdi.h:340:9, wingdi.h:340:9, wingdi.h:340:9 */
pub const POSTSCRIPT_IGNORE: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* wingdi.h:341:9, wingdi.h:341:9, wingdi.h:341:9 */
pub const MOUSETRAILS: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* wingdi.h:342:9, wingdi.h:342:9, wingdi.h:342:9 */
pub const GETDEVICEUNITS: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* wingdi.h:343:9, wingdi.h:343:9, wingdi.h:343:9 */
pub const GETEXTENDEDTEXTMETRICS: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:345:9, wingdi.h:345:9, wingdi.h:345:9 */
pub const GETEXTENTTABLE: i32 = 0x101i32; /* Integer(257, Yes, Unknown) */ /* wingdi.h:346:9, wingdi.h:346:9, wingdi.h:346:9 */
pub const GETPAIRKERNTABLE: i32 = 0x102i32; /* Integer(258, Yes, Unknown) */ /* wingdi.h:347:9, wingdi.h:347:9, wingdi.h:347:9 */
pub const GETTRACKKERNTABLE: i32 = 0x103i32; /* Integer(259, Yes, Unknown) */ /* wingdi.h:348:9, wingdi.h:348:9, wingdi.h:348:9 */
pub const EXTTEXTOUT: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:349:9, wingdi.h:349:9, wingdi.h:349:9 */
pub const GETFACENAME: i32 = 0x201i32; /* Integer(513, Yes, Unknown) */ /* wingdi.h:350:9, wingdi.h:350:9, wingdi.h:350:9 */
pub const DOWNLOADFACE: i32 = 0x202i32; /* Integer(514, Yes, Unknown) */ /* wingdi.h:351:9, wingdi.h:351:9, wingdi.h:351:9 */
pub const ENABLERELATIVEWIDTHS: i32 = 0x300i32; /* Integer(768, Yes, Unknown) */ /* wingdi.h:352:9, wingdi.h:352:9, wingdi.h:352:9 */
pub const ENABLEPAIRKERNING: i32 = 0x301i32; /* Integer(769, Yes, Unknown) */ /* wingdi.h:353:9, wingdi.h:353:9, wingdi.h:353:9 */
pub const SETKERNTRACK: i32 = 0x302i32; /* Integer(770, Yes, Unknown) */ /* wingdi.h:354:9, wingdi.h:354:9, wingdi.h:354:9 */
pub const SETALLJUSTVALUES: i32 = 0x303i32; /* Integer(771, Yes, Unknown) */ /* wingdi.h:355:9, wingdi.h:355:9, wingdi.h:355:9 */
pub const SETCHARSET: i32 = 0x304i32; /* Integer(772, Yes, Unknown) */ /* wingdi.h:356:9, wingdi.h:356:9, wingdi.h:356:9 */
pub const STRETCHBLT: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:358:9, wingdi.h:358:9, wingdi.h:358:9 */
pub const METAFILE_DRIVER: i32 = 0x801i32; /* Integer(2049, Yes, Unknown) */ /* wingdi.h:359:9, wingdi.h:359:9, wingdi.h:359:9 */
pub const GETSETSCREENPARAMS: i32 = 0xc00i32; /* Integer(3072, Yes, Unknown) */ /* wingdi.h:360:9, wingdi.h:360:9, wingdi.h:360:9 */
pub const QUERYDIBSUPPORT: i32 = 0xc01i32; /* Integer(3073, Yes, Unknown) */ /* wingdi.h:361:9, wingdi.h:361:9, wingdi.h:361:9 */
pub const BEGIN_PATH: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:362:9, wingdi.h:362:9, wingdi.h:362:9 */
pub const CLIP_TO_PATH: i32 = 0x1001i32; /* Integer(4097, Yes, Unknown) */ /* wingdi.h:363:9, wingdi.h:363:9, wingdi.h:363:9 */
pub const END_PATH: i32 = 0x1002i32; /* Integer(4098, Yes, Unknown) */ /* wingdi.h:364:9, wingdi.h:364:9, wingdi.h:364:9 */
pub const EXT_DEVICE_CAPS: i32 = 0x1003i32; /* Integer(4099, Yes, Unknown) */ /* wingdi.h:365:9, wingdi.h:365:9, wingdi.h:365:9 */
pub const RESTORE_CTM: i32 = 0x1004i32; /* Integer(4100, Yes, Unknown) */ /* wingdi.h:366:9, wingdi.h:366:9, wingdi.h:366:9 */
pub const SAVE_CTM: i32 = 0x1005i32; /* Integer(4101, Yes, Unknown) */ /* wingdi.h:367:9, wingdi.h:367:9, wingdi.h:367:9 */
pub const SET_ARC_DIRECTION: i32 = 0x1006i32; /* Integer(4102, Yes, Unknown) */ /* wingdi.h:368:9, wingdi.h:368:9, wingdi.h:368:9 */
pub const SET_BACKGROUND_COLOR: i32 = 0x1007i32; /* Integer(4103, Yes, Unknown) */ /* wingdi.h:369:9, wingdi.h:369:9, wingdi.h:369:9 */
pub const SET_POLY_MODE: i32 = 0x1008i32; /* Integer(4104, Yes, Unknown) */ /* wingdi.h:370:9, wingdi.h:370:9, wingdi.h:370:9 */
pub const SET_SCREEN_ANGLE: i32 = 0x1009i32; /* Integer(4105, Yes, Unknown) */ /* wingdi.h:371:9, wingdi.h:371:9, wingdi.h:371:9 */
pub const SET_SPREAD: i32 = 0x100ai32; /* Integer(4106, Yes, Unknown) */ /* wingdi.h:372:9, wingdi.h:372:9, wingdi.h:372:9 */
pub const TRANSFORM_CTM: i32 = 0x100bi32; /* Integer(4107, Yes, Unknown) */ /* wingdi.h:373:9, wingdi.h:373:9, wingdi.h:373:9 */
pub const SET_CLIP_BOX: i32 = 0x100ci32; /* Integer(4108, Yes, Unknown) */ /* wingdi.h:374:9, wingdi.h:374:9, wingdi.h:374:9 */
pub const SET_BOUNDS: i32 = 0x100di32; /* Integer(4109, Yes, Unknown) */ /* wingdi.h:375:9, wingdi.h:375:9, wingdi.h:375:9 */
pub const SET_MIRROR_MODE: i32 = 0x100ei32; /* Integer(4110, Yes, Unknown) */ /* wingdi.h:376:9, wingdi.h:376:9, wingdi.h:376:9 */
pub const OPENCHANNEL: i32 = 0x100ei32; /* Integer(4110, Yes, Unknown) */ /* wingdi.h:377:9, wingdi.h:377:9, wingdi.h:377:9 */
pub const DOWNLOADHEADER: i32 = 0x100fi32; /* Integer(4111, Yes, Unknown) */ /* wingdi.h:378:9, wingdi.h:378:9, wingdi.h:378:9 */
pub const CLOSECHANNEL: i32 = 0x1010i32; /* Integer(4112, Yes, Unknown) */ /* wingdi.h:379:9, wingdi.h:379:9, wingdi.h:379:9 */
pub const POSTSCRIPT_PASSTHROUGH: i32 = 0x1013i32; /* Integer(4115, Yes, Unknown) */ /* wingdi.h:380:9, wingdi.h:380:9, wingdi.h:380:9 */
pub const ENCAPSULATED_POSTSCRIPT: i32 = 0x1014i32; /* Integer(4116, Yes, Unknown) */ /* wingdi.h:381:9, wingdi.h:381:9, wingdi.h:381:9 */
pub const POSTSCRIPT_IDENTIFY: i32 = 0x1015i32; /* Integer(4117, Yes, Unknown) */ /* wingdi.h:383:9, wingdi.h:383:9, wingdi.h:383:9 */
pub const POSTSCRIPT_INJECTION: i32 = 0x1016i32; /* Integer(4118, Yes, Unknown) */ /* wingdi.h:384:9, wingdi.h:384:9, wingdi.h:384:9 */
pub const CHECKJPEGFORMAT: i32 = 0x1017i32; /* Integer(4119, Yes, Unknown) */ /* wingdi.h:386:9, wingdi.h:386:9, wingdi.h:386:9 */
pub const CHECKPNGFORMAT: i32 = 0x1018i32; /* Integer(4120, Yes, Unknown) */ /* wingdi.h:387:9, wingdi.h:387:9, wingdi.h:387:9 */
pub const GET_PS_FEATURESETTING: i32 = 0x1019i32; /* Integer(4121, Yes, Unknown) */ /* wingdi.h:389:9, wingdi.h:389:9, wingdi.h:389:9 */
pub const GDIPLUS_TS_QUERYVER: i32 = 0x101ai32; /* Integer(4122, Yes, Unknown) */ /* wingdi.h:391:9, wingdi.h:391:9, wingdi.h:391:9 */
pub const GDIPLUS_TS_RECORD: i32 = 0x101bi32; /* Integer(4123, Yes, Unknown) */ /* wingdi.h:392:9, wingdi.h:392:9, wingdi.h:392:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MILCORE_TS_QUERYVER_RESULT_FALSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:400:9, wingdi.h:400:9, wingdi.h:400:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const MILCORE_TS_QUERYVER_RESULT_TRUE: i32 = 0x7fffffffi32; /* Integer(2147483647, Yes, Unknown) */ /* wingdi.h:401:9, wingdi.h:401:9, wingdi.h:401:9 */
pub const SPCLPASSTHROUGH2: i32 = 0x11d8i32; /* Integer(4568, Yes, Unknown) */ /* wingdi.h:405:9, wingdi.h:405:9, wingdi.h:405:9 */
pub const PSIDENT_GDICENTRIC: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:411:9, wingdi.h:411:9, wingdi.h:411:9 */
pub const PSIDENT_PSCENTRIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:412:9, wingdi.h:412:9, wingdi.h:412:9 */
pub const PSINJECT_BEGINSTREAM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:438:9, wingdi.h:438:9, wingdi.h:438:9 */
pub const PSINJECT_PSADOBE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:439:9, wingdi.h:439:9, wingdi.h:439:9 */
pub const PSINJECT_PAGESATEND: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:440:9, wingdi.h:440:9, wingdi.h:440:9 */
pub const PSINJECT_PAGES: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:441:9, wingdi.h:441:9, wingdi.h:441:9 */
pub const PSINJECT_DOCNEEDEDRES: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:443:9, wingdi.h:443:9, wingdi.h:443:9 */
pub const PSINJECT_DOCSUPPLIEDRES: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:444:9, wingdi.h:444:9, wingdi.h:444:9 */
pub const PSINJECT_PAGEORDER: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:445:9, wingdi.h:445:9, wingdi.h:445:9 */
pub const PSINJECT_ORIENTATION: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:446:9, wingdi.h:446:9, wingdi.h:446:9 */
pub const PSINJECT_BOUNDINGBOX: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:447:9, wingdi.h:447:9, wingdi.h:447:9 */
pub const PSINJECT_DOCUMENTPROCESSCOLORS: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:448:9, wingdi.h:448:9, wingdi.h:448:9 */
pub const PSINJECT_COMMENTS: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:450:9, wingdi.h:450:9, wingdi.h:450:9 */
pub const PSINJECT_BEGINDEFAULTS: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:451:9, wingdi.h:451:9, wingdi.h:451:9 */
pub const PSINJECT_ENDDEFAULTS: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:452:9, wingdi.h:452:9, wingdi.h:452:9 */
pub const PSINJECT_BEGINPROLOG: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:453:9, wingdi.h:453:9, wingdi.h:453:9 */
pub const PSINJECT_ENDPROLOG: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:454:9, wingdi.h:454:9, wingdi.h:454:9 */
pub const PSINJECT_BEGINSETUP: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:455:9, wingdi.h:455:9, wingdi.h:455:9 */
pub const PSINJECT_ENDSETUP: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:456:9, wingdi.h:456:9, wingdi.h:456:9 */
pub const PSINJECT_TRAILER: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:457:9, wingdi.h:457:9, wingdi.h:457:9 */
pub const PSINJECT_EOF: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:458:9, wingdi.h:458:9, wingdi.h:458:9 */
pub const PSINJECT_ENDSTREAM: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:459:9, wingdi.h:459:9, wingdi.h:459:9 */
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* wingdi.h:460:9, wingdi.h:460:9, wingdi.h:460:9 */
pub const PSINJECT_PAGENUMBER: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* wingdi.h:462:9, wingdi.h:462:9, wingdi.h:462:9 */
pub const PSINJECT_BEGINPAGESETUP: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* wingdi.h:463:9, wingdi.h:463:9, wingdi.h:463:9 */
pub const PSINJECT_ENDPAGESETUP: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* wingdi.h:464:9, wingdi.h:464:9, wingdi.h:464:9 */
pub const PSINJECT_PAGETRAILER: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* wingdi.h:465:9, wingdi.h:465:9, wingdi.h:465:9 */
pub const PSINJECT_PLATECOLOR: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* wingdi.h:466:9, wingdi.h:466:9, wingdi.h:466:9 */
pub const PSINJECT_SHOWPAGE: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* wingdi.h:468:9, wingdi.h:468:9, wingdi.h:468:9 */
pub const PSINJECT_PAGEBBOX: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* wingdi.h:469:9, wingdi.h:469:9, wingdi.h:469:9 */
pub const PSINJECT_ENDPAGECOMMENTS: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* wingdi.h:470:9, wingdi.h:470:9, wingdi.h:470:9 */
pub const PSINJECT_VMSAVE: i32 = 0xc8i32; /* Integer(200, Yes, Unknown) */ /* wingdi.h:472:9, wingdi.h:472:9, wingdi.h:472:9 */
pub const PSINJECT_VMRESTORE: i32 = 0xc9i32; /* Integer(201, Yes, Unknown) */ /* wingdi.h:473:9, wingdi.h:473:9, wingdi.h:473:9 */
pub const PSINJECT_DLFONT: i32 = 0xddddddddi32; /* Integer(3722304989, Yes, Unknown) */ /* wingdi.h:479:9, wingdi.h:479:9, wingdi.h:479:9 */
pub const FEATURESETTING_NUP: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:485:9, wingdi.h:485:9, wingdi.h:485:9 */
pub const FEATURESETTING_OUTPUT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:486:9, wingdi.h:486:9, wingdi.h:486:9 */
pub const FEATURESETTING_PSLEVEL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:487:9, wingdi.h:487:9, wingdi.h:487:9 */
pub const FEATURESETTING_CUSTPAPER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:488:9, wingdi.h:488:9, wingdi.h:488:9 */
pub const FEATURESETTING_MIRROR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:489:9, wingdi.h:489:9, wingdi.h:489:9 */
pub const FEATURESETTING_NEGATIVE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:490:9, wingdi.h:490:9, wingdi.h:490:9 */
pub const FEATURESETTING_PROTOCOL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:491:9, wingdi.h:491:9, wingdi.h:491:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FEATURESETTING_PRIVATE_BEGIN: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:498:9, wingdi.h:498:9, wingdi.h:498:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const FEATURESETTING_PRIVATE_END: i32 = 0x1fffi32; /* Integer(8191, Yes, Unknown) */ /* wingdi.h:499:9, wingdi.h:499:9, wingdi.h:499:9 */
pub const PSPROTOCOL_ASCII: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:534:9, wingdi.h:534:9, wingdi.h:534:9 */
pub const PSPROTOCOL_BCP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:535:9, wingdi.h:535:9, wingdi.h:535:9 */
pub const PSPROTOCOL_TBCP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:536:9, wingdi.h:536:9, wingdi.h:536:9 */
pub const PSPROTOCOL_BINARY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:537:9, wingdi.h:537:9, wingdi.h:537:9 */
pub const QDI_SETDIBITS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:540:9, wingdi.h:540:9, wingdi.h:540:9 */
pub const QDI_GETDIBITS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:541:9, wingdi.h:541:9, wingdi.h:541:9 */
pub const QDI_DIBTOSCREEN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:542:9, wingdi.h:542:9, wingdi.h:542:9 */
pub const QDI_STRETCHDIB: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:543:9, wingdi.h:543:9, wingdi.h:543:9 */
pub const SP_NOTREPORTED: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:546:9, wingdi.h:546:9, wingdi.h:546:9 */
pub const PR_JOBSTATUS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:553:9, wingdi.h:553:9, wingdi.h:553:9 */
pub const OBJ_PEN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:556:9, wingdi.h:556:9, wingdi.h:556:9 */
pub const OBJ_BRUSH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:557:9, wingdi.h:557:9, wingdi.h:557:9 */
pub const OBJ_DC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:558:9, wingdi.h:558:9, wingdi.h:558:9 */
pub const OBJ_METADC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:559:9, wingdi.h:559:9, wingdi.h:559:9 */
pub const OBJ_PAL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:560:9, wingdi.h:560:9, wingdi.h:560:9 */
pub const OBJ_FONT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:561:9, wingdi.h:561:9, wingdi.h:561:9 */
pub const OBJ_BITMAP: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:562:9, wingdi.h:562:9, wingdi.h:562:9 */
pub const OBJ_REGION: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:563:9, wingdi.h:563:9, wingdi.h:563:9 */
pub const OBJ_METAFILE: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:564:9, wingdi.h:564:9, wingdi.h:564:9 */
pub const OBJ_MEMDC: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:565:9, wingdi.h:565:9, wingdi.h:565:9 */
pub const OBJ_EXTPEN: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:566:9, wingdi.h:566:9, wingdi.h:566:9 */
pub const OBJ_ENHMETADC: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:567:9, wingdi.h:567:9, wingdi.h:567:9 */
pub const OBJ_ENHMETAFILE: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:568:9, wingdi.h:568:9, wingdi.h:568:9 */
pub const OBJ_COLORSPACE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:569:9, wingdi.h:569:9, wingdi.h:569:9 */
#[doc(inline)] pub use ::wingdi::OBJ_COLORSPACE as GDI_OBJ_LAST; /* wingdi.h:571:9, wingdi.h:571:9, wingdi.h:571:9 */
pub const MWT_IDENTITY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:574:9, wingdi.h:574:9, wingdi.h:574:9 */
pub const MWT_LEFTMULTIPLY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:575:9, wingdi.h:575:9, wingdi.h:575:9 */
pub const MWT_RIGHTMULTIPLY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:576:9, wingdi.h:576:9, wingdi.h:576:9 */
#[doc(inline)] pub use ::wingdi::MWT_IDENTITY as MWT_MIN; /* wingdi.h:578:9, wingdi.h:578:9, wingdi.h:578:9 */
#[doc(inline)] pub use ::wingdi::MWT_RIGHTMULTIPLY as MWT_MAX; /* wingdi.h:579:9, wingdi.h:579:9, wingdi.h:579:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CS_ENABLE: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:652:9, wingdi.h:652:9, wingdi.h:652:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CS_DISABLE: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:653:9, wingdi.h:653:9, wingdi.h:653:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const CS_DELETE_TRANSFORM: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* wingdi.h:654:9, wingdi.h:654:9, wingdi.h:654:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LCS_CALIBRATED_RGB: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* wingdi.h:670:9, wingdi.h:670:9, wingdi.h:670:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LCS_GM_BUSINESS: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:674:9, wingdi.h:674:9, wingdi.h:674:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LCS_GM_GRAPHICS: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:675:9, wingdi.h:675:9, wingdi.h:675:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LCS_GM_IMAGES: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:676:9, wingdi.h:676:9, wingdi.h:676:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const LCS_GM_ABS_COLORIMETRIC: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* wingdi.h:677:9, wingdi.h:677:9, wingdi.h:677:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CM_OUT_OF_GAMUT: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* wingdi.h:680:9, wingdi.h:680:9, wingdi.h:680:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const CM_IN_GAMUT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:681:9, wingdi.h:681:9, wingdi.h:681:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_ADDPROFILE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:684:9, wingdi.h:684:9, wingdi.h:684:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_DELETEPROFILE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:685:9, wingdi.h:685:9, wingdi.h:685:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_QUERYPROFILE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:686:9, wingdi.h:686:9, wingdi.h:686:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_SETDEFAULTPROFILE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:687:9, wingdi.h:687:9, wingdi.h:687:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_REGISTERICMATCHER: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:688:9, wingdi.h:688:9, wingdi.h:688:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_UNREGISTERICMATCHER: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:689:9, wingdi.h:689:9, wingdi.h:689:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_QUERYMATCH: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:690:9, wingdi.h:690:9, wingdi.h:690:9 */
pub const BI_RGB: i64 = 0x0i64; /* Integer(0, Yes, Long) */ /* wingdi.h:897:9, wingdi.h:897:9, wingdi.h:897:9 */
pub const BI_RLE8: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:898:9, wingdi.h:898:9, wingdi.h:898:9 */
pub const BI_RLE4: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:899:9, wingdi.h:899:9, wingdi.h:899:9 */
pub const BI_BITFIELDS: i64 = 0x3i64; /* Integer(3, Yes, Long) */ /* wingdi.h:900:9, wingdi.h:900:9, wingdi.h:900:9 */
pub const BI_JPEG: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:901:9, wingdi.h:901:9, wingdi.h:901:9 */
pub const BI_PNG: i64 = 0x5i64; /* Integer(5, Yes, Long) */ /* wingdi.h:902:9, wingdi.h:902:9, wingdi.h:902:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TCI_SRCCHARSET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:970:9, wingdi.h:970:9, wingdi.h:970:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TCI_SRCCODEPAGE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:971:9, wingdi.h:971:9, wingdi.h:971:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TCI_SRCFONTSIG: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:972:9, wingdi.h:972:9, wingdi.h:972:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const TCI_SRCLOCALE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:974:9, wingdi.h:974:9, wingdi.h:974:9 */
pub const TMPF_FIXED_PITCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1116:9, wingdi.h:1116:9, wingdi.h:1116:9 */
pub const TMPF_VECTOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1117:9, wingdi.h:1117:9, wingdi.h:1117:9 */
pub const TMPF_DEVICE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1118:9, wingdi.h:1118:9, wingdi.h:1118:9 */
pub const TMPF_TRUETYPE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1119:9, wingdi.h:1119:9, wingdi.h:1119:9 */
pub const NTM_REGULAR: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* wingdi.h:1208:9, wingdi.h:1208:9, wingdi.h:1208:9 */
pub const NTM_BOLD: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* wingdi.h:1209:9, wingdi.h:1209:9, wingdi.h:1209:9 */
pub const NTM_ITALIC: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:1210:9, wingdi.h:1210:9, wingdi.h:1210:9 */
pub const NTM_NONNEGATIVE_AC: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wingdi.h:1214:9, wingdi.h:1214:9, wingdi.h:1214:9 */
pub const NTM_PS_OPENTYPE: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* wingdi.h:1215:9, wingdi.h:1215:9, wingdi.h:1215:9 */
pub const NTM_TT_OPENTYPE: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* wingdi.h:1216:9, wingdi.h:1216:9, wingdi.h:1216:9 */
pub const NTM_MULTIPLEMASTER: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* wingdi.h:1217:9, wingdi.h:1217:9, wingdi.h:1217:9 */
pub const NTM_TYPE1: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* wingdi.h:1218:9, wingdi.h:1218:9, wingdi.h:1218:9 */
pub const NTM_DSIG: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* wingdi.h:1219:9, wingdi.h:1219:9, wingdi.h:1219:9 */
#[cfg(feature="winapi_app")] pub const LF_FACESIZE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:1438:9, wingdi.h:1438:9, wingdi.h:1438:9 */
pub const LF_FULLFACESIZE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:1489:9, wingdi.h:1489:9, wingdi.h:1489:9 */
pub const OUT_DEFAULT_PRECIS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1543:9, wingdi.h:1543:9, wingdi.h:1543:9 */
pub const OUT_STRING_PRECIS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1544:9, wingdi.h:1544:9, wingdi.h:1544:9 */
pub const OUT_CHARACTER_PRECIS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1545:9, wingdi.h:1545:9, wingdi.h:1545:9 */
pub const OUT_STROKE_PRECIS: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1546:9, wingdi.h:1546:9, wingdi.h:1546:9 */
pub const OUT_TT_PRECIS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1547:9, wingdi.h:1547:9, wingdi.h:1547:9 */
pub const OUT_DEVICE_PRECIS: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1548:9, wingdi.h:1548:9, wingdi.h:1548:9 */
pub const OUT_RASTER_PRECIS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1549:9, wingdi.h:1549:9, wingdi.h:1549:9 */
pub const OUT_TT_ONLY_PRECIS: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1550:9, wingdi.h:1550:9, wingdi.h:1550:9 */
pub const OUT_OUTLINE_PRECIS: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1551:9, wingdi.h:1551:9, wingdi.h:1551:9 */
pub const OUT_SCREEN_OUTLINE_PRECIS: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1552:9, wingdi.h:1552:9, wingdi.h:1552:9 */
pub const OUT_PS_ONLY_PRECIS: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1553:9, wingdi.h:1553:9, wingdi.h:1553:9 */
pub const CLIP_DEFAULT_PRECIS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1555:9, wingdi.h:1555:9, wingdi.h:1555:9 */
pub const CLIP_CHARACTER_PRECIS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1556:9, wingdi.h:1556:9, wingdi.h:1556:9 */
pub const CLIP_STROKE_PRECIS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1557:9, wingdi.h:1557:9, wingdi.h:1557:9 */
pub const CLIP_MASK: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:1558:9, wingdi.h:1558:9, wingdi.h:1558:9 */
pub const DEFAULT_QUALITY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1566:9, wingdi.h:1566:9, wingdi.h:1566:9 */
pub const DRAFT_QUALITY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1567:9, wingdi.h:1567:9, wingdi.h:1567:9 */
pub const PROOF_QUALITY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1568:9, wingdi.h:1568:9, wingdi.h:1568:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const NONANTIALIASED_QUALITY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1570:9, wingdi.h:1570:9, wingdi.h:1570:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ANTIALIASED_QUALITY: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1571:9, wingdi.h:1571:9, wingdi.h:1571:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CLEARTYPE_QUALITY: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1575:9, wingdi.h:1575:9, wingdi.h:1575:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const CLEARTYPE_NATURAL_QUALITY: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1576:9, wingdi.h:1576:9, wingdi.h:1576:9 */
pub const DEFAULT_PITCH: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1579:9, wingdi.h:1579:9, wingdi.h:1579:9 */
pub const FIXED_PITCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1580:9, wingdi.h:1580:9, wingdi.h:1580:9 */
pub const VARIABLE_PITCH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1581:9, wingdi.h:1581:9, wingdi.h:1581:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MONO_FONT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1583:9, wingdi.h:1583:9, wingdi.h:1583:9 */
pub const ANSI_CHARSET: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1586:9, wingdi.h:1586:9, wingdi.h:1586:9 */
pub const DEFAULT_CHARSET: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1587:9, wingdi.h:1587:9, wingdi.h:1587:9 */
pub const SYMBOL_CHARSET: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1588:9, wingdi.h:1588:9, wingdi.h:1588:9 */
pub const SHIFTJIS_CHARSET: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:1589:9, wingdi.h:1589:9, wingdi.h:1589:9 */
pub const HANGEUL_CHARSET: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* wingdi.h:1590:9, wingdi.h:1590:9, wingdi.h:1590:9 */
pub const HANGUL_CHARSET: i32 = 0x81i32; /* Integer(129, Yes, Unknown) */ /* wingdi.h:1591:9, wingdi.h:1591:9, wingdi.h:1591:9 */
pub const GB2312_CHARSET: i32 = 0x86i32; /* Integer(134, Yes, Unknown) */ /* wingdi.h:1592:9, wingdi.h:1592:9, wingdi.h:1592:9 */
pub const CHINESEBIG5_CHARSET: i32 = 0x88i32; /* Integer(136, Yes, Unknown) */ /* wingdi.h:1593:9, wingdi.h:1593:9, wingdi.h:1593:9 */
pub const OEM_CHARSET: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* wingdi.h:1594:9, wingdi.h:1594:9, wingdi.h:1594:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const JOHAB_CHARSET: i32 = 0x82i32; /* Integer(130, Yes, Unknown) */ /* wingdi.h:1596:9, wingdi.h:1596:9, wingdi.h:1596:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const HEBREW_CHARSET: i32 = 0xb1i32; /* Integer(177, Yes, Unknown) */ /* wingdi.h:1597:9, wingdi.h:1597:9, wingdi.h:1597:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const ARABIC_CHARSET: i32 = 0xb2i32; /* Integer(178, Yes, Unknown) */ /* wingdi.h:1598:9, wingdi.h:1598:9, wingdi.h:1598:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GREEK_CHARSET: i32 = 0xa1i32; /* Integer(161, Yes, Unknown) */ /* wingdi.h:1599:9, wingdi.h:1599:9, wingdi.h:1599:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const TURKISH_CHARSET: i32 = 0xa2i32; /* Integer(162, Yes, Unknown) */ /* wingdi.h:1600:9, wingdi.h:1600:9, wingdi.h:1600:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const VIETNAMESE_CHARSET: i32 = 0xa3i32; /* Integer(163, Yes, Unknown) */ /* wingdi.h:1601:9, wingdi.h:1601:9, wingdi.h:1601:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const THAI_CHARSET: i32 = 0xdei32; /* Integer(222, Yes, Unknown) */ /* wingdi.h:1602:9, wingdi.h:1602:9, wingdi.h:1602:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const EASTEUROPE_CHARSET: i32 = 0xeei32; /* Integer(238, Yes, Unknown) */ /* wingdi.h:1603:9, wingdi.h:1603:9, wingdi.h:1603:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const RUSSIAN_CHARSET: i32 = 0xcci32; /* Integer(204, Yes, Unknown) */ /* wingdi.h:1604:9, wingdi.h:1604:9, wingdi.h:1604:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const MAC_CHARSET: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* wingdi.h:1606:9, wingdi.h:1606:9, wingdi.h:1606:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const BALTIC_CHARSET: i32 = 0xbai32; /* Integer(186, Yes, Unknown) */ /* wingdi.h:1607:9, wingdi.h:1607:9, wingdi.h:1607:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_LATIN1: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:1609:9, wingdi.h:1609:9, wingdi.h:1609:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_LATIN2: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:1610:9, wingdi.h:1610:9, wingdi.h:1610:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_CYRILLIC: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:1611:9, wingdi.h:1611:9, wingdi.h:1611:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_GREEK: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* wingdi.h:1612:9, wingdi.h:1612:9, wingdi.h:1612:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_TURKISH: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* wingdi.h:1613:9, wingdi.h:1613:9, wingdi.h:1613:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_HEBREW: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* wingdi.h:1614:9, wingdi.h:1614:9, wingdi.h:1614:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_ARABIC: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* wingdi.h:1615:9, wingdi.h:1615:9, wingdi.h:1615:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_BALTIC: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* wingdi.h:1616:9, wingdi.h:1616:9, wingdi.h:1616:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_VIETNAMESE: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* wingdi.h:1617:9, wingdi.h:1617:9, wingdi.h:1617:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_THAI: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* wingdi.h:1618:9, wingdi.h:1618:9, wingdi.h:1618:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_JISJAPAN: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* wingdi.h:1619:9, wingdi.h:1619:9, wingdi.h:1619:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_CHINESESIMP: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* wingdi.h:1620:9, wingdi.h:1620:9, wingdi.h:1620:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_WANSUNG: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* wingdi.h:1621:9, wingdi.h:1621:9, wingdi.h:1621:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_CHINESETRAD: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* wingdi.h:1622:9, wingdi.h:1622:9, wingdi.h:1622:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_JOHAB: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* wingdi.h:1623:9, wingdi.h:1623:9, wingdi.h:1623:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FS_SYMBOL: i64 = 0x80000000i64; /* Integer(2147483648, Yes, Long) */ /* wingdi.h:1624:9, wingdi.h:1624:9, wingdi.h:1624:9 */
pub const FW_DONTCARE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1639:9, wingdi.h:1639:9, wingdi.h:1639:9 */
pub const FW_THIN: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* wingdi.h:1640:9, wingdi.h:1640:9, wingdi.h:1640:9 */
pub const FW_EXTRALIGHT: i32 = 0xc8i32; /* Integer(200, Yes, Unknown) */ /* wingdi.h:1641:9, wingdi.h:1641:9, wingdi.h:1641:9 */
pub const FW_LIGHT: i32 = 0x12ci32; /* Integer(300, Yes, Unknown) */ /* wingdi.h:1642:9, wingdi.h:1642:9, wingdi.h:1642:9 */
pub const FW_NORMAL: i32 = 0x190i32; /* Integer(400, Yes, Unknown) */ /* wingdi.h:1643:9, wingdi.h:1643:9, wingdi.h:1643:9 */
pub const FW_MEDIUM: i32 = 0x1f4i32; /* Integer(500, Yes, Unknown) */ /* wingdi.h:1644:9, wingdi.h:1644:9, wingdi.h:1644:9 */
pub const FW_SEMIBOLD: i32 = 0x258i32; /* Integer(600, Yes, Unknown) */ /* wingdi.h:1645:9, wingdi.h:1645:9, wingdi.h:1645:9 */
pub const FW_BOLD: i32 = 0x2bci32; /* Integer(700, Yes, Unknown) */ /* wingdi.h:1646:9, wingdi.h:1646:9, wingdi.h:1646:9 */
pub const FW_EXTRABOLD: i32 = 0x320i32; /* Integer(800, Yes, Unknown) */ /* wingdi.h:1647:9, wingdi.h:1647:9, wingdi.h:1647:9 */
pub const FW_HEAVY: i32 = 0x384i32; /* Integer(900, Yes, Unknown) */ /* wingdi.h:1648:9, wingdi.h:1648:9, wingdi.h:1648:9 */
#[doc(inline)] pub use ::wingdi::FW_EXTRALIGHT as FW_ULTRALIGHT; /* wingdi.h:1650:9, wingdi.h:1650:9, wingdi.h:1650:9 */
#[doc(inline)] pub use ::wingdi::FW_NORMAL as FW_REGULAR; /* wingdi.h:1651:9, wingdi.h:1651:9, wingdi.h:1651:9 */
#[doc(inline)] pub use ::wingdi::FW_SEMIBOLD as FW_DEMIBOLD; /* wingdi.h:1652:9, wingdi.h:1652:9, wingdi.h:1652:9 */
#[doc(inline)] pub use ::wingdi::FW_EXTRABOLD as FW_ULTRABOLD; /* wingdi.h:1653:9, wingdi.h:1653:9, wingdi.h:1653:9 */
#[doc(inline)] pub use ::wingdi::FW_HEAVY as FW_BLACK; /* wingdi.h:1654:9, wingdi.h:1654:9, wingdi.h:1654:9 */
pub const PANOSE_COUNT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1656:9, wingdi.h:1656:9, wingdi.h:1656:9 */
pub const PAN_FAMILYTYPE_INDEX: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1657:9, wingdi.h:1657:9, wingdi.h:1657:9 */
pub const PAN_SERIFSTYLE_INDEX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1658:9, wingdi.h:1658:9, wingdi.h:1658:9 */
pub const PAN_WEIGHT_INDEX: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1659:9, wingdi.h:1659:9, wingdi.h:1659:9 */
pub const PAN_PROPORTION_INDEX: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1660:9, wingdi.h:1660:9, wingdi.h:1660:9 */
pub const PAN_CONTRAST_INDEX: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1661:9, wingdi.h:1661:9, wingdi.h:1661:9 */
pub const PAN_STROKEVARIATION_INDEX: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1662:9, wingdi.h:1662:9, wingdi.h:1662:9 */
pub const PAN_ARMSTYLE_INDEX: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1663:9, wingdi.h:1663:9, wingdi.h:1663:9 */
pub const PAN_LETTERFORM_INDEX: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1664:9, wingdi.h:1664:9, wingdi.h:1664:9 */
pub const PAN_MIDLINE_INDEX: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1665:9, wingdi.h:1665:9, wingdi.h:1665:9 */
pub const PAN_XHEIGHT_INDEX: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1666:9, wingdi.h:1666:9, wingdi.h:1666:9 */
pub const PAN_CULTURE_LATIN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1668:9, wingdi.h:1668:9, wingdi.h:1668:9 */
#[cfg(feature="winapi_app")] pub const PAN_ANY: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1687:9, wingdi.h:1687:9, wingdi.h:1687:9 */
#[cfg(feature="winapi_app")] pub const PAN_NO_FIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1688:9, wingdi.h:1688:9, wingdi.h:1688:9 */
#[cfg(feature="winapi_app")] pub const PAN_FAMILY_TEXT_DISPLAY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1690:9, wingdi.h:1690:9, wingdi.h:1690:9 */
#[cfg(feature="winapi_app")] pub const PAN_FAMILY_SCRIPT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1691:9, wingdi.h:1691:9, wingdi.h:1691:9 */
#[cfg(feature="winapi_app")] pub const PAN_FAMILY_DECORATIVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1692:9, wingdi.h:1692:9, wingdi.h:1692:9 */
#[cfg(feature="winapi_app")] pub const PAN_FAMILY_PICTORIAL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1693:9, wingdi.h:1693:9, wingdi.h:1693:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_COVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1695:9, wingdi.h:1695:9, wingdi.h:1695:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_OBTUSE_COVE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1696:9, wingdi.h:1696:9, wingdi.h:1696:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_SQUARE_COVE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1697:9, wingdi.h:1697:9, wingdi.h:1697:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_OBTUSE_SQUARE_COVE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1698:9, wingdi.h:1698:9, wingdi.h:1698:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_SQUARE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1699:9, wingdi.h:1699:9, wingdi.h:1699:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_THIN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1700:9, wingdi.h:1700:9, wingdi.h:1700:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_BONE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1701:9, wingdi.h:1701:9, wingdi.h:1701:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_EXAGGERATED: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1702:9, wingdi.h:1702:9, wingdi.h:1702:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_TRIANGLE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1703:9, wingdi.h:1703:9, wingdi.h:1703:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_NORMAL_SANS: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1704:9, wingdi.h:1704:9, wingdi.h:1704:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_OBTUSE_SANS: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1705:9, wingdi.h:1705:9, wingdi.h:1705:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_PERP_SANS: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:1706:9, wingdi.h:1706:9, wingdi.h:1706:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_FLARED: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:1707:9, wingdi.h:1707:9, wingdi.h:1707:9 */
#[cfg(feature="winapi_app")] pub const PAN_SERIF_ROUNDED: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:1708:9, wingdi.h:1708:9, wingdi.h:1708:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_VERY_LIGHT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1710:9, wingdi.h:1710:9, wingdi.h:1710:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_LIGHT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1711:9, wingdi.h:1711:9, wingdi.h:1711:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_THIN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1712:9, wingdi.h:1712:9, wingdi.h:1712:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_BOOK: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1713:9, wingdi.h:1713:9, wingdi.h:1713:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_MEDIUM: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1714:9, wingdi.h:1714:9, wingdi.h:1714:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_DEMI: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1715:9, wingdi.h:1715:9, wingdi.h:1715:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_BOLD: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1716:9, wingdi.h:1716:9, wingdi.h:1716:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_HEAVY: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1717:9, wingdi.h:1717:9, wingdi.h:1717:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_BLACK: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1718:9, wingdi.h:1718:9, wingdi.h:1718:9 */
#[cfg(feature="winapi_app")] pub const PAN_WEIGHT_NORD: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1719:9, wingdi.h:1719:9, wingdi.h:1719:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_OLD_STYLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1721:9, wingdi.h:1721:9, wingdi.h:1721:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_MODERN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1722:9, wingdi.h:1722:9, wingdi.h:1722:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_EVEN_WIDTH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1723:9, wingdi.h:1723:9, wingdi.h:1723:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_EXPANDED: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1724:9, wingdi.h:1724:9, wingdi.h:1724:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_CONDENSED: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1725:9, wingdi.h:1725:9, wingdi.h:1725:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_VERY_EXPANDED: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1726:9, wingdi.h:1726:9, wingdi.h:1726:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_VERY_CONDENSED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1727:9, wingdi.h:1727:9, wingdi.h:1727:9 */
#[cfg(feature="winapi_app")] pub const PAN_PROP_MONOSPACED: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1728:9, wingdi.h:1728:9, wingdi.h:1728:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_NONE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1730:9, wingdi.h:1730:9, wingdi.h:1730:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_VERY_LOW: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1731:9, wingdi.h:1731:9, wingdi.h:1731:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_LOW: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1732:9, wingdi.h:1732:9, wingdi.h:1732:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_MEDIUM_LOW: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1733:9, wingdi.h:1733:9, wingdi.h:1733:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_MEDIUM: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1734:9, wingdi.h:1734:9, wingdi.h:1734:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_MEDIUM_HIGH: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1735:9, wingdi.h:1735:9, wingdi.h:1735:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_HIGH: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1736:9, wingdi.h:1736:9, wingdi.h:1736:9 */
#[cfg(feature="winapi_app")] pub const PAN_CONTRAST_VERY_HIGH: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1737:9, wingdi.h:1737:9, wingdi.h:1737:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_GRADUAL_DIAG: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1739:9, wingdi.h:1739:9, wingdi.h:1739:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_GRADUAL_TRAN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1740:9, wingdi.h:1740:9, wingdi.h:1740:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_GRADUAL_VERT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1741:9, wingdi.h:1741:9, wingdi.h:1741:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_GRADUAL_HORZ: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1742:9, wingdi.h:1742:9, wingdi.h:1742:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_RAPID_VERT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1743:9, wingdi.h:1743:9, wingdi.h:1743:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_RAPID_HORZ: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1744:9, wingdi.h:1744:9, wingdi.h:1744:9 */
#[cfg(feature="winapi_app")] pub const PAN_STROKE_INSTANT_VERT: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1745:9, wingdi.h:1745:9, wingdi.h:1745:9 */
#[cfg(feature="winapi_app")] pub const PAN_STRAIGHT_ARMS_HORZ: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1747:9, wingdi.h:1747:9, wingdi.h:1747:9 */
#[cfg(feature="winapi_app")] pub const PAN_STRAIGHT_ARMS_WEDGE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1748:9, wingdi.h:1748:9, wingdi.h:1748:9 */
#[cfg(feature="winapi_app")] pub const PAN_STRAIGHT_ARMS_VERT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1749:9, wingdi.h:1749:9, wingdi.h:1749:9 */
#[cfg(feature="winapi_app")] pub const PAN_STRAIGHT_ARMS_SINGLE_SERIF: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1750:9, wingdi.h:1750:9, wingdi.h:1750:9 */
#[cfg(feature="winapi_app")] pub const PAN_STRAIGHT_ARMS_DOUBLE_SERIF: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1751:9, wingdi.h:1751:9, wingdi.h:1751:9 */
#[cfg(feature="winapi_app")] pub const PAN_BENT_ARMS_HORZ: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1752:9, wingdi.h:1752:9, wingdi.h:1752:9 */
#[cfg(feature="winapi_app")] pub const PAN_BENT_ARMS_WEDGE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1753:9, wingdi.h:1753:9, wingdi.h:1753:9 */
#[cfg(feature="winapi_app")] pub const PAN_BENT_ARMS_VERT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1754:9, wingdi.h:1754:9, wingdi.h:1754:9 */
#[cfg(feature="winapi_app")] pub const PAN_BENT_ARMS_SINGLE_SERIF: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1755:9, wingdi.h:1755:9, wingdi.h:1755:9 */
#[cfg(feature="winapi_app")] pub const PAN_BENT_ARMS_DOUBLE_SERIF: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1756:9, wingdi.h:1756:9, wingdi.h:1756:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_CONTACT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1758:9, wingdi.h:1758:9, wingdi.h:1758:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_WEIGHTED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1759:9, wingdi.h:1759:9, wingdi.h:1759:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_BOXED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1760:9, wingdi.h:1760:9, wingdi.h:1760:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_FLATTENED: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1761:9, wingdi.h:1761:9, wingdi.h:1761:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_ROUNDED: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1762:9, wingdi.h:1762:9, wingdi.h:1762:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_OFF_CENTER: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1763:9, wingdi.h:1763:9, wingdi.h:1763:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_NORMAL_SQUARE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1764:9, wingdi.h:1764:9, wingdi.h:1764:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_CONTACT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1765:9, wingdi.h:1765:9, wingdi.h:1765:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_WEIGHTED: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1766:9, wingdi.h:1766:9, wingdi.h:1766:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_BOXED: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1767:9, wingdi.h:1767:9, wingdi.h:1767:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_FLATTENED: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1768:9, wingdi.h:1768:9, wingdi.h:1768:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_ROUNDED: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:1769:9, wingdi.h:1769:9, wingdi.h:1769:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_OFF_CENTER: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:1770:9, wingdi.h:1770:9, wingdi.h:1770:9 */
#[cfg(feature="winapi_app")] pub const PAN_LETT_OBLIQUE_SQUARE: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:1771:9, wingdi.h:1771:9, wingdi.h:1771:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_STANDARD_TRIMMED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1773:9, wingdi.h:1773:9, wingdi.h:1773:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_STANDARD_POINTED: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1774:9, wingdi.h:1774:9, wingdi.h:1774:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_STANDARD_SERIFED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1775:9, wingdi.h:1775:9, wingdi.h:1775:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_HIGH_TRIMMED: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1776:9, wingdi.h:1776:9, wingdi.h:1776:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_HIGH_POINTED: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1777:9, wingdi.h:1777:9, wingdi.h:1777:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_HIGH_SERIFED: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1778:9, wingdi.h:1778:9, wingdi.h:1778:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_CONSTANT_TRIMMED: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1779:9, wingdi.h:1779:9, wingdi.h:1779:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_CONSTANT_POINTED: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1780:9, wingdi.h:1780:9, wingdi.h:1780:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_CONSTANT_SERIFED: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1781:9, wingdi.h:1781:9, wingdi.h:1781:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_LOW_TRIMMED: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1782:9, wingdi.h:1782:9, wingdi.h:1782:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_LOW_POINTED: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1783:9, wingdi.h:1783:9, wingdi.h:1783:9 */
#[cfg(feature="winapi_app")] pub const PAN_MIDLINE_LOW_SERIFED: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:1784:9, wingdi.h:1784:9, wingdi.h:1784:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_CONSTANT_SMALL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1786:9, wingdi.h:1786:9, wingdi.h:1786:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_CONSTANT_STD: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1787:9, wingdi.h:1787:9, wingdi.h:1787:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_CONSTANT_LARGE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1788:9, wingdi.h:1788:9, wingdi.h:1788:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_DUCKING_SMALL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1789:9, wingdi.h:1789:9, wingdi.h:1789:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_DUCKING_STD: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1790:9, wingdi.h:1790:9, wingdi.h:1790:9 */
#[cfg(feature="winapi_app")] pub const PAN_XHEIGHT_DUCKING_LARGE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1791:9, wingdi.h:1791:9, wingdi.h:1791:9 */
#[cfg(feature="winapi_app")] pub const ELF_VENDOR_SIZE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1794:9, wingdi.h:1794:9, wingdi.h:1794:9 */
pub const ELF_VERSION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1838:9, wingdi.h:1838:9, wingdi.h:1838:9 */
pub const ELF_CULTURE_LATIN: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1839:9, wingdi.h:1839:9, wingdi.h:1839:9 */
pub const RASTER_FONTTYPE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1842:9, wingdi.h:1842:9, wingdi.h:1842:9 */
pub const DEVICE_FONTTYPE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1843:9, wingdi.h:1843:9, wingdi.h:1843:9 */
pub const TRUETYPE_FONTTYPE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1844:9, wingdi.h:1844:9, wingdi.h:1844:9 */
pub const PC_RESERVED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1852:9, wingdi.h:1852:9, wingdi.h:1852:9 */
pub const PC_EXPLICIT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1853:9, wingdi.h:1853:9, wingdi.h:1853:9 */
pub const PC_NOCOLLAPSE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1854:9, wingdi.h:1854:9, wingdi.h:1854:9 */
pub const TRANSPARENT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1861:9, wingdi.h:1861:9, wingdi.h:1861:9 */
pub const OPAQUE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1862:9, wingdi.h:1862:9, wingdi.h:1862:9 */
pub const BKMODE_LAST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1863:9, wingdi.h:1863:9, wingdi.h:1863:9 */
pub const GM_COMPATIBLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1867:9, wingdi.h:1867:9, wingdi.h:1867:9 */
pub const GM_ADVANCED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1868:9, wingdi.h:1868:9, wingdi.h:1868:9 */
pub const GM_LAST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1869:9, wingdi.h:1869:9, wingdi.h:1869:9 */
pub const PT_CLOSEFIGURE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1872:9, wingdi.h:1872:9, wingdi.h:1872:9 */
pub const PT_LINETO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1873:9, wingdi.h:1873:9, wingdi.h:1873:9 */
pub const PT_BEZIERTO: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1874:9, wingdi.h:1874:9, wingdi.h:1874:9 */
pub const PT_MOVETO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1875:9, wingdi.h:1875:9, wingdi.h:1875:9 */
pub const MM_TEXT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1878:9, wingdi.h:1878:9, wingdi.h:1878:9 */
pub const MM_LOMETRIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1879:9, wingdi.h:1879:9, wingdi.h:1879:9 */
pub const MM_HIMETRIC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1880:9, wingdi.h:1880:9, wingdi.h:1880:9 */
pub const MM_LOENGLISH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1881:9, wingdi.h:1881:9, wingdi.h:1881:9 */
pub const MM_HIENGLISH: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1882:9, wingdi.h:1882:9, wingdi.h:1882:9 */
pub const MM_TWIPS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1883:9, wingdi.h:1883:9, wingdi.h:1883:9 */
pub const MM_ISOTROPIC: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1884:9, wingdi.h:1884:9, wingdi.h:1884:9 */
pub const MM_ANISOTROPIC: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1885:9, wingdi.h:1885:9, wingdi.h:1885:9 */
#[doc(inline)] pub use ::wingdi::MM_TEXT as MM_MIN; /* wingdi.h:1888:9, wingdi.h:1888:9, wingdi.h:1888:9 */
#[doc(inline)] pub use ::wingdi::MM_ANISOTROPIC as MM_MAX; /* wingdi.h:1889:9, wingdi.h:1889:9, wingdi.h:1889:9 */
#[doc(inline)] pub use ::wingdi::MM_TWIPS as MM_MAX_FIXEDSCALE; /* wingdi.h:1890:9, wingdi.h:1890:9, wingdi.h:1890:9 */
pub const ABSOLUTE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1893:9, wingdi.h:1893:9, wingdi.h:1893:9 */
pub const RELATIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1894:9, wingdi.h:1894:9, wingdi.h:1894:9 */
pub const WHITE_BRUSH: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1897:9, wingdi.h:1897:9, wingdi.h:1897:9 */
pub const LTGRAY_BRUSH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1898:9, wingdi.h:1898:9, wingdi.h:1898:9 */
pub const GRAY_BRUSH: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1899:9, wingdi.h:1899:9, wingdi.h:1899:9 */
pub const DKGRAY_BRUSH: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1900:9, wingdi.h:1900:9, wingdi.h:1900:9 */
pub const BLACK_BRUSH: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1901:9, wingdi.h:1901:9, wingdi.h:1901:9 */
pub const NULL_BRUSH: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1902:9, wingdi.h:1902:9, wingdi.h:1902:9 */
#[doc(inline)] pub use ::wingdi::NULL_BRUSH as HOLLOW_BRUSH; /* wingdi.h:1903:9, wingdi.h:1903:9, wingdi.h:1903:9 */
pub const WHITE_PEN: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1904:9, wingdi.h:1904:9, wingdi.h:1904:9 */
pub const BLACK_PEN: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1905:9, wingdi.h:1905:9, wingdi.h:1905:9 */
pub const NULL_PEN: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1906:9, wingdi.h:1906:9, wingdi.h:1906:9 */
pub const OEM_FIXED_FONT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1907:9, wingdi.h:1907:9, wingdi.h:1907:9 */
pub const ANSI_FIXED_FONT: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:1908:9, wingdi.h:1908:9, wingdi.h:1908:9 */
pub const ANSI_VAR_FONT: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1909:9, wingdi.h:1909:9, wingdi.h:1909:9 */
pub const SYSTEM_FONT: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:1910:9, wingdi.h:1910:9, wingdi.h:1910:9 */
pub const DEVICE_DEFAULT_FONT: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:1911:9, wingdi.h:1911:9, wingdi.h:1911:9 */
pub const DEFAULT_PALETTE: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:1912:9, wingdi.h:1912:9, wingdi.h:1912:9 */
pub const SYSTEM_FIXED_FONT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:1913:9, wingdi.h:1913:9, wingdi.h:1913:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DEFAULT_GUI_FONT: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:1916:9, wingdi.h:1916:9, wingdi.h:1916:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_BRUSH: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:1920:9, wingdi.h:1920:9, wingdi.h:1920:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PEN: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:1921:9, wingdi.h:1921:9, wingdi.h:1921:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const STOCK_LAST: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:1925:9, wingdi.h:1925:9, wingdi.h:1925:9 */
pub const CLR_INVALID: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* wingdi.h:1932:9, wingdi.h:1932:9, wingdi.h:1932:9 */
pub const BS_SOLID: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1935:9, wingdi.h:1935:9, wingdi.h:1935:9 */
pub const BS_NULL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1936:9, wingdi.h:1936:9, wingdi.h:1936:9 */
#[doc(inline)] pub use ::wingdi::BS_NULL as BS_HOLLOW; /* wingdi.h:1937:9, wingdi.h:1937:9, wingdi.h:1937:9 */
pub const BS_HATCHED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1938:9, wingdi.h:1938:9, wingdi.h:1938:9 */
pub const BS_PATTERN: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1939:9, wingdi.h:1939:9, wingdi.h:1939:9 */
pub const BS_INDEXED: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1940:9, wingdi.h:1940:9, wingdi.h:1940:9 */
pub const BS_DIBPATTERN: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1941:9, wingdi.h:1941:9, wingdi.h:1941:9 */
pub const BS_DIBPATTERNPT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1942:9, wingdi.h:1942:9, wingdi.h:1942:9 */
pub const BS_PATTERN8X8: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1943:9, wingdi.h:1943:9, wingdi.h:1943:9 */
pub const BS_DIBPATTERN8X8: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1944:9, wingdi.h:1944:9, wingdi.h:1944:9 */
pub const BS_MONOPATTERN: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:1945:9, wingdi.h:1945:9, wingdi.h:1945:9 */
pub const HS_HORIZONTAL: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1948:9, wingdi.h:1948:9, wingdi.h:1948:9 */
pub const HS_VERTICAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1949:9, wingdi.h:1949:9, wingdi.h:1949:9 */
pub const HS_FDIAGONAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1950:9, wingdi.h:1950:9, wingdi.h:1950:9 */
pub const HS_BDIAGONAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1951:9, wingdi.h:1951:9, wingdi.h:1951:9 */
pub const HS_CROSS: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1952:9, wingdi.h:1952:9, wingdi.h:1952:9 */
pub const HS_DIAGCROSS: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1953:9, wingdi.h:1953:9, wingdi.h:1953:9 */
pub const HS_API_MAX: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1954:9, wingdi.h:1954:9, wingdi.h:1954:9 */
pub const PS_SOLID: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1957:9, wingdi.h:1957:9, wingdi.h:1957:9 */
pub const PS_DASH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1958:9, wingdi.h:1958:9, wingdi.h:1958:9 */
pub const PS_DOT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1959:9, wingdi.h:1959:9, wingdi.h:1959:9 */
pub const PS_DASHDOT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:1960:9, wingdi.h:1960:9, wingdi.h:1960:9 */
pub const PS_DASHDOTDOT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1961:9, wingdi.h:1961:9, wingdi.h:1961:9 */
pub const PS_NULL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:1962:9, wingdi.h:1962:9, wingdi.h:1962:9 */
pub const PS_INSIDEFRAME: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1963:9, wingdi.h:1963:9, wingdi.h:1963:9 */
pub const PS_USERSTYLE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:1964:9, wingdi.h:1964:9, wingdi.h:1964:9 */
pub const PS_ALTERNATE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1965:9, wingdi.h:1965:9, wingdi.h:1965:9 */
pub const PS_STYLE_MASK: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:1966:9, wingdi.h:1966:9, wingdi.h:1966:9 */
pub const PS_ENDCAP_ROUND: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1968:9, wingdi.h:1968:9, wingdi.h:1968:9 */
pub const PS_ENDCAP_SQUARE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:1969:9, wingdi.h:1969:9, wingdi.h:1969:9 */
pub const PS_ENDCAP_FLAT: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:1970:9, wingdi.h:1970:9, wingdi.h:1970:9 */
pub const PS_ENDCAP_MASK: i32 = 0xf00i32; /* Integer(3840, Yes, Unknown) */ /* wingdi.h:1971:9, wingdi.h:1971:9, wingdi.h:1971:9 */
pub const PS_JOIN_ROUND: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1973:9, wingdi.h:1973:9, wingdi.h:1973:9 */
pub const PS_JOIN_BEVEL: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:1974:9, wingdi.h:1974:9, wingdi.h:1974:9 */
pub const PS_JOIN_MITER: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:1975:9, wingdi.h:1975:9, wingdi.h:1975:9 */
pub const PS_JOIN_MASK: i32 = 0xf000i32; /* Integer(61440, Yes, Unknown) */ /* wingdi.h:1976:9, wingdi.h:1976:9, wingdi.h:1976:9 */
pub const PS_COSMETIC: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1978:9, wingdi.h:1978:9, wingdi.h:1978:9 */
pub const PS_GEOMETRIC: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wingdi.h:1979:9, wingdi.h:1979:9, wingdi.h:1979:9 */
pub const PS_TYPE_MASK: i32 = 0xf0000i32; /* Integer(983040, Yes, Unknown) */ /* wingdi.h:1980:9, wingdi.h:1980:9, wingdi.h:1980:9 */
pub const AD_COUNTERCLOCKWISE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:1982:9, wingdi.h:1982:9, wingdi.h:1982:9 */
pub const AD_CLOCKWISE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1983:9, wingdi.h:1983:9, wingdi.h:1983:9 */
pub const DRIVERVERSION: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:1986:9, wingdi.h:1986:9, wingdi.h:1986:9 */
pub const TECHNOLOGY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:1987:9, wingdi.h:1987:9, wingdi.h:1987:9 */
pub const HORZSIZE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:1988:9, wingdi.h:1988:9, wingdi.h:1988:9 */
pub const VERTSIZE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:1989:9, wingdi.h:1989:9, wingdi.h:1989:9 */
pub const HORZRES: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:1990:9, wingdi.h:1990:9, wingdi.h:1990:9 */
pub const VERTRES: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:1991:9, wingdi.h:1991:9, wingdi.h:1991:9 */
pub const BITSPIXEL: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:1992:9, wingdi.h:1992:9, wingdi.h:1992:9 */
pub const PLANES: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:1993:9, wingdi.h:1993:9, wingdi.h:1993:9 */
pub const NUMBRUSHES: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:1994:9, wingdi.h:1994:9, wingdi.h:1994:9 */
pub const NUMPENS: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:1995:9, wingdi.h:1995:9, wingdi.h:1995:9 */
pub const NUMMARKERS: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:1996:9, wingdi.h:1996:9, wingdi.h:1996:9 */
pub const NUMFONTS: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* wingdi.h:1997:9, wingdi.h:1997:9, wingdi.h:1997:9 */
pub const NUMCOLORS: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:1998:9, wingdi.h:1998:9, wingdi.h:1998:9 */
pub const PDEVICESIZE: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* wingdi.h:1999:9, wingdi.h:1999:9, wingdi.h:1999:9 */
pub const CURVECAPS: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* wingdi.h:2000:9, wingdi.h:2000:9, wingdi.h:2000:9 */
pub const LINECAPS: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:2001:9, wingdi.h:2001:9, wingdi.h:2001:9 */
pub const POLYGONALCAPS: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2002:9, wingdi.h:2002:9, wingdi.h:2002:9 */
pub const TEXTCAPS: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* wingdi.h:2003:9, wingdi.h:2003:9, wingdi.h:2003:9 */
pub const CLIPCAPS: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* wingdi.h:2004:9, wingdi.h:2004:9, wingdi.h:2004:9 */
pub const RASTERCAPS: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* wingdi.h:2005:9, wingdi.h:2005:9, wingdi.h:2005:9 */
pub const ASPECTX: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* wingdi.h:2006:9, wingdi.h:2006:9, wingdi.h:2006:9 */
pub const ASPECTY: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* wingdi.h:2007:9, wingdi.h:2007:9, wingdi.h:2007:9 */
pub const ASPECTXY: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* wingdi.h:2008:9, wingdi.h:2008:9, wingdi.h:2008:9 */
pub const LOGPIXELSX: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* wingdi.h:2010:9, wingdi.h:2010:9, wingdi.h:2010:9 */
pub const LOGPIXELSY: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* wingdi.h:2011:9, wingdi.h:2011:9, wingdi.h:2011:9 */
pub const SIZEPALETTE: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* wingdi.h:2013:9, wingdi.h:2013:9, wingdi.h:2013:9 */
pub const NUMRESERVED: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* wingdi.h:2014:9, wingdi.h:2014:9, wingdi.h:2014:9 */
pub const COLORRES: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* wingdi.h:2015:9, wingdi.h:2015:9, wingdi.h:2015:9 */
pub const PHYSICALWIDTH: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* wingdi.h:2019:9, wingdi.h:2019:9, wingdi.h:2019:9 */
pub const PHYSICALHEIGHT: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* wingdi.h:2020:9, wingdi.h:2020:9, wingdi.h:2020:9 */
pub const PHYSICALOFFSETX: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* wingdi.h:2021:9, wingdi.h:2021:9, wingdi.h:2021:9 */
pub const PHYSICALOFFSETY: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* wingdi.h:2022:9, wingdi.h:2022:9, wingdi.h:2022:9 */
pub const SCALINGFACTORX: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* wingdi.h:2023:9, wingdi.h:2023:9, wingdi.h:2023:9 */
pub const SCALINGFACTORY: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* wingdi.h:2024:9, wingdi.h:2024:9, wingdi.h:2024:9 */
pub const VREFRESH: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* wingdi.h:2028:9, wingdi.h:2028:9, wingdi.h:2028:9 */
pub const DESKTOPVERTRES: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* wingdi.h:2030:9, wingdi.h:2030:9, wingdi.h:2030:9 */
pub const DESKTOPHORZRES: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* wingdi.h:2032:9, wingdi.h:2032:9, wingdi.h:2032:9 */
pub const BLTALIGNMENT: i32 = 0x77i32; /* Integer(119, Yes, Unknown) */ /* wingdi.h:2034:9, wingdi.h:2034:9, wingdi.h:2034:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SHADEBLENDCAPS: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* wingdi.h:2037:9, wingdi.h:2037:9, wingdi.h:2037:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const COLORMGMTCAPS: i32 = 0x79i32; /* Integer(121, Yes, Unknown) */ /* wingdi.h:2038:9, wingdi.h:2038:9, wingdi.h:2038:9 */
pub const DT_PLOTTER: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2046:9, wingdi.h:2046:9, wingdi.h:2046:9 */
pub const DT_RASDISPLAY: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2047:9, wingdi.h:2047:9, wingdi.h:2047:9 */
pub const DT_RASPRINTER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2048:9, wingdi.h:2048:9, wingdi.h:2048:9 */
pub const DT_RASCAMERA: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2049:9, wingdi.h:2049:9, wingdi.h:2049:9 */
pub const DT_CHARSTREAM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2050:9, wingdi.h:2050:9, wingdi.h:2050:9 */
pub const DT_METAFILE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:2051:9, wingdi.h:2051:9, wingdi.h:2051:9 */
pub const DT_DISPFILE: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:2052:9, wingdi.h:2052:9, wingdi.h:2052:9 */
pub const CC_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2055:9, wingdi.h:2055:9, wingdi.h:2055:9 */
pub const CC_CIRCLES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2056:9, wingdi.h:2056:9, wingdi.h:2056:9 */
pub const CC_PIE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2057:9, wingdi.h:2057:9, wingdi.h:2057:9 */
pub const CC_CHORD: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2058:9, wingdi.h:2058:9, wingdi.h:2058:9 */
pub const CC_ELLIPSES: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2059:9, wingdi.h:2059:9, wingdi.h:2059:9 */
pub const CC_WIDE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2060:9, wingdi.h:2060:9, wingdi.h:2060:9 */
pub const CC_STYLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2061:9, wingdi.h:2061:9, wingdi.h:2061:9 */
pub const CC_WIDESTYLED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2062:9, wingdi.h:2062:9, wingdi.h:2062:9 */
pub const CC_INTERIORS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:2063:9, wingdi.h:2063:9, wingdi.h:2063:9 */
pub const CC_ROUNDRECT: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2064:9, wingdi.h:2064:9, wingdi.h:2064:9 */
pub const LC_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2067:9, wingdi.h:2067:9, wingdi.h:2067:9 */
pub const LC_POLYLINE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2068:9, wingdi.h:2068:9, wingdi.h:2068:9 */
pub const LC_MARKER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2069:9, wingdi.h:2069:9, wingdi.h:2069:9 */
pub const LC_POLYMARKER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2070:9, wingdi.h:2070:9, wingdi.h:2070:9 */
pub const LC_WIDE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2071:9, wingdi.h:2071:9, wingdi.h:2071:9 */
pub const LC_STYLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2072:9, wingdi.h:2072:9, wingdi.h:2072:9 */
pub const LC_WIDESTYLED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2073:9, wingdi.h:2073:9, wingdi.h:2073:9 */
pub const LC_INTERIORS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:2074:9, wingdi.h:2074:9, wingdi.h:2074:9 */
pub const PC_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2077:9, wingdi.h:2077:9, wingdi.h:2077:9 */
pub const PC_POLYGON: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2078:9, wingdi.h:2078:9, wingdi.h:2078:9 */
pub const PC_RECTANGLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2079:9, wingdi.h:2079:9, wingdi.h:2079:9 */
pub const PC_WINDPOLYGON: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2080:9, wingdi.h:2080:9, wingdi.h:2080:9 */
pub const PC_TRAPEZOID: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2081:9, wingdi.h:2081:9, wingdi.h:2081:9 */
pub const PC_SCANLINE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2082:9, wingdi.h:2082:9, wingdi.h:2082:9 */
pub const PC_WIDE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2083:9, wingdi.h:2083:9, wingdi.h:2083:9 */
pub const PC_STYLED: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2084:9, wingdi.h:2084:9, wingdi.h:2084:9 */
pub const PC_WIDESTYLED: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2085:9, wingdi.h:2085:9, wingdi.h:2085:9 */
pub const PC_INTERIORS: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:2086:9, wingdi.h:2086:9, wingdi.h:2086:9 */
pub const PC_POLYPOLYGON: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2087:9, wingdi.h:2087:9, wingdi.h:2087:9 */
pub const PC_PATHS: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:2088:9, wingdi.h:2088:9, wingdi.h:2088:9 */
pub const CP_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2091:9, wingdi.h:2091:9, wingdi.h:2091:9 */
pub const CP_RECTANGLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2092:9, wingdi.h:2092:9, wingdi.h:2092:9 */
pub const CP_REGION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2093:9, wingdi.h:2093:9, wingdi.h:2093:9 */
pub const TC_OP_CHARACTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2096:9, wingdi.h:2096:9, wingdi.h:2096:9 */
pub const TC_OP_STROKE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2097:9, wingdi.h:2097:9, wingdi.h:2097:9 */
pub const TC_CP_STROKE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2098:9, wingdi.h:2098:9, wingdi.h:2098:9 */
pub const TC_CR_90: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2099:9, wingdi.h:2099:9, wingdi.h:2099:9 */
pub const TC_CR_ANY: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2100:9, wingdi.h:2100:9, wingdi.h:2100:9 */
pub const TC_SF_X_YINDEP: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2101:9, wingdi.h:2101:9, wingdi.h:2101:9 */
pub const TC_SA_DOUBLE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2102:9, wingdi.h:2102:9, wingdi.h:2102:9 */
pub const TC_SA_INTEGER: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:2103:9, wingdi.h:2103:9, wingdi.h:2103:9 */
pub const TC_SA_CONTIN: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2104:9, wingdi.h:2104:9, wingdi.h:2104:9 */
pub const TC_EA_DOUBLE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:2105:9, wingdi.h:2105:9, wingdi.h:2105:9 */
pub const TC_IA_ABLE: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:2106:9, wingdi.h:2106:9, wingdi.h:2106:9 */
pub const TC_UA_ABLE: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:2107:9, wingdi.h:2107:9, wingdi.h:2107:9 */
pub const TC_SO_ABLE: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:2108:9, wingdi.h:2108:9, wingdi.h:2108:9 */
pub const TC_RA_ABLE: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:2109:9, wingdi.h:2109:9, wingdi.h:2109:9 */
pub const TC_VA_ABLE: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:2110:9, wingdi.h:2110:9, wingdi.h:2110:9 */
pub const TC_RESERVED: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:2111:9, wingdi.h:2111:9, wingdi.h:2111:9 */
pub const TC_SCROLLBLT: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wingdi.h:2112:9, wingdi.h:2112:9, wingdi.h:2112:9 */
pub const RC_BITBLT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2118:9, wingdi.h:2118:9, wingdi.h:2118:9 */
pub const RC_BANDING: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2119:9, wingdi.h:2119:9, wingdi.h:2119:9 */
pub const RC_SCALING: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2120:9, wingdi.h:2120:9, wingdi.h:2120:9 */
pub const RC_BITMAP64: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2121:9, wingdi.h:2121:9, wingdi.h:2121:9 */
pub const RC_GDI20_OUTPUT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2122:9, wingdi.h:2122:9, wingdi.h:2122:9 */
pub const RC_GDI20_STATE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2123:9, wingdi.h:2123:9, wingdi.h:2123:9 */
pub const RC_SAVEBITMAP: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2124:9, wingdi.h:2124:9, wingdi.h:2124:9 */
pub const RC_DI_BITMAP: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:2125:9, wingdi.h:2125:9, wingdi.h:2125:9 */
pub const RC_PALETTE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2126:9, wingdi.h:2126:9, wingdi.h:2126:9 */
pub const RC_DIBTODEV: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:2127:9, wingdi.h:2127:9, wingdi.h:2127:9 */
pub const RC_BIGFONT: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:2128:9, wingdi.h:2128:9, wingdi.h:2128:9 */
pub const RC_STRETCHBLT: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:2129:9, wingdi.h:2129:9, wingdi.h:2129:9 */
pub const RC_FLOODFILL: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:2130:9, wingdi.h:2130:9, wingdi.h:2130:9 */
pub const RC_STRETCHDIB: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:2131:9, wingdi.h:2131:9, wingdi.h:2131:9 */
pub const RC_OP_DX_OUTPUT: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:2132:9, wingdi.h:2132:9, wingdi.h:2132:9 */
pub const RC_DEVBITS: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:2133:9, wingdi.h:2133:9, wingdi.h:2133:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2138:9, wingdi.h:2138:9, wingdi.h:2138:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_CONST_ALPHA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2139:9, wingdi.h:2139:9, wingdi.h:2139:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_PIXEL_ALPHA: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2140:9, wingdi.h:2140:9, wingdi.h:2140:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_PREMULT_ALPHA: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2141:9, wingdi.h:2141:9, wingdi.h:2141:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_GRAD_RECT: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2143:9, wingdi.h:2143:9, wingdi.h:2143:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const SB_GRAD_TRI: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2144:9, wingdi.h:2144:9, wingdi.h:2144:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CM_NONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2147:9, wingdi.h:2147:9, wingdi.h:2147:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CM_DEVICE_ICM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2148:9, wingdi.h:2148:9, wingdi.h:2148:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CM_GAMMA_RAMP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2149:9, wingdi.h:2149:9, wingdi.h:2149:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const CM_CMYK_COLOR: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2150:9, wingdi.h:2150:9, wingdi.h:2150:9 */
pub const DIB_RGB_COLORS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2157:9, wingdi.h:2157:9, wingdi.h:2157:9 */
pub const DIB_PAL_COLORS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2158:9, wingdi.h:2158:9, wingdi.h:2158:9 */
pub const SYSPAL_ERROR: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2162:9, wingdi.h:2162:9, wingdi.h:2162:9 */
pub const SYSPAL_STATIC: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2163:9, wingdi.h:2163:9, wingdi.h:2163:9 */
pub const SYSPAL_NOSTATIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2164:9, wingdi.h:2164:9, wingdi.h:2164:9 */
pub const SYSPAL_NOSTATIC256: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2165:9, wingdi.h:2165:9, wingdi.h:2165:9 */
pub const CBM_INIT: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:2168:9, wingdi.h:2168:9, wingdi.h:2168:9 */
pub const FLOODFILLBORDER: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2172:10, wingdi.h:2172:10, wingdi.h:2172:10 */
pub const FLOODFILLSURFACE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2173:10, wingdi.h:2173:10, wingdi.h:2173:10 */
pub const CCHDEVICENAME: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2176:9, wingdi.h:2176:9, wingdi.h:2176:9 */
pub const CCHFORMNAME: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2179:9, wingdi.h:2179:9, wingdi.h:2179:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_SPECVERSION: i32 = 0x401i32; /* Integer(1025, Yes, Unknown) */ /* wingdi.h:2419:9, wingdi.h:2419:9, wingdi.h:2419:9 */
pub const DM_ORIENTATION: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:2427:9, wingdi.h:2427:9, wingdi.h:2427:9 */
pub const DM_PAPERSIZE: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:2428:9, wingdi.h:2428:9, wingdi.h:2428:9 */
pub const DM_PAPERLENGTH: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:2429:9, wingdi.h:2429:9, wingdi.h:2429:9 */
pub const DM_PAPERWIDTH: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* wingdi.h:2430:9, wingdi.h:2430:9, wingdi.h:2430:9 */
pub const DM_SCALE: i64 = 0x10i64; /* Integer(16, Yes, Long) */ /* wingdi.h:2431:9, wingdi.h:2431:9, wingdi.h:2431:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DM_POSITION: i64 = 0x20i64; /* Integer(32, Yes, Long) */ /* wingdi.h:2433:9, wingdi.h:2433:9, wingdi.h:2433:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DM_NUP: i64 = 0x40i64; /* Integer(64, Yes, Long) */ /* wingdi.h:2434:9, wingdi.h:2434:9, wingdi.h:2434:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DM_DISPLAYORIENTATION: i64 = 0x80i64; /* Integer(128, Yes, Long) */ /* wingdi.h:2437:9, wingdi.h:2437:9, wingdi.h:2437:9 */
pub const DM_COPIES: i64 = 0x100i64; /* Integer(256, Yes, Long) */ /* wingdi.h:2439:9, wingdi.h:2439:9, wingdi.h:2439:9 */
pub const DM_DEFAULTSOURCE: i64 = 0x200i64; /* Integer(512, Yes, Long) */ /* wingdi.h:2440:9, wingdi.h:2440:9, wingdi.h:2440:9 */
pub const DM_PRINTQUALITY: i64 = 0x400i64; /* Integer(1024, Yes, Long) */ /* wingdi.h:2441:9, wingdi.h:2441:9, wingdi.h:2441:9 */
pub const DM_COLOR: i64 = 0x800i64; /* Integer(2048, Yes, Long) */ /* wingdi.h:2442:9, wingdi.h:2442:9, wingdi.h:2442:9 */
pub const DM_DUPLEX: i64 = 0x1000i64; /* Integer(4096, Yes, Long) */ /* wingdi.h:2443:9, wingdi.h:2443:9, wingdi.h:2443:9 */
pub const DM_YRESOLUTION: i64 = 0x2000i64; /* Integer(8192, Yes, Long) */ /* wingdi.h:2444:9, wingdi.h:2444:9, wingdi.h:2444:9 */
pub const DM_TTOPTION: i64 = 0x4000i64; /* Integer(16384, Yes, Long) */ /* wingdi.h:2445:9, wingdi.h:2445:9, wingdi.h:2445:9 */
pub const DM_COLLATE: i64 = 0x8000i64; /* Integer(32768, Yes, Long) */ /* wingdi.h:2446:9, wingdi.h:2446:9, wingdi.h:2446:9 */
pub const DM_FORMNAME: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* wingdi.h:2447:9, wingdi.h:2447:9, wingdi.h:2447:9 */
pub const DM_LOGPIXELS: i64 = 0x20000i64; /* Integer(131072, Yes, Long) */ /* wingdi.h:2448:9, wingdi.h:2448:9, wingdi.h:2448:9 */
pub const DM_BITSPERPEL: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* wingdi.h:2449:9, wingdi.h:2449:9, wingdi.h:2449:9 */
pub const DM_PELSWIDTH: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* wingdi.h:2450:9, wingdi.h:2450:9, wingdi.h:2450:9 */
pub const DM_PELSHEIGHT: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* wingdi.h:2451:9, wingdi.h:2451:9, wingdi.h:2451:9 */
pub const DM_DISPLAYFLAGS: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* wingdi.h:2452:9, wingdi.h:2452:9, wingdi.h:2452:9 */
pub const DM_DISPLAYFREQUENCY: i64 = 0x400000i64; /* Integer(4194304, Yes, Long) */ /* wingdi.h:2453:9, wingdi.h:2453:9, wingdi.h:2453:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_ICMMETHOD: i64 = 0x800000i64; /* Integer(8388608, Yes, Long) */ /* wingdi.h:2455:9, wingdi.h:2455:9, wingdi.h:2455:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_ICMINTENT: i64 = 0x1000000i64; /* Integer(16777216, Yes, Long) */ /* wingdi.h:2456:9, wingdi.h:2456:9, wingdi.h:2456:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_MEDIATYPE: i64 = 0x2000000i64; /* Integer(33554432, Yes, Long) */ /* wingdi.h:2457:9, wingdi.h:2457:9, wingdi.h:2457:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_DITHERTYPE: i64 = 0x4000000i64; /* Integer(67108864, Yes, Long) */ /* wingdi.h:2458:9, wingdi.h:2458:9, wingdi.h:2458:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_PANNINGWIDTH: i64 = 0x8000000i64; /* Integer(134217728, Yes, Long) */ /* wingdi.h:2459:9, wingdi.h:2459:9, wingdi.h:2459:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DM_PANNINGHEIGHT: i64 = 0x10000000i64; /* Integer(268435456, Yes, Long) */ /* wingdi.h:2460:9, wingdi.h:2460:9, wingdi.h:2460:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DM_DISPLAYFIXEDOUTPUT: i64 = 0x20000000i64; /* Integer(536870912, Yes, Long) */ /* wingdi.h:2463:9, wingdi.h:2463:9, wingdi.h:2463:9 */
pub const DMORIENT_PORTRAIT: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2468:9, wingdi.h:2468:9, wingdi.h:2468:9 */
pub const DMORIENT_LANDSCAPE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2469:9, wingdi.h:2469:9, wingdi.h:2469:9 */
pub const DMPAPER_LETTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2473:9, wingdi.h:2473:9, wingdi.h:2473:9 */
pub const DMPAPER_LETTERSMALL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2474:9, wingdi.h:2474:9, wingdi.h:2474:9 */
pub const DMPAPER_TABLOID: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2475:9, wingdi.h:2475:9, wingdi.h:2475:9 */
pub const DMPAPER_LEDGER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2476:9, wingdi.h:2476:9, wingdi.h:2476:9 */
pub const DMPAPER_LEGAL: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:2477:9, wingdi.h:2477:9, wingdi.h:2477:9 */
pub const DMPAPER_STATEMENT: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:2478:9, wingdi.h:2478:9, wingdi.h:2478:9 */
pub const DMPAPER_EXECUTIVE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:2479:9, wingdi.h:2479:9, wingdi.h:2479:9 */
pub const DMPAPER_A3: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2480:9, wingdi.h:2480:9, wingdi.h:2480:9 */
pub const DMPAPER_A4: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:2481:9, wingdi.h:2481:9, wingdi.h:2481:9 */
pub const DMPAPER_A4SMALL: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:2482:9, wingdi.h:2482:9, wingdi.h:2482:9 */
pub const DMPAPER_A5: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:2483:9, wingdi.h:2483:9, wingdi.h:2483:9 */
pub const DMPAPER_B4: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:2484:9, wingdi.h:2484:9, wingdi.h:2484:9 */
pub const DMPAPER_B5: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:2485:9, wingdi.h:2485:9, wingdi.h:2485:9 */
pub const DMPAPER_FOLIO: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:2486:9, wingdi.h:2486:9, wingdi.h:2486:9 */
pub const DMPAPER_QUARTO: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:2487:9, wingdi.h:2487:9, wingdi.h:2487:9 */
pub const DMPAPER_10X14: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2488:9, wingdi.h:2488:9, wingdi.h:2488:9 */
pub const DMPAPER_11X17: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:2489:9, wingdi.h:2489:9, wingdi.h:2489:9 */
pub const DMPAPER_NOTE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:2490:9, wingdi.h:2490:9, wingdi.h:2490:9 */
pub const DMPAPER_ENV_9: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:2491:9, wingdi.h:2491:9, wingdi.h:2491:9 */
pub const DMPAPER_ENV_10: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:2492:9, wingdi.h:2492:9, wingdi.h:2492:9 */
pub const DMPAPER_ENV_11: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* wingdi.h:2493:9, wingdi.h:2493:9, wingdi.h:2493:9 */
pub const DMPAPER_ENV_12: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* wingdi.h:2494:9, wingdi.h:2494:9, wingdi.h:2494:9 */
pub const DMPAPER_ENV_14: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* wingdi.h:2495:9, wingdi.h:2495:9, wingdi.h:2495:9 */
pub const DMPAPER_CSHEET: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:2496:9, wingdi.h:2496:9, wingdi.h:2496:9 */
pub const DMPAPER_DSHEET: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* wingdi.h:2497:9, wingdi.h:2497:9, wingdi.h:2497:9 */
pub const DMPAPER_ESHEET: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* wingdi.h:2498:9, wingdi.h:2498:9, wingdi.h:2498:9 */
pub const DMPAPER_ENV_DL: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* wingdi.h:2499:9, wingdi.h:2499:9, wingdi.h:2499:9 */
pub const DMPAPER_ENV_C5: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* wingdi.h:2500:9, wingdi.h:2500:9, wingdi.h:2500:9 */
pub const DMPAPER_ENV_C3: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* wingdi.h:2501:9, wingdi.h:2501:9, wingdi.h:2501:9 */
pub const DMPAPER_ENV_C4: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:2502:9, wingdi.h:2502:9, wingdi.h:2502:9 */
pub const DMPAPER_ENV_C6: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* wingdi.h:2503:9, wingdi.h:2503:9, wingdi.h:2503:9 */
pub const DMPAPER_ENV_C65: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2504:9, wingdi.h:2504:9, wingdi.h:2504:9 */
pub const DMPAPER_ENV_B4: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* wingdi.h:2505:9, wingdi.h:2505:9, wingdi.h:2505:9 */
pub const DMPAPER_ENV_B5: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* wingdi.h:2506:9, wingdi.h:2506:9, wingdi.h:2506:9 */
pub const DMPAPER_ENV_B6: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* wingdi.h:2507:9, wingdi.h:2507:9, wingdi.h:2507:9 */
pub const DMPAPER_ENV_ITALY: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* wingdi.h:2508:9, wingdi.h:2508:9, wingdi.h:2508:9 */
pub const DMPAPER_ENV_MONARCH: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* wingdi.h:2509:9, wingdi.h:2509:9, wingdi.h:2509:9 */
pub const DMPAPER_ENV_PERSONAL: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* wingdi.h:2510:9, wingdi.h:2510:9, wingdi.h:2510:9 */
pub const DMPAPER_FANFOLD_US: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* wingdi.h:2511:9, wingdi.h:2511:9, wingdi.h:2511:9 */
pub const DMPAPER_FANFOLD_STD_GERMAN: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* wingdi.h:2512:9, wingdi.h:2512:9, wingdi.h:2512:9 */
pub const DMPAPER_FANFOLD_LGL_GERMAN: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* wingdi.h:2513:9, wingdi.h:2513:9, wingdi.h:2513:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_ISO_B4: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* wingdi.h:2515:9, wingdi.h:2515:9, wingdi.h:2515:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_JAPANESE_POSTCARD: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* wingdi.h:2516:9, wingdi.h:2516:9, wingdi.h:2516:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_9X11: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* wingdi.h:2517:9, wingdi.h:2517:9, wingdi.h:2517:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_10X11: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* wingdi.h:2518:9, wingdi.h:2518:9, wingdi.h:2518:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_15X11: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* wingdi.h:2519:9, wingdi.h:2519:9, wingdi.h:2519:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_ENV_INVITE: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* wingdi.h:2520:9, wingdi.h:2520:9, wingdi.h:2520:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_RESERVED_48: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* wingdi.h:2521:9, wingdi.h:2521:9, wingdi.h:2521:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_RESERVED_49: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* wingdi.h:2522:9, wingdi.h:2522:9, wingdi.h:2522:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_LETTER_EXTRA: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* wingdi.h:2523:9, wingdi.h:2523:9, wingdi.h:2523:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_LEGAL_EXTRA: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* wingdi.h:2524:9, wingdi.h:2524:9, wingdi.h:2524:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_TABLOID_EXTRA: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* wingdi.h:2525:9, wingdi.h:2525:9, wingdi.h:2525:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A4_EXTRA: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* wingdi.h:2526:9, wingdi.h:2526:9, wingdi.h:2526:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_LETTER_TRANSVERSE: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* wingdi.h:2527:9, wingdi.h:2527:9, wingdi.h:2527:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A4_TRANSVERSE: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* wingdi.h:2528:9, wingdi.h:2528:9, wingdi.h:2528:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_LETTER_EXTRA_TRANSVERSE: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* wingdi.h:2529:9, wingdi.h:2529:9, wingdi.h:2529:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A_PLUS: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* wingdi.h:2530:9, wingdi.h:2530:9, wingdi.h:2530:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_B_PLUS: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* wingdi.h:2531:9, wingdi.h:2531:9, wingdi.h:2531:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_LETTER_PLUS: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* wingdi.h:2532:9, wingdi.h:2532:9, wingdi.h:2532:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A4_PLUS: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* wingdi.h:2533:9, wingdi.h:2533:9, wingdi.h:2533:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A5_TRANSVERSE: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* wingdi.h:2534:9, wingdi.h:2534:9, wingdi.h:2534:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_B5_TRANSVERSE: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* wingdi.h:2535:9, wingdi.h:2535:9, wingdi.h:2535:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A3_EXTRA: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* wingdi.h:2536:9, wingdi.h:2536:9, wingdi.h:2536:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A5_EXTRA: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2537:9, wingdi.h:2537:9, wingdi.h:2537:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_B5_EXTRA: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* wingdi.h:2538:9, wingdi.h:2538:9, wingdi.h:2538:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A2: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* wingdi.h:2539:9, wingdi.h:2539:9, wingdi.h:2539:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A3_TRANSVERSE: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* wingdi.h:2540:9, wingdi.h:2540:9, wingdi.h:2540:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMPAPER_A3_EXTRA_TRANSVERSE: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* wingdi.h:2541:9, wingdi.h:2541:9, wingdi.h:2541:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_DBL_JAPANESE_POSTCARD: i32 = 0x45i32; /* Integer(69, Yes, Unknown) */ /* wingdi.h:2545:9, wingdi.h:2545:9, wingdi.h:2545:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_A6: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* wingdi.h:2546:9, wingdi.h:2546:9, wingdi.h:2546:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_KAKU2: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* wingdi.h:2547:9, wingdi.h:2547:9, wingdi.h:2547:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_KAKU3: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* wingdi.h:2548:9, wingdi.h:2548:9, wingdi.h:2548:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_CHOU3: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* wingdi.h:2549:9, wingdi.h:2549:9, wingdi.h:2549:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_CHOU4: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* wingdi.h:2550:9, wingdi.h:2550:9, wingdi.h:2550:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_LETTER_ROTATED: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* wingdi.h:2551:9, wingdi.h:2551:9, wingdi.h:2551:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_A3_ROTATED: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* wingdi.h:2552:9, wingdi.h:2552:9, wingdi.h:2552:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_A4_ROTATED: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* wingdi.h:2553:9, wingdi.h:2553:9, wingdi.h:2553:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_A5_ROTATED: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* wingdi.h:2554:9, wingdi.h:2554:9, wingdi.h:2554:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_B4_JIS_ROTATED: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* wingdi.h:2555:9, wingdi.h:2555:9, wingdi.h:2555:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_B5_JIS_ROTATED: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* wingdi.h:2556:9, wingdi.h:2556:9, wingdi.h:2556:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JAPANESE_POSTCARD_ROTATED: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* wingdi.h:2557:9, wingdi.h:2557:9, wingdi.h:2557:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_DBL_JAPANESE_POSTCARD_ROTATED: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* wingdi.h:2558:9, wingdi.h:2558:9, wingdi.h:2558:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_A6_ROTATED: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* wingdi.h:2559:9, wingdi.h:2559:9, wingdi.h:2559:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_KAKU2_ROTATED: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* wingdi.h:2560:9, wingdi.h:2560:9, wingdi.h:2560:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_KAKU3_ROTATED: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* wingdi.h:2561:9, wingdi.h:2561:9, wingdi.h:2561:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_CHOU3_ROTATED: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* wingdi.h:2562:9, wingdi.h:2562:9, wingdi.h:2562:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_CHOU4_ROTATED: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* wingdi.h:2563:9, wingdi.h:2563:9, wingdi.h:2563:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_B6_JIS: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* wingdi.h:2564:9, wingdi.h:2564:9, wingdi.h:2564:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_B6_JIS_ROTATED: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* wingdi.h:2565:9, wingdi.h:2565:9, wingdi.h:2565:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_12X11: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* wingdi.h:2566:9, wingdi.h:2566:9, wingdi.h:2566:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_YOU4: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* wingdi.h:2567:9, wingdi.h:2567:9, wingdi.h:2567:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_JENV_YOU4_ROTATED: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* wingdi.h:2568:9, wingdi.h:2568:9, wingdi.h:2568:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P16K: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* wingdi.h:2569:9, wingdi.h:2569:9, wingdi.h:2569:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P32K: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* wingdi.h:2570:9, wingdi.h:2570:9, wingdi.h:2570:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P32KBIG: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* wingdi.h:2571:9, wingdi.h:2571:9, wingdi.h:2571:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_1: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* wingdi.h:2572:9, wingdi.h:2572:9, wingdi.h:2572:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_2: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* wingdi.h:2573:9, wingdi.h:2573:9, wingdi.h:2573:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_3: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* wingdi.h:2574:9, wingdi.h:2574:9, wingdi.h:2574:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_4: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* wingdi.h:2575:9, wingdi.h:2575:9, wingdi.h:2575:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_5: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* wingdi.h:2576:9, wingdi.h:2576:9, wingdi.h:2576:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_6: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* wingdi.h:2577:9, wingdi.h:2577:9, wingdi.h:2577:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_7: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* wingdi.h:2578:9, wingdi.h:2578:9, wingdi.h:2578:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_8: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* wingdi.h:2579:9, wingdi.h:2579:9, wingdi.h:2579:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_9: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* wingdi.h:2580:9, wingdi.h:2580:9, wingdi.h:2580:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_10: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* wingdi.h:2581:9, wingdi.h:2581:9, wingdi.h:2581:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P16K_ROTATED: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* wingdi.h:2582:9, wingdi.h:2582:9, wingdi.h:2582:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P32K_ROTATED: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* wingdi.h:2583:9, wingdi.h:2583:9, wingdi.h:2583:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_P32KBIG_ROTATED: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* wingdi.h:2584:9, wingdi.h:2584:9, wingdi.h:2584:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_1_ROTATED: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* wingdi.h:2585:9, wingdi.h:2585:9, wingdi.h:2585:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_2_ROTATED: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* wingdi.h:2586:9, wingdi.h:2586:9, wingdi.h:2586:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_3_ROTATED: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* wingdi.h:2587:9, wingdi.h:2587:9, wingdi.h:2587:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_4_ROTATED: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* wingdi.h:2588:9, wingdi.h:2588:9, wingdi.h:2588:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_5_ROTATED: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* wingdi.h:2589:9, wingdi.h:2589:9, wingdi.h:2589:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_6_ROTATED: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* wingdi.h:2590:9, wingdi.h:2590:9, wingdi.h:2590:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_7_ROTATED: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* wingdi.h:2591:9, wingdi.h:2591:9, wingdi.h:2591:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_8_ROTATED: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* wingdi.h:2592:9, wingdi.h:2592:9, wingdi.h:2592:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_9_ROTATED: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* wingdi.h:2593:9, wingdi.h:2593:9, wingdi.h:2593:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DMPAPER_PENV_10_ROTATED: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* wingdi.h:2594:9, wingdi.h:2594:9, wingdi.h:2594:9 */
#[cfg(any(feature="winapi_ver_05000000"))] #[doc(inline)] pub use ::wingdi::DMPAPER_PENV_10_ROTATED as DMPAPER_LAST; /* wingdi.h:2598:9, wingdi.h:2598:9, wingdi.h:2598:9 */
pub const DMPAPER_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2605:9, wingdi.h:2605:9, wingdi.h:2605:9 */
pub const DMBIN_UPPER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2609:9, wingdi.h:2609:9, wingdi.h:2609:9 */
pub const DMBIN_ONLYONE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2610:9, wingdi.h:2610:9, wingdi.h:2610:9 */
pub const DMBIN_LOWER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2611:9, wingdi.h:2611:9, wingdi.h:2611:9 */
pub const DMBIN_MIDDLE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2612:9, wingdi.h:2612:9, wingdi.h:2612:9 */
pub const DMBIN_MANUAL: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2613:9, wingdi.h:2613:9, wingdi.h:2613:9 */
pub const DMBIN_ENVELOPE: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:2614:9, wingdi.h:2614:9, wingdi.h:2614:9 */
pub const DMBIN_ENVMANUAL: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:2615:9, wingdi.h:2615:9, wingdi.h:2615:9 */
pub const DMBIN_AUTO: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:2616:9, wingdi.h:2616:9, wingdi.h:2616:9 */
pub const DMBIN_TRACTOR: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2617:9, wingdi.h:2617:9, wingdi.h:2617:9 */
pub const DMBIN_SMALLFMT: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:2618:9, wingdi.h:2618:9, wingdi.h:2618:9 */
pub const DMBIN_LARGEFMT: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:2619:9, wingdi.h:2619:9, wingdi.h:2619:9 */
pub const DMBIN_LARGECAPACITY: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:2620:9, wingdi.h:2620:9, wingdi.h:2620:9 */
pub const DMBIN_CASSETTE: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:2621:9, wingdi.h:2621:9, wingdi.h:2621:9 */
pub const DMBIN_FORMSOURCE: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:2622:9, wingdi.h:2622:9, wingdi.h:2622:9 */
#[doc(inline)] pub use ::wingdi::DMBIN_FORMSOURCE as DMBIN_LAST; /* wingdi.h:2623:9, wingdi.h:2623:9, wingdi.h:2623:9 */
pub const DMBIN_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2625:9, wingdi.h:2625:9, wingdi.h:2625:9 */
pub const DMCOLOR_MONOCHROME: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2634:9, wingdi.h:2634:9, wingdi.h:2634:9 */
pub const DMCOLOR_COLOR: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2635:9, wingdi.h:2635:9, wingdi.h:2635:9 */
pub const DMDUP_SIMPLEX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2638:9, wingdi.h:2638:9, wingdi.h:2638:9 */
pub const DMDUP_VERTICAL: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2639:9, wingdi.h:2639:9, wingdi.h:2639:9 */
pub const DMDUP_HORIZONTAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2640:9, wingdi.h:2640:9, wingdi.h:2640:9 */
pub const DMTT_BITMAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2643:9, wingdi.h:2643:9, wingdi.h:2643:9 */
pub const DMTT_DOWNLOAD: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2644:9, wingdi.h:2644:9, wingdi.h:2644:9 */
pub const DMTT_SUBDEV: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2645:9, wingdi.h:2645:9, wingdi.h:2645:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMTT_DOWNLOAD_OUTLINE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2647:9, wingdi.h:2647:9, wingdi.h:2647:9 */
pub const DMCOLLATE_FALSE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2651:9, wingdi.h:2651:9, wingdi.h:2651:9 */
pub const DMCOLLATE_TRUE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2652:9, wingdi.h:2652:9, wingdi.h:2652:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDO_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2656:9, wingdi.h:2656:9, wingdi.h:2656:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDO_90: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2657:9, wingdi.h:2657:9, wingdi.h:2657:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDO_180: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2658:9, wingdi.h:2658:9, wingdi.h:2658:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDO_270: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2659:9, wingdi.h:2659:9, wingdi.h:2659:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDFO_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:2662:9, wingdi.h:2662:9, wingdi.h:2662:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDFO_STRETCH: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2663:9, wingdi.h:2663:9, wingdi.h:2663:9 */
#[cfg(any(feature="winapi_ver_05010000"))] pub const DMDFO_CENTER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2664:9, wingdi.h:2664:9, wingdi.h:2664:9 */
pub const DM_INTERLACED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2670:9, wingdi.h:2670:9, wingdi.h:2670:9 */
pub const DMDISPLAYFLAGS_TEXTMODE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2671:9, wingdi.h:2671:9, wingdi.h:2671:9 */
pub const DMNUP_SYSTEM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2674:9, wingdi.h:2674:9, wingdi.h:2674:9 */
pub const DMNUP_ONEUP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2675:9, wingdi.h:2675:9, wingdi.h:2675:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICMMETHOD_NONE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2679:9, wingdi.h:2679:9, wingdi.h:2679:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICMMETHOD_SYSTEM: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2680:9, wingdi.h:2680:9, wingdi.h:2680:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICMMETHOD_DRIVER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2681:9, wingdi.h:2681:9, wingdi.h:2681:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICMMETHOD_DEVICE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2682:9, wingdi.h:2682:9, wingdi.h:2682:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICMMETHOD_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2684:9, wingdi.h:2684:9, wingdi.h:2684:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICM_SATURATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2687:9, wingdi.h:2687:9, wingdi.h:2687:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICM_CONTRAST: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2688:9, wingdi.h:2688:9, wingdi.h:2688:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICM_COLORIMETRIC: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2689:9, wingdi.h:2689:9, wingdi.h:2689:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICM_ABS_COLORIMETRIC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2690:9, wingdi.h:2690:9, wingdi.h:2690:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMICM_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2692:9, wingdi.h:2692:9, wingdi.h:2692:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMMEDIA_STANDARD: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2696:9, wingdi.h:2696:9, wingdi.h:2696:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMMEDIA_TRANSPARENCY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2697:9, wingdi.h:2697:9, wingdi.h:2697:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMMEDIA_GLOSSY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2698:9, wingdi.h:2698:9, wingdi.h:2698:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMMEDIA_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2700:9, wingdi.h:2700:9, wingdi.h:2700:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_NONE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2703:9, wingdi.h:2703:9, wingdi.h:2703:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_COARSE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2704:9, wingdi.h:2704:9, wingdi.h:2704:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_FINE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:2705:9, wingdi.h:2705:9, wingdi.h:2705:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_LINEART: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2706:9, wingdi.h:2706:9, wingdi.h:2706:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_ERRORDIFFUSION: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:2707:9, wingdi.h:2707:9, wingdi.h:2707:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_RESERVED6: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:2708:9, wingdi.h:2708:9, wingdi.h:2708:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_RESERVED7: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:2709:9, wingdi.h:2709:9, wingdi.h:2709:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_RESERVED8: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2710:9, wingdi.h:2710:9, wingdi.h:2710:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_RESERVED9: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:2711:9, wingdi.h:2711:9, wingdi.h:2711:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_GRAYSCALE: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:2712:9, wingdi.h:2712:9, wingdi.h:2712:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DMDITHER_USER: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:2714:9, wingdi.h:2714:9, wingdi.h:2714:9 */
pub const DISPLAY_DEVICE_ATTACHED_TO_DESKTOP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2749:9, wingdi.h:2749:9, wingdi.h:2749:9 */
pub const DISPLAY_DEVICE_MULTI_DRIVER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2750:9, wingdi.h:2750:9, wingdi.h:2750:9 */
pub const DISPLAY_DEVICE_PRIMARY_DEVICE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2751:9, wingdi.h:2751:9, wingdi.h:2751:9 */
pub const DISPLAY_DEVICE_MIRRORING_DRIVER: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2752:9, wingdi.h:2752:9, wingdi.h:2752:9 */
pub const DISPLAY_DEVICE_VGA_COMPATIBLE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2753:9, wingdi.h:2753:9, wingdi.h:2753:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DISPLAY_DEVICE_REMOVABLE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:2755:9, wingdi.h:2755:9, wingdi.h:2755:9 */
#[cfg(any(feature="winapi_ver_06020000"))] pub const DISPLAY_DEVICE_ACC_DRIVER: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:2758:9, wingdi.h:2758:9, wingdi.h:2758:9 */
pub const DISPLAY_DEVICE_MODESPRUNED: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* wingdi.h:2760:9, wingdi.h:2760:9, wingdi.h:2760:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DISPLAY_DEVICE_REMOTE: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* wingdi.h:2762:9, wingdi.h:2762:9, wingdi.h:2762:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DISPLAY_DEVICE_DISCONNECT: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* wingdi.h:2763:9, wingdi.h:2763:9, wingdi.h:2763:9 */
pub const DISPLAY_DEVICE_TS_COMPATIBLE: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* wingdi.h:2765:9, wingdi.h:2765:9, wingdi.h:2765:9 */
#[cfg(any(feature="winapi_ver_06000000"))] pub const DISPLAY_DEVICE_UNSAFE_MODES_ON: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* wingdi.h:2767:9, wingdi.h:2767:9, wingdi.h:2767:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DISPLAY_DEVICE_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2772:9, wingdi.h:2772:9, wingdi.h:2772:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DISPLAY_DEVICE_ATTACHED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2773:9, wingdi.h:2773:9, wingdi.h:2773:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_MAXPATH: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:2778:9, wingdi.h:2778:9, wingdi.h:2778:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_PATH_MODE_IDX_INVALID: i32 = 0xffffffffi32; /* Integer(4294967295, Yes, Unknown) */ /* wingdi.h:2918:9, wingdi.h:2918:9, wingdi.h:2918:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_SOURCE_IN_USE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2932:9, wingdi.h:2932:9, wingdi.h:2932:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_TARGET_IN_USE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2951:9, wingdi.h:2951:9, wingdi.h:2951:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_TARGET_FORCIBLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:2952:9, wingdi.h:2952:9, wingdi.h:2952:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:2953:9, wingdi.h:2953:9, wingdi.h:2953:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:2954:9, wingdi.h:2954:9, wingdi.h:2954:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:2955:9, wingdi.h:2955:9, wingdi.h:2955:9 */
#[cfg(feature="winapi_app")] #[cfg(any(feature="winapi_ver_06010000"))] pub const DISPLAYCONFIG_PATH_ACTIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:2968:9, wingdi.h:2968:9, wingdi.h:2968:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const QDC_ALL_PATHS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3086:9, wingdi.h:3086:9, wingdi.h:3086:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const QDC_ONLY_ACTIVE_PATHS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3087:9, wingdi.h:3087:9, wingdi.h:3087:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const QDC_DATABASE_CURRENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3088:9, wingdi.h:3088:9, wingdi.h:3088:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_TOPOLOGY_INTERNAL: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3094:9, wingdi.h:3094:9, wingdi.h:3094:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_TOPOLOGY_CLONE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3095:9, wingdi.h:3095:9, wingdi.h:3095:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_TOPOLOGY_EXTEND: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3096:9, wingdi.h:3096:9, wingdi.h:3096:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_TOPOLOGY_EXTERNAL: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3097:9, wingdi.h:3097:9, wingdi.h:3097:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_TOPOLOGY_SUPPLIED: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:3098:9, wingdi.h:3098:9, wingdi.h:3098:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_USE_SUPPLIED_DISPLAY_CONFIG: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:3101:9, wingdi.h:3101:9, wingdi.h:3101:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_VALIDATE: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:3102:9, wingdi.h:3102:9, wingdi.h:3102:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_APPLY: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:3103:9, wingdi.h:3103:9, wingdi.h:3103:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_NO_OPTIMIZATION: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:3104:9, wingdi.h:3104:9, wingdi.h:3104:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_SAVE_TO_DATABASE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:3105:9, wingdi.h:3105:9, wingdi.h:3105:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_ALLOW_CHANGES: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:3106:9, wingdi.h:3106:9, wingdi.h:3106:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_PATH_PERSIST_IF_REQUIRED: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:3107:9, wingdi.h:3107:9, wingdi.h:3107:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_FORCE_MODE_ENUMERATION: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:3108:9, wingdi.h:3108:9, wingdi.h:3108:9 */
#[cfg(any(feature="winapi_ver_06010000"))] pub const SDC_ALLOW_PATH_ORDER_CHANGES: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:3109:9, wingdi.h:3109:9, wingdi.h:3109:9 */
pub const RDH_RECTANGLES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3116:9, wingdi.h:3116:9, wingdi.h:3116:9 */
pub const SYSRGN: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3139:9, wingdi.h:3139:9, wingdi.h:3139:9 */
pub const GGO_METRICS: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:3332:9, wingdi.h:3332:9, wingdi.h:3332:9 */
pub const GGO_BITMAP: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3333:9, wingdi.h:3333:9, wingdi.h:3333:9 */
pub const GGO_NATIVE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3334:9, wingdi.h:3334:9, wingdi.h:3334:9 */
pub const GGO_BEZIER: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3335:9, wingdi.h:3335:9, wingdi.h:3335:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GGO_GRAY2_BITMAP: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3338:10, wingdi.h:3338:10, wingdi.h:3338:10 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GGO_GRAY4_BITMAP: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:3339:10, wingdi.h:3339:10, wingdi.h:3339:10 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GGO_GRAY8_BITMAP: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:3340:10, wingdi.h:3340:10, wingdi.h:3340:10 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GGO_GLYPH_INDEX: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:3341:10, wingdi.h:3341:10, wingdi.h:3341:10 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const GGO_UNHINTED: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:3345:10, wingdi.h:3345:10, wingdi.h:3345:10 */
pub const TT_POLYGON_TYPE: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:3348:9, wingdi.h:3348:9, wingdi.h:3348:9 */
pub const TT_PRIM_LINE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3350:9, wingdi.h:3350:9, wingdi.h:3350:9 */
pub const TT_PRIM_QSPLINE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3351:9, wingdi.h:3351:9, wingdi.h:3351:9 */
pub const TT_PRIM_CSPLINE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3352:9, wingdi.h:3352:9, wingdi.h:3352:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_DBCS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3382:9, wingdi.h:3382:9, wingdi.h:3382:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_REORDER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3383:9, wingdi.h:3383:9, wingdi.h:3383:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_USEKERNING: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3384:9, wingdi.h:3384:9, wingdi.h:3384:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_GLYPHSHAPE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:3385:9, wingdi.h:3385:9, wingdi.h:3385:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_LIGATE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:3386:9, wingdi.h:3386:9, wingdi.h:3386:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_DIACRITIC: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:3388:9, wingdi.h:3388:9, wingdi.h:3388:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_KASHIDA: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:3389:9, wingdi.h:3389:9, wingdi.h:3389:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_ERROR: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:3390:9, wingdi.h:3390:9, wingdi.h:3390:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FLI_MASK: i32 = 0x103bi32; /* Integer(4155, Yes, Unknown) */ /* wingdi.h:3391:9, wingdi.h:3391:9, wingdi.h:3391:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_JUSTIFY: i64 = 0x10000i64; /* Integer(65536, Yes, Long) */ /* wingdi.h:3393:9, wingdi.h:3393:9, wingdi.h:3393:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const FLI_GLYPHS: i64 = 0x40000i64; /* Integer(262144, Yes, Long) */ /* wingdi.h:3395:9, wingdi.h:3395:9, wingdi.h:3395:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_CLASSIN: i64 = 0x80000i64; /* Integer(524288, Yes, Long) */ /* wingdi.h:3396:9, wingdi.h:3396:9, wingdi.h:3396:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_MAXEXTENT: i64 = 0x100000i64; /* Integer(1048576, Yes, Long) */ /* wingdi.h:3397:9, wingdi.h:3397:9, wingdi.h:3397:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_JUSTIFYIN: i64 = 0x200000i64; /* Integer(2097152, Yes, Long) */ /* wingdi.h:3398:9, wingdi.h:3398:9, wingdi.h:3398:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_DISPLAYZWG: i64 = 0x400000i64; /* Integer(4194304, Yes, Long) */ /* wingdi.h:3399:9, wingdi.h:3399:9, wingdi.h:3399:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_SYMSWAPOFF: i64 = 0x800000i64; /* Integer(8388608, Yes, Long) */ /* wingdi.h:3400:9, wingdi.h:3400:9, wingdi.h:3400:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_NUMERICOVERRIDE: i64 = 0x1000000i64; /* Integer(16777216, Yes, Long) */ /* wingdi.h:3401:9, wingdi.h:3401:9, wingdi.h:3401:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_NEUTRALOVERRIDE: i64 = 0x2000000i64; /* Integer(33554432, Yes, Long) */ /* wingdi.h:3402:9, wingdi.h:3402:9, wingdi.h:3402:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_NUMERICSLATIN: i64 = 0x4000000i64; /* Integer(67108864, Yes, Long) */ /* wingdi.h:3403:9, wingdi.h:3403:9, wingdi.h:3403:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCP_NUMERICSLOCAL: i64 = 0x8000000i64; /* Integer(134217728, Yes, Long) */ /* wingdi.h:3404:9, wingdi.h:3404:9, wingdi.h:3404:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_LATIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3406:9, wingdi.h:3406:9, wingdi.h:3406:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_HEBREW: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3407:9, wingdi.h:3407:9, wingdi.h:3407:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_ARABIC: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3408:9, wingdi.h:3408:9, wingdi.h:3408:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_NEUTRAL: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3409:9, wingdi.h:3409:9, wingdi.h:3409:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_LOCALNUMBER: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3410:9, wingdi.h:3410:9, wingdi.h:3410:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_LATINNUMBER: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:3411:9, wingdi.h:3411:9, wingdi.h:3411:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_LATINNUMERICTERMINATOR: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:3412:9, wingdi.h:3412:9, wingdi.h:3412:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_LATINNUMERICSEPARATOR: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:3413:9, wingdi.h:3413:9, wingdi.h:3413:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_NUMERICSEPARATOR: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3414:9, wingdi.h:3414:9, wingdi.h:3414:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_PREBOUNDLTR: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:3415:9, wingdi.h:3415:9, wingdi.h:3415:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_PREBOUNDRTL: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:3416:9, wingdi.h:3416:9, wingdi.h:3416:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_POSTBOUNDLTR: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:3417:9, wingdi.h:3417:9, wingdi.h:3417:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPCLASS_POSTBOUNDRTL: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:3418:9, wingdi.h:3418:9, wingdi.h:3418:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPGLYPH_LINKBEFORE: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:3420:9, wingdi.h:3420:9, wingdi.h:3420:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const GCPGLYPH_LINKAFTER: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:3421:9, wingdi.h:3421:9, wingdi.h:3421:9 */
pub const TT_AVAILABLE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3476:9, wingdi.h:3476:9, wingdi.h:3476:9 */
pub const TT_ENABLED: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3477:9, wingdi.h:3477:9, wingdi.h:3477:9 */
pub const PFD_TYPE_RGBA: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:3517:9, wingdi.h:3517:9, wingdi.h:3517:9 */
pub const PFD_TYPE_COLORINDEX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3518:9, wingdi.h:3518:9, wingdi.h:3518:9 */
pub const PFD_MAIN_PLANE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:3521:9, wingdi.h:3521:9, wingdi.h:3521:9 */
pub const PFD_OVERLAY_PLANE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3522:9, wingdi.h:3522:9, wingdi.h:3522:9 */
pub const PFD_DOUBLEBUFFER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3526:9, wingdi.h:3526:9, wingdi.h:3526:9 */
pub const PFD_STEREO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3527:9, wingdi.h:3527:9, wingdi.h:3527:9 */
pub const PFD_DRAW_TO_WINDOW: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3528:9, wingdi.h:3528:9, wingdi.h:3528:9 */
pub const PFD_DRAW_TO_BITMAP: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3529:9, wingdi.h:3529:9, wingdi.h:3529:9 */
pub const PFD_SUPPORT_GDI: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:3530:9, wingdi.h:3530:9, wingdi.h:3530:9 */
pub const PFD_SUPPORT_OPENGL: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:3531:9, wingdi.h:3531:9, wingdi.h:3531:9 */
pub const PFD_GENERIC_FORMAT: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:3532:9, wingdi.h:3532:9, wingdi.h:3532:9 */
pub const PFD_NEED_PALETTE: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:3533:9, wingdi.h:3533:9, wingdi.h:3533:9 */
pub const PFD_NEED_SYSTEM_PALETTE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:3534:9, wingdi.h:3534:9, wingdi.h:3534:9 */
pub const PFD_SWAP_EXCHANGE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:3535:9, wingdi.h:3535:9, wingdi.h:3535:9 */
pub const PFD_SWAP_COPY: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:3536:9, wingdi.h:3536:9, wingdi.h:3536:9 */
pub const PFD_SWAP_LAYER_BUFFERS: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:3537:9, wingdi.h:3537:9, wingdi.h:3537:9 */
pub const PFD_GENERIC_ACCELERATED: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:3538:9, wingdi.h:3538:9, wingdi.h:3538:9 */
pub const PFD_SUPPORT_DIRECTDRAW: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:3539:9, wingdi.h:3539:9, wingdi.h:3539:9 */
pub const PFD_DIRECT3D_ACCELERATED: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:3540:9, wingdi.h:3540:9, wingdi.h:3540:9 */
pub const PFD_SUPPORT_COMPOSITION: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:3541:9, wingdi.h:3541:9, wingdi.h:3541:9 */
pub const PFD_DEPTH_DONTCARE: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* wingdi.h:3544:9, wingdi.h:3544:9, wingdi.h:3544:9 */
pub const PFD_DOUBLEBUFFER_DONTCARE: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* wingdi.h:3545:9, wingdi.h:3545:9, wingdi.h:3545:9 */
pub const PFD_STEREO_DONTCARE: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* wingdi.h:3546:9, wingdi.h:3546:9, wingdi.h:3546:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::OLDFONTENUMPROCW as OLDFONTENUMPROC; /* wingdi.h:3558:9, wingdi.h:3558:9, wingdi.h:3558:9 */
pub const DM_UPDATE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3718:9, wingdi.h:3718:9, wingdi.h:3718:9 */
pub const DM_COPY: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3719:9, wingdi.h:3719:9, wingdi.h:3719:9 */
pub const DM_PROMPT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3720:9, wingdi.h:3720:9, wingdi.h:3720:9 */
pub const DM_MODIFY: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3721:9, wingdi.h:3721:9, wingdi.h:3721:9 */
#[doc(inline)] pub use ::wingdi::DM_MODIFY as DM_IN_BUFFER; /* wingdi.h:3723:9, wingdi.h:3723:9, wingdi.h:3723:9 */
#[doc(inline)] pub use ::wingdi::DM_PROMPT as DM_IN_PROMPT; /* wingdi.h:3724:9, wingdi.h:3724:9, wingdi.h:3724:9 */
#[doc(inline)] pub use ::wingdi::DM_COPY as DM_OUT_BUFFER; /* wingdi.h:3725:9, wingdi.h:3725:9, wingdi.h:3725:9 */
#[doc(inline)] pub use ::wingdi::DM_UPDATE as DM_OUT_DEFAULT; /* wingdi.h:3726:9, wingdi.h:3726:9, wingdi.h:3726:9 */
pub const DC_FIELDS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3729:9, wingdi.h:3729:9, wingdi.h:3729:9 */
pub const DC_PAPERS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3730:9, wingdi.h:3730:9, wingdi.h:3730:9 */
pub const DC_PAPERSIZE: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3731:9, wingdi.h:3731:9, wingdi.h:3731:9 */
pub const DC_MINEXTENT: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3732:9, wingdi.h:3732:9, wingdi.h:3732:9 */
pub const DC_MAXEXTENT: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:3733:9, wingdi.h:3733:9, wingdi.h:3733:9 */
pub const DC_BINS: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:3734:9, wingdi.h:3734:9, wingdi.h:3734:9 */
pub const DC_DUPLEX: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:3735:9, wingdi.h:3735:9, wingdi.h:3735:9 */
pub const DC_SIZE: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:3736:9, wingdi.h:3736:9, wingdi.h:3736:9 */
pub const DC_EXTRA: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:3737:9, wingdi.h:3737:9, wingdi.h:3737:9 */
pub const DC_VERSION: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:3738:9, wingdi.h:3738:9, wingdi.h:3738:9 */
pub const DC_DRIVER: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:3739:9, wingdi.h:3739:9, wingdi.h:3739:9 */
pub const DC_BINNAMES: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:3740:9, wingdi.h:3740:9, wingdi.h:3740:9 */
pub const DC_ENUMRESOLUTIONS: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:3741:9, wingdi.h:3741:9, wingdi.h:3741:9 */
pub const DC_FILEDEPENDENCIES: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:3742:9, wingdi.h:3742:9, wingdi.h:3742:9 */
pub const DC_TRUETYPE: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:3743:9, wingdi.h:3743:9, wingdi.h:3743:9 */
pub const DC_PAPERNAMES: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:3744:9, wingdi.h:3744:9, wingdi.h:3744:9 */
pub const DC_ORIENTATION: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:3745:9, wingdi.h:3745:9, wingdi.h:3745:9 */
pub const DC_COPIES: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:3746:9, wingdi.h:3746:9, wingdi.h:3746:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_BINADJUST: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:3748:9, wingdi.h:3748:9, wingdi.h:3748:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_EMF_COMPLIANT: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:3749:9, wingdi.h:3749:9, wingdi.h:3749:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_DATATYPE_PRODUCED: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* wingdi.h:3750:9, wingdi.h:3750:9, wingdi.h:3750:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_COLLATE: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* wingdi.h:3751:9, wingdi.h:3751:9, wingdi.h:3751:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_MANUFACTURER: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* wingdi.h:3752:9, wingdi.h:3752:9, wingdi.h:3752:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DC_MODEL: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:3753:9, wingdi.h:3753:9, wingdi.h:3753:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PERSONALITY: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* wingdi.h:3757:9, wingdi.h:3757:9, wingdi.h:3757:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PRINTRATE: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* wingdi.h:3758:9, wingdi.h:3758:9, wingdi.h:3758:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PRINTRATEUNIT: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* wingdi.h:3759:9, wingdi.h:3759:9, wingdi.h:3759:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const PRINTRATEUNIT_PPM: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3760:11, wingdi.h:3760:11, wingdi.h:3760:11 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const PRINTRATEUNIT_CPS: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3761:11, wingdi.h:3761:11, wingdi.h:3761:11 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const PRINTRATEUNIT_LPM: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3762:11, wingdi.h:3762:11, wingdi.h:3762:11 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const PRINTRATEUNIT_IPM: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:3763:11, wingdi.h:3763:11, wingdi.h:3763:11 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PRINTERMEM: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* wingdi.h:3764:9, wingdi.h:3764:9, wingdi.h:3764:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_MEDIAREADY: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* wingdi.h:3765:9, wingdi.h:3765:9, wingdi.h:3765:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_STAPLE: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:3766:9, wingdi.h:3766:9, wingdi.h:3766:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_PRINTRATEPPM: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* wingdi.h:3767:9, wingdi.h:3767:9, wingdi.h:3767:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_COLORDEVICE: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:3768:9, wingdi.h:3768:9, wingdi.h:3768:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_NUP: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* wingdi.h:3769:9, wingdi.h:3769:9, wingdi.h:3769:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_MEDIATYPENAMES: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* wingdi.h:3771:9, wingdi.h:3771:9, wingdi.h:3771:9 */
#[cfg(any(feature="winapi_ver_05000000"))] pub const DC_MEDIATYPES: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* wingdi.h:3772:9, wingdi.h:3772:9, wingdi.h:3772:9 */
pub const DCTT_BITMAP: i64 = 0x1i64; /* Integer(1, Yes, Long) */ /* wingdi.h:3777:9, wingdi.h:3777:9, wingdi.h:3777:9 */
pub const DCTT_DOWNLOAD: i64 = 0x2i64; /* Integer(2, Yes, Long) */ /* wingdi.h:3778:9, wingdi.h:3778:9, wingdi.h:3778:9 */
pub const DCTT_SUBDEV: i64 = 0x4i64; /* Integer(4, Yes, Long) */ /* wingdi.h:3779:9, wingdi.h:3779:9, wingdi.h:3779:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCTT_DOWNLOAD_OUTLINE: i64 = 0x8i64; /* Integer(8, Yes, Long) */ /* wingdi.h:3781:9, wingdi.h:3781:9, wingdi.h:3781:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEUPNONE: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:3784:9, wingdi.h:3784:9, wingdi.h:3784:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEUPCENTER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:3785:9, wingdi.h:3785:9, wingdi.h:3785:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEUPLEFT: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:3786:9, wingdi.h:3786:9, wingdi.h:3786:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEUPRIGHT: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:3787:9, wingdi.h:3787:9, wingdi.h:3787:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEDOWNNONE: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:3788:9, wingdi.h:3788:9, wingdi.h:3788:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEDOWNCENTER: i32 = 0x101i32; /* Integer(257, Yes, Unknown) */ /* wingdi.h:3789:9, wingdi.h:3789:9, wingdi.h:3789:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEDOWNLEFT: i32 = 0x102i32; /* Integer(258, Yes, Unknown) */ /* wingdi.h:3790:9, wingdi.h:3790:9, wingdi.h:3790:9 */
#[cfg(any(feature="winapi_ver_04000000"))] pub const DCBA_FACEDOWNRIGHT: i32 = 0x103i32; /* Integer(259, Yes, Unknown) */ /* wingdi.h:3791:9, wingdi.h:3791:9, wingdi.h:3791:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const GS_8BIT_INDICES: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4173:9, wingdi.h:4173:9, wingdi.h:4173:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const GGI_MARK_NONEXISTING_GLYPHS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4177:9, wingdi.h:4177:9, wingdi.h:4177:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MM_MAX_NUMAXES: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:4214:9, wingdi.h:4214:9, wingdi.h:4214:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FR_PRIVATE: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:4245:9, wingdi.h:4245:9, wingdi.h:4245:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const FR_NOT_ENUM: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:4246:9, wingdi.h:4246:9, wingdi.h:4246:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const MM_MAX_AXES_NAMELEN: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:4252:9, wingdi.h:4252:9, wingdi.h:4252:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const AC_SRC_OVER: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:4567:9, wingdi.h:4567:9, wingdi.h:4567:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const AC_SRC_ALPHA: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4573:9, wingdi.h:4573:9, wingdi.h:4573:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GRADIENT_FILL_RECT_H: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:4606:9, wingdi.h:4606:9, wingdi.h:4606:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GRADIENT_FILL_RECT_V: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4607:9, wingdi.h:4607:9, wingdi.h:4607:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GRADIENT_FILL_TRIANGLE: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:4608:9, wingdi.h:4608:9, wingdi.h:4608:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const GRADIENT_FILL_OP_FLAG: i32 = 0xffi32; /* Integer(255, Yes, Unknown) */ /* wingdi.h:4609:9, wingdi.h:4609:9, wingdi.h:4609:9 */
#[cfg(feature="winapi_desktop")] pub const CA_NEGATIVE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4812:9, wingdi.h:4812:9, wingdi.h:4812:9 */
#[cfg(feature="winapi_desktop")] pub const CA_LOG_FILTER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:4813:9, wingdi.h:4813:9, wingdi.h:4813:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_DEVICE_DEFAULT: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:4816:9, wingdi.h:4816:9, wingdi.h:4816:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_A: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4817:9, wingdi.h:4817:9, wingdi.h:4817:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_B: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:4818:9, wingdi.h:4818:9, wingdi.h:4818:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_C: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:4819:9, wingdi.h:4819:9, wingdi.h:4819:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_D50: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:4820:9, wingdi.h:4820:9, wingdi.h:4820:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_D55: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:4821:9, wingdi.h:4821:9, wingdi.h:4821:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_D65: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:4822:9, wingdi.h:4822:9, wingdi.h:4822:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_D75: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:4823:9, wingdi.h:4823:9, wingdi.h:4823:9 */
#[cfg(feature="winapi_desktop")] pub const ILLUMINANT_F2: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:4824:9, wingdi.h:4824:9, wingdi.h:4824:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::ILLUMINANT_F2 as ILLUMINANT_MAX_INDEX; /* wingdi.h:4825:9, wingdi.h:4825:9, wingdi.h:4825:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::ILLUMINANT_A as ILLUMINANT_TUNGSTEN; /* wingdi.h:4827:9, wingdi.h:4827:9, wingdi.h:4827:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::ILLUMINANT_C as ILLUMINANT_DAYLIGHT; /* wingdi.h:4828:9, wingdi.h:4828:9, wingdi.h:4828:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::ILLUMINANT_F2 as ILLUMINANT_FLUORESCENT; /* wingdi.h:4829:9, wingdi.h:4829:9, wingdi.h:4829:9 */
#[cfg(feature="winapi_desktop")] #[doc(inline)] pub use ::wingdi::ILLUMINANT_C as ILLUMINANT_NTSC; /* wingdi.h:4830:9, wingdi.h:4830:9, wingdi.h:4830:9 */
#[cfg(feature="winapi_desktop")] pub const RGB_GAMMA_MIN: ::minwindef::WORD = 0x9c4i32 as ::minwindef::WORD; /* Cast { value: Integer(2500, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4833:9, wingdi.h:4833:9, wingdi.h:4833:9 */
#[cfg(feature="winapi_desktop")] pub const RGB_GAMMA_MAX: ::minwindef::WORD = 0xfde8i32 as ::minwindef::WORD; /* Cast { value: Integer(65000, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4834:9, wingdi.h:4834:9, wingdi.h:4834:9 */
#[cfg(feature="winapi_desktop")] pub const REFERENCE_WHITE_MIN: ::minwindef::WORD = 0x1770i32 as ::minwindef::WORD; /* Cast { value: Integer(6000, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4837:9, wingdi.h:4837:9, wingdi.h:4837:9 */
#[cfg(feature="winapi_desktop")] pub const REFERENCE_WHITE_MAX: ::minwindef::WORD = 0x2710i32 as ::minwindef::WORD; /* Cast { value: Integer(10000, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4838:9, wingdi.h:4838:9, wingdi.h:4838:9 */
#[cfg(feature="winapi_desktop")] pub const REFERENCE_BLACK_MIN: ::minwindef::WORD = 0x0i32 as ::minwindef::WORD; /* Cast { value: Integer(0, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4839:9, wingdi.h:4839:9, wingdi.h:4839:9 */
#[cfg(feature="winapi_desktop")] pub const REFERENCE_BLACK_MAX: ::minwindef::WORD = 0xfa0i32 as ::minwindef::WORD; /* Cast { value: Integer(4000, Yes, Unknown), ty: Type("WORD", false) } */ /* wingdi.h:4840:9, wingdi.h:4840:9, wingdi.h:4840:9 */
#[cfg(feature="winapi_desktop")] pub const COLOR_ADJ_MIN: ::winnt::SHORT = -0x64i32 as ::winnt::SHORT; /* Cast { value: Unary(Neg, Integer(100, Yes, Unknown)), ty: Type("SHORT", false) } */ /* wingdi.h:4843:9, wingdi.h:4843:9, wingdi.h:4843:9 */
#[cfg(feature="winapi_desktop")] pub const COLOR_ADJ_MAX: ::winnt::SHORT = 0x64i32 as ::winnt::SHORT; /* Cast { value: Integer(100, Yes, Unknown), ty: Type("SHORT", false) } */ /* wingdi.h:4844:9, wingdi.h:4844:9, wingdi.h:4844:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_APPBANDING: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:4898:9, wingdi.h:4898:9, wingdi.h:4898:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const DI_ROPS_READ_DESTINATION: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:4899:9, wingdi.h:4899:9, wingdi.h:4899:9 */
#[cfg(feature="winapi_desktop")] pub const FONTMAPPER_MAX: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:5024:9, wingdi.h:5024:9, wingdi.h:5024:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_OFF: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5055:9, wingdi.h:5055:9, wingdi.h:5055:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_ON: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:5056:9, wingdi.h:5056:9, wingdi.h:5056:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_QUERY: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:5057:9, wingdi.h:5057:9, wingdi.h:5057:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const ICM_DONE_OUTSIDEDC: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:5058:9, wingdi.h:5058:9, wingdi.h:5058:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[doc(inline)] pub use ::wingdi::ICMENUMPROCW as ICMENUMPROC; /* wingdi.h:5063:9, wingdi.h:5063:9, wingdi.h:5063:9 */
#[cfg(feature="winapi_desktop")] pub const ENHMETA_SIGNATURE: i32 = 0x464d4520i32; /* Integer(1179469088, Yes, Unknown) */ /* wingdi.h:5148:9, wingdi.h:5148:9, wingdi.h:5148:9 */
#[cfg(feature="winapi_desktop")] pub const ENHMETA_STOCK_OBJECT: i32 = 0x80000000i32; /* Integer(2147483648, Yes, Unknown) */ /* wingdi.h:5158:9, wingdi.h:5158:9, wingdi.h:5158:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_HEADER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5162:9, wingdi.h:5162:9, wingdi.h:5162:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYBEZIER: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:5163:9, wingdi.h:5163:9, wingdi.h:5163:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYGON: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:5164:9, wingdi.h:5164:9, wingdi.h:5164:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYLINE: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:5165:9, wingdi.h:5165:9, wingdi.h:5165:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYBEZIERTO: i32 = 0x5i32; /* Integer(5, Yes, Unknown) */ /* wingdi.h:5166:9, wingdi.h:5166:9, wingdi.h:5166:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYLINETO: i32 = 0x6i32; /* Integer(6, Yes, Unknown) */ /* wingdi.h:5167:9, wingdi.h:5167:9, wingdi.h:5167:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYPOLYLINE: i32 = 0x7i32; /* Integer(7, Yes, Unknown) */ /* wingdi.h:5168:9, wingdi.h:5168:9, wingdi.h:5168:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYPOLYGON: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:5169:9, wingdi.h:5169:9, wingdi.h:5169:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETWINDOWEXTEX: i32 = 0x9i32; /* Integer(9, Yes, Unknown) */ /* wingdi.h:5170:9, wingdi.h:5170:9, wingdi.h:5170:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETWINDOWORGEX: i32 = 0xai32; /* Integer(10, Yes, Unknown) */ /* wingdi.h:5171:9, wingdi.h:5171:9, wingdi.h:5171:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETVIEWPORTEXTEX: i32 = 0xbi32; /* Integer(11, Yes, Unknown) */ /* wingdi.h:5172:9, wingdi.h:5172:9, wingdi.h:5172:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETVIEWPORTORGEX: i32 = 0xci32; /* Integer(12, Yes, Unknown) */ /* wingdi.h:5173:9, wingdi.h:5173:9, wingdi.h:5173:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETBRUSHORGEX: i32 = 0xdi32; /* Integer(13, Yes, Unknown) */ /* wingdi.h:5174:9, wingdi.h:5174:9, wingdi.h:5174:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EOF: i32 = 0xei32; /* Integer(14, Yes, Unknown) */ /* wingdi.h:5175:9, wingdi.h:5175:9, wingdi.h:5175:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETPIXELV: i32 = 0xfi32; /* Integer(15, Yes, Unknown) */ /* wingdi.h:5176:9, wingdi.h:5176:9, wingdi.h:5176:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETMAPPERFLAGS: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:5177:9, wingdi.h:5177:9, wingdi.h:5177:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETMAPMODE: i32 = 0x11i32; /* Integer(17, Yes, Unknown) */ /* wingdi.h:5178:9, wingdi.h:5178:9, wingdi.h:5178:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETBKMODE: i32 = 0x12i32; /* Integer(18, Yes, Unknown) */ /* wingdi.h:5179:9, wingdi.h:5179:9, wingdi.h:5179:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETPOLYFILLMODE: i32 = 0x13i32; /* Integer(19, Yes, Unknown) */ /* wingdi.h:5180:9, wingdi.h:5180:9, wingdi.h:5180:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETROP2: i32 = 0x14i32; /* Integer(20, Yes, Unknown) */ /* wingdi.h:5181:9, wingdi.h:5181:9, wingdi.h:5181:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETSTRETCHBLTMODE: i32 = 0x15i32; /* Integer(21, Yes, Unknown) */ /* wingdi.h:5182:9, wingdi.h:5182:9, wingdi.h:5182:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETTEXTALIGN: i32 = 0x16i32; /* Integer(22, Yes, Unknown) */ /* wingdi.h:5183:9, wingdi.h:5183:9, wingdi.h:5183:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETCOLORADJUSTMENT: i32 = 0x17i32; /* Integer(23, Yes, Unknown) */ /* wingdi.h:5184:9, wingdi.h:5184:9, wingdi.h:5184:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETTEXTCOLOR: i32 = 0x18i32; /* Integer(24, Yes, Unknown) */ /* wingdi.h:5185:9, wingdi.h:5185:9, wingdi.h:5185:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETBKCOLOR: i32 = 0x19i32; /* Integer(25, Yes, Unknown) */ /* wingdi.h:5186:9, wingdi.h:5186:9, wingdi.h:5186:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_OFFSETCLIPRGN: i32 = 0x1ai32; /* Integer(26, Yes, Unknown) */ /* wingdi.h:5187:9, wingdi.h:5187:9, wingdi.h:5187:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_MOVETOEX: i32 = 0x1bi32; /* Integer(27, Yes, Unknown) */ /* wingdi.h:5188:9, wingdi.h:5188:9, wingdi.h:5188:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETMETARGN: i32 = 0x1ci32; /* Integer(28, Yes, Unknown) */ /* wingdi.h:5189:9, wingdi.h:5189:9, wingdi.h:5189:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXCLUDECLIPRECT: i32 = 0x1di32; /* Integer(29, Yes, Unknown) */ /* wingdi.h:5190:9, wingdi.h:5190:9, wingdi.h:5190:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_INTERSECTCLIPRECT: i32 = 0x1ei32; /* Integer(30, Yes, Unknown) */ /* wingdi.h:5191:9, wingdi.h:5191:9, wingdi.h:5191:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SCALEVIEWPORTEXTEX: i32 = 0x1fi32; /* Integer(31, Yes, Unknown) */ /* wingdi.h:5192:9, wingdi.h:5192:9, wingdi.h:5192:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SCALEWINDOWEXTEX: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:5193:9, wingdi.h:5193:9, wingdi.h:5193:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SAVEDC: i32 = 0x21i32; /* Integer(33, Yes, Unknown) */ /* wingdi.h:5194:9, wingdi.h:5194:9, wingdi.h:5194:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_RESTOREDC: i32 = 0x22i32; /* Integer(34, Yes, Unknown) */ /* wingdi.h:5195:9, wingdi.h:5195:9, wingdi.h:5195:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETWORLDTRANSFORM: i32 = 0x23i32; /* Integer(35, Yes, Unknown) */ /* wingdi.h:5196:9, wingdi.h:5196:9, wingdi.h:5196:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_MODIFYWORLDTRANSFORM: i32 = 0x24i32; /* Integer(36, Yes, Unknown) */ /* wingdi.h:5197:9, wingdi.h:5197:9, wingdi.h:5197:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SELECTOBJECT: i32 = 0x25i32; /* Integer(37, Yes, Unknown) */ /* wingdi.h:5198:9, wingdi.h:5198:9, wingdi.h:5198:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CREATEPEN: i32 = 0x26i32; /* Integer(38, Yes, Unknown) */ /* wingdi.h:5199:9, wingdi.h:5199:9, wingdi.h:5199:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CREATEBRUSHINDIRECT: i32 = 0x27i32; /* Integer(39, Yes, Unknown) */ /* wingdi.h:5200:9, wingdi.h:5200:9, wingdi.h:5200:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_DELETEOBJECT: i32 = 0x28i32; /* Integer(40, Yes, Unknown) */ /* wingdi.h:5201:9, wingdi.h:5201:9, wingdi.h:5201:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ANGLEARC: i32 = 0x29i32; /* Integer(41, Yes, Unknown) */ /* wingdi.h:5202:9, wingdi.h:5202:9, wingdi.h:5202:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ELLIPSE: i32 = 0x2ai32; /* Integer(42, Yes, Unknown) */ /* wingdi.h:5203:9, wingdi.h:5203:9, wingdi.h:5203:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_RECTANGLE: i32 = 0x2bi32; /* Integer(43, Yes, Unknown) */ /* wingdi.h:5204:9, wingdi.h:5204:9, wingdi.h:5204:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ROUNDRECT: i32 = 0x2ci32; /* Integer(44, Yes, Unknown) */ /* wingdi.h:5205:9, wingdi.h:5205:9, wingdi.h:5205:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ARC: i32 = 0x2di32; /* Integer(45, Yes, Unknown) */ /* wingdi.h:5206:9, wingdi.h:5206:9, wingdi.h:5206:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CHORD: i32 = 0x2ei32; /* Integer(46, Yes, Unknown) */ /* wingdi.h:5207:9, wingdi.h:5207:9, wingdi.h:5207:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_PIE: i32 = 0x2fi32; /* Integer(47, Yes, Unknown) */ /* wingdi.h:5208:9, wingdi.h:5208:9, wingdi.h:5208:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SELECTPALETTE: i32 = 0x30i32; /* Integer(48, Yes, Unknown) */ /* wingdi.h:5209:9, wingdi.h:5209:9, wingdi.h:5209:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CREATEPALETTE: i32 = 0x31i32; /* Integer(49, Yes, Unknown) */ /* wingdi.h:5210:9, wingdi.h:5210:9, wingdi.h:5210:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETPALETTEENTRIES: i32 = 0x32i32; /* Integer(50, Yes, Unknown) */ /* wingdi.h:5211:9, wingdi.h:5211:9, wingdi.h:5211:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_RESIZEPALETTE: i32 = 0x33i32; /* Integer(51, Yes, Unknown) */ /* wingdi.h:5212:9, wingdi.h:5212:9, wingdi.h:5212:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_REALIZEPALETTE: i32 = 0x34i32; /* Integer(52, Yes, Unknown) */ /* wingdi.h:5213:9, wingdi.h:5213:9, wingdi.h:5213:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTFLOODFILL: i32 = 0x35i32; /* Integer(53, Yes, Unknown) */ /* wingdi.h:5214:9, wingdi.h:5214:9, wingdi.h:5214:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_LINETO: i32 = 0x36i32; /* Integer(54, Yes, Unknown) */ /* wingdi.h:5215:9, wingdi.h:5215:9, wingdi.h:5215:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ARCTO: i32 = 0x37i32; /* Integer(55, Yes, Unknown) */ /* wingdi.h:5216:9, wingdi.h:5216:9, wingdi.h:5216:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYDRAW: i32 = 0x38i32; /* Integer(56, Yes, Unknown) */ /* wingdi.h:5217:9, wingdi.h:5217:9, wingdi.h:5217:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETARCDIRECTION: i32 = 0x39i32; /* Integer(57, Yes, Unknown) */ /* wingdi.h:5218:9, wingdi.h:5218:9, wingdi.h:5218:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETMITERLIMIT: i32 = 0x3ai32; /* Integer(58, Yes, Unknown) */ /* wingdi.h:5219:9, wingdi.h:5219:9, wingdi.h:5219:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_BEGINPATH: i32 = 0x3bi32; /* Integer(59, Yes, Unknown) */ /* wingdi.h:5220:9, wingdi.h:5220:9, wingdi.h:5220:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ENDPATH: i32 = 0x3ci32; /* Integer(60, Yes, Unknown) */ /* wingdi.h:5221:9, wingdi.h:5221:9, wingdi.h:5221:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CLOSEFIGURE: i32 = 0x3di32; /* Integer(61, Yes, Unknown) */ /* wingdi.h:5222:9, wingdi.h:5222:9, wingdi.h:5222:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_FILLPATH: i32 = 0x3ei32; /* Integer(62, Yes, Unknown) */ /* wingdi.h:5223:9, wingdi.h:5223:9, wingdi.h:5223:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_STROKEANDFILLPATH: i32 = 0x3fi32; /* Integer(63, Yes, Unknown) */ /* wingdi.h:5224:9, wingdi.h:5224:9, wingdi.h:5224:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_STROKEPATH: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:5225:9, wingdi.h:5225:9, wingdi.h:5225:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_FLATTENPATH: i32 = 0x41i32; /* Integer(65, Yes, Unknown) */ /* wingdi.h:5226:9, wingdi.h:5226:9, wingdi.h:5226:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_WIDENPATH: i32 = 0x42i32; /* Integer(66, Yes, Unknown) */ /* wingdi.h:5227:9, wingdi.h:5227:9, wingdi.h:5227:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SELECTCLIPPATH: i32 = 0x43i32; /* Integer(67, Yes, Unknown) */ /* wingdi.h:5228:9, wingdi.h:5228:9, wingdi.h:5228:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_ABORTPATH: i32 = 0x44i32; /* Integer(68, Yes, Unknown) */ /* wingdi.h:5229:9, wingdi.h:5229:9, wingdi.h:5229:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_GDICOMMENT: i32 = 0x46i32; /* Integer(70, Yes, Unknown) */ /* wingdi.h:5231:9, wingdi.h:5231:9, wingdi.h:5231:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_FILLRGN: i32 = 0x47i32; /* Integer(71, Yes, Unknown) */ /* wingdi.h:5232:9, wingdi.h:5232:9, wingdi.h:5232:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_FRAMERGN: i32 = 0x48i32; /* Integer(72, Yes, Unknown) */ /* wingdi.h:5233:9, wingdi.h:5233:9, wingdi.h:5233:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_INVERTRGN: i32 = 0x49i32; /* Integer(73, Yes, Unknown) */ /* wingdi.h:5234:9, wingdi.h:5234:9, wingdi.h:5234:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_PAINTRGN: i32 = 0x4ai32; /* Integer(74, Yes, Unknown) */ /* wingdi.h:5235:9, wingdi.h:5235:9, wingdi.h:5235:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTSELECTCLIPRGN: i32 = 0x4bi32; /* Integer(75, Yes, Unknown) */ /* wingdi.h:5236:9, wingdi.h:5236:9, wingdi.h:5236:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_BITBLT: i32 = 0x4ci32; /* Integer(76, Yes, Unknown) */ /* wingdi.h:5237:9, wingdi.h:5237:9, wingdi.h:5237:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_STRETCHBLT: i32 = 0x4di32; /* Integer(77, Yes, Unknown) */ /* wingdi.h:5238:9, wingdi.h:5238:9, wingdi.h:5238:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_MASKBLT: i32 = 0x4ei32; /* Integer(78, Yes, Unknown) */ /* wingdi.h:5239:9, wingdi.h:5239:9, wingdi.h:5239:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_PLGBLT: i32 = 0x4fi32; /* Integer(79, Yes, Unknown) */ /* wingdi.h:5240:9, wingdi.h:5240:9, wingdi.h:5240:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_SETDIBITSTODEVICE: i32 = 0x50i32; /* Integer(80, Yes, Unknown) */ /* wingdi.h:5241:9, wingdi.h:5241:9, wingdi.h:5241:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_STRETCHDIBITS: i32 = 0x51i32; /* Integer(81, Yes, Unknown) */ /* wingdi.h:5242:9, wingdi.h:5242:9, wingdi.h:5242:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTCREATEFONTINDIRECTW: i32 = 0x52i32; /* Integer(82, Yes, Unknown) */ /* wingdi.h:5243:9, wingdi.h:5243:9, wingdi.h:5243:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTTEXTOUTA: i32 = 0x53i32; /* Integer(83, Yes, Unknown) */ /* wingdi.h:5244:9, wingdi.h:5244:9, wingdi.h:5244:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTTEXTOUTW: i32 = 0x54i32; /* Integer(84, Yes, Unknown) */ /* wingdi.h:5245:9, wingdi.h:5245:9, wingdi.h:5245:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYBEZIER16: i32 = 0x55i32; /* Integer(85, Yes, Unknown) */ /* wingdi.h:5246:9, wingdi.h:5246:9, wingdi.h:5246:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYGON16: i32 = 0x56i32; /* Integer(86, Yes, Unknown) */ /* wingdi.h:5247:9, wingdi.h:5247:9, wingdi.h:5247:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYLINE16: i32 = 0x57i32; /* Integer(87, Yes, Unknown) */ /* wingdi.h:5248:9, wingdi.h:5248:9, wingdi.h:5248:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYBEZIERTO16: i32 = 0x58i32; /* Integer(88, Yes, Unknown) */ /* wingdi.h:5249:9, wingdi.h:5249:9, wingdi.h:5249:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYLINETO16: i32 = 0x59i32; /* Integer(89, Yes, Unknown) */ /* wingdi.h:5250:9, wingdi.h:5250:9, wingdi.h:5250:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYPOLYLINE16: i32 = 0x5ai32; /* Integer(90, Yes, Unknown) */ /* wingdi.h:5251:9, wingdi.h:5251:9, wingdi.h:5251:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYPOLYGON16: i32 = 0x5bi32; /* Integer(91, Yes, Unknown) */ /* wingdi.h:5252:9, wingdi.h:5252:9, wingdi.h:5252:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYDRAW16: i32 = 0x5ci32; /* Integer(92, Yes, Unknown) */ /* wingdi.h:5253:9, wingdi.h:5253:9, wingdi.h:5253:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CREATEMONOBRUSH: i32 = 0x5di32; /* Integer(93, Yes, Unknown) */ /* wingdi.h:5254:9, wingdi.h:5254:9, wingdi.h:5254:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_CREATEDIBPATTERNBRUSHPT: i32 = 0x5ei32; /* Integer(94, Yes, Unknown) */ /* wingdi.h:5255:9, wingdi.h:5255:9, wingdi.h:5255:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_EXTCREATEPEN: i32 = 0x5fi32; /* Integer(95, Yes, Unknown) */ /* wingdi.h:5256:9, wingdi.h:5256:9, wingdi.h:5256:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYTEXTOUTA: i32 = 0x60i32; /* Integer(96, Yes, Unknown) */ /* wingdi.h:5257:9, wingdi.h:5257:9, wingdi.h:5257:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_POLYTEXTOUTW: i32 = 0x61i32; /* Integer(97, Yes, Unknown) */ /* wingdi.h:5258:9, wingdi.h:5258:9, wingdi.h:5258:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_SETICMMODE: i32 = 0x62i32; /* Integer(98, Yes, Unknown) */ /* wingdi.h:5261:9, wingdi.h:5261:9, wingdi.h:5261:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_CREATECOLORSPACE: i32 = 0x63i32; /* Integer(99, Yes, Unknown) */ /* wingdi.h:5262:9, wingdi.h:5262:9, wingdi.h:5262:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_SETCOLORSPACE: i32 = 0x64i32; /* Integer(100, Yes, Unknown) */ /* wingdi.h:5263:9, wingdi.h:5263:9, wingdi.h:5263:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_DELETECOLORSPACE: i32 = 0x65i32; /* Integer(101, Yes, Unknown) */ /* wingdi.h:5264:9, wingdi.h:5264:9, wingdi.h:5264:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_GLSRECORD: i32 = 0x66i32; /* Integer(102, Yes, Unknown) */ /* wingdi.h:5265:9, wingdi.h:5265:9, wingdi.h:5265:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_GLSBOUNDEDRECORD: i32 = 0x67i32; /* Integer(103, Yes, Unknown) */ /* wingdi.h:5266:9, wingdi.h:5266:9, wingdi.h:5266:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] pub const EMR_PIXELFORMAT: i32 = 0x68i32; /* Integer(104, Yes, Unknown) */ /* wingdi.h:5267:9, wingdi.h:5267:9, wingdi.h:5267:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_105: i32 = 0x69i32; /* Integer(105, Yes, Unknown) */ /* wingdi.h:5271:9, wingdi.h:5271:9, wingdi.h:5271:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_106: i32 = 0x6ai32; /* Integer(106, Yes, Unknown) */ /* wingdi.h:5272:9, wingdi.h:5272:9, wingdi.h:5272:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_107: i32 = 0x6bi32; /* Integer(107, Yes, Unknown) */ /* wingdi.h:5273:9, wingdi.h:5273:9, wingdi.h:5273:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_108: i32 = 0x6ci32; /* Integer(108, Yes, Unknown) */ /* wingdi.h:5274:9, wingdi.h:5274:9, wingdi.h:5274:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_109: i32 = 0x6di32; /* Integer(109, Yes, Unknown) */ /* wingdi.h:5275:9, wingdi.h:5275:9, wingdi.h:5275:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_110: i32 = 0x6ei32; /* Integer(110, Yes, Unknown) */ /* wingdi.h:5276:9, wingdi.h:5276:9, wingdi.h:5276:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_COLORCORRECTPALETTE: i32 = 0x6fi32; /* Integer(111, Yes, Unknown) */ /* wingdi.h:5277:9, wingdi.h:5277:9, wingdi.h:5277:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_SETICMPROFILEA: i32 = 0x70i32; /* Integer(112, Yes, Unknown) */ /* wingdi.h:5278:9, wingdi.h:5278:9, wingdi.h:5278:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_SETICMPROFILEW: i32 = 0x71i32; /* Integer(113, Yes, Unknown) */ /* wingdi.h:5279:9, wingdi.h:5279:9, wingdi.h:5279:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_ALPHABLEND: i32 = 0x72i32; /* Integer(114, Yes, Unknown) */ /* wingdi.h:5280:9, wingdi.h:5280:9, wingdi.h:5280:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_SETLAYOUT: i32 = 0x73i32; /* Integer(115, Yes, Unknown) */ /* wingdi.h:5281:9, wingdi.h:5281:9, wingdi.h:5281:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_TRANSPARENTBLT: i32 = 0x74i32; /* Integer(116, Yes, Unknown) */ /* wingdi.h:5282:9, wingdi.h:5282:9, wingdi.h:5282:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_117: i32 = 0x75i32; /* Integer(117, Yes, Unknown) */ /* wingdi.h:5284:9, wingdi.h:5284:9, wingdi.h:5284:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_GRADIENTFILL: i32 = 0x76i32; /* Integer(118, Yes, Unknown) */ /* wingdi.h:5286:9, wingdi.h:5286:9, wingdi.h:5286:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_119: i32 = 0x77i32; /* Integer(119, Yes, Unknown) */ /* wingdi.h:5287:9, wingdi.h:5287:9, wingdi.h:5287:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_RESERVED_120: i32 = 0x78i32; /* Integer(120, Yes, Unknown) */ /* wingdi.h:5288:9, wingdi.h:5288:9, wingdi.h:5288:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_COLORMATCHTOTARGETW: i32 = 0x79i32; /* Integer(121, Yes, Unknown) */ /* wingdi.h:5289:9, wingdi.h:5289:9, wingdi.h:5289:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_CREATECOLORSPACEW: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* wingdi.h:5290:9, wingdi.h:5290:9, wingdi.h:5290:9 */
#[cfg(feature="winapi_desktop")] pub const EMR_MIN: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5293:9, wingdi.h:5293:9, wingdi.h:5293:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const EMR_MAX: i32 = 0x7ai32; /* Integer(122, Yes, Unknown) */ /* wingdi.h:5296:9, wingdi.h:5296:9, wingdi.h:5296:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const SETICMPROFILE_EMBEDED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5954:9, wingdi.h:5954:9, wingdi.h:5954:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const CREATECOLORSPACE_EMBEDED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5967:9, wingdi.h:5967:9, wingdi.h:5967:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const COLORMATCHTOTARGET_EMBEDED: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:5979:9, wingdi.h:5979:9, wingdi.h:5979:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_IDENTIFIER: i32 = 0x43494447i32; /* Integer(1128875079, Yes, Unknown) */ /* wingdi.h:6059:9, wingdi.h:6059:9, wingdi.h:6059:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_WINDOWS_METAFILE: i32 = 0x80000001i32; /* Integer(2147483649, Yes, Unknown) */ /* wingdi.h:6060:9, wingdi.h:6060:9, wingdi.h:6060:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_BEGINGROUP: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:6061:9, wingdi.h:6061:9, wingdi.h:6061:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_ENDGROUP: i32 = 0x3i32; /* Integer(3, Yes, Unknown) */ /* wingdi.h:6062:9, wingdi.h:6062:9, wingdi.h:6062:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_MULTIFORMATS: i32 = 0x40000004i32; /* Integer(1073741828, Yes, Unknown) */ /* wingdi.h:6063:9, wingdi.h:6063:9, wingdi.h:6063:9 */
#[cfg(feature="winapi_desktop")] pub const EPS_SIGNATURE: i32 = 0x46535045i32; /* Integer(1179865157, Yes, Unknown) */ /* wingdi.h:6064:9, wingdi.h:6064:9, wingdi.h:6064:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_UNICODE_STRING: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:6065:9, wingdi.h:6065:9, wingdi.h:6065:9 */
#[cfg(feature="winapi_desktop")] pub const GDICOMMENT_UNICODE_END: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:6066:9, wingdi.h:6066:9, wingdi.h:6066:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_FONT_LINES: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:6104:9, wingdi.h:6104:9, wingdi.h:6104:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_FONT_POLYGONS: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:6105:9, wingdi.h:6105:9, wingdi.h:6105:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_DOUBLEBUFFER: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:6145:9, wingdi.h:6145:9, wingdi.h:6145:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_STEREO: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:6146:9, wingdi.h:6146:9, wingdi.h:6146:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SUPPORT_GDI: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:6147:9, wingdi.h:6147:9, wingdi.h:6147:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SUPPORT_OPENGL: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:6148:9, wingdi.h:6148:9, wingdi.h:6148:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SHARE_DEPTH: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:6149:9, wingdi.h:6149:9, wingdi.h:6149:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SHARE_STENCIL: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:6150:9, wingdi.h:6150:9, wingdi.h:6150:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SHARE_ACCUM: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:6151:9, wingdi.h:6151:9, wingdi.h:6151:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SWAP_EXCHANGE: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:6152:9, wingdi.h:6152:9, wingdi.h:6152:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_SWAP_COPY: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:6153:9, wingdi.h:6153:9, wingdi.h:6153:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_TRANSPARENT: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:6154:9, wingdi.h:6154:9, wingdi.h:6154:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_TYPE_RGBA: i32 = 0x0i32; /* Integer(0, Yes, Unknown) */ /* wingdi.h:6156:9, wingdi.h:6156:9, wingdi.h:6156:9 */
#[cfg(feature="winapi_desktop")] pub const LPD_TYPE_COLORINDEX: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:6157:9, wingdi.h:6157:9, wingdi.h:6157:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_MAIN_PLANE: i32 = 0x1i32; /* Integer(1, Yes, Unknown) */ /* wingdi.h:6160:9, wingdi.h:6160:9, wingdi.h:6160:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY1: i32 = 0x2i32; /* Integer(2, Yes, Unknown) */ /* wingdi.h:6161:9, wingdi.h:6161:9, wingdi.h:6161:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY2: i32 = 0x4i32; /* Integer(4, Yes, Unknown) */ /* wingdi.h:6162:9, wingdi.h:6162:9, wingdi.h:6162:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY3: i32 = 0x8i32; /* Integer(8, Yes, Unknown) */ /* wingdi.h:6163:9, wingdi.h:6163:9, wingdi.h:6163:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY4: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:6164:9, wingdi.h:6164:9, wingdi.h:6164:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY5: i32 = 0x20i32; /* Integer(32, Yes, Unknown) */ /* wingdi.h:6165:9, wingdi.h:6165:9, wingdi.h:6165:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY6: i32 = 0x40i32; /* Integer(64, Yes, Unknown) */ /* wingdi.h:6166:9, wingdi.h:6166:9, wingdi.h:6166:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY7: i32 = 0x80i32; /* Integer(128, Yes, Unknown) */ /* wingdi.h:6167:9, wingdi.h:6167:9, wingdi.h:6167:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY8: i32 = 0x100i32; /* Integer(256, Yes, Unknown) */ /* wingdi.h:6168:9, wingdi.h:6168:9, wingdi.h:6168:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY9: i32 = 0x200i32; /* Integer(512, Yes, Unknown) */ /* wingdi.h:6169:9, wingdi.h:6169:9, wingdi.h:6169:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY10: i32 = 0x400i32; /* Integer(1024, Yes, Unknown) */ /* wingdi.h:6170:9, wingdi.h:6170:9, wingdi.h:6170:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY11: i32 = 0x800i32; /* Integer(2048, Yes, Unknown) */ /* wingdi.h:6171:9, wingdi.h:6171:9, wingdi.h:6171:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY12: i32 = 0x1000i32; /* Integer(4096, Yes, Unknown) */ /* wingdi.h:6172:9, wingdi.h:6172:9, wingdi.h:6172:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY13: i32 = 0x2000i32; /* Integer(8192, Yes, Unknown) */ /* wingdi.h:6173:9, wingdi.h:6173:9, wingdi.h:6173:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY14: i32 = 0x4000i32; /* Integer(16384, Yes, Unknown) */ /* wingdi.h:6174:9, wingdi.h:6174:9, wingdi.h:6174:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_OVERLAY15: i32 = 0x8000i32; /* Integer(32768, Yes, Unknown) */ /* wingdi.h:6175:9, wingdi.h:6175:9, wingdi.h:6175:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY1: i32 = 0x10000i32; /* Integer(65536, Yes, Unknown) */ /* wingdi.h:6176:9, wingdi.h:6176:9, wingdi.h:6176:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY2: i32 = 0x20000i32; /* Integer(131072, Yes, Unknown) */ /* wingdi.h:6177:9, wingdi.h:6177:9, wingdi.h:6177:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY3: i32 = 0x40000i32; /* Integer(262144, Yes, Unknown) */ /* wingdi.h:6178:9, wingdi.h:6178:9, wingdi.h:6178:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY4: i32 = 0x80000i32; /* Integer(524288, Yes, Unknown) */ /* wingdi.h:6179:9, wingdi.h:6179:9, wingdi.h:6179:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY5: i32 = 0x100000i32; /* Integer(1048576, Yes, Unknown) */ /* wingdi.h:6180:9, wingdi.h:6180:9, wingdi.h:6180:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY6: i32 = 0x200000i32; /* Integer(2097152, Yes, Unknown) */ /* wingdi.h:6181:9, wingdi.h:6181:9, wingdi.h:6181:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY7: i32 = 0x400000i32; /* Integer(4194304, Yes, Unknown) */ /* wingdi.h:6182:9, wingdi.h:6182:9, wingdi.h:6182:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY8: i32 = 0x800000i32; /* Integer(8388608, Yes, Unknown) */ /* wingdi.h:6183:9, wingdi.h:6183:9, wingdi.h:6183:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY9: i32 = 0x1000000i32; /* Integer(16777216, Yes, Unknown) */ /* wingdi.h:6184:9, wingdi.h:6184:9, wingdi.h:6184:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY10: i32 = 0x2000000i32; /* Integer(33554432, Yes, Unknown) */ /* wingdi.h:6185:9, wingdi.h:6185:9, wingdi.h:6185:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY11: i32 = 0x4000000i32; /* Integer(67108864, Yes, Unknown) */ /* wingdi.h:6186:9, wingdi.h:6186:9, wingdi.h:6186:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY12: i32 = 0x8000000i32; /* Integer(134217728, Yes, Unknown) */ /* wingdi.h:6187:9, wingdi.h:6187:9, wingdi.h:6187:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY13: i32 = 0x10000000i32; /* Integer(268435456, Yes, Unknown) */ /* wingdi.h:6188:9, wingdi.h:6188:9, wingdi.h:6188:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY14: i32 = 0x20000000i32; /* Integer(536870912, Yes, Unknown) */ /* wingdi.h:6189:9, wingdi.h:6189:9, wingdi.h:6189:9 */
#[cfg(feature="winapi_desktop")] pub const WGL_SWAP_UNDERLAY15: i32 = 0x40000000i32; /* Integer(1073741824, Yes, Unknown) */ /* wingdi.h:6190:9, wingdi.h:6190:9, wingdi.h:6190:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_05000000"))] pub const WGL_SWAPMULTIPLE_MAX: i32 = 0x10i32; /* Integer(16, Yes, Unknown) */ /* wingdi.h:6209:9, wingdi.h:6209:9, wingdi.h:6209:9 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type OLDFONTENUMPROCA = extern "system" fn(*const ::wingdi::LOGFONTA, *const ::wingdi::TEXTMETRICA, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:3555:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type OLDFONTENUMPROCW = extern "system" fn(*const ::wingdi::LOGFONTW, *const ::wingdi::TEXTMETRICW, ::libc::c_ulong, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:3556:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type GOBJENUMPROC = extern "system" fn(*mut ::libc::c_void, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:3580:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type LINEDDAPROC = extern "system" fn(::libc::c_int, ::libc::c_int, ::libc::c_longlong); /* wingdi.h:3581:25 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type MFENUMPROC = extern "system" fn(*mut ::windef::HDC__, *mut ::wingdi::HANDLETABLE, *mut ::wingdi::METARECORD, ::libc::c_int, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:4657:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(target_arch="x86_64"))] pub type ENHMFENUMPROC = extern "system" fn(*mut ::windef::HDC__, *mut ::wingdi::HANDLETABLE, *const ::wingdi::ENHMETARECORD, ::libc::c_int, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:4660:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86_64"))] pub type ICMENUMPROCA = extern "system" fn(*mut ::libc::c_schar, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:5060:24 */
#[cfg(feature="winapi_desktop")] #[cfg(any(feature="winapi_ver_04000000"))] #[cfg(any(target_arch="x86_64"))] pub type ICMENUMPROCW = extern "system" fn(*mut ::libc::c_ushort, ::libc::c_longlong) -> ::libc::c_int; /* wingdi.h:5061:24 */
