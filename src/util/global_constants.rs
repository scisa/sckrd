// cli
pub const APP_NAME: &str = "sckrd";
pub const APP_VERSION: &str = "0.1.0";
pub const APP_AUTHOR: &str = "scisa <scisa@web.de>";
pub const APP_DESCRIPTION: &str = "search crypto keys in ram dumps";


// calculation
pub const ENTROPY_BORDER: f32 = 4.60;

//output
pub const OUTPUT_FILE_PATH: &str = "output.sckrd";

// chars never in hashes
pub const BYTE_NEWLINE: u8 = 10; //\n
pub const BYTE_SPACE: u8 = 32; //
pub const BYTE_EXCLAMATION_MARK: u8 = 33; //\!
pub const BYTE_DOLLAR: u8 = 36; //$
pub const BYTE_ASTERISK: u8 = 42; //*
pub const BYTE_COLON: u8 = 58; //:
pub const BYTE_SEMICOLON: u8 = 59; //;
pub const BYTE_BACKSLASH: u8 = 92; //\\

pub const EXCLUDES: [u8; 8] = [
    BYTE_NEWLINE,
    BYTE_SPACE,
    BYTE_EXCLAMATION_MARK,
    BYTE_DOLLAR,
    BYTE_ASTERISK,
    BYTE_COLON,
    BYTE_SEMICOLON,
    BYTE_BACKSLASH,
];
