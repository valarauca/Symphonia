// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use symphonia_core::codecs::video::VideoExtraData;
use symphonia_core::codecs::video::well_known::CODEC_ID_AV1;
use symphonia_core::codecs::video::well_known::extra_data::VIDEO_EXTRA_DATA_ID_AV1_DECODER_CONFIG;
use symphonia_core::errors::{Result, decode_error};
use symphonia_core::io::ReadBytes;

use crate::atoms::stsd::VisualSampleEntry;
use crate::atoms::{Atom, AtomHeader};

const MAX_ATOM_SIZE: u64 = 4096;

#[derive(Debug)]
pub struct Av1CAtom {
    extra_data: VideoExtraData,
}

impl Atom for Av1CAtom {
    fn read<B: ReadBytes>(reader: &mut B, header: AtomHeader) -> Result<Self> {
        let len = match header.data_len() {
            Some(len) if len <= MAX_ATOM_SIZE => len as usize,
            Some(_) => return decode_error("isomp4 (av1C): atom size is greater than 4kb"),
            None => return decode_error("isomp4 (av1C): expected atom size to be known"),
        };

        let extra_data = VideoExtraData {
            id: VIDEO_EXTRA_DATA_ID_AV1_DECODER_CONFIG,
            data: reader.read_boxed_slice_exact(len)?,
        };

        Ok(Self { extra_data })
    }
}

impl Av1CAtom {
    pub fn fill_video_sample_entry(&self, entry: &mut VisualSampleEntry) {
        entry.codec_id = CODEC_ID_AV1;
        entry.extra_data.push(self.extra_data.clone());
    }
}
