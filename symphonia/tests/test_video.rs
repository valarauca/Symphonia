
macro_rules! file_test {
    (
        module_name: $module_name: ident;
        input_file: $input_file_path: expr;
    ) => {
        #[cfg(test)]
        mod $module_name {

            use symphonia::core::formats::FormatOptions;
            use symphonia::core::formats::probe::Hint;
            use symphonia::core::io::MediaSourceStream;
            use symphonia::core::meta::MetadataOptions;
            use symphonia_core::formats::TrackType;

            use std::{
                io::Cursor,
                path::Path,
            };

            const DATA: &'static [u8] = include_bytes!($input_file_path);

            #[test]
            fn $module_name() {
                let path = Path::new($input_file_path);
                let cursor = std::io::Cursor::new(DATA);
                let mut mss = MediaSourceStream::new(Box::new(cursor), Default::default());
                let mut hint = Hint::new();
                if let Some(ext) = path.extension().map(|x| x.to_str()).flatten() {
                    hint.with_extension(ext);
                }
                let meta_opts: MetadataOptions = Default::default();
                let fmt_opts: FormatOptions = Default::default();
                let mut _format = symphonia::default::get_probe()
                    .probe(&hint, mss, fmt_opts, meta_opts)
                    .unwrap();
            }
        }
    };
}


file_test! {
    module_name: libx264_aac;
    input_file: "video_data/test_libx264_aac_no_sub.mp4";
}
