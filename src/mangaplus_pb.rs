// Automatically generated rust module for 'mangaplus.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Response<'a> {
    pub result: Option<SuccessResult<'a>>,
    pub error: Option<ErrorResult<'a>>,
}

impl<'a> MessageRead<'a> for Response<'a> {
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

impl<'a> MessageWrite for Response<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SuccessResult<'a> {
    pub registration_data: Option<RegistrationData<'a>>,
    pub viewer: Option<MangaViewer<'a>>,
}

impl<'a> MessageRead<'a> for SuccessResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(18) => msg.registration_data = Some(r.read_message::<RegistrationData>(bytes)?),
                Ok(82) => msg.viewer = Some(r.read_message::<MangaViewer>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SuccessResult<'a> {
    fn get_size(&self) -> usize {
        0
        + self.registration_data.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.viewer.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.registration_data { w.write_with_tag(18, |w| w.write_message(s))?; }
        if let Some(ref s) = self.viewer { w.write_with_tag(82, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ErrorResult<'a> {
    pub action: i32,
    pub englishPopup: Option<OSDefault<'a>>,
    pub spanishPopup: Option<OSDefault<'a>>,
    pub popups: Vec<OSDefault<'a>>,
    pub debug: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for ErrorResult<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.action = r.read_int32(bytes)?,
                Ok(18) => msg.englishPopup = Some(r.read_message::<OSDefault>(bytes)?),
                Ok(26) => msg.spanishPopup = Some(r.read_message::<OSDefault>(bytes)?),
                Ok(42) => msg.popups.push(r.read_message::<OSDefault>(bytes)?),
                Ok(34) => msg.debug = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ErrorResult<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct RegistrationData<'a> {
    pub device_secret: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for RegistrationData<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.device_secret = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for RegistrationData<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.device_secret).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.device_secret))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MangaViewer<'a> {
    pub pages: Vec<Page<'a>>,
    pub chapter_id: Option<u32>,
    pub chapters: Vec<Chapter<'a>>,
    pub sns: Option<SNS<'a>>,
    pub title: Option<Cow<'a, str>>,
    pub chapter_name: Option<Cow<'a, str>>,
    pub num_comments: Option<u32>,
    pub vertical_only: Option<bool>,
    pub title_id: Option<u32>,
    pub start_from_right: Option<bool>,
    pub region: Option<Cow<'a, str>>,
    pub horizontal_only: Option<bool>,
}

impl<'a> MessageRead<'a> for MangaViewer<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.pages.push(r.read_message::<Page>(bytes)?),
                Ok(16) => msg.chapter_id = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.chapters.push(r.read_message::<Chapter>(bytes)?),
                Ok(34) => msg.sns = Some(r.read_message::<SNS>(bytes)?),
                Ok(42) => msg.title = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(50) => msg.chapter_name = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(56) => msg.num_comments = Some(r.read_uint32(bytes)?),
                Ok(64) => msg.vertical_only = Some(r.read_bool(bytes)?),
                Ok(72) => msg.title_id = Some(r.read_uint32(bytes)?),
                Ok(80) => msg.start_from_right = Some(r.read_bool(bytes)?),
                Ok(90) => msg.region = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(96) => msg.horizontal_only = Some(r.read_bool(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MangaViewer<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Chapter<'a> {
    pub title_id: u32,
    pub chapter_id: u32,
    pub name: Cow<'a, str>,
    pub subtitle: Cow<'a, str>,
    pub thumbail_url: Cow<'a, str>,
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub viewed: Option<bool>,
    pub viewed_free: Option<bool>,
    pub vertical_only: Option<bool>,
    pub horizontal_only: Option<bool>,
    pub chapter_ticket_end_time: Option<u32>,
}

impl<'a> MessageRead<'a> for Chapter<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.title_id = r.read_uint32(bytes)?,
                Ok(16) => msg.chapter_id = r.read_uint32(bytes)?,
                Ok(26) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.subtitle = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(42) => msg.thumbail_url = r.read_string(bytes).map(Cow::Borrowed)?,
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

impl<'a> MessageWrite for Chapter<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct SNS<'a> {
    pub body: Cow<'a, str>,
    pub url: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SNS<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SNS<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Page<'a> {
    pub page: Option<MangaPage<'a>>,
    pub banner_list: Option<BannerList<'a>>,
    pub insert_banner_list: Option<BannerList<'a>>,
    pub last_page: Option<LastPage<'a>>,
    pub ads: Option<AdNetworkList<'a>>,
}

impl<'a> MessageRead<'a> for Page<'a> {
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

impl<'a> MessageWrite for Page<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MangaPage<'a> {
    pub image_url: Option<Cow<'a, str>>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub type_pb: Option<i32>,
    pub encryption_key: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for MangaPage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(16) => msg.width = Some(r.read_uint32(bytes)?),
                Ok(24) => msg.height = Some(r.read_uint32(bytes)?),
                Ok(32) => msg.type_pb = Some(r.read_int32(bytes)?),
                Ok(42) => msg.encryption_key = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MangaPage<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct BannerList<'a> {
    pub title: Option<Cow<'a, str>>,
    pub banner: Vec<Banner<'a>>,
}

impl<'a> MessageRead<'a> for BannerList<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.title = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(18) => msg.banner.push(r.read_message::<Banner>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for BannerList<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Banner<'a> {
    pub image_url: Cow<'a, str>,
    pub action: TransitionAction<'a>,
    pub id: u32,
}

impl<'a> MessageRead<'a> for Banner<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.action = r.read_message::<TransitionAction>(bytes)?,
                Ok(24) => msg.id = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Banner<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct TransitionAction<'a> {
    pub method: i32,
    pub url: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for TransitionAction<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.method = r.read_int32(bytes)?,
                Ok(18) => msg.url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for TransitionAction<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.method) as u64)
        + 1 + sizeof_len((&self.url).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_int32(*&self.method))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.url))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LastPage<'a> {
    pub current_chapter: Chapter<'a>,
    pub next_chapter: Chapter<'a>,
    pub top_comments: Vec<Comment<'a>>,
    pub is_subscribed: Option<bool>,
    pub next_timestamp: Option<u32>,
    pub chapter_type: i32,
    pub ads: Option<AdNetworkList<'a>>,
    pub popup: Option<Popup<'a>>,
    pub banner: Vec<Banner<'a>>,
    pub titles: Vec<Title<'a>>,
    pub publisher_banner: Option<Banner<'a>>,
    pub user_tickets: Option<UserTickets>,
    pub is_next_chapter_read: Option<bool>,
    pub is_next_chapter_one_time_free: Option<bool>,
    pub free_view_dialogue: Option<FreeViewDialogue<'a>>,
}

impl<'a> MessageRead<'a> for LastPage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.current_chapter = r.read_message::<Chapter>(bytes)?,
                Ok(18) => msg.next_chapter = r.read_message::<Chapter>(bytes)?,
                Ok(26) => msg.top_comments.push(r.read_message::<Comment>(bytes)?),
                Ok(32) => msg.is_subscribed = Some(r.read_bool(bytes)?),
                Ok(40) => msg.next_timestamp = Some(r.read_uint32(bytes)?),
                Ok(48) => msg.chapter_type = r.read_int32(bytes)?,
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

impl<'a> MessageWrite for LastPage<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.current_chapter).get_size())
        + 1 + sizeof_len((&self.next_chapter).get_size())
        + self.top_comments.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
        + self.is_subscribed.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + self.next_timestamp.as_ref().map_or(0, |m| 1 + sizeof_varint(*(m) as u64))
        + 1 + sizeof_varint(*(&self.chapter_type) as u64)
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
        w.write_with_tag(18, |w| w.write_message(&self.next_chapter))?;
        for s in &self.top_comments { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.is_subscribed { w.write_with_tag(32, |w| w.write_bool(*s))?; }
        if let Some(ref s) = self.next_timestamp { w.write_with_tag(40, |w| w.write_uint32(*s))?; }
        w.write_with_tag(48, |w| w.write_int32(*&self.chapter_type))?;
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Comment<'a> {
    pub id: u32,
    pub index: Option<u32>,
    pub username: Cow<'a, str>,
    pub icon: Option<Cow<'a, str>>,
    pub is_my_comment: Option<bool>,
    pub liked: Option<bool>,
    pub likes: u32,
    pub body: Cow<'a, str>,
    pub created: u32,
}

impl<'a> MessageRead<'a> for Comment<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(16) => msg.index = Some(r.read_uint32(bytes)?),
                Ok(26) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.icon = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.is_my_comment = Some(r.read_bool(bytes)?),
                Ok(56) => msg.liked = Some(r.read_bool(bytes)?),
                Ok(72) => msg.likes = r.read_uint32(bytes)?,
                Ok(82) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(88) => msg.created = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Comment<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Popup<'a> {
    pub id: u32,
    pub os_def: OSDefault<'a>,
    pub app_def: AppDefault<'a>,
    pub movie_reward: MovieReward<'a>,
    pub one_image: OneImage<'a>,
}

impl<'a> MessageRead<'a> for Popup<'a> {
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

impl<'a> MessageWrite for Popup<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OSDefault<'a> {
    pub subject: Cow<'a, str>,
    pub body: Cow<'a, str>,
    pub ok_button: Option<Button<'a>>,
    pub neutral_button: Option<Button<'a>>,
    pub cancel_button: Option<Button<'a>>,
    pub language: i32,
}

impl<'a> MessageRead<'a> for OSDefault<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subject = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.ok_button = Some(r.read_message::<Button>(bytes)?),
                Ok(34) => msg.neutral_button = Some(r.read_message::<Button>(bytes)?),
                Ok(42) => msg.cancel_button = Some(r.read_message::<Button>(bytes)?),
                Ok(48) => msg.language = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OSDefault<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.subject).len())
        + 1 + sizeof_len((&self.body).len())
        + self.ok_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.neutral_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + self.cancel_button.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
        + 1 + sizeof_varint(*(&self.language) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.subject))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.body))?;
        if let Some(ref s) = self.ok_button { w.write_with_tag(26, |w| w.write_message(s))?; }
        if let Some(ref s) = self.neutral_button { w.write_with_tag(34, |w| w.write_message(s))?; }
        if let Some(ref s) = self.cancel_button { w.write_with_tag(42, |w| w.write_message(s))?; }
        w.write_with_tag(48, |w| w.write_int32(*&self.language))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Button<'a> {
    pub text: Cow<'a, str>,
    pub action: Option<TransitionAction<'a>>,
}

impl<'a> MessageRead<'a> for Button<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Button<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppDefault<'a> {
    pub subject: Cow<'a, str>,
    pub body: Cow<'a, str>,
    pub action: Option<TransitionAction<'a>>,
    pub image_url: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for AppDefault<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.subject = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.body = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(34) => msg.image_url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for AppDefault<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct MovieReward<'a> {
    pub image_url: Cow<'a, str>,
    pub ads: Option<AdNetworkList<'a>>,
}

impl<'a> MessageRead<'a> for MovieReward<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.image_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.ads = Some(r.read_message::<AdNetworkList>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MovieReward<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct OneImage<'a> {
    pub action: Option<TransitionAction<'a>>,
    pub image_url: Option<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for OneImage<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.action = Some(r.read_message::<TransitionAction>(bytes)?),
                Ok(18) => msg.image_url = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for OneImage<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Title<'a> {
    pub id: u32,
    pub name: Cow<'a, str>,
    pub author: Cow<'a, str>,
    pub portrait_image: Option<Cow<'a, str>>,
    pub landscape_image: Option<Cow<'a, str>>,
    pub views: u32,
    pub language: i32,
}

impl<'a> MessageRead<'a> for Title<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.id = r.read_uint32(bytes)?,
                Ok(18) => msg.name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.author = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.portrait_image = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(42) => msg.landscape_image = Some(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(48) => msg.views = r.read_uint32(bytes)?,
                Ok(56) => msg.language = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Title<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_varint(*(&self.id) as u64)
        + 1 + sizeof_len((&self.name).len())
        + 1 + sizeof_len((&self.author).len())
        + self.portrait_image.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + self.landscape_image.as_ref().map_or(0, |m| 1 + sizeof_len((m).len()))
        + 1 + sizeof_varint(*(&self.views) as u64)
        + 1 + sizeof_varint(*(&self.language) as u64)
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(8, |w| w.write_uint32(*&self.id))?;
        w.write_with_tag(18, |w| w.write_string(&**&self.name))?;
        w.write_with_tag(26, |w| w.write_string(&**&self.author))?;
        if let Some(ref s) = self.portrait_image { w.write_with_tag(34, |w| w.write_string(&**s))?; }
        if let Some(ref s) = self.landscape_image { w.write_with_tag(42, |w| w.write_string(&**s))?; }
        w.write_with_tag(48, |w| w.write_uint32(*&self.views))?;
        w.write_with_tag(56, |w| w.write_int32(*&self.language))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FreeViewDialogue<'a> {
    pub platform: i32,
    pub dialogue_url: Cow<'a, str>,
    pub publisher_banner: Banner<'a>,
}

impl<'a> MessageRead<'a> for FreeViewDialogue<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.platform = r.read_int32(bytes)?,
                Ok(18) => msg.dialogue_url = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.publisher_banner = r.read_message::<Banner>(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for FreeViewDialogue<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdNetworkList<'a> {
    pub ads: Vec<AdNetwork<'a>>,
}

impl<'a> MessageRead<'a> for AdNetworkList<'a> {
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

impl<'a> MessageWrite for AdNetworkList<'a> {
    fn get_size(&self) -> usize {
        0
        + self.ads.iter().map(|s| 1 + sizeof_len((s).get_size())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.ads { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct AdNetwork<'a> {
    pub fb: Option<mod_AdNetwork::Facebook<'a>>,
    pub admob: Option<mod_AdNetwork::Admob<'a>>,
    pub mopub: Option<mod_AdNetwork::Mopub<'a>>,
    pub adsense: Option<mod_AdNetwork::Adsense<'a>>,
    pub applovin: Option<mod_AdNetwork::Applovin<'a>>,
    pub applovin_max: Option<mod_AdNetwork::ApplovinMax<'a>>,
}

impl<'a> MessageRead<'a> for AdNetwork<'a> {
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

impl<'a> MessageWrite for AdNetwork<'a> {
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

pub mod mod_AdNetwork {

use std::borrow::Cow;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Facebook<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Facebook<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Facebook<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Admob<'a> {
    pub id: Cow<'a, str>,
    pub location: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Admob<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.location = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Admob<'a> {
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Mopub<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Mopub<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Mopub<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Adsense<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Adsense<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Adsense<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Applovin<'a> {
    pub id: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Applovin<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Applovin<'a> {
    fn get_size(&self) -> usize {
        0
        + 1 + sizeof_len((&self.id).len())
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        w.write_with_tag(10, |w| w.write_string(&**&self.id))?;
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ApplovinMax<'a> {
    pub id: Cow<'a, str>,
    pub type_pb: i32,
}

impl<'a> MessageRead<'a> for ApplovinMax<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.id = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(16) => msg.type_pb = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ApplovinMax<'a> {
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

}

