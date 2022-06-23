// cli
pub const APP_NAME: &str = "sckrd";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "scisa <scisa@web.de>";
pub const APP_DESCRIPTION: &str = "search crypto keys in ram dumps";

// parallelism
pub const MINIMAL_THREAD_VALUE: usize = 1;

// calculation
pub const DEFAULT_ENTROPY_DELTA: f32 = 0.2;
pub const SMALLEST_KEY_LENGTH_BIT: usize = 56;
pub const GREATEST_KEY_LENGTH_BIT: usize = 1024;

//output
pub const OUTPUT_FILE_PATH: &str = "./sckrd.keys";
