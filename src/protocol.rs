use phf::phf_map;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Copy)]
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

#[derive(Clone)]
struct RecordClass {
    value: u16,
}

impl RecordClass {
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

#[derive(Clone)]
struct Question {
    name: String,
    type_: RecordType,
    class: RecordClass,
}

impl Question {
    fn parse<R: Read>(bytes: &mut R) -> Result<Self, ParseError> {
        unimplemented!()
    }
}

#[derive(Clone)]
struct Record {
    name: String,
    type_: RecordType,
    class: RecordClass,
    ttl: u32,
    rdata: Vec<u8>,
}

impl Record {
    fn parse<R: Read>(bytes: &mut R) -> Result<Self, ParseError> {
        unimplemented!()
    }
}

struct Flags {
    value: u16,
}

struct Message {
    id: u16,
    flags: Flags,
    questions: Vec<Question>,
    answers: Vec<Record>,
    authority_rrs: Vec<Record>,
    additional_rrs: Vec<Record>,
}

// FIXME
enum ParseError {}

use std::io::Read;

impl Message {
    fn parse<R: Read>(bytes: &mut R) -> Result<Self, ParseError> {
        let mut header_bytes = [0u8; 12];
        bytes.read_exact(&mut header_bytes).unwrap(); // FIXME

        let header_word = |index: usize| {
            ((header_bytes[2 * index] as u16) << 8) + (header_bytes[2 * index + 1] as u16)
        };

        let id = header_word(0);
        let flags = header_word(1);
        let num_questions = header_word(2);
        let num_answers = header_word(3);
        let num_authority_rrs = header_word(4);
        let num_additional_rrs = header_word(5);

        fn parse_many<R: Read, T>(
            num: u16,
            bytes: &mut R,
            parse: fn(&mut R) -> Result<T, ParseError>,
        ) -> Result<Vec<T>, ParseError> {
            (0..num).fold(Ok(vec![]), |result, _| {
                result.and_then(|mut vec| {
                    parse(bytes).map(|t| {
                        vec.push(t);
                        vec
                    })
                })
            })
        }

        Ok(Message {
            id,
            flags: Flags { value: flags },
            questions: parse_many(num_questions, bytes, Question::parse)?,
            answers: parse_many(num_answers, bytes, Record::parse)?,
            authority_rrs: parse_many(num_authority_rrs, bytes, Record::parse)?,
            additional_rrs: parse_many(num_additional_rrs, bytes, Record::parse)?,
        })
    }
}
