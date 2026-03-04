
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
    module_name: libx264_aac_mp4;
    input_file: "video_data/test_libx264_aac_no_sub.mp4";

    audio_codec: CODEC_ID_AAC;
    audio_layout: CHANNEL_LAYOUT_STEREO;
    audio_rate: 48000;


    video_codec: CODEC_ID_H264;
    video_width: 480;
    video_height: 270;
}

file_test! {
    module_name: libx264_aac_mkv;
    input_file: "video_data/test_libx264_aac_no_sub.mkv";

    audio_codec: CODEC_ID_AAC;
    audio_layout: CHANNEL_LAYOUT_STEREO;
    audio_rate: 48000;


    video_codec: CODEC_ID_H264;
    video_width: 480;
    video_height: 270;
}

file_test! {
    module_name: av1_flac_mkv;
    input_file: "video_data/test_libaom-av1_flac_no_sub.mkv";

    audio_codec: CODEC_ID_FLAC;
    audio_layout: CHANNEL_LAYOUT_STEREO;
    audio_rate: 48000;


    video_codec: CODEC_ID_AV1;
    video_width: 480;
    video_height: 270;
}

file_test! { module_name: av1_aac_mp4; input_file: "video_data/test_libaom-av1_aac_no_sub.mp4"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_ac3_mkv; input_file: "video_data/test_libaom-av1_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_ac3_mp4; input_file: "video_data/test_libaom-av1_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_alac_mkv; input_file: "video_data/test_libaom-av1_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_alac_mp4; input_file: "video_data/test_libaom-av1_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_eac3_mkv; input_file: "video_data/test_libaom-av1_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_eac3_mp4; input_file: "video_data/test_libaom-av1_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_flac_mp4; input_file: "video_data/test_libaom-av1_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_mp3_mkv; input_file: "video_data/test_libaom-av1_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_mp3_mp4; input_file: "video_data/test_libaom-av1_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_opus_mkv; input_file: "video_data/test_libaom-av1_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_opus_mp4; input_file: "video_data/test_libaom-av1_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_vorbis_mkv; input_file: "video_data/test_libaom-av1_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_pcm_s16le_mkv; input_file: "video_data/test_libaom-av1_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: av1_wavpack_mkv; input_file: "video_data/test_libaom-av1_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_AV1; video_width: 480; video_height: 270; }
file_test! { module_name: theora_aac_mkv; input_file: "video_data/test_libtheora_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_ac3_mkv; input_file: "video_data/test_libtheora_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_alac_mkv; input_file: "video_data/test_libtheora_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_eac3_mkv; input_file: "video_data/test_libtheora_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_flac_mkv; input_file: "video_data/test_libtheora_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_flac_ogg; input_file: "video_data/test_libtheora_flac.ogg"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_mp3_mkv; input_file: "video_data/test_libtheora_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_opus_mkv; input_file: "video_data/test_libtheora_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_opus_ogg; input_file: "video_data/test_libtheora_libopus.ogg"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_speex_ogg; input_file: "video_data/test_libtheora_libspeex.ogg"; audio_codec: CODEC_ID_SPEEX; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 32000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_vorbis_mkv; input_file: "video_data/test_libtheora_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_vorbis_ogg; input_file: "video_data/test_libtheora_libvorbis.ogg"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_pcm_s16le_mkv; input_file: "video_data/test_libtheora_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: theora_wavpack_mkv; input_file: "video_data/test_libtheora_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_THEORA; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_aac_mkv; input_file: "video_data/test_libvpx-vp9_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_ac3_mkv; input_file: "video_data/test_libvpx-vp9_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_alac_mkv; input_file: "video_data/test_libvpx-vp9_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_eac3_mkv; input_file: "video_data/test_libvpx-vp9_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_flac_mkv; input_file: "video_data/test_libvpx-vp9_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_mp3_mkv; input_file: "video_data/test_libvpx-vp9_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_opus_mkv; input_file: "video_data/test_libvpx-vp9_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_vorbis_mkv; input_file: "video_data/test_libvpx-vp9_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_pcm_s16le_mkv; input_file: "video_data/test_libvpx-vp9_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp9_wavpack_mkv; input_file: "video_data/test_libvpx-vp9_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP9; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_aac_mkv; input_file: "video_data/test_libvpx_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_ac3_mkv; input_file: "video_data/test_libvpx_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_alac_mkv; input_file: "video_data/test_libvpx_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_eac3_mkv; input_file: "video_data/test_libvpx_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_flac_mkv; input_file: "video_data/test_libvpx_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_mp3_mkv; input_file: "video_data/test_libvpx_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_opus_mkv; input_file: "video_data/test_libvpx_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_vorbis_mkv; input_file: "video_data/test_libvpx_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_pcm_s16le_mkv; input_file: "video_data/test_libvpx_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: vp8_wavpack_mkv; input_file: "video_data/test_libvpx_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_VP8; video_width: 480; video_height: 270; }
file_test! { module_name: h264_ac3_mkv; input_file: "video_data/test_libx264_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_ac3_mp4; input_file: "video_data/test_libx264_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_alac_mkv; input_file: "video_data/test_libx264_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_alac_mp4; input_file: "video_data/test_libx264_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_eac3_mkv; input_file: "video_data/test_libx264_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_eac3_mp4; input_file: "video_data/test_libx264_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_flac_mkv; input_file: "video_data/test_libx264_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_flac_mp4; input_file: "video_data/test_libx264_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_mp3_mkv; input_file: "video_data/test_libx264_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_mp3_mp4; input_file: "video_data/test_libx264_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_opus_mkv; input_file: "video_data/test_libx264_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_opus_mp4; input_file: "video_data/test_libx264_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_vorbis_mkv; input_file: "video_data/test_libx264_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_pcm_s16le_mkv; input_file: "video_data/test_libx264_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: h264_wavpack_mkv; input_file: "video_data/test_libx264_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_H264; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_aac_mkv; input_file: "video_data/test_libx265_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_aac_mp4; input_file: "video_data/test_libx265_aac_no_sub.mp4"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_ac3_mkv; input_file: "video_data/test_libx265_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_ac3_mp4; input_file: "video_data/test_libx265_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_alac_mkv; input_file: "video_data/test_libx265_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_alac_mp4; input_file: "video_data/test_libx265_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_eac3_mkv; input_file: "video_data/test_libx265_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_eac3_mp4; input_file: "video_data/test_libx265_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_flac_mkv; input_file: "video_data/test_libx265_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_flac_mp4; input_file: "video_data/test_libx265_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_mp3_mkv; input_file: "video_data/test_libx265_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_mp3_mp4; input_file: "video_data/test_libx265_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_opus_mkv; input_file: "video_data/test_libx265_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_opus_mp4; input_file: "video_data/test_libx265_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_vorbis_mkv; input_file: "video_data/test_libx265_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_pcm_s16le_mkv; input_file: "video_data/test_libx265_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: hevc_wavpack_mkv; input_file: "video_data/test_libx265_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_HEVC; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_aac_mkv; input_file: "video_data/test_mjpeg_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_aac_mp4; input_file: "video_data/test_mjpeg_aac_no_sub.mp4"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_ac3_mkv; input_file: "video_data/test_mjpeg_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_ac3_mp4; input_file: "video_data/test_mjpeg_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_alac_mkv; input_file: "video_data/test_mjpeg_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_alac_mp4; input_file: "video_data/test_mjpeg_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_eac3_mkv; input_file: "video_data/test_mjpeg_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_eac3_mp4; input_file: "video_data/test_mjpeg_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_flac_mkv; input_file: "video_data/test_mjpeg_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_flac_mp4; input_file: "video_data/test_mjpeg_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_mp3_mkv; input_file: "video_data/test_mjpeg_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_mp3_mp4; input_file: "video_data/test_mjpeg_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_opus_mkv; input_file: "video_data/test_mjpeg_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_opus_mp4; input_file: "video_data/test_mjpeg_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_vorbis_mkv; input_file: "video_data/test_mjpeg_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_pcm_s16le_mkv; input_file: "video_data/test_mjpeg_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mjpeg_wavpack_mkv; input_file: "video_data/test_mjpeg_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MJPEG; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_aac_mkv; input_file: "video_data/test_mpeg1video_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_ac3_mkv; input_file: "video_data/test_mpeg1video_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_alac_mkv; input_file: "video_data/test_mpeg1video_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_eac3_mkv; input_file: "video_data/test_mpeg1video_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_flac_mkv; input_file: "video_data/test_mpeg1video_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_mp3_mkv; input_file: "video_data/test_mpeg1video_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_opus_mkv; input_file: "video_data/test_mpeg1video_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_vorbis_mkv; input_file: "video_data/test_mpeg1video_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_pcm_s16le_mkv; input_file: "video_data/test_mpeg1video_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg1_wavpack_mkv; input_file: "video_data/test_mpeg1video_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG1; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_aac_mkv; input_file: "video_data/test_mpeg2video_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_aac_mp4; input_file: "video_data/test_mpeg2video_aac_no_sub.mp4"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_ac3_mkv; input_file: "video_data/test_mpeg2video_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_ac3_mp4; input_file: "video_data/test_mpeg2video_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_alac_mkv; input_file: "video_data/test_mpeg2video_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_alac_mp4; input_file: "video_data/test_mpeg2video_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_eac3_mkv; input_file: "video_data/test_mpeg2video_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_eac3_mp4; input_file: "video_data/test_mpeg2video_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_flac_mkv; input_file: "video_data/test_mpeg2video_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_flac_mp4; input_file: "video_data/test_mpeg2video_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_mp3_mkv; input_file: "video_data/test_mpeg2video_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_mp3_mp4; input_file: "video_data/test_mpeg2video_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_opus_mkv; input_file: "video_data/test_mpeg2video_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_opus_mp4; input_file: "video_data/test_mpeg2video_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_vorbis_mkv; input_file: "video_data/test_mpeg2video_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_pcm_s16le_mkv; input_file: "video_data/test_mpeg2video_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg2_wavpack_mkv; input_file: "video_data/test_mpeg2video_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG2; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_aac_mkv; input_file: "video_data/test_mpeg4_aac_no_sub.mkv"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_aac_mp4; input_file: "video_data/test_mpeg4_aac_no_sub.mp4"; audio_codec: CODEC_ID_AAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_ac3_mkv; input_file: "video_data/test_mpeg4_ac3_no_sub.mkv"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_ac3_mp4; input_file: "video_data/test_mpeg4_ac3_no_sub.mp4"; audio_codec: CODEC_ID_AC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_alac_mkv; input_file: "video_data/test_mpeg4_alac_no_sub.mkv"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_alac_mp4; input_file: "video_data/test_mpeg4_alac_no_sub.mp4"; audio_codec: CODEC_ID_ALAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_eac3_mkv; input_file: "video_data/test_mpeg4_eac3_no_sub.mkv"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_eac3_mp4; input_file: "video_data/test_mpeg4_eac3_no_sub.mp4"; audio_codec: CODEC_ID_EAC3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_flac_mkv; input_file: "video_data/test_mpeg4_flac_no_sub.mkv"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_flac_mp4; input_file: "video_data/test_mpeg4_flac_no_sub.mp4"; audio_codec: CODEC_ID_FLAC; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_mp3_mkv; input_file: "video_data/test_mpeg4_libmp3lame_no_sub.mkv"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_mp3_mp4; input_file: "video_data/test_mpeg4_libmp3lame_no_sub.mp4"; audio_codec: CODEC_ID_MP3; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_opus_mkv; input_file: "video_data/test_mpeg4_libopus_no_sub.mkv"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_opus_mp4; input_file: "video_data/test_mpeg4_libopus_no_sub.mp4"; audio_codec: CODEC_ID_OPUS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_vorbis_mkv; input_file: "video_data/test_mpeg4_libvorbis_no_sub.mkv"; audio_codec: CODEC_ID_VORBIS; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_pcm_s16le_mkv; input_file: "video_data/test_mpeg4_pcm_s16le_no_sub.mkv"; audio_codec: CODEC_ID_PCM_S16LE; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
file_test! { module_name: mpeg4_wavpack_mkv; input_file: "video_data/test_mpeg4_wavpack_no_sub.mkv"; audio_codec: CODEC_ID_WAVPACK; audio_layout: CHANNEL_LAYOUT_STEREO; audio_rate: 48000; video_codec: CODEC_ID_MPEG4; video_width: 480; video_height: 270; }
