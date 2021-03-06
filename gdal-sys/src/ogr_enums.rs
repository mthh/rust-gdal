#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
#[repr(C)]
pub enum OGRErr {
    OGRERR_NONE = 0,                       
    OGRERR_NOT_ENOUGH_DATA = 1,
    OGRERR_NOT_ENOUGH_MEMORY = 2,
    OGRERR_UNSUPPORTED_GEOMETRY_TYPE = 3,
    OGRERR_UNSUPPORTED_OPERATION = 4,
    OGRERR_CORRUPT_DATA = 5,
    OGRERR_FAILURE = 6,
    OGRERR_UNSUPPORTED_SRS = 7,
    OGRERR_INVALID_HANDLE = 8,
    OGRERR_NON_EXISTING_FEATURE = 9
}