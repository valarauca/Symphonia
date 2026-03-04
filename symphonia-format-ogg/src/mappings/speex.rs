// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::SideData;

use super::{MapResult, Mapper, PacketParser};

use symphonia_core::audio::{Channels, Position};
use symphonia_core::codecs::CodecParameters;
use symphonia_core::codecs::audio::AudioCodecParameters;
use symphonia_core::codecs::audio::well_known::CODEC_ID_SPEEX;
use symphonia_core::errors::Result;
use symphonia_core::formats::Track;
use symphonia_core::io::{BufReader, ReadBytes};
use symphonia_core::units::Duration;

/// Minimum size of the Speex identification header packet.
const SPEEX_IDENT_PACKET_SIZE: usize = 80;

/// The Speex magic signature (8 bytes: "Speex   ").
const SPEEX_SIGNATURE: &[u8] = b"Speex   ";

pub fn detect(serial: u32, buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    if buf.len() < SPEEX_IDENT_PACKET_SIZE {
        return Ok(None);
    }

    let mut reader = BufReader::new(buf);

    let mut magic = [0u8; 8];
    reader.read_buf_exact(&mut magic)?;

    if magic != *SPEEX_SIGNATURE {
        return Ok(None);
    }

    // Skip speex_version string (20 bytes) + speex_version_id (4 bytes) + header_size (4 bytes).
    reader.ignore_bytes(28)?;

    let sample_rate = reader.read_u32()?;

    // Skip mode (4 bytes) + mode_bitstream_version (4 bytes).
    reader.ignore_bytes(8)?;

    let nb_channels = reader.read_u32()?;

    let channels = match nb_channels {
        1 => Channels::Positioned(Position::FRONT_LEFT),
        2 => Channels::Positioned(Position::FRONT_LEFT | Position::FRONT_RIGHT),
        _ => return Ok(None),
    };

    let mut codec_params = AudioCodecParameters::new();

    codec_params
        .for_codec(CODEC_ID_SPEEX)
        .with_sample_rate(sample_rate)
        .with_channels(channels)
        .with_extra_data(Box::from(buf));

    let mut track = Track::new(serial);

    track.with_codec_params(CodecParameters::Audio(codec_params));

    Ok(Some(Box::new(SpeexMapper { track, need_comment: true })))
}

struct SpeexMapper {
    track: Track,
    need_comment: bool,
}

impl Mapper for SpeexMapper {
    fn name(&self) -> &'static str {
        "speex"
    }

    fn reset(&mut self) {}

    fn track(&self) -> &Track {
        &self.track
    }

    fn track_mut(&mut self) -> &mut Track {
        &mut self.track
    }

    fn make_parser(&self) -> Option<Box<dyn PacketParser>> {
        None
    }

    fn map_packet(&mut self, _packet: &[u8]) -> Result<MapResult> {
        if self.need_comment {
            self.need_comment = false;
            return Ok(MapResult::Setup);
        }

        Ok(MapResult::StreamData { dur: Duration::ZERO, discard: Duration::ZERO })
    }
}
