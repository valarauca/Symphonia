// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::audio::Channels;
use symphonia_core::audio::channels::layouts;
use symphonia_core::io::{BitReaderLtr, ReadBitsLtr};

/// Parse the `channelConfiguration` field from an AAC `AudioSpecificConfig` bitstream.
///
/// Returns `None` if the buffer is too short or the configuration is inband (value 0).
pub fn parse_aac_channel_config(extra_data: &[u8]) -> Option<usize> {
    let mut bs = BitReaderLtr::new(extra_data);

    // audioObjectType: 5 bits; if 31, read 6 more extension bits.
    let aot = bs.read_bits_leq32(5).ok()?;
    if aot == 31 {
        bs.read_bits_leq32(6).ok()?;
    }

    // samplingFrequencyIndex: 4 bits; if 0xf, skip 24-bit custom rate.
    let sfi = bs.read_bits_leq32(4).ok()?;
    if sfi == 0xf {
        bs.read_bits_leq32(24).ok()?;
    }

    // channelConfiguration: 4 bits.
    Some(bs.read_bits_leq32(4).ok()? as usize)
}

/// Map an AAC `channelConfiguration` value to a [`Channels`] layout.
///
/// Returns `None` for value 0 (inband/program config element) and any unrecognised value.
pub fn aac_channel_config_to_channels(channel_config: usize) -> Option<Channels> {
    match channel_config {
        1 => Some(layouts::CHANNEL_LAYOUT_MONO),
        2 => Some(layouts::CHANNEL_LAYOUT_STEREO),
        3 => Some(layouts::CHANNEL_LAYOUT_AAC_3P0),
        4 => Some(layouts::CHANNEL_LAYOUT_AAC_4P0),
        5 => Some(layouts::CHANNEL_LAYOUT_AAC_5P0),
        6 => Some(layouts::CHANNEL_LAYOUT_AAC_5P1),
        7 => Some(layouts::CHANNEL_LAYOUT_AAC_7P1),
        _ => None,
    }
}
