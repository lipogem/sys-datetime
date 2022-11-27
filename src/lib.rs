use std::{
    fmt::Display,
    time::{Duration, SystemTime},
};

use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Datetime
/// # Example
/// ```no_run
/// let mut dt = Datetime::default();
/// dt.add_years(1970).add_months(1).add_days(1);
/// dt.add_seconds(Datetime::timestamp().as_secs() as i64);
///
/// let now = Datetime::now();
///
/// assert!(dt == now);
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Datetime {
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Datetime {
    /// plus years
    pub fn add_years(&mut self, years: i64) -> &mut Self {
        if self.year < 0 {
            self.year += years;
            if self.year >= 0 {
                self.year += 1;
            }
        } else if self.year > 0 {
            self.year += years;
            if self.year <= 0 {
                self.year -= 1;
            }
        } else {
            self.year = years;
        }

        if self.month == 2 && self.day > 28 {
            let yz = if self.year < 0 {
                self.year + 1
            } else {
                self.year
            };

            if self.day >= 29 && yz % 4 == 0 && (yz % 100 != 0 || yz % 400 == 0) {
                self.day = 29;
            } else {
                self.day = 28;
            }
        }

        self
    }

    /// plus months
    pub fn add_months(&mut self, months: i64) -> &mut Self {
        let ys = (self.month as i64 + months) / 12;
        let mut ms = (self.month as i64 + months) % 12;

        if ms < 1 {
            ms += 12;
            self.month = ms as u8;

            self.add_years(ys - 1);
        } else {
            self.month = ms as u8;

            self.add_years(ys);
        }

        match self.month {
            4 | 6 | 9 | 11 if self.day > 30 => {
                self.day = 30;
            }
            _ => {}
        }

        self
    }

    /// plus days
    pub fn add_days(&mut self, days: i64) -> &mut Self {
        let mut ds = self.day as i64 + days;

        self.day = 1;

        if ds >= 1 {
            loop {
                let mut yz = if self.year < 0 {
                    self.year + 1
                } else {
                    self.year
                };

                if self.month > 2 {
                    yz += 1;
                }

                if yz % 4 == 0 && (yz % 100 != 0 || yz % 400 == 0) {
                    if ds <= 366 {
                        break;
                    }
                    ds -= 366;
                } else {
                    if ds <= 365 {
                        break;
                    }
                    ds -= 365;
                }

                self.add_years(1);
            }

            loop {
                let yz = if self.year < 0 {
                    self.year + 1
                } else {
                    self.year
                };

                match self.month {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                        if ds <= 31 {
                            break;
                        }
                        ds -= 31;
                    }
                    2 => {
                        if yz % 4 == 0 && (yz % 100 != 0 || yz % 400 == 0) {
                            if ds <= 29 {
                                break;
                            }
                            ds -= 29;
                        } else {
                            if ds <= 28 {
                                break;
                            }
                            ds -= 28;
                        }
                    }
                    4 | 6 | 9 | 11 => {
                        if ds <= 30 {
                            break;
                        }
                        ds -= 30;
                    }
                    _ => {}
                }

                self.add_months(1);
            }
        } else {
            loop {
                let mut yz = if self.year < 0 {
                    self.year + 1
                } else {
                    self.year
                };

                if self.month <= 2 {
                    yz -= 1;
                }

                if yz % 4 == 0 && (yz % 100 != 0 || yz % 400 == 0) {
                    if ds >= -366 {
                        break;
                    }
                    ds += 366;
                } else {
                    if ds >= -365 {
                        break;
                    }
                    ds += 365;
                }

                self.add_years(-1);
            }

            loop {
                if ds >= 1 {
                    break;
                }

                self.add_months(-1);

                let yz = if self.year < 0 {
                    self.year + 1
                } else {
                    self.year
                };

                match self.month {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => {
                        ds += 31;
                    }
                    2 => {
                        if yz % 4 == 0 && (yz % 100 != 0 || yz % 400 == 0) {
                            ds += 29;
                        } else {
                            ds += 28;
                        }
                    }
                    4 | 6 | 9 | 11 => {
                        ds += 30;
                    }
                    _ => {}
                }
            }
        }

        self.day = ds as u8;

        self
    }

    /// plus hours
    /// ```no_run
    /// let mut dt = Datetime::now();
    /// dt.add_hours(8);
    /// println!("{}", dt);
    /// ```
    pub fn add_hours(&mut self, hours: i64) -> &mut Self {
        let mut hs = (self.hour as i64 + hours) % 24;

        if hs < 0 {
            hs += 24;

            self.add_days((self.hour as i64 + hours) / 24 - 1);
        } else {
            self.add_days((self.hour as i64 + hours) / 24);
        }

        self.hour = hs as u8;

        self
    }

    /// plus minutes
    pub fn add_minutes(&mut self, minutes: i64) -> &mut Self {
        let mut ms = (self.minute as i64 + minutes) % 60;

        if ms < 0 {
            ms += 60;

            self.add_hours((self.minute as i64 + minutes) / 60 - 1);
        } else {
            self.add_hours((self.minute as i64 + minutes) / 60);
        }

        self.minute = ms as u8;

        self
    }

    /// plus seconds
    pub fn add_seconds(&mut self, seconds: i64) -> &mut Self {
        let mut ss = (self.second as i64 + seconds) % 60;

        if ss < 0 {
            ss += 60;

            self.add_minutes((self.second as i64 + seconds) / 60 - 1);
        } else {
            self.add_minutes((self.second as i64 + seconds) / 60);
        }

        self.second = ss as u8;

        self
    }

    #[inline(always)]
    pub fn year(&self) -> i64 {
        self.year
    }

    #[inline(always)]
    pub fn month(&self) -> i64 {
        self.month as i64
    }

    #[inline(always)]
    pub fn day(&self) -> i64 {
        self.day as i64
    }

    #[inline(always)]
    pub fn hour(&self) -> i64 {
        self.hour as i64
    }

    #[inline(always)]
    pub fn minute(&self) -> i64 {
        self.minute as i64
    }

    #[inline(always)]
    pub fn second(&self) -> i64 {
        self.second as i64
    }

    /// create from string
    pub fn from_str(dt: &str) -> Option<Self> {
        if let Ok(re) = Regex::new("(\\d+)\\D+(\\d+)\\D+(\\d+)\\D*(\\d*)\\D*(\\d*)\\D*(\\d*)(\\D*)")
        {
            if let Some(caps) = re.captures(dt) {
                let year = if matches!(caps.get(7),Some(b) if b.as_str().contains("BC")) {
                    -caps
                        .get(1)
                        .map_or(0, |m| m.as_str().parse().unwrap_or_default())
                } else {
                    caps.get(1)
                        .map_or(0, |m| m.as_str().parse().unwrap_or_default())
                };
                let month = caps
                    .get(2)
                    .map_or(0, |m| m.as_str().parse().unwrap_or_default());
                let day = caps
                    .get(3)
                    .map_or(0, |m| m.as_str().parse().unwrap_or_default());
                let hour = caps
                    .get(4)
                    .map_or(0, |m| m.as_str().parse().unwrap_or_default());
                let minute = caps
                    .get(5)
                    .map_or(0, |m| m.as_str().parse().unwrap_or_default());
                let second = caps
                    .get(6)
                    .map_or(0, |m| m.as_str().parse().unwrap_or_default());

                return Some(Self {
                    year,
                    month,
                    day,
                    hour,
                    minute,
                    second,
                });
            }
        }

        None
    }

    /// current system timestamp
    pub fn timestamp() -> Duration {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
    }

    /// current time
    pub fn now() -> Self {
        let mut epoch = Self {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        };
        epoch.add_seconds(Datetime::timestamp().as_secs() as i64);
        epoch
    }
}

impl Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.year >= 0 {
            write!(
                f,
                "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}",
                self.year, self.month, self.day, self.hour, self.minute, self.second
            )
        } else {
            write!(
                f,
                "{:0>4}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2} BC",
                -self.year, self.month, self.day, self.hour, self.minute, self.second
            )
        }
    }
}

impl Serialize for Datetime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_string();
        String::serialize(&s, serializer)
    }
}

impl<'de> Deserialize<'de> for Datetime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match Datetime::from_str(&s) {
            Some(r) => Ok(r),
            None => Err(serde::de::Error::custom("The data format is not correct")),
        }
    }
}

#[cfg(feature = "sqlx")]
impl<'r, DB: sqlx::Database> sqlx::Type<DB> for Datetime
where
    DB: sqlx::Database,
    &'r str: sqlx::Type<DB>,
{
    fn type_info() -> <DB as sqlx::Database>::TypeInfo {
        <&str>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        match ty.to_string().as_str() {
            #[cfg(feature = "mysql")]
            "TIMESTAMP" | "DATETIME" | "DATE" => true,
            #[cfg(feature = "postgres")]
            "TIMESTAMP" | "TIMESTAMPTZ" | "DATE" => true,
            _ => false,
        }
    }
}

#[cfg(feature = "sqlx")]
impl<'r, DB> sqlx::Decode<'r, DB> for Datetime
where
    DB: sqlx::Database,
    &'r [u8]: sqlx::Decode<'r, DB>,
    &'r str: sqlx::Decode<'r, DB>,
    i64: sqlx::Decode<'r, DB>,
    i32: sqlx::Decode<'r, DB>,
{
    fn decode(
        value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        Ok(
            match sqlx::ValueRef::type_info(&value)
                .as_ref()
                .to_string()
                .as_str()
            {
                #[cfg(feature = "mysql")]
                "TIMESTAMP" | "DATETIME" | "DATE" => {
                    let buf = <&[u8]>::decode(value)?;
                    let len = buf[0];
                    let mut dt = Self {
                        year: ((buf[2] as i64) << 8) + buf[1] as i64,
                        month: buf[3],
                        day: buf[4],
                        hour: 0,
                        minute: 0,
                        second: 0,
                    };
                    if len > 4 {
                        dt.hour = buf[5];
                        dt.minute = buf[6];
                        dt.second = buf[7];
                    }
                    dt
                }
                #[cfg(feature = "postgres")]
                "TIMESTAMP" | "TIMESTAMPTZ" => {
                    let mut epoch = Self {
                        year: 2000,
                        month: 1,
                        day: 1,
                        hour: 0,
                        minute: 0,
                        second: 0,
                    };
                    epoch.add_seconds(i64::decode(value)? / 1000000);
                    epoch
                }
                #[cfg(feature = "postgres")]
                "DATE" => {
                    let mut epoch = Self {
                        year: 2000,
                        month: 1,
                        day: 1,
                        hour: 0,
                        minute: 0,
                        second: 0,
                    };
                    epoch.add_days(i32::decode(value)? as i64);
                    epoch
                }
                _ => Datetime::from_str(<&str>::decode(value)?).unwrap_or_default(),
            },
        )
    }
}
