use crate::gps::types::{Coordinate, UtcDate, UtcDateTime, UtcTime};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavigationEvent {
    Fix(NavigationFix),
    Time(UtcDateTime),
    FixStatus {
        valid: bool,
        utc_time: Option<UtcDateTime>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NavigationFix {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub utc_time: UtcDateTime,
    pub satellites: u8,
    pub hdop_centi: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseError {
    Utf8,
    UnsupportedSentence,
    Malformed,
}

pub struct NmeaParser;

impl NmeaParser {
    pub const fn new() -> Self {
        Self
    }

    pub fn parse(&mut self, sentence: &[u8]) -> Result<Option<NavigationEvent>, ParseError> {
        let sentence = core::str::from_utf8(sentence).map_err(|_| ParseError::Utf8)?;
        let body = sentence
            .strip_prefix('$')
            .ok_or(ParseError::Malformed)?
            .split_once('*')
            .map(|(body, _)| body)
            .ok_or(ParseError::Malformed)?;

        let talker_sentence = body.split(',').next().ok_or(ParseError::Malformed)?;
        let sentence_id = talker_sentence
            .get(talker_sentence.len().saturating_sub(3)..)
            .ok_or(ParseError::Malformed)?;

        match sentence_id {
            "GGA" => parse_gga(body).map(Some),
            "RMC" => parse_rmc(body).map(Some),
            _ => Ok(None),
        }
    }
}

fn parse_gga(body: &str) -> Result<NavigationEvent, ParseError> {
    let mut f = body.split(',');
    let _kind = f.next();
    let utc_time = parse_utc_time(f.next().ok_or(ParseError::Malformed)?)?;
    let latitude = parse_coordinate(
        f.next().ok_or(ParseError::Malformed)?,
        f.next().ok_or(ParseError::Malformed)?,
    )?;
    let longitude = parse_coordinate(
        f.next().ok_or(ParseError::Malformed)?,
        f.next().ok_or(ParseError::Malformed)?,
    )?;
    let quality = parse_u8(f.next().ok_or(ParseError::Malformed)?)?;
    let satellites = parse_u8(f.next().ok_or(ParseError::Malformed)?)?;
    let hdop_centi = parse_decimal_centi(f.next().ok_or(ParseError::Malformed)?)?;

    if quality == 0 {
        return Ok(NavigationEvent::FixStatus {
            valid: false,
            utc_time: Some(UtcDateTime {
                date: None,
                time: utc_time,
            }),
        });
    }

    Ok(NavigationEvent::Fix(NavigationFix {
        latitude,
        longitude,
        utc_time: UtcDateTime {
            date: None,
            time: utc_time,
        },
        satellites,
        hdop_centi: Some(hdop_centi),
    }))
}

fn parse_rmc(body: &str) -> Result<NavigationEvent, ParseError> {
    let mut f = body.split(',');
    let _kind = f.next();
    let utc_time = parse_utc_time(f.next().ok_or(ParseError::Malformed)?)?;
    let valid = f.next().ok_or(ParseError::Malformed)? == "A";
    let latitude = f.next().ok_or(ParseError::Malformed)?;
    let ns = f.next().ok_or(ParseError::Malformed)?;
    let longitude = f.next().ok_or(ParseError::Malformed)?;
    let ew = f.next().ok_or(ParseError::Malformed)?;
    let _speed = f.next();
    let _course = f.next();
    let date = parse_utc_date(f.next().ok_or(ParseError::Malformed)?)?;

    let utc_time = UtcDateTime {
        date: Some(date),
        time: utc_time,
    };

    if !valid {
        return Ok(NavigationEvent::FixStatus {
            valid: false,
            utc_time: Some(utc_time),
        });
    }

    Ok(NavigationEvent::Fix(NavigationFix {
        latitude: parse_coordinate(latitude, ns)?,
        longitude: parse_coordinate(longitude, ew)?,
        utc_time,
        satellites: 0,
        hdop_centi: None,
    }))
}

fn parse_utc_time(value: &str) -> Result<UtcTime, ParseError> {
    if value.len() < 6 {
        return Err(ParseError::Malformed);
    }

    Ok(UtcTime {
        hour: parse_u8(&value[0..2])?,
        minute: parse_u8(&value[2..4])?,
        second: parse_u8(&value[4..6])?,
    })
}

fn parse_utc_date(value: &str) -> Result<UtcDate, ParseError> {
    if value.len() != 6 {
        return Err(ParseError::Malformed);
    }

    let year = parse_u16(&value[4..6])?;
    Ok(UtcDate {
        day: parse_u8(&value[0..2])?,
        month: parse_u8(&value[2..4])?,
        year: if year >= 80 { 1900 + year } else { 2000 + year },
    })
}

fn parse_coordinate(value: &str, hemisphere: &str) -> Result<Coordinate, ParseError> {
    let dot = value.find('.').ok_or(ParseError::Malformed)?;
    if dot < 3 {
        return Err(ParseError::Malformed);
    }

    let degree_digits = dot - 2;
    let degrees = parse_i32(&value[..degree_digits])?;
    let minutes_e7 = parse_decimal_e7(&value[degree_digits..])?;
    let mut degrees_e7 = degrees
        .checked_mul(10_000_000)
        .and_then(|v| v.checked_add(minutes_e7 / 60))
        .ok_or(ParseError::Malformed)?;

    match hemisphere {
        "S" | "W" => degrees_e7 = -degrees_e7,
        "N" | "E" => {}
        _ => return Err(ParseError::Malformed),
    }

    Ok(Coordinate { degrees_e7 })
}

fn parse_decimal_e7(value: &str) -> Result<i32, ParseError> {
    let mut parts = value.split('.');
    let whole = parse_i32(parts.next().ok_or(ParseError::Malformed)?)?;
    let frac = parts.next().unwrap_or("");
    let mut frac_e7 = 0i32;
    let mut scale = 1_000_000i32;

    for b in frac.bytes().take(7) {
        if !b.is_ascii_digit() {
            return Err(ParseError::Malformed);
        }
        frac_e7 += ((b - b'0') as i32) * scale;
        scale /= 10;
    }

    Ok(whole * 10_000_000 + frac_e7)
}

fn parse_decimal_centi(value: &str) -> Result<u16, ParseError> {
    if value.is_empty() {
        return Err(ParseError::Malformed);
    }

    let mut parts = value.split('.');
    let whole = parse_u16(parts.next().ok_or(ParseError::Malformed)?)?;
    let frac = parts.next().unwrap_or("");
    let mut centi = whole.checked_mul(100).ok_or(ParseError::Malformed)?;

    for (idx, b) in frac.bytes().take(2).enumerate() {
        if !b.is_ascii_digit() {
            return Err(ParseError::Malformed);
        }
        let digit = (b - b'0') as u16;
        centi += if idx == 0 { digit * 10 } else { digit };
    }

    Ok(centi)
}

fn parse_u8(value: &str) -> Result<u8, ParseError> {
    value.parse().map_err(|_| ParseError::Malformed)
}

fn parse_u16(value: &str) -> Result<u16, ParseError> {
    value.parse().map_err(|_| ParseError::Malformed)
}

fn parse_i32(value: &str) -> Result<i32, ParseError> {
    value.parse().map_err(|_| ParseError::Malformed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_gga_fix() {
        let mut parser = NmeaParser::new();
        let event = parser
            .parse(b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47")
            .unwrap()
            .unwrap();

        match event {
            NavigationEvent::Fix(fix) => {
                assert_eq!(fix.latitude.degrees_e7, 481173000);
                assert_eq!(fix.longitude.degrees_e7, 115166666);
                assert_eq!(fix.satellites, 8);
                assert_eq!(fix.hdop_centi, Some(90));
            }
            _ => panic!("expected fix"),
        }
    }

    #[test]
    fn parses_rmc_fix_with_date() {
        let mut parser = NmeaParser::new();
        let event = parser
            .parse(b"$GPRMC,092751.000,A,5321.6802,N,00630.3372,W,0.06,31.66,280511,,,A*43")
            .unwrap()
            .unwrap();

        match event {
            NavigationEvent::Fix(fix) => {
                assert_eq!(fix.latitude.degrees_e7, 533613366);
                assert_eq!(fix.longitude.degrees_e7, -65056200);
                assert_eq!(fix.utc_time.date.unwrap().year, 2011);
            }
            _ => panic!("expected fix"),
        }
    }
}
