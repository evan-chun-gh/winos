/// Links an exported function from the specified library.
#[macro_export]
macro_rules! link {
    ($loc:literal, $($def:tt)*) => {
        #[allow(improper_ctypes)]
        #[link(name = $loc)]
        extern "system" { $($def)*; }
    };
}

/// Compile time conversion from &str to a null terminated cstring
#[macro_export]
macro_rules! s {
    ($s:literal) => {
        core::concat!($s, '\0').as_ptr()
    };
}

/// Compile time conversion from &str to a null terminated wstring
#[macro_export]
macro_rules! w {
    ($s:literal) => {{
        const S: &[u8] = $s.as_bytes();
        const W: &[u16] = {
            let mut buf = ['\0' as u16; S.len() + 1];
            let mut i = 0;

            while i < S.len() {
                buf[i] = S[i] as _;
                i += 1;
            }

            &{ buf }
        };

        W.as_ptr()
    }};
}
