
macro_rules! file_test {
    (
        module_name: $module_name: ident;
        input_file: $input_file_path: expr;

        audio_codec: $audio_codec: ident;
        $(audio_layout: $audio_channels: ident;)?
        $(audio_rate: $audio_sample_rate: expr;)?

        video_codec: $video_codec: ident;
        $(video_width: $video_width: expr;)?
        $(video_height: $video_height: expr;)?
    ) => {
        #[cfg(test)]
        mod $module_name {

            use symphonia::core::formats::{FormatOptions,TrackType};
            use symphonia::core::formats::probe::Hint;
            use symphonia::core::io::MediaSourceStream;
            use symphonia::core::meta::MetadataOptions;
            #[allow(unused_imports)] use symphonia_core::codecs::audio::well_known::*;
            #[allow(unused_imports)] use symphonia_core::codecs::video::{VideoCodecId, VideoCodecParameters, well_known::{*}};
            #[allow(unused_imports)] use symphonia_core::audio::channels::{Channels, layouts::{*}};

            use std::{
                io::Cursor,
                path::Path,
            };

            const DATA: &'static [u8] = include_bytes!($input_file_path);

            #[test]
            fn $module_name() {
                let path = Path::new($input_file_path);
                let cursor = std::io::Cursor::new(DATA);
                let mss = MediaSourceStream::new(Box::new(cursor), Default::default());
                let mut hint = Hint::new();
                if let Some(ext) = path.extension().map(|x| x.to_str()).flatten() {
                    hint.with_extension(ext);
                }
                let meta_opts: MetadataOptions = Default::default();
                let fmt_opts: FormatOptions = Default::default();
                let format = symphonia::default::get_probe()
                    .probe(&hint, mss, fmt_opts, meta_opts)
                    .expect("probe failed to identify file");

                let audio = format.default_track(TrackType::Audio).expect("expecting audio track");
                let audio_params = audio.codec_params.as_ref().expect("expecting codec params").audio().expect("expecting audo info");
                let audio_codec = audio_params.codec.clone();
                assert_eq!(audio_codec, $audio_codec);
                $(assert_eq!(audio_params.sample_rate.clone().expect("expecting sample rate"), $audio_sample_rate);)?
                $(assert_eq!(audio_params.channels.clone().expect("expecting channels"), $audio_channels);)?

                let video = format.default_track(TrackType::Video).expect("expecting video track");
                let video_params: &VideoCodecParameters = video.codec_params.as_ref().expect("expecting video codec").video().expect("expecting codec info");
                let video_codec: VideoCodecId = video_params.codec.clone();
                assert_eq!(video_codec, $video_codec);
                $(assert_eq!(video_params.width.clone().expect("should understand video width"), $video_width);)?
                $(assert_eq!(video_params.height.clone().expect("should understand video height"), $video_height);)?
            }
        }
    };
}


file_test! {
    module_name: libx264_aac;
    input_file: "video_data/test_libx264_aac_no_sub.mp4";

    audio_codec: CODEC_ID_AAC;
    audio_layout: CHANNEL_LAYOUT_STEREO;
    audio_rate: 48000;


    video_codec: CODEC_ID_H264;
    video_width: 480;
    video_height: 270;
}
