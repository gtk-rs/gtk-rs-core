// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt::Debug, io};

use crate::ffi;

#[derive(Debug, Clone, PartialEq, Copy, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_status_t")]
pub enum Error {
    #[doc(alias = "STATUS_NO_MEMORY")]
    NoMemory,
    #[doc(alias = "STATUS_INVALID_RESTORE")]
    InvalidRestore,
    #[doc(alias = "STATUS_INVALID_POP_GROUP")]
    InvalidPopGroup,
    #[doc(alias = "STATUS_NO_CURRENT_POINT")]
    NoCurrentPoint,
    #[doc(alias = "STATUS_INVALID_MATRIX")]
    InvalidMatrix,
    #[doc(alias = "STATUS_INVALID_STATUS")]
    InvalidStatus,
    #[doc(alias = "STATUS_NULL_POINTER")]
    NullPointer,
    #[doc(alias = "STATUS_INVALID_STRING")]
    InvalidString,
    #[doc(alias = "STATUS_INVALID_PATH_DATA")]
    InvalidPathData,
    #[doc(alias = "STATUS_READ_ERROR")]
    ReadError,
    #[doc(alias = "STATUS_WRITE_ERROR")]
    WriteError,
    #[doc(alias = "STATUS_SURFACE_FINISHED")]
    SurfaceFinished,
    #[doc(alias = "STATUS_SURFACE_TYPE_MISMATCH")]
    SurfaceTypeMismatch,
    #[doc(alias = "STATUS_PATTERN_TYPE_MISMATCH")]
    PatternTypeMismatch,
    #[doc(alias = "STATUS_INVALID_CONTENT")]
    InvalidContent,
    #[doc(alias = "STATUS_INVALID_FORMAT")]
    InvalidFormat,
    #[doc(alias = "STATUS_INVALID_VISUAL")]
    InvalidVisual,
    #[doc(alias = "STATUS_FILE_NOT_FOUND")]
    FileNotFound,
    #[doc(alias = "STATUS_INVALID_DASH")]
    InvalidDash,
    #[doc(alias = "STATUS_INVALID_DSC_COMMENT")]
    InvalidDscComment,
    #[doc(alias = "STATUS_INVALID_INDEX")]
    InvalidIndex,
    #[doc(alias = "STATUS_CLIP_NOT_REPRESENTABLE")]
    ClipNotRepresentable,
    #[doc(alias = "STATUS_TEMP_FILE_ERROR")]
    TempFileError,
    #[doc(alias = "STATUS_INVALID_STRIDE")]
    InvalidStride,
    #[doc(alias = "STATUS_FONT_TYPE_MISMATCH")]
    FontTypeMismatch,
    #[doc(alias = "STATUS_USER_FONT_IMMUTABLE")]
    UserFontImmutable,
    #[doc(alias = "STATUS_USER_FONT_ERROR")]
    UserFontError,
    #[doc(alias = "STATUS_NEGATIVE_COUNT")]
    NegativeCount,
    #[doc(alias = "STATUS_INVALID_CLUSTERS")]
    InvalidClusters,
    #[doc(alias = "STATUS_INVALID_SLANT")]
    InvalidSlant,
    #[doc(alias = "STATUS_INVALID_WEIGHT")]
    InvalidWeight,
    #[doc(alias = "STATUS_INVALID_SIZE")]
    InvalidSize,
    #[doc(alias = "STATUS_USER_FONT_NOT_IMPLEMENTED")]
    UserFontNotImplemented,
    #[doc(alias = "STATUS_DEVICE_TYPE_MISMATCH")]
    DeviceTypeMismatch,
    #[doc(alias = "STATUS_DEVICE_ERROR")]
    DeviceError,
    #[doc(alias = "STATUS_INVALID_MESH_CONSTRUCTION")]
    InvalidMeshConstruction,
    #[doc(alias = "STATUS_DEVICE_FINISHED")]
    DeviceFinished,
    #[doc(alias = "STATUS_J_BIG2_GLOBAL_MISSING")]
    JBig2GlobalMissing,
    #[doc(alias = "STATUS_PNG_ERROR")]
    PngError,
    #[doc(alias = "STATUS_FREETYPE_ERROR")]
    FreetypeError,
    #[doc(alias = "STATUS_WIN32_GDI_ERROR")]
    Win32GdiError,
    #[cfg(feature = "v1_16")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_16")))]
    #[doc(alias = "STATUS_TAG_ERROR")]
    TagError,
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "STATUS_DWRITE_ERROR")]
    DwriteError,
    #[doc(alias = "STATUS_LAST_STATUS")]
    LastStatus,
    #[doc(hidden)]
    __Unknown(i32),
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NoMemory => f.write_str("No Memory"),
            Error::InvalidRestore => f.write_str("Invalid Restore"),
            Error::InvalidPopGroup => f.write_str("Invalid Pop Group"),
            Error::NoCurrentPoint => f.write_str("No Current Point"),
            Error::InvalidMatrix => f.write_str("Invalid Matrix"),
            Error::InvalidStatus => f.write_str("Invalid Status"),
            Error::NullPointer => f.write_str("Null Pointer"),
            Error::InvalidString => f.write_str("Invalid String"),
            Error::InvalidPathData => f.write_str("Invalid Path Data"),
            Error::ReadError => f.write_str("Read Error"),
            Error::WriteError => f.write_str("Write Error"),
            Error::SurfaceFinished => f.write_str("Surface Finished"),
            Error::SurfaceTypeMismatch => f.write_str("Surface Type Mismatch"),
            Error::PatternTypeMismatch => f.write_str("Pattern Type Mismatch"),
            Error::InvalidContent => f.write_str("Invalid Content"),
            Error::InvalidFormat => f.write_str("Invalid Format"),
            Error::InvalidVisual => f.write_str("Invalid Visual"),
            Error::FileNotFound => f.write_str("File Not Found"),
            Error::InvalidDash => f.write_str("Invalid Dash"),
            Error::InvalidDscComment => f.write_str("Invalid Dash Comment"),
            Error::InvalidIndex => f.write_str("Invalid Index"),
            Error::ClipNotRepresentable => f.write_str("Clip Not Representable"),
            Error::TempFileError => f.write_str("Temp File Error"),
            Error::InvalidStride => f.write_str("Invalid Stride"),
            Error::FontTypeMismatch => f.write_str("Font Type Mismatch"),
            Error::UserFontImmutable => f.write_str("User Font Immutable"),
            Error::UserFontError => f.write_str("User Font Error"),
            Error::NegativeCount => f.write_str("Negative Count"),
            Error::InvalidClusters => f.write_str("Invalid Clusters"),
            Error::InvalidSlant => f.write_str("Invalid Slant"),
            Error::InvalidWeight => f.write_str("Invalid Weight"),
            Error::InvalidSize => f.write_str("Invalid Size"),
            Error::UserFontNotImplemented => f.write_str("User Font Not Implemented"),
            Error::DeviceTypeMismatch => f.write_str("Device Type Mismatch"),
            Error::DeviceError => f.write_str("Device Error"),
            Error::InvalidMeshConstruction => f.write_str("Invalid Mesh Construction"),
            Error::DeviceFinished => f.write_str("Device Finished"),
            Error::JBig2GlobalMissing => f.write_str("JBig2Global Missing"),
            Error::PngError => f.write_str("PNG Error"),
            Error::FreetypeError => f.write_str("Freetype Error"),
            Error::Win32GdiError => f.write_str("Win32Gdi Error"),
            #[cfg(feature = "v1_16")]
            Error::TagError => f.write_str("Tag Error"),
            #[cfg(feature = "v1_18")]
            Error::DwriteError => f.write_str("Dwrite Error"),
            Error::LastStatus => f.write_str("LastStatus"),
            Error::__Unknown(e) => f.write_fmt(format_args!("Unknown {e}")),
        }
    }
}

#[doc(hidden)]
impl From<Error> for ffi::cairo_status_t {
    fn from(err: Error) -> ffi::cairo_status_t {
        match err {
            Error::NoMemory => ffi::STATUS_NO_MEMORY,
            Error::InvalidRestore => ffi::STATUS_INVALID_RESTORE,
            Error::InvalidPopGroup => ffi::STATUS_INVALID_POP_GROUP,
            Error::NoCurrentPoint => ffi::STATUS_NO_CURRENT_POINT,
            Error::InvalidMatrix => ffi::STATUS_INVALID_MATRIX,
            Error::InvalidStatus => ffi::STATUS_INVALID_STATUS,
            Error::NullPointer => ffi::STATUS_NULL_POINTER,
            Error::InvalidString => ffi::STATUS_INVALID_STRING,
            Error::InvalidPathData => ffi::STATUS_INVALID_PATH_DATA,
            Error::ReadError => ffi::STATUS_READ_ERROR,
            Error::WriteError => ffi::STATUS_WRITE_ERROR,
            Error::SurfaceFinished => ffi::STATUS_SURFACE_FINISHED,
            Error::SurfaceTypeMismatch => ffi::STATUS_SURFACE_TYPE_MISMATCH,
            Error::PatternTypeMismatch => ffi::STATUS_PATTERN_TYPE_MISMATCH,
            Error::InvalidContent => ffi::STATUS_INVALID_CONTENT,
            Error::InvalidFormat => ffi::STATUS_INVALID_FORMAT,
            Error::InvalidVisual => ffi::STATUS_INVALID_VISUAL,
            Error::FileNotFound => ffi::STATUS_FILE_NOT_FOUND,
            Error::InvalidDash => ffi::STATUS_INVALID_DASH,
            Error::InvalidDscComment => ffi::STATUS_INVALID_DSC_COMMENT,
            Error::InvalidIndex => ffi::STATUS_INVALID_INDEX,
            Error::ClipNotRepresentable => ffi::STATUS_CLIP_NOT_REPRESENTABLE,
            Error::TempFileError => ffi::STATUS_TEMP_FILE_ERROR,
            Error::InvalidStride => ffi::STATUS_INVALID_STRIDE,
            Error::FontTypeMismatch => ffi::STATUS_FONT_TYPE_MISMATCH,
            Error::UserFontImmutable => ffi::STATUS_USER_FONT_IMMUTABLE,
            Error::UserFontError => ffi::STATUS_USER_FONT_ERROR,
            Error::NegativeCount => ffi::STATUS_NEGATIVE_COUNT,
            Error::InvalidClusters => ffi::STATUS_INVALID_CLUSTERS,
            Error::InvalidSlant => ffi::STATUS_INVALID_SLANT,
            Error::InvalidWeight => ffi::STATUS_INVALID_WEIGHT,
            Error::InvalidSize => ffi::STATUS_INVALID_SIZE,
            Error::UserFontNotImplemented => ffi::STATUS_USER_FONT_NOT_IMPLEMENTED,
            Error::DeviceTypeMismatch => ffi::STATUS_DEVICE_TYPE_MISMATCH,
            Error::DeviceError => ffi::STATUS_DEVICE_ERROR,
            Error::InvalidMeshConstruction => ffi::STATUS_INVALID_MESH_CONSTRUCTION,
            Error::DeviceFinished => ffi::STATUS_DEVICE_FINISHED,
            Error::JBig2GlobalMissing => ffi::STATUS_J_BIG2_GLOBAL_MISSING,
            Error::PngError => ffi::STATUS_PNG_ERROR,
            Error::FreetypeError => ffi::STATUS_FREETYPE_ERROR,
            Error::Win32GdiError => ffi::STATUS_WIN32_GDI_ERROR,
            #[cfg(feature = "v1_16")]
            Error::TagError => ffi::STATUS_TAG_ERROR,
            #[cfg(feature = "v1_18")]
            Error::DwriteError => ffi::STATUS_DWRITE_ERROR,
            Error::LastStatus => ffi::STATUS_LAST_STATUS,
            Error::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_status_t> for Error {
    fn from(value: ffi::cairo_status_t) -> Self {
        match value {
            ffi::STATUS_NO_MEMORY => Self::NoMemory,
            ffi::STATUS_INVALID_RESTORE => Self::InvalidRestore,
            ffi::STATUS_INVALID_POP_GROUP => Self::InvalidPopGroup,
            ffi::STATUS_NO_CURRENT_POINT => Self::NoCurrentPoint,
            ffi::STATUS_INVALID_MATRIX => Self::InvalidMatrix,
            ffi::STATUS_INVALID_STATUS => Self::InvalidStatus,
            ffi::STATUS_NULL_POINTER => Self::NullPointer,
            ffi::STATUS_INVALID_STRING => Self::InvalidString,
            ffi::STATUS_INVALID_PATH_DATA => Self::InvalidPathData,
            ffi::STATUS_READ_ERROR => Self::ReadError,
            ffi::STATUS_WRITE_ERROR => Self::WriteError,
            ffi::STATUS_SURFACE_FINISHED => Self::SurfaceFinished,
            ffi::STATUS_SURFACE_TYPE_MISMATCH => Self::SurfaceTypeMismatch,
            ffi::STATUS_PATTERN_TYPE_MISMATCH => Self::PatternTypeMismatch,
            ffi::STATUS_INVALID_CONTENT => Self::InvalidContent,
            ffi::STATUS_INVALID_FORMAT => Self::InvalidFormat,
            ffi::STATUS_INVALID_VISUAL => Self::InvalidVisual,
            ffi::STATUS_FILE_NOT_FOUND => Self::FileNotFound,
            ffi::STATUS_INVALID_DASH => Self::InvalidDash,
            ffi::STATUS_INVALID_DSC_COMMENT => Self::InvalidDscComment,
            ffi::STATUS_INVALID_INDEX => Self::InvalidIndex,
            ffi::STATUS_CLIP_NOT_REPRESENTABLE => Self::ClipNotRepresentable,
            ffi::STATUS_TEMP_FILE_ERROR => Self::TempFileError,
            ffi::STATUS_INVALID_STRIDE => Self::InvalidStride,
            ffi::STATUS_FONT_TYPE_MISMATCH => Self::FontTypeMismatch,
            ffi::STATUS_USER_FONT_IMMUTABLE => Self::UserFontImmutable,
            ffi::STATUS_USER_FONT_ERROR => Self::UserFontError,
            ffi::STATUS_NEGATIVE_COUNT => Self::NegativeCount,
            ffi::STATUS_INVALID_CLUSTERS => Self::InvalidClusters,
            ffi::STATUS_INVALID_SLANT => Self::InvalidSlant,
            ffi::STATUS_INVALID_WEIGHT => Self::InvalidWeight,
            ffi::STATUS_INVALID_SIZE => Self::InvalidSize,
            ffi::STATUS_USER_FONT_NOT_IMPLEMENTED => Self::UserFontNotImplemented,
            ffi::STATUS_DEVICE_TYPE_MISMATCH => Self::DeviceTypeMismatch,
            ffi::STATUS_DEVICE_ERROR => Self::DeviceError,
            ffi::STATUS_INVALID_MESH_CONSTRUCTION => Self::InvalidMeshConstruction,
            ffi::STATUS_DEVICE_FINISHED => Self::DeviceFinished,
            ffi::STATUS_J_BIG2_GLOBAL_MISSING => Self::JBig2GlobalMissing,
            ffi::STATUS_PNG_ERROR => Self::PngError,
            ffi::STATUS_FREETYPE_ERROR => Self::FreetypeError,
            ffi::STATUS_WIN32_GDI_ERROR => Self::Win32GdiError,
            #[cfg(feature = "v1_16")]
            ffi::STATUS_TAG_ERROR => Self::TagError,
            #[cfg(feature = "v1_18")]
            ffi::STATUS_DWRITE_ERROR => Self::DwriteError,
            ffi::STATUS_LAST_STATUS => Self::LastStatus,
            value => Self::__Unknown(value),
        }
    }
}

#[derive(Debug)]
pub enum IoError {
    Cairo(Error),
    Io(io::Error),
}

impl std::error::Error for IoError {}
impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::Cairo(e) => {
                f.write_fmt(format_args!("Failed to borrow with Cairo error: {e}"))
            }
            IoError::Io(e) => f.write_fmt(format_args!("IO error: {e}")),
        }
    }
}

impl From<Error> for IoError {
    fn from(value: Error) -> Self {
        Self::Cairo(value)
    }
}

#[derive(Debug)]
pub enum BorrowError {
    Cairo(Error),
    NonExclusive,
}

impl std::error::Error for BorrowError {}
impl std::fmt::Display for BorrowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorrowError::Cairo(e) => {
                f.write_fmt(format_args!("Failed to borrow with Cairo error: {e}"))
            }
            BorrowError::NonExclusive => f.write_str("Can't get exclusive access"),
        }
    }
}
impl From<Error> for BorrowError {
    fn from(value: Error) -> Self {
        Self::Cairo(value)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
