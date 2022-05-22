use phf::phf_map;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RecordType {
    value: u16,
}

impl FromStr for RecordType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::TYPE_NAMES
            .get(value)
            .map(|&value| Ok(Self { value }))
            .unwrap_or(Err(()))
    }
}

impl RecordType {
    const TYPE_NAMES: phf::Map<&'static str, u16> = phf_map! {
        "A" => 1,
        "NS" => 2,
        "MD" => 3,
        "MF" => 4,
        "CNAME" => 5,
        "SOA" => 6,
        "MB" => 7,
        "MG" => 8,
        "MR" => 9,
        "NULL" => 10,
        "WKS" => 11,
        "PTR" => 12,
        "HINFO" => 13,
        "MINFO" => 14,
        "MX" => 15,
        "TXT" => 16,
        "RP" => 17,
        "AFSDB" => 18,
        "X25" => 19,
        "ISDN" => 20,
        "RT" => 21,
        "NSAP" => 22,
        "NSAP-PTR" => 23,
        "SIG" => 24,
        "KEY" => 25,
        "PX" => 26,
        "GPOS" => 27,
        "AAAA" => 28,
        "LOC" => 29,
        "NXT" => 30,
        "EID" => 31,
        "NIMLOC" => 32,
        "SRV" => 33,
        "ATMA" => 34,
        "NAPTR" => 35,
        "KX" => 36,
        "CERT" => 37,
        "A6" => 38,
        "DNAME" => 39,
        "SINK" => 40,
        "OPT" => 41,
        "APL" => 42,
        "DS" => 43,
        "SSHFP" => 44,
        "IPSECKEY" => 45,
        "RRSIG" => 46,
        "NSEC" => 47,
        "DNSKEY" => 48,
        "DHCID" => 49,
        "NSEC3" => 50,
        "NSEC3PARAM" => 51,
        "TLSA" => 52,
        "SMIMEA" => 53,
        "Unassigned" => 54,
        "HIP" => 55,
        "NINFO" => 56,
        "RKEY" => 57,
        "TALINK" => 58,
        "CDS" => 59,
        "CDNSKEY" => 60,
        "OPENPGPKEY" => 61,
        "CSYNC" => 62,
        "ZONEMD" => 63,
        "SVCB" => 64,
        "HTTPS" => 65,
        "SPF" => 99,
        "UINFO" => 100,
        "UID" => 101,
        "GID" => 102,
        "UNSPEC" => 103,
        "NID" => 104,
        "L32" => 105,
        "L64" => 106,
        "LP" => 107,
        "EUI48" => 108,
        "EUI64" => 109,
        "TKEY" => 249,
        "TSIG" => 250,
        "IXFR" => 251,
        "AXFR" => 252,
        "MAILB" => 253,
        "MAILA" => 254,
        "*" => 255,
        "URI" => 256,
        "CAA" => 257,
        "AVC" => 258,
        "DOA" => 259,
        "AMTRELAY" => 260,
        "TA" => 32768,
        "DLV" => 32769,
    };

    const A: Self = Self::new(1);
    const AAAA: Self = Self::new(28);
    const CNAME: Self = Self::new(5);

    pub const fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Display for RecordType {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let name = match self.value {
            0 => "Reserved",
            1 => "A",
            2 => "NS",
            3 => "MD",
            4 => "MF",
            5 => "CNAME",
            6 => "SOA",
            7 => "MB",
            8 => "MG",
            9 => "MR",
            10 => "NULL",
            11 => "WKS",
            12 => "PTR",
            13 => "HINFO",
            14 => "MINFO",
            15 => "MX",
            16 => "TXT",
            17 => "RP",
            18 => "AFSDB",
            19 => "X25",
            20 => "ISDN",
            21 => "RT",
            22 => "NSAP",
            23 => "NSAP-PTR",
            24 => "SIG",
            25 => "KEY",
            26 => "PX",
            27 => "GPOS",
            28 => "AAAA",
            29 => "LOC",
            30 => "NXT",
            31 => "EID",
            32 => "NIMLOC",
            33 => "SRV",
            34 => "ATMA",
            35 => "NAPTR",
            36 => "KX",
            37 => "CERT",
            38 => "A6",
            39 => "DNAME",
            40 => "SINK",
            41 => "OPT",
            42 => "APL",
            43 => "DS",
            44 => "SSHFP",
            45 => "IPSECKEY",
            46 => "RRSIG",
            47 => "NSEC",
            48 => "DNSKEY",
            49 => "DHCID",
            50 => "NSEC3",
            51 => "NSEC3PARAM",
            52 => "TLSA",
            53 => "SMIMEA",
            54 => "Unassigned",
            55 => "HIP",
            56 => "NINFO",
            57 => "RKEY",
            58 => "TALINK",
            59 => "CDS",
            60 => "CDNSKEY",
            61 => "OPENPGPKEY",
            62 => "CSYNC",
            63 => "ZONEMD",
            64 => "SVCB",
            65 => "HTTPS",
            66..=98 => "Unassigned",
            99 => "SPF",
            100 => "UINFO",
            101 => "UID",
            102 => "GID",
            103 => "UNSPEC",
            104 => "NID",
            105 => "L32",
            106 => "L64",
            107 => "LP",
            108 => "EUI48",
            109 => "EUI64",
            110..=248 => "Unassigned",
            249 => "TKEY",
            250 => "TSIG",
            251 => "IXFR",
            252 => "AXFR",
            253 => "MAILB",
            254 => "MAILA",
            255 => "*",
            256 => "URI",
            257 => "CAA",
            258 => "AVC",
            259 => "DOA",
            260 => "AMTRELAY",
            261..=32767 => "Unassigned",
            32768 => "TA",
            32769 => "DLV",
            32770..=65279 => "Unassigned",
            65280..=65534 => "Private use",
            65535 => "Reserved",
        };
        write!(fmt, "{} ({})", name, self.value)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RecordClass {
    value: u16,
}

impl RecordClass {
    fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Display for RecordClass {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let memo = match self.value {
            0 => "Reserved",
            1 => "Internet (IN)",
            2 => "Unassigned",
            3 => "Chaos (CH)",
            4 => "Hesiod (HS)",
            5..=253 => "Unassigned",
            254 => "QCLASS NONE",
            255 => "QCLASS * (ANY)",
            256..=65279 => "Unassigned",
            65280..=65534 => "Reserved for Private Use",
            65535 => "Reserved",
        };
        write!(fmt, "{} ({})", memo, self.value)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Name {
    name: Vec<u8>,
}

impl Name {
    // Individual domain names must be parsed from the full payload of the DNS message, in order to
    // support compressed labels referencing other names in the message
    fn parse(bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        let mut name = Vec::with_capacity(64);

        loop {
            if *cursor >= bytes.len() {
                return Err(ParseError::Truncated);
            }

            let byte = bytes[*cursor];

            if byte == 0 {
                *cursor += 1;
                return Ok(Self { name });
            }

            match byte >> 6 {
                0 => {
                    if !name.is_empty() {
                        name.push(b'.');
                    }

                    if *cursor + 1 + (byte as usize) > bytes.len() {
                        return Err(ParseError::Truncated);
                    }

                    name.extend(&bytes[(*cursor + 1)..(*cursor + 1 + (byte as usize))]);
                    *cursor += 1 + (byte as usize);
                }
                3 => {
                    if *cursor + 2 > bytes.len() {
                        return Err(ParseError::Truncated);
                    }

                    let pointer = (((byte ^ 0b11000000) as u16) << 8) | (bytes[*cursor + 1] as u16);

                    // Only expand compressed labels pointing backwards in the message, in order to
                    // prevent infinite recursion
                    if (pointer as usize) >= *cursor {
                        return Err(ParseError::Invalid);
                    }

                    let tail = {
                        let mut pointer_cursor = pointer as usize;
                        Self::parse(bytes, &mut pointer_cursor)
                    }?;

                    *cursor += 2;

                    name.extend(&tail.name);

                    return Ok(Self { name });
                }
                _ => return Err(ParseError::Invalid),
            }
        }
    }
}

impl FromStr for Name {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: name.as_bytes().to_vec(),
        })
    }
}

impl Display for Name {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        for &byte in self.name.iter() {
            if byte.is_ascii() && !byte.is_ascii_control() {
                write!(fmt, "{}", byte as char)?;
            } else {
                write!(fmt, "\\x{:02x}", byte)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Ttl {
    seconds: u32,
}

impl Ttl {
    fn new(seconds: u32) -> Self {
        Self { seconds }
    }
}

impl Display for Ttl {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let mut seconds = self.seconds;

        let days = seconds / 86400;
        seconds %= 86400;
        if days != 0 {
            write!(fmt, "{}d", days)?;
        }

        let hours = seconds / 3600;
        seconds %= 3600;
        if hours != 0 {
            write!(fmt, "{}h", hours)?;
        }

        let minutes = seconds / 60;
        seconds %= 60;
        if minutes != 0 {
            write!(fmt, "{}m", minutes)?;
        }

        if seconds != 0 || days + hours + minutes == 0 {
            write!(fmt, "{}s", seconds)?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Rdata {
    A { ip: Ipv4Addr },
    Aaaa { ip: Ipv6Addr },
    Cname { name: Name },
    Other { data: Vec<u8> },
}

impl Rdata {
    fn parse(type_: RecordType, bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        if *cursor + 2 > bytes.len() {
            println!("wtf lmao");
            return Err(ParseError::Truncated);
        }

        let len = ((bytes[*cursor] as usize) << 8) | (bytes[*cursor + 1] as usize);
        *cursor += 2;

        if *cursor + len > bytes.len() {
            println!("{} {} {}", *cursor, len, type_);
            return Err(ParseError::Truncated);
        }

        let rdata = match type_ {
            RecordType::A => {
                if len != 4 {
                    return Err(ParseError::Invalid);
                }

                let ip = Ipv4Addr::new(
                    bytes[*cursor],
                    bytes[*cursor + 1],
                    bytes[*cursor + 2],
                    bytes[*cursor + 3],
                );
                Self::A { ip }
            }
            RecordType::AAAA => {
                if len != 16 {
                    return Err(ParseError::Invalid);
                }

                let mut raw_ip = [0; 16];
                raw_ip[0..16].clone_from_slice(&bytes[*cursor..*cursor + 16]);
                let ip = Ipv6Addr::from(raw_ip);
                Self::Aaaa { ip }
            }
            RecordType::CNAME => {
                let mut name_cursor = *cursor;
                let name = Name::parse(&bytes[0..*cursor + len], &mut name_cursor)?;

                if name_cursor < *cursor + len {
                    return Err(ParseError::Extra);
                }

                Self::Cname { name }
            }
            _ => {
                let data = bytes[*cursor..*cursor + len].to_vec();
                Self::Other { data }
            }
        };

        *cursor += len;
        Ok(rdata)
    }
}

impl Display for Rdata {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            Self::A { ip } => {
                write!(fmt, "{}", ip)?;
            }
            Self::Aaaa { ip } => {
                write!(fmt, "{}", ip)?;
            }
            Self::Cname { name } => {
                write!(fmt, "{}", name)?;
            }
            Self::Other { data } => {
                for &byte in data.iter() {
                    if byte.is_ascii() && !byte.is_ascii_control() {
                        write!(fmt, "{}", byte as char)?;
                    } else {
                        write!(fmt, "\\x{:02x}", byte)?;
                    }
                }
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Question {
    name: Name,
    type_: RecordType,
    class: RecordClass,
}

impl Question {
    fn parse(bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        let name = Name::parse(bytes, cursor)?;

        if *cursor + 4 > bytes.len() {
            return Err(ParseError::Truncated);
        }

        let word = |index: usize| ((bytes[index] as u16) << 8) | (bytes[index + 1] as u16);

        let type_ = RecordType::new(word(*cursor));
        let class = RecordClass::new(word(*cursor + 2));
        *cursor += 4;

        Ok(Question { name, type_, class })
    }
}

impl Display for Question {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "Name: {}  Type: {}  Class:  {}",
            self.name, self.type_, self.class
        )?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    name: Name,
    type_: RecordType,
    class: RecordClass,
    ttl: Ttl,
    rdata: Rdata,
}

impl Record {
    fn parse(bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        let name = Name::parse(bytes, cursor)?;

        if *cursor + 8 > bytes.len() {
            return Err(ParseError::Truncated);
        }

        let word = |index: usize| ((bytes[index] as u16) << 8) | (bytes[index + 1] as u16);

        let dword = |index: usize| {
            ((bytes[index] as u32) << 24)
                | ((bytes[index + 1] as u32) << 16)
                | ((bytes[index + 2] as u32) << 8)
                | (bytes[index + 3] as u32)
        };

        let type_ = RecordType::new(word(*cursor));
        let class = RecordClass::new(word(*cursor + 2));
        let ttl = Ttl::new(dword(*cursor + 4));
        *cursor += 8;

        let rdata = Rdata::parse(type_, bytes, cursor)?;

        Ok(Record {
            name,
            type_,
            class,
            ttl,
            rdata,
        })
    }
}

impl Display for Record {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "Name: {}  Type: {}  Class:  {}  TTL: {}  Record data: {}",
            self.name, self.type_, self.class, self.ttl, self.rdata
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Flags {
    value: u16,
}

impl Flags {
    fn new(value: u16) -> Self {
        Flags { value }
    }
}

impl Display for Flags {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let type_ = if self.value >> 15 == 0 {
            "Query"
        } else {
            "Reply"
        };

        // FIXME
        write!(fmt, "Type: {}  Raw: {:04x}", type_, self.value)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Message {
    id: u16,
    flags: Flags,
    questions: Vec<Question>,
    answers: Vec<Record>,
    authority_rrs: Vec<Record>,
    additional_rrs: Vec<Record>,
}

#[derive(Debug)]
pub enum ParseError {
    Truncated,
    Extra,
    Invalid,
}

impl Message {
    pub fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        if bytes.len() < 12 {
            return Err(ParseError::Truncated);
        }

        let header_word =
            |index: usize| ((bytes[2 * index] as u16) << 8) | (bytes[2 * index + 1] as u16);

        let id = header_word(0);
        let flags = header_word(1);
        let num_questions = header_word(2);
        let num_answers = header_word(3);
        let num_authority_rrs = header_word(4);
        let num_additional_rrs = header_word(5);

        fn parse_many<T>(
            num: u16,
            bytes: &[u8],
            cursor: &mut usize,
            parse: fn(&[u8], &mut usize) -> Result<T, ParseError>,
        ) -> Result<Vec<T>, ParseError> {
            (0..num).fold(Ok(vec![]), |result, _| {
                result.and_then(|mut vec| {
                    parse(bytes, cursor).map(|t| {
                        vec.push(t);
                        vec
                    })
                })
            })
        }

        let mut cursor = 12;

        let message = Message {
            id,
            flags: Flags::new(flags),
            questions: parse_many(num_questions, bytes, &mut cursor, Question::parse)?,
            answers: parse_many(num_answers, bytes, &mut cursor, Record::parse)?,
            authority_rrs: parse_many(num_authority_rrs, bytes, &mut cursor, Record::parse)?,
            additional_rrs: parse_many(num_additional_rrs, bytes, &mut cursor, Record::parse)?,
        };

        if cursor < bytes.len() {
            return Err(ParseError::Extra);
        }

        Ok(message)
    }
}

impl Display for Message {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        writeln!(fmt, "ID: {}\nFlags: {}", self.id, self.flags)?;

        macro_rules! section {
            ($fmt:expr, $name:expr, $items:expr) => {
                if $items.len() == 0 {
                    writeln!($fmt, "{}: None", $name)?;
                } else {
                    writeln!($fmt, "{}:", $name)?;
                    for item in $items {
                        writeln!($fmt, "  {}", item)?;
                    }
                }
            };
        }

        section!(fmt, "Questions", &self.questions);
        section!(fmt, "Answers", &self.answers);
        section!(fmt, "Authority records", &self.authority_rrs);
        section!(fmt, "Additional records", &self.additional_rrs);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::protocol::{
        Flags, Message, Name, Question, Rdata, Record, RecordClass, RecordType, Ttl,
    };
    use std::str::FromStr;

    const XKCD_MESSAGE: [u8; 49] = [
        0x41, 0xde, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x04, 0x78, 0x6b,
        0x63, 0x64, 0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x29, 0x10,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x0a, 0x00, 0x08, 0x8f, 0x2d, 0xe3, 0x7b,
        0x74, 0x5d, 0x6b, 0x4d,
    ];

    #[test]
    fn test_message_parse() {
        let message = Message::parse(&XKCD_MESSAGE).unwrap();

        assert_eq!(
            message,
            Message {
                id: 0x41de,
                flags: Flags::new(0x0120),
                questions: vec![Question {
                    name: Name::from_str("xkcd.com").unwrap(),
                    type_: RecordType::from_str("A").unwrap(),
                    class: RecordClass::new(0x01),
                }],
                answers: vec![],
                authority_rrs: vec![],
                additional_rrs: vec![Record {
                    name: Name::from_str("").unwrap(),
                    type_: RecordType::from_str("OPT").unwrap(),
                    class: RecordClass::new(0x1000),
                    ttl: Ttl::new(0),
                    rdata: Rdata::Other { data: vec![] },
                }],
            }
        );
    }
}
