// arguments
pub const ERROR_EXTRACTING_ARGS_NOT_POSSIBLE: &str = "Extracting args not possible. Exiting";
pub const ERROR_KEYSIZE_NOT_USIZE: &str = "The keysize argument must be usize";
pub const ERROR_THREAD_COUNT_NOT_USIZE: &str = "The thread-count argument must be usize";
pub const ERROR_BYTE_COUNT_NOT_USIZE: &str = "The byte-count argument must be usize";
pub const ERROR_ENTROPY_DELTA_NOT_F32: &str = "The entropy-delta argument must be f32";
pub const ERROR_BUFFERSIZE_NOT_USIZE: &str = "The buffersize argument must be usize";

// read_bytes
pub const ERROR_READ_BYTES_TO_VECTOR_FAILED: &str = "Read Bytes to Vector failed";
pub const ERROR_OPEN_INPUT_FILE_FAILED: &str = "Open input file failed";

// write_ck
pub const ERROR_OUTPUT_FILE_CAN_NOT_BE_CREATED: &str = "Output file can not be created";
pub const ERROR_OUTPUT_FILE_IS_NOT_WRITEABLE: &str = "Output file not writeable";
pub const ERROR_EXISTING_OUTPUT_FILE_CAN_NOT_BE_REMOVED: &str =
    "Existing output File can't be removed";
pub const ERROR_OUTPUT_FOLDER_CAN_NOT_BE_CREATED: &str = "Output folder can not be created";
