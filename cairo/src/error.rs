// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt::{self, Debug};
use std::io;

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
    #[doc(alias = "STATUS_LAST_STATUS")]
    LastStatus,
    #[doc(hidden)]
    __Unknown(i32),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMemory => write!(f, "No Memory"),
            Self::InvalidRestore => write!(f, "Invalid Restore"),
            Self::InvalidPopGroup => write!(f, "Invalid Pop Group"),
            Self::NoCurrentPoint => write!(f, "No Current Point"),
            Self::InvalidMatrix => write!(f, "Invalid Matrix"),
            Self::InvalidStatus => write!(f, "Invalid Status"),
            Self::NullPointer => write!(f, "Null Pointer"),
            Self::InvalidString => write!(f, "Invalid String"),
            Self::InvalidPathData => write!(f, "Invalid Path Data"),
            Self::ReadError => write!(f, "Cairo: Read Error"),
            Self::WriteError => write!(f, "Write Error"),
            Self::SurfaceFinished => write!(f, "Surface Finished"),
            Self::SurfaceTypeMismatch => write!(f, "Surface Type Mismatch"),
            Self::PatternTypeMismatch => write!(f, "Pattern Type Mismatch"),
            Self::InvalidContent => write!(f, "Invalid Content"),
            Self::InvalidFormat => write!(f, "Invalid Format"),
            Self::InvalidVisual => write!(f, "Invalid Visual"),
            Self::FileNotFound => write!(f, "File Not Found"),
            Self::InvalidDash => write!(f, "Invalid Dash"),
            Self::InvalidDscComment => write!(f, "Invalid Dash Comment"),
            Self::InvalidIndex => write!(f, "Invalid Index"),
            Self::ClipNotRepresentable => write!(f, "Clip Not Representable"),
            Self::TempFileError => write!(f, "Temp File Error"),
            Self::InvalidStride => write!(f, "Invalid Stride"),
            Self::FontTypeMismatch => write!(f, "Font Type Mismatch"),
            Self::UserFontImmutable => write!(f, "User Font Immutable"),
            Self::UserFontError => write!(f, "User Font Error"),
            Self::NegativeCount => write!(f, "Negative Count"),
            Self::InvalidClusters => write!(f, "Invalid Clusters"),
            Self::InvalidSlant => write!(f, "Invalid Slant"),
            Self::InvalidWeight => write!(f, "Invalid Weight"),
            Self::InvalidSize => write!(f, "Invalid Size"),
            Self::UserFontNotImplemented => write!(f, "User Font Not Implemented"),
            Self::DeviceTypeMismatch => write!(f, "Device Type Mismatch"),
            Self::DeviceError => write!(f, "Device Error"),
            Self::InvalidMeshConstruction => write!(f, "Invalid Mesh Construction"),
            Self::DeviceFinished => write!(f, "Device Finished"),
            Self::JBig2GlobalMissing => write!(f, "JBig2Global Missing"),
            Self::PngError => write!(f, "PNG Error"),
            Self::FreetypeError => write!(f, "Freetype Error"),
            Self::Win32GdiError => write!(f, "Win32Gdi Error"),
            Self::LastStatus => write!(f, "LastStatus"),
            Self::__Unknown(e) => write!(f, "Unknown {}", e),
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

impl From<Error> for IoError {
    fn from(err: Error) -> Self {
        Self::Cairo(err)
    }
}

impl From<io::Error> for IoError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl std::error::Error for IoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cairo(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cairo(e) => write!(f, "Cairo error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
        }
    }
}

#[derive(Debug)]
pub enum BorrowError {
    Cairo(Error),
    NonExclusive,
}

impl From<Error> for BorrowError {
    fn from(err: Error) -> Self {
        Self::Cairo(err)
    }
}

impl std::error::Error for BorrowError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Cairo(e) => Some(e),
            Self::NonExclusive => None,
        }
    }
}

impl fmt::Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cairo(e) => write!(f, "Failed to borrow with Cairo error: {}", e),
            Self::NonExclusive => write!(f, "Can't get exclusive access"),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
