use arrayvec::ArrayString;
use binread::io::SeekFrom;
use binread::{BinRead, BinResult, ReadOptions};
use bitflags::bitflags;
use bv::BitVec;
use parse_display::Display;
use std::fmt;
use std::io::{Error, ErrorKind};
use std::iter::once;
use std::mem::{size_of, transmute};
use std::ops::Index;
use crate::mem::csgo::bsp::bspfile::LumpType;

#[derive(Clone)]
pub struct Directories {
    entries: [LumpEntry; 64],
}

impl BinRead for Directories {
    type Args = <LumpEntry as BinRead>::Args;

    fn read_options<R: binread::io::Read + binread::io::Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let mut entries = [LumpEntry::default(); 64];
        for i in 0..64 {
            entries[i] = LumpEntry::read_options(reader, options, args)?;
        }

        Ok(Directories { entries })
    }
}

impl Index<LumpType> for Directories {
    type Output = LumpEntry;

    fn index(&self, index: LumpType) -> &Self::Output {
        &self.entries[index as usize]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, BinRead)]
#[br(little)]
pub struct Header {
    pub v: u8,
    pub b: u8,
    pub s: u8,
    pub p: u8,
}

#[derive(Clone, Copy, Debug, Default, BinRead)]
#[br(little)]
pub struct LumpEntry {
    pub offset: u32,
    pub length: u32,
    pub version: u32,
    pub ident: u32,
}

#[derive(Debug, Clone, BinRead)]
pub struct LeafFace {
    pub face: u16,
}

#[derive(Clone)]
pub struct Entities {
    pub entities: String,
}

impl fmt::Debug for Entities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Debug)]
        struct Entities<'a> {
            entities: Vec<Entity<'a>>,
        }

        Entities {
            entities: self.iter().collect(),
        }
            .fmt(f)
    }
}

impl Entities {
    pub fn iter(&self) -> impl Iterator<Item = Entity<'_>> {
        struct Iter<'a> {
            buf: &'a str,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = Entity<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let start = self.buf.find('{')? + 1;
                let end = start + self.buf[start..].find('}')?;

                let out = &self.buf[start..end];

                self.buf = &self.buf[end + 1..];

                Some(Entity { buf: out })
            }
        }

        Iter {
            buf: &self.entities,
        }
    }
}

#[derive(Clone)]
pub struct Entity<'a> {
    buf: &'a str,
}

impl fmt::Debug for Entity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::collections::HashMap;

        self.properties().collect::<HashMap<_, _>>().fmt(f)
    }
}

impl<'a> Entity<'a> {
    pub fn properties(&self) -> impl Iterator<Item = (&'a str, &'a str)> {
        struct Iter<'a> {
            buf: &'a str,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = (&'a str, &'a str);

            fn next(&mut self) -> Option<Self::Item> {
                let start = self.buf.find('"')? + 1;
                let end = start + self.buf[start..].find('"')?;

                let key = &self.buf[start..end];

                let rest = &self.buf[end + 1..];

                let start = rest.find('"')? + 1;
                let end = start + rest[start..].find('"')?;

                let value = &rest[start..end];

                self.buf = &rest[end + 1..];

                Some((key, value))
            }
        }

        Iter { buf: &self.buf }
    }
}

bitflags! {
    #[derive(BinRead)]
    pub struct TextureFlags: u32 {
        const LIGHT      = 0b0000_0000_0000_0000_0001; // value will hold the light strength
        const SKY2D      = 0b0000_0000_0000_0000_0010; // don't draw, indicate we should skylight + draw 2d sky but don't draw the 3d skybox
        const SKY        = 0b0000_0000_0000_0000_0100; // don't draw, but add the skybox
        const WARP       = 0b0000_0000_0000_0000_1000; // turbulent water warp
        const TRANS      = 0b0000_0000_0000_0001_0000; // texture is translucent
        const NOPORTAL   = 0b0000_0000_0000_0010_0000; // the surface can't have a portal placed on it
        const TRIGGER    = 0b0000_0000_0000_0100_0000; // xbox hack to work around elimination of trigger surfaces
        const NODRAW     = 0b0000_0000_0000_1000_0000; // don't bother referencing the texture
        const HINT       = 0b0000_0000_0001_0000_0000; // make a primary bsp splitter
        const SKIP       = 0b0000_0000_0010_0000_0000; // completely ignore, allowing non-closed brushes
        const NOLIGHT    = 0b0000_0000_0100_0000_0000; // dont calculate light
        const BUMPLIGHT  = 0b0000_0000_1000_0000_0000; // calculate thee light maps for the surface for bump mapping
        const NOSHADOWS  = 0b0000_0001_0000_0000_0000; // don't receive shadows
        const NODECALS   = 0b0000_0010_0000_0000_0000; // don't receive decals
        const NOCHOP     = 0b0000_0100_0000_0000_0000; // don't subdivide patches on this surface
        const HITBOX     = 0b0000_1000_0000_0000_0000; // surface is part of a hitbox
    }
}

#[derive(Debug, Display, Clone)]
pub struct Name(ArrayString<[u8; 64]>);

impl BinRead for Name {
    type Args = ();

    fn read_options<R: binread::io::Read + binread::io::Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        use std::str;

        let mut name_buf: [u8; 64] = [0; 64];

        for i in 0..64 {
            name_buf[i] = u8::read_options(reader, options, args)?;
        }

        let zero_pos =
            name_buf
                .iter()
                .position(|c| *c == 0)
                .ok_or_else(|| binread::Error::AssertFail {
                    pos: reader.seek(SeekFrom::Current(0)).unwrap() as usize,
                    message: "Name not null terminated".to_string(),
                })?;
        let name = &name_buf[..zero_pos];
        Ok(Name(
            ArrayString::from(
                str::from_utf8(name).map_err(|err| Error::new(ErrorKind::InvalidData, err))?,
            )
                .expect(
                    "Programmer error: it should be impossible for the string to exceed the capacity",
                ),
        ))
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn iter(&self) -> impl Iterator<Item = f32> {
        once(self.x).chain(once(self.y)).chain(once(self.z))
    }

    pub fn dot(&self, other: Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl From<Vector> for [f32; 3] {
    fn from(vector: Vector) -> Self {
        [vector.x, vector.y, vector.z]
    }
}

impl From<&Vector> for [f32; 3] {
    fn from(vector: &Vector) -> Self {
        [vector.x, vector.y, vector.z]
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct TextureInfo {
    pub texture_scale: [f32; 4],
    pub texture_transform: [f32; 4],
    pub light_map_scale: [f32; 4],
    pub light_map_transform: [f32; 4],
    pub flags: TextureFlags,
    pub texture_data_index: i32,
}

static_assertions::const_assert_eq!(size_of::<TextureInfo>(), 72);

#[derive(Debug, Clone, BinRead)]
pub struct TextureData {
    pub reflectivity: Vector,
    pub name_string_table_id: i32,
    pub width: i32,
    pub height: i32,
    pub view_width: i32,
    pub view_height: i32,
}

#[derive(Debug, Clone, BinRead)]
pub struct Plane {
    pub normal: Vector,
    pub dist: f32,
    pub ty: i32,
}

#[derive(Debug, Clone, BinRead)]
pub struct Node {
    pub plane_index: i32,
    pub children: [i32; 2],
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_face: u16,
    pub face_cound: u16,
    pub area: i16,
    pub padding: i16,
}

static_assertions::const_assert_eq!(size_of::<Node>(), 32);

#[derive(Debug, Clone, BinRead)]
pub struct Leaf {
    pub contents: i32,
    pub cluster: i16,
    pub area_and_flags: i16,
    // first 9 bits is area, last 7 bits is flags
    pub mins: [i16; 3],
    pub maxs: [i16; 3],
    pub first_leaf_face: u16,
    pub leaf_face_count: u16,
    pub first_leaf_brush: u16,
    pub leaf_brush_count: u16,
    pub leaf_watter_data_id: i16,
}

static_assertions::const_assert_eq!(size_of::<Leaf>(), 32);

#[derive(Debug, Clone, BinRead)]
pub struct LeafBrush {
    pub brush: u16,
}

#[derive(Debug, Clone, BinRead)]
pub struct Model {
    pub mins: Vector,
    pub maxs: Vector,
    pub origin: Vector,
    pub head_node: i32,
    pub first_face: i32,
    pub face_count: i32,
}

static_assertions::const_assert_eq!(size_of::<Model>(), 48);

#[derive(Debug, Clone, BinRead, Copy)]
pub struct Brush {
    pub brush_side: u32,
    pub num_brush_sides: u32,
    pub texture: u32,
}

#[derive(Debug, Clone, BinRead)]
pub struct BrushSide {
    pub plane: u16,
    pub texture_info: i16,
    pub displacement_info: i16,
    pub bevel: i16,
}

#[derive(Debug, Clone, BinRead)]
pub struct Vertex {
    pub position: Vector,
}

#[derive(Debug, Clone, BinRead)]
pub struct Edge {
    pub start_index: u16,
    pub end_index: u16,
}

pub enum EdgeDirection {
    FirstToLast,
    LastToFirst,
}

#[derive(Debug, Clone, BinRead)]
pub struct SurfaceEdge {
    edge: i32,
}

impl SurfaceEdge {
    pub fn edge_index(&self) -> usize {
        self.edge.abs() as usize
    }

    pub fn direction(&self) -> EdgeDirection {
        if self.edge >= 0 {
            EdgeDirection::FirstToLast
        } else {
            EdgeDirection::LastToFirst
        }
    }
}

#[derive(Debug, Clone, BinRead)]
pub struct Face {
    pub plane_num: u16,
    pub side: u8,
    pub on_node: u8,
    pub first_edge: i32,
    pub num_edges: i16,
    pub texture_info: i16,
    pub displacement_info: i16,
    pub surface_fog_volume_id: i16,
    pub styles: [u8; 4],
    pub light_offset: i32,
    pub area: f32,
    pub light_map_texture_min: [i32; 2],
    pub light_map_texture_size: [i32; 2],
    pub original_face: i32,
    pub primitive_count: u16,
    pub first_primitive_index: u16,
    pub smoothing_groups: u32,
}

static_assertions::const_assert_eq!(size_of::<Face>(), 56);

#[derive(Default, Debug, Clone)]
pub struct VisData {
    pub cluster_count: u32,
    pub pvs_offsets: Vec<i32>,
    pub pas_offsets: Vec<i32>,
    pub data: Vec<u8>,
}

impl VisData {
    pub fn visible_clusters(&self, cluster: i16) -> BitVec<u8> {
        let offset = self.pvs_offsets[cluster as usize] as usize;
        let pvs_buffer = &self.data[offset..];
        let mut visible_clusters = BitVec::with_capacity(self.cluster_count as u64);
        visible_clusters.resize(self.cluster_count as u64, false);

        let mut cluster_index = 0;
        let mut buffer_index = 0;

        while cluster_index < self.cluster_count {
            if pvs_buffer[buffer_index] == 0 {
                let skip = pvs_buffer[buffer_index + 1];
                cluster_index += skip as u32;
                buffer_index += 2;
            } else {
                let packed = pvs_buffer[buffer_index];
                for i in 0..8 {
                    let bit = 1 << i;
                    if (packed & bit) == bit {
                        visible_clusters.set(cluster_index as u64, true);
                    }
                    cluster_index += 1;
                }
                buffer_index += 1;
            }
        }

        visible_clusters
    }
}