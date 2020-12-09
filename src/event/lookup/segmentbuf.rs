use crate::event::lookup::Segment;
use std::fmt::{Display, Formatter};

/// `SegmentBuf`s are chunks of a `LookupBuf`.
///
/// They represent either a field or an index. A sequence of `SegmentBuf`s can become a `LookupBuf`.
///
/// This is the owned, allocated side of a `Segement` for `LookupBuf.` It owns its fields unlike `Lookup`. Think of `String` to `&str` or `PathBuf` to `Path`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum SegmentBuf {
    Field {
        name: String,
        // This is a very lazy optimization to avoid having to scan for escapes.
        requires_quoting: bool
    },
    Index(usize),
    Coalesce(Vec<Self>),
}

impl SegmentBuf {
    pub const fn field(name: String, requires_quoting: bool) -> SegmentBuf {
        SegmentBuf::Field { name, requires_quoting }
    }

    pub fn is_field(&self) -> bool {
        matches!(self, SegmentBuf::Field { name: _, requires_quoting: _ })
    }

    pub const fn index(v: usize) -> SegmentBuf {
        SegmentBuf::Index(v)
    }

    pub fn is_index(&self) -> bool {
        matches!(self, SegmentBuf::Index(_))
    }

    pub const fn coalesce(v: Vec<Self>) -> SegmentBuf {
        SegmentBuf::Coalesce(v)
    }

    pub fn is_coalesce(&self) -> bool {
        matches!(self, SegmentBuf::Coalesce(_))
    }


    #[instrument]
    pub(crate) fn as_segment<'a>(&'a self) -> Segment<'a> {
        match self {
            SegmentBuf::Field { name, requires_quoting} => Segment::field(name.as_str(), *requires_quoting),
            SegmentBuf::Index(i) => Segment::index(*i),
            SegmentBuf::Coalesce(v) => Segment::coalesce(v.iter().map(|v| v.as_segment()).collect()),
        }
    }
}

impl Display for SegmentBuf {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SegmentBuf::Index(i) => write!(formatter, "{}", i),
            SegmentBuf::Field { name, requires_quoting: false } => write!(formatter, "{}", name),
            SegmentBuf::Field { name, requires_quoting: true } => write!(formatter, "\"{}\"", name),
            SegmentBuf::Coalesce(v) => write!(formatter, "{}", v.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" | ")),
        }
    }
}

impl From<String> for SegmentBuf {
    fn from(mut name: String) -> Self {
        let requires_quoting = name.starts_with("\"");
        if requires_quoting {
            // There is unfortunately not way to make an owned substring of a string.
            // So we have to take a slice and clone it.
            let len = name.len();
            name = name[1..len - 1].to_string();

        }
        Self::Field { name, requires_quoting }
    }
}

impl From<usize> for SegmentBuf {
    fn from(value: usize) -> Self {
        Self::index(value)
    }
}

impl From<Vec<SegmentBuf>> for SegmentBuf {
    fn from(value: Vec<SegmentBuf>) -> Self {
        Self::coalesce(value)
    }
}

impl<'a> From<Segment<'a>> for SegmentBuf {
    fn from(value: Segment<'a>) -> Self {
        value.as_segment_buf()
    }
}
