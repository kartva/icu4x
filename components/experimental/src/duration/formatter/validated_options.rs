use crate::duration::options::*;

/// Validated options for [DurationFormatter](DurationFormatter).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedDurationFormatterOptions {
    /// The style that will be applied to units
    /// unless overridden by a specific style.
    base: BaseStyle,

    /// Style for year
    year: FieldStyle,
    /// Visibility control for year
    year_visibility: FieldDisplay,
    /// Style for month
    month: FieldStyle,
    /// Visibility control for month
    month_visibility: FieldDisplay,
    /// Style for week
    week: FieldStyle,
    /// Visibility control for week
    week_visibility: FieldDisplay,
    /// Style for day
    day: FieldStyle,
    /// Visibility control for day
    day_visibility: FieldDisplay,
    /// Style for hour
    hour: FieldStyle,
    /// Visibility control for hour
    hour_visibility: FieldDisplay,
    /// Style for minute
    minute: FieldStyle,
    /// Visibility control for minute
    minute_visibility: FieldDisplay,
    /// Style for second
    second: FieldStyle,
    /// Visibility control for second
    second_visibility: FieldDisplay,
    /// Style for millisecond
    millisecond: FieldStyle,
    /// Visibility control for millisecond
    millisecond_visibility: FieldDisplay,
    /// Style for microsecond
    microsecond: FieldStyle,
    /// Visibility control for microsecond
    microsecond_visibility: FieldDisplay,
    /// Style for nanosecond
    nanosecond: FieldStyle,
    /// Visibility control for nanosecond
    nanosecond_visibility: FieldDisplay,

    /// Number of fractional digits to use when formatting sub-second units (milliseconds, microseconds, nanoseconds).
    /// ### Note:
    /// - Only takes effect when the subsecond units are styled as [`Numeric`](FieldStyle::Numeric).
    /// - Zero means no fractional digits.
    fractional_digits: FractionalDigits,
}

#[non_exhaustive]
pub enum DurationFormatterOptionsError {
    InvalidFractionalDigits,
}

impl ValidatedDurationFormatterOptions {
    pub(crate) fn validate(
        value: DurationFormatterOptions,
    ) -> Result<Self, DurationFormatterOptionsError> {
        let mut builder: ValidatedDurationFormatterOptionsBuilder = value.into();

        let units = builder.iter_units();

        // section 1.1.6
        let mut prev_style = None;
        for (unit, style, visibility) in units.into_iter() {
            // 2. Let displayDefault be "always".
            let mut default_visibility = FieldDisplay::Always;

            // 3. If style is undefined, then
            if style.is_none() {
                // a. If baseStyle is "digital", then
                if value.base == BaseStyle::Digital {
                    // i. If unit is not one of "hours", "minutes", or "seconds", then
                    if unit != Unit::Hour || unit != Unit::Minute || unit != Unit::Second {
                        // 1. Set displayDefault to "auto".
                        default_visibility = FieldDisplay::Auto;
                    }
                    // ii. Set style to digitalBase.
                    *style = Some(unit.digital_default());
                }
                // b. Else,
                else {
                    // i. If prevStyle is "fractional", "numeric" or "2-digit", then
                    if matches!(
                        prev_style,
                        Some(FieldStyle::Fractional | FieldStyle::Numeric | FieldStyle::TwoDigit)
                    ) {
                        // 1. If unit is not one of "minutes" or "seconds", then
                        if unit != Unit::Minute || unit != Unit::Second {
                            // a. Set displayDefault to "auto".
                            default_visibility = FieldDisplay::Auto;
                        }
                        // 2. Set style to "numeric".
                        *style = Some(FieldStyle::Numeric);
                    }
                    // ii. Else,
                    else {
                        // 1. Set displayDefault to "auto".
                        default_visibility = FieldDisplay::Auto;
                        // 2. Set style to baseStyle.
                        *style = Some(value.base.into());
                    }
                }
            }

            // 4. If style is "numeric", then
            if *style == Some(FieldStyle::Numeric) {
                // a. If unit is one of "milliseconds", "microseconds", or "nanoseconds", then
                if unit == Unit::Millisecond
                    || unit == Unit::Microsecond
                    || unit == Unit::Nanosecond
                {
                    // i. Set style to "fractional".
                    *style = Some(FieldStyle::Fractional);
                    // ii. Set displayDefault to "auto".
                    default_visibility = FieldDisplay::Auto;
                }
            }

            // 5. Let displayField be the string-concatenation of unit and "Display".
            // 6. Let display be ? GetOption(options, displayField, string, « "auto", "always" », displayDefault).
            if visibility.is_none() {
                *visibility = Some(default_visibility);
            }

            // 7. If display is "always" and style is "fractional", then
            if *visibility == Some(FieldDisplay::Always) && *style == Some(FieldStyle::Fractional) {
                // a. Throw a RangeError exception.
                return Err(DurationFormatterOptionsError::InvalidFractionalDigits);
            }

            // 8. If prevStyle is "fractional", then
            if prev_style == Some(FieldStyle::Fractional) {
                // a. If style is not "fractional", then
                if *style != Some(FieldStyle::Fractional) {
                    // i. Throw a RangeError exception.
                    return Err(DurationFormatterOptionsError::InvalidFractionalDigits);
                }
            }

            // 9. If prevStyle is "numeric" or "2-digit", then
            if prev_style == Some(FieldStyle::Numeric) || prev_style == Some(FieldStyle::TwoDigit) {
                // a. If style is not "fractional", "numeric" or "2-digit", then
                if !matches!(
                    *style,
                    Some(FieldStyle::Fractional | FieldStyle::Numeric | FieldStyle::TwoDigit)
                ) {
                    // i. Throw a RangeError exception.
                    return Err(DurationFormatterOptionsError::InvalidFractionalDigits);
                }
                // b. If unit is "minutes" or "seconds", then
                if unit == Unit::Minute || unit == Unit::Second {
                    // i. Set style to "2-digit".
                    *style = Some(FieldStyle::TwoDigit);
                }
            }

            // 10. If unit is "hours" and twoDigitHours is true, then
            if unit == Unit::Hour && todo!("twoDigitHours") {
                // a. Set style to "2-digit".
                *style = Some(FieldStyle::TwoDigit);
            }

            prev_style = *style;
        }

        Ok(builder.try_into().unwrap())
    }

    #[allow(dead_code)]
    pub(crate) fn iter_units(&mut self) -> [(Unit, &mut FieldStyle, &mut FieldDisplay); 10] {
        [
            (Unit::Year, &mut self.year, &mut self.year_visibility),
            (Unit::Month, &mut self.month, &mut self.month_visibility),
            (Unit::Week, &mut self.week, &mut self.week_visibility),
            (Unit::Day, &mut self.day, &mut self.day_visibility),
            (Unit::Hour, &mut self.hour, &mut self.hour_visibility),
            (Unit::Minute, &mut self.minute, &mut self.minute_visibility),
            (Unit::Second, &mut self.second, &mut self.second_visibility),
            (
                Unit::Millisecond,
                &mut self.millisecond,
                &mut self.millisecond_visibility,
            ),
            (
                Unit::Microsecond,
                &mut self.microsecond,
                &mut self.microsecond_visibility,
            ),
            (
                Unit::Nanosecond,
                &mut self.nanosecond,
                &mut self.nanosecond_visibility,
            ),
        ]
    }
}

/// Validated options builder for [DurationFormatter](DurationFormatter).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ValidatedDurationFormatterOptionsBuilder {
    base: BaseStyle,

    year: Option<FieldStyle>,
    year_visibility: Option<FieldDisplay>,
    month: Option<FieldStyle>,
    month_visibility: Option<FieldDisplay>,
    week: Option<FieldStyle>,
    week_visibility: Option<FieldDisplay>,
    day: Option<FieldStyle>,
    day_visibility: Option<FieldDisplay>,
    hour: Option<FieldStyle>,
    hour_visibility: Option<FieldDisplay>,
    minute: Option<FieldStyle>,
    minute_visibility: Option<FieldDisplay>,
    second: Option<FieldStyle>,
    second_visibility: Option<FieldDisplay>,
    millisecond: Option<FieldStyle>,
    millisecond_visibility: Option<FieldDisplay>,
    microsecond: Option<FieldStyle>,
    microsecond_visibility: Option<FieldDisplay>,
    nanosecond: Option<FieldStyle>,
    nanosecond_visibility: Option<FieldDisplay>,
    fractional_digits: FractionalDigits,
}

impl ValidatedDurationFormatterOptionsBuilder {
    fn iter_units(&mut self) -> [(Unit, &mut Option<FieldStyle>, &mut Option<FieldDisplay>); 10] {
        [
            (Unit::Year, &mut self.year, &mut self.year_visibility),
            (Unit::Month, &mut self.month, &mut self.month_visibility),
            (Unit::Week, &mut self.week, &mut self.week_visibility),
            (Unit::Day, &mut self.day, &mut self.day_visibility),
            (Unit::Hour, &mut self.hour, &mut self.hour_visibility),
            (Unit::Minute, &mut self.minute, &mut self.minute_visibility),
            (Unit::Second, &mut self.second, &mut self.second_visibility),
            (
                Unit::Millisecond,
                &mut self.millisecond,
                &mut self.millisecond_visibility,
            ),
            (
                Unit::Microsecond,
                &mut self.microsecond,
                &mut self.microsecond_visibility,
            ),
            (
                Unit::Nanosecond,
                &mut self.nanosecond,
                &mut self.nanosecond_visibility,
            ),
        ]
    }
}

impl From<DurationFormatterOptions> for ValidatedDurationFormatterOptionsBuilder {
    fn from(value: DurationFormatterOptions) -> Self {
        ValidatedDurationFormatterOptionsBuilder {
            base: value.base,
            year: value.year.map(FieldStyle::from),
            year_visibility: value.year_visibility,
            month: value.month.map(FieldStyle::from),
            month_visibility: value.month_visibility,
            week: value.week.map(FieldStyle::from),
            week_visibility: value.week_visibility,
            day: value.day.map(FieldStyle::from),
            day_visibility: value.day_visibility,
            hour: value.hour.map(FieldStyle::from),
            hour_visibility: value.hour_visibility,
            minute: value.minute.map(FieldStyle::from),
            minute_visibility: value.minute_visibility,
            second: value.second.map(FieldStyle::from),
            second_visibility: value.second_visibility,
            millisecond: value.millisecond.map(FieldStyle::from),
            millisecond_visibility: value.millisecond_visibility,
            microsecond: value.microsecond.map(FieldStyle::from),
            microsecond_visibility: value.microsecond_visibility,
            nanosecond: value.nanosecond.map(FieldStyle::from),
            nanosecond_visibility: value.nanosecond_visibility,
            fractional_digits: value.fractional_digits,
        }
    }
}

impl TryFrom<ValidatedDurationFormatterOptionsBuilder> for ValidatedDurationFormatterOptions {
    type Error = ();

    fn try_from(value: ValidatedDurationFormatterOptionsBuilder) -> Result<Self, Self::Error> {
        Ok(ValidatedDurationFormatterOptions {
            base: value.base,
            year: value.year.ok_or(())?,
            year_visibility: value.year_visibility.ok_or(())?,
            month: value.month.ok_or(())?,
            month_visibility: value.month_visibility.ok_or(())?,
            week: value.week.ok_or(())?,
            week_visibility: value.week_visibility.ok_or(())?,
            day: value.day.ok_or(())?,
            day_visibility: value.day_visibility.ok_or(())?,
            hour: value.hour.ok_or(())?,
            hour_visibility: value.hour_visibility.ok_or(())?,
            minute: value.minute.ok_or(())?,
            minute_visibility: value.minute_visibility.ok_or(())?,
            second: value.second.ok_or(())?,
            second_visibility: value.second_visibility.ok_or(())?,
            millisecond: value.millisecond.ok_or(())?,
            millisecond_visibility: value.millisecond_visibility.ok_or(())?,
            microsecond: value.microsecond.ok_or(())?,
            microsecond_visibility: value.microsecond_visibility.ok_or(())?,
            nanosecond: value.nanosecond.ok_or(())?,
            nanosecond_visibility: value.nanosecond_visibility.ok_or(())?,
            fractional_digits: value.fractional_digits,
        })
    }
}

/// An enum to specify the unit being used. Used with FieldStyle and FieldDisplay to indicate the field unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
}

impl Unit {
    pub(crate) fn digital_default(&self) -> FieldStyle {
        match self {
            Unit::Year => YearStyle::Short.into(),
            Unit::Month => MonthStyle::Short.into(),
            Unit::Week => WeekStyle::Short.into(),
            Unit::Day => DayStyle::Short.into(),
            Unit::Hour => HourStyle::Short.into(),
            Unit::Minute => MinuteStyle::Numeric.into(),
            Unit::Second => SecondStyle::Numeric.into(),
            Unit::Millisecond => MilliSecondStyle::Numeric.into(),
            Unit::Microsecond => MicroSecondStyle::Numeric.into(),
            Unit::Nanosecond => NanoSecondStyle::Numeric.into(),
        }
    }
}
