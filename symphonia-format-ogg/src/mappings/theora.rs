// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::common::SideData;

use super::{MapResult, Mapper, PacketParser};

use symphonia_core::codecs::CodecParameters;
use symphonia_core::codecs::video::VideoCodecParameters;
use symphonia_core::codecs::video::well_known::CODEC_ID_THEORA;
use symphonia_core::errors::Result;
use symphonia_core::formats::Track;
use symphonia_core::io::{BufReader, ReadBytes};
use symphonia_core::units::Duration;

/// Minimum size of the Theora identification header packet.
const THEORA_IDENT_PACKET_SIZE: usize = 42;

/// Packet type byte for the Theora identification header.
const THEORA_PACKET_TYPE_IDENT: u8 = 0x80;

/// Packet type byte for the Theora comment header.
const THEORA_PACKET_TYPE_COMMENT: u8 = 0x81;

/// Bit mask to distinguish header packets (MSB set) from data packets.
const THEORA_HEADER_PACKET_MASK: u8 = 0x80;

/// The "theora" signature bytes that follow the packet type byte.
const THEORA_SIGNATURE: &[u8] = b"theora";

pub fn detect(serial: u32, buf: &[u8]) -> Result<Option<Box<dyn Mapper>>> {
    if buf.len() < THEORA_IDENT_PACKET_SIZE {
        return Ok(None);
    }

    let mut reader = BufReader::new(buf);

    if reader.read_u8()? != THEORA_PACKET_TYPE_IDENT {
        return Ok(None);
    }

    let mut sig = [0u8; 6];
    reader.read_buf_exact(&mut sig)?;

    if sig != *THEORA_SIGNATURE {
        return Ok(None);
    }

    let vmaj = reader.read_u8()?;
    let vmin = reader.read_u8()?;

    if vmaj != 3 || vmin != 2 {
        return Ok(None);
    }

    let _vrev = reader.read_u8()?;

    let _fmbw = reader.read_be_u16()?;
    let _fmbh = reader.read_be_u16()?;

    let picw = (u32::from(reader.read_u8()?) << 16)
        | (u32::from(reader.read_u8()?) << 8)
        | u32::from(reader.read_u8()?);

    let pich = (u32::from(reader.read_u8()?) << 16)
        | (u32::from(reader.read_u8()?) << 8)
        | u32::from(reader.read_u8()?);

    let mut codec_params = VideoCodecParameters::default();

    codec_params
        .for_codec(CODEC_ID_THEORA)
        .with_width(picw as u16)
        .with_height(pich as u16);

    let mut track = Track::new(serial);

    track.with_codec_params(CodecParameters::Video(codec_params));

    Ok(Some(Box::new(TheoraMapper { track, headers_remaining: 2 })))
}

struct TheoraMapper {
    track: Track,
    headers_remaining: u8,
}

impl Mapper for TheoraMapper {
    fn name(&self) -> &'static str {
        "theora"
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

    fn is_ready(&self) -> bool {
        self.headers_remaining == 0
    }

    fn map_packet(&mut self, packet: &[u8]) -> Result<MapResult> {
        let packet_type = match packet.first() {
            Some(&b) => b,
            None => return Ok(MapResult::Unknown),
        };

        if packet_type & THEORA_HEADER_PACKET_MASK != 0 {
            if self.headers_remaining > 0 {
                self.headers_remaining -= 1;
            }

            if packet_type == THEORA_PACKET_TYPE_COMMENT {
                // Comment header — metadata, but not parsed here.
                return Ok(MapResult::Setup);
            }

            Ok(MapResult::Setup)
        }
        else {
            Ok(MapResult::StreamData { dur: Duration::ZERO, discard: Duration::ZERO })
        }
    }
}
