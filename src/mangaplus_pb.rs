// Automatically generated rust module for 'mangaplus_pb.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use core::convert::{TryFrom, TryInto};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response {
    pub result: Option<SuccessResult>,
    pub error: Option<ErrorResult>,
}

impl<'a> MessageRead<'a> for Response {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.result = Some(r.read_message::<SuccessResult>(bytes)?),
                Ok(18) => msg.error = Some(r.read_message::<ErrorResult>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Response {
    fn get_size(&self) -> usize {
        0
        + self.result.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.error.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.result { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.error { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Response {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Response::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SuccessResult {
    pub registration_data: Option<RegistrationData>,
    pub title_detail_view: Option<TitleDetailView>,
    pub viewer: Option<MangaViewer>,
}

impl<'a> MessageRead<'a> for SuccessResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.registration_data = Some(r.read_message::<RegistrationData>(bytes)?),
                Ok(66) => msg.title_detail_view = Some(r.read_message::<TitleDetailView>(bytes)?),
                Ok(82) => msg.viewer = Some(r.read_message::<MangaViewer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SuccessResult {
    fn get_size(&self) -> usize {
        0
        + self.registration_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.title_detail_view.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.viewer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.registration_data { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.title_detail_view { w.write_with_tag(66, |w| w.write_message(s))?; }
        if let Some(ref s) = self.viewer { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SuccessResult {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SuccessResult::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ErrorResult {
    pub action: i32,
    pub englishPopup: Option<OSDefault>,
    pub spanishPopup: Option<OSDefault>,
    pub popups: Vec<OSDefault>,
    pub debug: Option<String>,
}

impl<'a> MessageRead<'a> for ErrorResult {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.action = r.read_int32(bytes)?,
                Ok(18) => msg.englishPopup = Some(r.read_message::<OSDefault>(bytes)?),
                Ok(26) => msg.spanishPopup = Some(r.read_message::<OSDefault>(bytes)?),
                Ok(42) => msg.popups.push(r.read_message::<OSDefault>(bytes)?),
                Ok(34) => msg.debug = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ErrorResult {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.action) as u64)
        + self.englishPopup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.spanishPopup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.popups.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.debug.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.action))?;
        if let Some(ref s) = self.englishPopup { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.spanishPopup { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.popups { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.debug { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for ErrorResult {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(ErrorResult::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegistrationData {
    pub device_secret: String,
}

impl<'a> MessageRead<'a> for RegistrationData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device_secret = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RegistrationData {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.device_secret).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.device_secret))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for RegistrationData {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(RegistrationData::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ChapterListGroup {
    pub first_chapter_list: Vec<Chapter>,
    pub mid_chapter_list: Vec<Chapter>,
    pub last_chapter_list: Vec<Chapter>,
}

impl<'a> MessageRead<'a> for ChapterListGroup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.first_chapter_list.push(r.read_message::<Chapter>(bytes)?),
                Ok(26) => msg.mid_chapter_list.push(r.read_message::<Chapter>(bytes)?),
                Ok(34) => msg.last_chapter_list.push(r.read_message::<Chapter>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ChapterListGroup {
    fn get_size(&self) -> usize {
        0
        + self.first_chapter_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.mid_chapter_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.last_chapter_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.first_chapter_list { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.mid_chapter_list { w.write_with_tag(26, |w| w.write_message(s))?; }
        for s in &self.last_chapter_list { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for ChapterListGroup {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(ChapterListGroup::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TitleDetailView {
    pub title: Title,
    pub title_image_url: String,
    pub overview: String,
    pub background_image_url: String,
    pub next_timestamp: Option<u32>,
    pub update_timing: Option<i32>,
    pub viewing_period_description: Option<String>,
    pub non_appearance_info: Option<String>,
    pub first_chapter_list: Vec<Chapter>,
    pub last_chapter_list: Vec<Chapter>,
    pub banners: Vec<Banner>,
    pub recommended_title_list: Vec<Title>,
    pub sns: Option<SNS>,
    pub is_simul_released: bool,
    pub is_subscribed: Option<bool>,
    pub rating: Option<i32>,
    pub chapters_descending: Option<bool>,
    pub number_of_views: Option<u32>,
    pub chapter_list_group: ChapterListGroup,
}

impl<'a> MessageRead<'a> for TitleDetailView {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = r.read_message::<Title>(bytes)?,
                Ok(18) => msg.title_image_url = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.overview = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.background_image_url = r.read_string(bytes)?.to_owned(),
                Ok(40) => msg.next_timestamp = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.update_timing = Some(r.read_int32(bytes)?),
                Ok(58) => msg.viewing_period_description = Some(r.read_string(bytes)?.to_owned()),
                Ok(66) => msg.non_appearance_info = Some(r.read_string(bytes)?.to_owned()),
                Ok(74) => msg.first_chapter_list.push(r.read_message::<Chapter>(bytes)?),
                Ok(82) => msg.last_chapter_list.push(r.read_message::<Chapter>(bytes)?),
                Ok(90) => msg.banners.push(r.read_message::<Banner>(bytes)?),
                Ok(98) => msg.recommended_title_list.push(r.read_message::<Title>(bytes)?),
                Ok(106) => msg.sns = Some(r.read_message::<SNS>(bytes)?),
                Ok(112) => msg.is_simul_released = r.read_bool(bytes)?,
                Ok(120) => msg.is_subscribed = Some(r.read_bool(bytes)?),
                Ok(128) => msg.rating = Some(r.read_int32(bytes)?),
                Ok(136) => msg.chapters_descending = Some(r.read_bool(bytes)?),
                Ok(144) => msg.number_of_views = Some(r.read_uint32(bytes)?),
                Ok(226) => msg.chapter_list_group = r.read_message::<ChapterListGroup>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TitleDetailView {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.title).get_size())
        + 1 + sizeof_len((&self.title_image_url).len())
        + 1 + sizeof_len((&self.overview).len())
        + 1 + sizeof_len((&self.background_image_url).len())
        + self.next_timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.update_timing.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.viewing_period_description.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.non_appearance_info.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.first_chapter_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.last_chapter_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.banners.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.recommended_title_list.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.sns.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_varint(*(&self.is_simul_released) as u64)
        + self.is_subscribed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.rating.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.chapters_descending.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + self.number_of_views.as_ref().map_or(0, |m| 2 + sizeof_varint(*(m) as u64))
        + 2 + sizeof_len((&self.chapter_list_group).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.title))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.title_image_url))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.overview))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.background_image_url))?;
        if let Some(ref s) = self.next_timestamp { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.update_timing { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.viewing_period_description { w.write_with_tag(58, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.non_appearance_info { w.write_with_tag(66, |w| w.write_string(&**s))?; }
        for s in &self.first_chapter_list { w.write_with_tag(74, |w| w.write_message(s))?; }
        for s in &self.last_chapter_list { w.write_with_tag(82, |w| w.write_message(s))?; }
        for s in &self.banners { w.write_with_tag(90, |w| w.write_message(s))?; }
        for s in &self.recommended_title_list { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sns { w.write_with_tag(106, |w| w.write_message(s))?; }
        w.write_with_tag(112, |w| w.write_bool(*&self.is_simul_released))?;
        if let Some(ref s) = self.is_subscribed { w.write_with_tag(120, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.rating { w.write_with_tag(128, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.chapters_descending { w.write_with_tag(136, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.number_of_views { w.write_with_tag(144, |w| w.write_uint32(*s))?; }
        w.write_with_tag(226, |w| w.write_message(&self.chapter_list_group))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for TitleDetailView {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(TitleDetailView::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MangaViewer {
    pub pages: Vec<Page>,
    pub chapter_id: Option<u32>,
    pub chapters: Vec<Chapter>,
    pub sns: Option<SNS>,
    pub title: Option<String>,
    pub chapter_name: Option<String>,
    pub num_comments: Option<u32>,
    pub vertical_only: Option<bool>,
    pub title_id: Option<u32>,
    pub start_from_right: Option<bool>,
    pub region: Option<String>,
    pub horizontal_only: Option<bool>,
}

impl<'a> MessageRead<'a> for MangaViewer {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pages.push(r.read_message::<Page>(bytes)?),
                Ok(16) => msg.chapter_id = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.chapters.push(r.read_message::<Chapter>(bytes)?),
                Ok(34) => msg.sns = Some(r.read_message::<SNS>(bytes)?),
                Ok(42) => msg.title = Some(r.read_string(bytes)?.to_owned()),
                Ok(50) => msg.chapter_name = Some(r.read_string(bytes)?.to_owned()),
                Ok(56) => msg.num_comments = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.vertical_only = Some(r.read_bool(bytes)?),
                Ok(72) => msg.title_id = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.start_from_right = Some(r.read_bool(bytes)?),
                Ok(90) => msg.region = Some(r.read_string(bytes)?.to_owned()),
                Ok(96) => msg.horizontal_only = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MangaViewer {
    fn get_size(&self) -> usize {
        0
        + self.pages.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.chapter_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chapters.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.sns.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.chapter_name.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.num_comments.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vertical_only.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.title_id.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.start_from_right.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.region.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.horizontal_only.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.pages { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.chapter_id { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        for s in &self.chapters { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.sns { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.title { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.chapter_name { w.write_with_tag(50, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.num_comments { w.write_with_tag(56, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.vertical_only { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.title_id { w.write_with_tag(72, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.start_from_right { w.write_with_tag(80, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.region { w.write_with_tag(90, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.horizontal_only { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for MangaViewer {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(MangaViewer::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chapter {
    pub title_id: u32,
    pub chapter_id: u32,
    pub name: String,
    pub subtitle: String,
    pub thumbail_url: String,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub viewed: Option<bool>,
    pub viewed_free: Option<bool>,
    pub vertical_only: Option<bool>,
    pub horizontal_only: Option<bool>,
    pub chapter_ticket_end_time: Option<u32>,
}

impl<'a> MessageRead<'a> for Chapter {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.title_id = r.read_uint32(bytes)?,
                Ok(16) => msg.chapter_id = r.read_uint32(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.subtitle = r.read_string(bytes)?.to_owned(),
                Ok(42) => msg.thumbail_url = r.read_string(bytes)?.to_owned(),
                Ok(48) => msg.start_timestamp = r.read_uint32(bytes)?,
                Ok(56) => msg.end_timestamp = r.read_uint32(bytes)?,
                Ok(64) => msg.viewed = Some(r.read_bool(bytes)?),
                Ok(88) => msg.viewed_free = Some(r.read_bool(bytes)?),
                Ok(72) => msg.vertical_only = Some(r.read_bool(bytes)?),
                Ok(96) => msg.horizontal_only = Some(r.read_bool(bytes)?),
                Ok(80) => msg.chapter_ticket_end_time = Some(r.read_uint32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Chapter {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.title_id) as u64)
        + 1 + sizeof_varint(*(&self.chapter_id) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.subtitle).len())
        + 1 + sizeof_len((&self.thumbail_url).len())
        + 1 + sizeof_varint(*(&self.start_timestamp) as u64)
        + 1 + sizeof_varint(*(&self.end_timestamp) as u64)
        + self.viewed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.viewed_free.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.vertical_only.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.horizontal_only.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chapter_ticket_end_time.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.title_id))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.chapter_id))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(34, |w| w.write_string(&**&self.subtitle))?;
        w.write_with_tag(42, |w| w.write_string(&**&self.thumbail_url))?;
        w.write_with_tag(48, |w| w.write_uint32(*&self.start_timestamp))?;
        w.write_with_tag(56, |w| w.write_uint32(*&self.end_timestamp))?;
        if let Some(ref s) = self.viewed { w.write_with_tag(64, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.viewed_free { w.write_with_tag(88, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.vertical_only { w.write_with_tag(72, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.horizontal_only { w.write_with_tag(96, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.chapter_ticket_end_time { w.write_with_tag(80, |w| w.write_uint32(*s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Chapter {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Chapter::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SNS {
    pub body: String,
    pub url: String,
}

impl<'a> MessageRead<'a> for SNS {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.body = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.url = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SNS {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.body).len())
        + 1 + sizeof_len((&self.url).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.body))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.url))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for SNS {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(SNS::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Page {
    pub page: Option<MangaPage>,
    pub banner_list: Option<BannerList>,
    pub insert_banner_list: Option<BannerList>,
    pub last_page: Option<LastPage>,
    pub ads: Option<AdNetworkList>,
}

impl<'a> MessageRead<'a> for Page {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.page = Some(r.read_message::<MangaPage>(bytes)?),
                Ok(18) => msg.banner_list = Some(r.read_message::<BannerList>(bytes)?),
                Ok(42) => msg.insert_banner_list = Some(r.read_message::<BannerList>(bytes)?),
                Ok(26) => msg.last_page = Some(r.read_message::<LastPage>(bytes)?),
                Ok(34) => msg.ads = Some(r.read_message::<AdNetworkList>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Page {
    fn get_size(&self) -> usize {
        0
        + self.page.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.banner_list.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.insert_banner_list.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.last_page.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.ads.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.page { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.banner_list { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.insert_banner_list { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.last_page { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.ads { w.write_with_tag(34, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Page {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Page::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MangaPage {
    pub image_url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub type_pb: Option<i32>,
    pub encryption_key: Option<String>,
}

impl<'a> MessageRead<'a> for MangaPage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = Some(r.read_string(bytes)?.to_owned()),
                Ok(16) => msg.width = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.height = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.type_pb = Some(r.read_int32(bytes)?),
                Ok(42) => msg.encryption_key = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MangaPage {
    fn get_size(&self) -> usize {
        0
        + self.image_url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.width.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.height.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.type_pb.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.encryption_key.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.image_url { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.width { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.height { w.write_with_tag(24, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.type_pb { w.write_with_tag(32, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.encryption_key { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for MangaPage {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(MangaPage::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BannerList {
    pub title: Option<String>,
    pub banner: Vec<Banner>,
}

impl<'a> MessageRead<'a> for BannerList {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = Some(r.read_string(bytes)?.to_owned()),
                Ok(18) => msg.banner.push(r.read_message::<Banner>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for BannerList {
    fn get_size(&self) -> usize {
        0
        + self.title.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.banner.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.title { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        for s in &self.banner { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for BannerList {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(BannerList::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Banner {
    pub image_url: String,
    pub action: TransitionAction,
    pub id: u32,
}

impl<'a> MessageRead<'a> for Banner {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.action = r.read_message::<TransitionAction>(bytes)?,
                Ok(24) => msg.id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Banner {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.image_url).len())
        + 1 + sizeof_len((&self.action).get_size())
        + 1 + sizeof_varint(*(&self.id) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.image_url))?;
        w.write_with_tag(18, |w| w.write_message(&self.action))?;
        w.write_with_tag(24, |w| w.write_uint32(*&self.id))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Banner {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Banner::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransitionAction {
    pub method: Option<i32>,
    pub url: Option<String>,
}

impl<'a> MessageRead<'a> for TransitionAction {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.method = Some(r.read_int32(bytes)?),
                Ok(18) => msg.url = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TransitionAction {
    fn get_size(&self) -> usize {
        0
        + self.method.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.method { w.write_with_tag(8, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.url { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for TransitionAction {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(TransitionAction::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LastPage {
    pub current_chapter: Chapter,
    pub next_chapter: Option<Chapter>,
    pub top_comments: Vec<Comment>,
    pub is_subscribed: Option<bool>,
    pub next_timestamp: Option<u32>,
    pub chapter_type: Option<i32>,
    pub ads: Option<AdNetworkList>,
    pub popup: Option<Popup>,
    pub banner: Vec<Banner>,
    pub titles: Vec<Title>,
    pub publisher_banner: Option<Banner>,
    pub user_tickets: Option<UserTickets>,
    pub is_next_chapter_read: Option<bool>,
    pub is_next_chapter_one_time_free: Option<bool>,
    pub free_view_dialogue: Option<FreeViewDialogue>,
}

impl<'a> MessageRead<'a> for LastPage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.current_chapter = r.read_message::<Chapter>(bytes)?,
                Ok(18) => msg.next_chapter = Some(r.read_message::<Chapter>(bytes)?),
                Ok(26) => msg.top_comments.push(r.read_message::<Comment>(bytes)?),
                Ok(32) => msg.is_subscribed = Some(r.read_bool(bytes)?),
                Ok(40) => msg.next_timestamp = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.chapter_type = Some(r.read_int32(bytes)?),
                Ok(58) => msg.ads = Some(r.read_message::<AdNetworkList>(bytes)?),
                Ok(66) => msg.popup = Some(r.read_message::<Popup>(bytes)?),
                Ok(74) => msg.banner.push(r.read_message::<Banner>(bytes)?),
                Ok(82) => msg.titles.push(r.read_message::<Title>(bytes)?),
                Ok(90) => msg.publisher_banner = Some(r.read_message::<Banner>(bytes)?),
                Ok(98) => msg.user_tickets = Some(r.read_message::<UserTickets>(bytes)?),
                Ok(104) => msg.is_next_chapter_read = Some(r.read_bool(bytes)?),
                Ok(112) => msg.is_next_chapter_one_time_free = Some(r.read_bool(bytes)?),
                Ok(122) => msg.free_view_dialogue = Some(r.read_message::<FreeViewDialogue>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LastPage {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.current_chapter).get_size())
        + self.next_chapter.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.top_comments.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.is_subscribed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.next_timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.chapter_type.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.ads.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.popup.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.banner.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.titles.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.publisher_banner.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.user_tickets.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.is_next_chapter_read.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.is_next_chapter_one_time_free.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.free_view_dialogue.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_message(&self.current_chapter))?;
        if let Some(ref s) = self.next_chapter { w.write_with_tag(18, |w| w.write_message(s))?; }
        for s in &self.top_comments { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.is_subscribed { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.next_timestamp { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        if let Some(ref s) = self.chapter_type { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        if let Some(ref s) = self.ads { w.write_with_tag(58, |w| w.write_message(s))?; }
        if let Some(ref s) = self.popup { w.write_with_tag(66, |w| w.write_message(s))?; }
        for s in &self.banner { w.write_with_tag(74, |w| w.write_message(s))?; }
        for s in &self.titles { w.write_with_tag(82, |w| w.write_message(s))?; }
        if let Some(ref s) = self.publisher_banner { w.write_with_tag(90, |w| w.write_message(s))?; }
        if let Some(ref s) = self.user_tickets { w.write_with_tag(98, |w| w.write_message(s))?; }
        if let Some(ref s) = self.is_next_chapter_read { w.write_with_tag(104, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.is_next_chapter_one_time_free { w.write_with_tag(112, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.free_view_dialogue { w.write_with_tag(122, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for LastPage {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(LastPage::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Comment {
    pub id: u32,
    pub index: Option<u32>,
    pub username: String,
    pub icon: Option<String>,
    pub is_my_comment: Option<bool>,
    pub liked: Option<bool>,
    pub likes: u32,
    pub body: String,
    pub created: u32,
}

impl<'a> MessageRead<'a> for Comment {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(16) => msg.index = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.username = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.icon = Some(r.read_string(bytes)?.to_owned()),
                Ok(48) => msg.is_my_comment = Some(r.read_bool(bytes)?),
                Ok(56) => msg.liked = Some(r.read_bool(bytes)?),
                Ok(72) => msg.likes = r.read_uint32(bytes)?,
                Ok(82) => msg.body = r.read_string(bytes)?.to_owned(),
                Ok(88) => msg.created = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Comment {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + self.index.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_len((&self.username).len())
        + self.icon.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.is_my_comment.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.liked.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_varint(*(&self.likes) as u64)
        + 1 + sizeof_len((&self.body).len())
        + 1 + sizeof_varint(*(&self.created) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        if let Some(ref s) = self.index { w.write_with_tag(16, |w| w.write_uint32(*s))?; }
        w.write_with_tag(26, |w| w.write_string(&**&self.username))?;
        if let Some(ref s) = self.icon { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.is_my_comment { w.write_with_tag(48, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.liked { w.write_with_tag(56, |w| w.write_bool(*s))?; }
        w.write_with_tag(72, |w| w.write_uint32(*&self.likes))?;
        w.write_with_tag(82, |w| w.write_string(&**&self.body))?;
        w.write_with_tag(88, |w| w.write_uint32(*&self.created))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Comment {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Comment::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Popup {
    pub id: u32,
    pub os_def: OSDefault,
    pub app_def: AppDefault,
    pub movie_reward: MovieReward,
    pub one_image: OneImage,
}

impl<'a> MessageRead<'a> for Popup {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(40) => msg.id = r.read_uint32(bytes)?,
                Ok(10) => msg.os_def = r.read_message::<OSDefault>(bytes)?,
                Ok(18) => msg.app_def = r.read_message::<AppDefault>(bytes)?,
                Ok(26) => msg.movie_reward = r.read_message::<MovieReward>(bytes)?,
                Ok(34) => msg.one_image = r.read_message::<OneImage>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Popup {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.os_def).get_size())
        + 1 + sizeof_len((&self.app_def).get_size())
        + 1 + sizeof_len((&self.movie_reward).get_size())
        + 1 + sizeof_len((&self.one_image).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(40, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(10, |w| w.write_message(&self.os_def))?;
        w.write_with_tag(18, |w| w.write_message(&self.app_def))?;
        w.write_with_tag(26, |w| w.write_message(&self.movie_reward))?;
        w.write_with_tag(34, |w| w.write_message(&self.one_image))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Popup {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Popup::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OSDefault {
    pub subject: String,
    pub body: String,
    pub ok_button: Option<Button>,
    pub neutral_button: Option<Button>,
    pub cancel_button: Option<Button>,
    pub language: Option<i32>,
}

impl<'a> MessageRead<'a> for OSDefault {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subject = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.body = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.ok_button = Some(r.read_message::<Button>(bytes)?),
                Ok(34) => msg.neutral_button = Some(r.read_message::<Button>(bytes)?),
                Ok(42) => msg.cancel_button = Some(r.read_message::<Button>(bytes)?),
                Ok(48) => msg.language = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OSDefault {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.subject).len())
        + 1 + sizeof_len((&self.body).len())
        + self.ok_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.neutral_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.cancel_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.language.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.subject))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.body))?;
        if let Some(ref s) = self.ok_button { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.neutral_button { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cancel_button { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.language { w.write_with_tag(48, |w| w.write_int32(*s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for OSDefault {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(OSDefault::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Button {
    pub text: String,
    pub action: Option<TransitionAction>,
}

impl<'a> MessageRead<'a> for Button {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Button {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.text).len())
        + self.action.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.text))?;
        if let Some(ref s) = self.action { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Button {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Button::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppDefault {
    pub subject: String,
    pub body: String,
    pub action: Option<TransitionAction>,
    pub image_url: Option<String>,
}

impl<'a> MessageRead<'a> for AppDefault {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subject = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.body = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(34) => msg.image_url = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AppDefault {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.subject).len())
        + 1 + sizeof_len((&self.body).len())
        + self.action.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.image_url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.subject))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.body))?;
        if let Some(ref s) = self.action { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.image_url { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for AppDefault {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(AppDefault::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MovieReward {
    pub image_url: String,
    pub ads: Option<AdNetworkList>,
}

impl<'a> MessageRead<'a> for MovieReward {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.ads = Some(r.read_message::<AdNetworkList>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MovieReward {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.image_url).len())
        + self.ads.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.image_url))?;
        if let Some(ref s) = self.ads { w.write_with_tag(18, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for MovieReward {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(MovieReward::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OneImage {
    pub action: Option<TransitionAction>,
    pub image_url: Option<String>,
}

impl<'a> MessageRead<'a> for OneImage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(18) => msg.image_url = Some(r.read_string(bytes)?.to_owned()),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for OneImage {
    fn get_size(&self) -> usize {
        0
        + self.action.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.image_url.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.action { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.image_url { w.write_with_tag(18, |w| w.write_string(&**s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for OneImage {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(OneImage::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UserTickets {
    pub current_tickets: u32,
    pub next_ticket_timestamp: u32,
}

impl<'a> MessageRead<'a> for UserTickets {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.current_tickets = r.read_uint32(bytes)?,
                Ok(16) => msg.next_ticket_timestamp = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for UserTickets {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.current_tickets) as u64)
        + 1 + sizeof_varint(*(&self.next_ticket_timestamp) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.current_tickets))?;
        w.write_with_tag(16, |w| w.write_uint32(*&self.next_ticket_timestamp))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for UserTickets {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(UserTickets::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Title {
    pub id: u32,
    pub name: String,
    pub author: String,
    pub portrait_image: Option<String>,
    pub landscape_image: Option<String>,
    pub views: u32,
    pub language: Option<i32>,
}

impl<'a> MessageRead<'a> for Title {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.author = r.read_string(bytes)?.to_owned(),
                Ok(34) => msg.portrait_image = Some(r.read_string(bytes)?.to_owned()),
                Ok(42) => msg.landscape_image = Some(r.read_string(bytes)?.to_owned()),
                Ok(48) => msg.views = r.read_uint32(bytes)?,
                Ok(56) => msg.language = Some(r.read_int32(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Title {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.author).len())
        + self.portrait_image.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.landscape_image.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.views) as u64)
        + self.language.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.author))?;
        if let Some(ref s) = self.portrait_image { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.landscape_image { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        w.write_with_tag(48, |w| w.write_uint32(*&self.views))?;
        if let Some(ref s) = self.language { w.write_with_tag(56, |w| w.write_int32(*s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Title {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Title::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FreeViewDialogue {
    pub platform: i32,
    pub dialogue_url: String,
    pub publisher_banner: Banner,
}

impl<'a> MessageRead<'a> for FreeViewDialogue {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.platform = r.read_int32(bytes)?,
                Ok(18) => msg.dialogue_url = r.read_string(bytes)?.to_owned(),
                Ok(26) => msg.publisher_banner = r.read_message::<Banner>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FreeViewDialogue {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.platform) as u64)
        + 1 + sizeof_len((&self.dialogue_url).len())
        + 1 + sizeof_len((&self.publisher_banner).get_size())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.platform))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.dialogue_url))?;
        w.write_with_tag(26, |w| w.write_message(&self.publisher_banner))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for FreeViewDialogue {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(FreeViewDialogue::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdNetworkList {
    pub ads: Vec<AdNetwork>,
}

impl<'a> MessageRead<'a> for AdNetworkList {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.ads.push(r.read_message::<AdNetwork>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AdNetworkList {
    fn get_size(&self) -> usize {
        0
        + self.ads.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ads { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for AdNetworkList {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(AdNetworkList::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdNetwork {
    pub fb: Option<mod_AdNetwork::Facebook>,
    pub admob: Option<mod_AdNetwork::Admob>,
    pub mopub: Option<mod_AdNetwork::Mopub>,
    pub adsense: Option<mod_AdNetwork::Adsense>,
    pub applovin: Option<mod_AdNetwork::Applovin>,
    pub applovin_max: Option<mod_AdNetwork::ApplovinMax>,
}

impl<'a> MessageRead<'a> for AdNetwork {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.fb = Some(r.read_message::<mod_AdNetwork::Facebook>(bytes)?),
                Ok(18) => msg.admob = Some(r.read_message::<mod_AdNetwork::Admob>(bytes)?),
                Ok(26) => msg.mopub = Some(r.read_message::<mod_AdNetwork::Mopub>(bytes)?),
                Ok(34) => msg.adsense = Some(r.read_message::<mod_AdNetwork::Adsense>(bytes)?),
                Ok(42) => msg.applovin = Some(r.read_message::<mod_AdNetwork::Applovin>(bytes)?),
                Ok(50) => msg.applovin_max = Some(r.read_message::<mod_AdNetwork::ApplovinMax>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for AdNetwork {
    fn get_size(&self) -> usize {
        0
        + self.fb.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.admob.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.mopub.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.adsense.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applovin.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.applovin_max.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.fb { w.write_with_tag(10, |w| w.write_message(s))?; }
        if let Some(ref s) = self.admob { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.mopub { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.adsense { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applovin { w.write_with_tag(42, |w| w.write_message(s))?; }
        if let Some(ref s) = self.applovin_max { w.write_with_tag(50, |w| w.write_message(s))?; }
        Ok(())
    }
}


            impl TryFrom<&[u8]> for AdNetwork {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(AdNetwork::from_reader(&mut reader, &buf)?)
                }
            }
            
pub mod mod_AdNetwork {

use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Facebook {
    pub id: String,
}

impl<'a> MessageRead<'a> for Facebook {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Facebook {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Facebook {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Facebook::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Admob {
    pub id: String,
    pub location: String,
}

impl<'a> MessageRead<'a> for Admob {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(18) => msg.location = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Admob {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_len((&self.location).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.location))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Admob {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Admob::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mopub {
    pub id: String,
}

impl<'a> MessageRead<'a> for Mopub {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Mopub {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Mopub {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Mopub::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Adsense {
    pub id: String,
}

impl<'a> MessageRead<'a> for Adsense {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Adsense {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Adsense {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Adsense::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Applovin {
    pub id: String,
}

impl<'a> MessageRead<'a> for Applovin {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Applovin {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for Applovin {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(Applovin::from_reader(&mut reader, &buf)?)
                }
            }
            
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplovinMax {
    pub id: String,
    pub type_pb: i32,
}

impl<'a> MessageRead<'a> for ApplovinMax {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes)?.to_owned(),
                Ok(16) => msg.type_pb = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ApplovinMax {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
        + 1 + sizeof_varint(*(&self.type_pb) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        w.write_with_tag(16, |w| w.write_int32(*&self.type_pb))?;
        Ok(())
    }
}


            impl TryFrom<&[u8]> for ApplovinMax {
                type Error=quick_protobuf::Error;

                fn try_from(buf: &[u8]) -> Result<Self> {
                    let mut reader = BytesReader::from_bytes(&buf);
                    Ok(ApplovinMax::from_reader(&mut reader, &buf)?)
                }
            }
            
}

