#!/bin/bash
SCRIPT_DIR="$(cd "$( dirname "${BASH_SOURCE[0]}" )" &>/dev/null && pwd -P)"

hash ffmpeg &>/dev/null || {
    echo "cannot locate ffmpeg, it does not appear to be within \$PATH" >&2
    exit 1
}

function isomp4_test_data() {
    declare -a mp4_video_codec=( libx264 libx265 mpeg4 mpeg2video mjpeg libaom-av1 )
    declare -a mp4_audio_codec=( aac libmp3lame ac3 eac3 alac flac libopus )

    for vc in "${mp4_video_codec[@]}"; do
        for ac in "${mp4_audio_codec[@]}"; do
            local output="test_${vc}_${ac}_no_sub.mp4"
            local -a ffmpeg_args=( -i "${1}" -map 0:v:0 -map 0:a:0 -c:v "${vc}" -c:a "${ac}" -strict -2 "${output}" )
            ffmpeg "${ffmpeg_args[@]}" || {
				echo "failed to encode file '${output}'";
				return 1;
			}
        done
    done
}

function mkv_test_data() {
    declare -a mkv_video_codec=( libx264 libx265 mpeg1video mpeg2video mpeg4 mjpeg libaom-av1 libvpx libvpx-vp9 libtheora )
    declare -a mkv_audio_codec=( aac libmp3lame libvorbis libopus flac ac3 eac3 alac pcm_s16le wavpack )

    for vc in "${mkv_video_codec[@]}"; do
        for ac in "${mkv_audio_codec[@]}"; do
            local output="test_${vc}_${ac}_no_sub.mkv"
            local -a ffmpeg_args=( -i "${1}" -map 0:v:0 -map 0:a:0 -c:v "${vc}" -c:a "${ac}" "${output}" )
            ffmpeg "${ffmpeg_args[@]}" || {
				echo "failed to encode file '${output}'";
				return 1;
			};
        done
    done
}

function ogg_test_data() {
    declare -a ogg_video_codec=( libtheora )
    declare -a ogg_audio_codec=( libvorbis libopus flac libspeex )

    for vc in "${ogg_video_codec[@]}"; do
        for ac in "${ogg_audio_codec[@]}"; do
            local output="test_${vc}_${ac}.ogg"
            local -a ffmpeg_args=( -i "${1}" -map 0:v:0 -map 0:a:0 -c:v "${vc}" -c:a "${ac}" "${output}" )
            ffmpeg "${ffmpeg_args[@]}" || {
				echo "failed to encode file '${output}'";
				return 1;
			};
        done
    done
}

function main() {
    if [[ ! -d "${SCRIPT_DIR}" ]]; then
        echo "could not resolve local directory" >&2;
        return 1;
    fi

    local test_file="${SCRIPT_DIR}/test.mp4"

	isomp4_test_data "${test_file}" || {
        echo "failed to generate mp4 test data" >&2;
        return 1;
    };

	mkv_test_data "${test_file}" || {
        echo "failed to generate mkv test data" >&2;
        return 1;
    };

	ogg_test_data "${test_file}" || {
        echo "failed to generate ogg test data" >&2;
        return 1;
    };
}

main
