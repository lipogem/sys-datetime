# sys_datetime
Local system date time

#### Cargo Feature Flags

-   `mysql`: Sqlx mysql support

-   `postgres`: Sqlx postgres support

-   `sqlx`: Sqlx support

*Examples*

```
use sys_datetime::Datetime;

fn main() {
    let dt = Datetime::now();
    let mut dt2 = Datetime::default();
    dt2.add_years(dt.year())
        .add_months(dt.month())
        .add_days(dt.day())
        .add_hours(dt.hour())
        .add_minutes(dt.minute())
        .add_seconds(dt.second());

    assert!(dt == dt2);

    let mut dt = Datetime::default();
    dt.add_years(-1).add_months(1).add_days(31);
    dt.add_days(29);

    assert!(Some(dt) == Datetime::from_str("0001-02-29 00:00:00 BC"));

    // Eastern 8th District Time
    let mut dt = Datetime::now();
    dt.add_hours(8);
}
```

## License

lrpc is provided under the MIT license. See [LICENSE](LICENSE).
