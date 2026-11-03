//! Incremental SSE framing. No networking, JSON decoding, or reconnect policy.
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub(crate) struct Frame {
    pub data: String,
    pub id: Option<String>,
    pub event: String,
    pub retry: Option<Duration>,
}

pub(crate) struct Decoder {
    line: Vec<u8>,
    first_line: bool,
    skip_lf: bool,
    bytes: usize,
    limit: usize,
    data: String,
    has_data: bool,
    event: String,
    id: Option<String>,
    retry: Option<Duration>,
}
impl Decoder {
    pub fn new(limit: usize) -> Self {
        Self {
            line: Vec::new(),
            first_line: true,
            skip_lf: false,
            bytes: 0,
            limit,
            data: String::new(),
            has_data: false,
            event: String::new(),
            id: None,
            retry: None,
        }
    }
    /// Consume one byte. EOF deliberately does not dispatch an unfinished block.
    pub fn push(&mut self, byte: u8) -> Result<Option<Frame>, ()> {
        if self.skip_lf {
            self.skip_lf = false;
            if byte == b'\n' {
                return Ok(None);
            }
        }
        if self.bytes >= self.limit {
            return Err(());
        }
        self.bytes += 1;
        if byte != b'\r' && byte != b'\n' {
            self.line.push(byte);
            return Ok(None);
        }
        self.skip_lf = byte == b'\r';
        let line = std::mem::take(&mut self.line);
        let mut line = String::from_utf8_lossy(&line);
        if self.first_line {
            self.first_line = false;
            if line.starts_with('\u{feff}') {
                line = std::borrow::Cow::Owned(line['\u{feff}'.len_utf8()..].to_owned());
            }
        }
        if line.is_empty() {
            self.bytes = 0;
            let data = std::mem::take(&mut self.data);
            let event = std::mem::take(&mut self.event);
            if !std::mem::take(&mut self.has_data) {
                return Ok(None);
            }
            return Ok(Some(Frame {
                data,
                id: self.id.clone(),
                event: if event.is_empty() {
                    "message".into()
                } else {
                    event
                },
                retry: self.retry,
            }));
        }
        if line.starts_with(':') {
            return Ok(None);
        }
        let (field, value) = line
            .split_once(':')
            .map(|(k, v)| (k, v.strip_prefix(' ').unwrap_or(v)))
            .unwrap_or((&line, ""));
        match field {
            "data" => {
                if self.has_data {
                    self.data.push('\n');
                }
                self.has_data = true;
                self.data.push_str(value);
            }
            "event" => self.event = value.to_owned(),
            "id" if !value.contains('\0') => self.id = Some(value.to_owned()),
            "retry" if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => {
                if let Ok(n) = value.parse::<u64>() {
                    self.retry = Some(Duration::from_millis(n));
                }
            }
            _ => {}
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn decode(bytes: &[u8], limit: usize) -> Result<Vec<Frame>, ()> {
        let mut d = Decoder::new(limit);
        let mut frames = Vec::new();
        for &b in bytes {
            if let Some(frame) = d.push(b)? {
                frames.push(frame);
            }
        }
        Ok(frames)
    }
    #[test]
    fn bytewise_utf8_bom_mixed_newlines_and_multiline_data() {
        let v = decode(
            "\u{feff}: hello\r\ndata: 雪\rdata:  two\ndata\r\rdata: next\n\n".as_bytes(),
            1024,
        )
        .unwrap();
        assert_eq!(v[0].data, "雪\n two\n");
        assert_eq!(v[1].data, "next");
        assert_eq!(v[0].event, "message");
        assert_eq!(decode(b"data: \xff\n\n", 1024).unwrap()[0].data, "�");
    }
    #[test]
    fn metadata_persists_and_resets_with_standard_field_rules() {
        let v=decode(b"id: one\nretry: 12\nevent: kind\n\ndata: a\n\nid: bad\0id\nretry: -3\nevent: custom\ndata: b\n\nid\nretry: 999999999999999999999\ndata: c\n\n",1024).unwrap();
        assert_eq!(v[0].id.as_deref(), Some("one"));
        assert_eq!(v[0].event, "message");
        assert_eq!(v[1].id.as_deref(), Some("one"));
        assert_eq!(v[1].event, "custom");
        assert_eq!(v[2].id.as_deref(), Some(""));
        assert_eq!(v[2].retry, Some(Duration::from_millis(12)));
    }
    #[test]
    fn comments_unknown_fields_and_unterminated_blocks_do_not_dispatch() {
        for b in [
            b": ping\n\n".as_slice(),
            b"unknown: value\n\n",
            b"data: unfinished",
            b"data: unfinished\n",
        ] {
            assert!(decode(b, 1024).unwrap().is_empty());
        }
        assert_eq!(decode(b"data\n\n", 1024).unwrap()[0].data, "");
    }
    #[test]
    fn bounds_cover_ignored_lines_and_aggregate_data_but_reset_per_block() {
        for b in [
            b": very long comment".as_slice(),
            b"x: ignored long line",
            b"data:a\ndata:b\n\n",
        ] {
            assert!(decode(b, 10).is_err());
        }
        assert_eq!(decode(b"data:a\n\ndata:b\n\n", 8).unwrap().len(), 2);
        assert!(decode(b"x", 0).is_err());
    }
}
