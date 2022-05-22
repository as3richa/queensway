use phf::phf_map;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn new(value: u16) -> Self {
        Self { value }
    }

    pub fn to_str(self) -> &'static str {
        match self.value {
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
        }
    }
}

impl Display for RecordType {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{} ({})", self.to_str(), self.value)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RecordClass {
    value: u16,
}

impl RecordClass {
    pub fn new(value: u16) -> Self {
        Self { value }
    }

    fn to_str(&self) -> &'static str {
        match self.value {
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
        }
    }
}

impl Display for RecordClass {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{} ({})", self.to_str(), self.value)?;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Question {
    name: Vec<u8>,
    type_: RecordType,
    class: RecordClass,
}

impl Question {
    fn parse(bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        let name = parse_name(bytes, cursor)?;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    name: Vec<u8>,
    type_: RecordType,
    class: RecordClass,
    ttl: u32,
    rdata: Vec<u8>,
}

impl Record {
    fn parse(bytes: &[u8], cursor: &mut usize) -> Result<Self, ParseError> {
        let name = parse_name(bytes, cursor)?;

        if *cursor + 10 > bytes.len() {
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
        let ttl = dword(*cursor + 4);
        let rdata_len = word(*cursor + 8);
        *cursor += 10;

        if *cursor + (rdata_len as usize) > bytes.len() {
            return Err(ParseError::Truncated);
        }

        let rdata = bytes[*cursor..*cursor + (rdata_len as usize)].to_vec();

        *cursor += rdata_len as usize;

        Ok(Record {
            name,
            type_,
            class,
            ttl,
            rdata,
        })
    }
}

fn parse_name(bytes: &[u8], cursor: &mut usize) -> Result<Vec<u8>, ParseError> {
    let mut name = Vec::with_capacity(64);

    loop {
        if *cursor >= bytes.len() {
            return Err(ParseError::Truncated);
        }

        let byte = bytes[*cursor];

        if byte == 0 {
            *cursor += 1;
            return Ok(name);
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
                    parse_name(bytes, &mut pointer_cursor)
                }?;

                *cursor += 2;

                name.extend(&tail);

                return Ok(name);
            }
            _ => return Err(ParseError::Invalid),
        }
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
            flags: Flags { value: flags },
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

#[cfg(test)]
mod test {
    use crate::protocol::{Flags, Message, Question, Record, RecordClass, RecordType};
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
                    name: "xkcd.com".as_bytes().to_vec(),
                    type_: RecordType::from_str("A").unwrap(),
                    class: RecordClass::new(0x01),
                }],
                answers: vec![],
                authority_rrs: vec![],
                additional_rrs: vec![Record {
                    name: vec![],
                    type_: RecordType::from_str("OPT").unwrap(),
                    class: RecordClass::new(0x1000),
                    ttl: 0,
                    rdata: vec![
                        0x00, 0x0a, 0x00, 0x08, 0x8f, 0x2d, 0xe3, 0x7b, 0x74, 0x5d, 0x6b, 0x4d
                    ]
                }],
            }
        );
    }
}
