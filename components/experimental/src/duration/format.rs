// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{options::*, Duration, DurationFormatter, DurationSign};

use super::validated_options::Unit;
use core::fmt::Write;
use fixed_decimal::{FixedDecimal, SignDisplay};
use writeable::{PartsWrite, Writeable};

pub mod parts {
    use writeable::Part;

    pub const HOUR: Part = Part {
        category: "unit",
        value: "hour",
    };

    pub const MINUTE: Part = Part {
        category: "unit",
        value: "minute",
    };

    pub const SECOND: Part = Part {
        category: "unit",
        value: "second",
    };

    pub const LITERAL: Part = Part {
        category: "literal",
        value: "literal",
    };
}

/// The [`Writeable`] implementation that is returned by [`DurationFormatter::format`]. See
/// the [`writeable`] crate for how to consume this.
#[derive(Debug)]
pub struct FormattedDuration<'l> {
    pub(crate) fmt: &'l DurationFormatter,
    pub(crate) duration: &'l Duration,
}

impl<'a> FormattedDuration<'a> {
    /// Section 1.1.9
    /// Formats numeric hours to sink. Requires hours formatting style to be either Numeric or TwoDigit.
    fn format_numeric_hours<V: PartsWrite + ?Sized>(
        &self,
        hour_val: u64,
        sign_displayed: &mut bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Let hoursStyle be durationFormat.[[HoursStyle]].

        // 2. Assert: hoursStyle is "numeric" or hoursStyle is "2-digit".
        debug_assert!(
            self.fmt.options.hour == FieldStyle::Numeric
                || self.fmt.options.hour == FieldStyle::TwoDigit
        );
        // 3. Let result be a new empty List.
        // 4. Let nfOpts be OrdinaryObjectCreate(null).
        // 5. Let numberingSystem be durationFormat.[[NumberingSystem]].
        // 6. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).

        let mut fd = FixedDecimal::from(hour_val);

        // 7. If hoursStyle is "2-digit", then
        if self.fmt.options.hour == FieldStyle::TwoDigit {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumIntegerDigits", 2𝔽).
            fd.pad_start(2);
        }

        // 8. If signDisplayed is false, then
        if *sign_displayed {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
            fd.apply_sign_display(SignDisplay::Never);
        }

        // 9. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).
        // 10. Let hoursParts be ! PartitionNumberPattern(nf, hoursValue).
        // 11. For each Record { [[Type]], [[Value]] } part of hoursParts, do
        // a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: "hour" } to result.

        let ffd = self.fmt.fdf.format(&fd);
        sink.with_part(parts::HOUR, |e| ffd.write_to_parts(e))?;

        // 12. Return result.
        Ok(())
    }

    /// Section 1.1.10
    /// Formats numeric minutes to sink. Requires minutes formatting style to be either Numeric or TwoDigit.
    fn format_numeric_minutes<V: PartsWrite + ?Sized>(
        &self,
        minute_val: u64,
        sign_displayed: &mut bool,
        hours_formatted: bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Let result be a new empty List.

        // 2. If hoursDisplayed is true, then
        if hours_formatted {
            // a. Let separator be durationFormat.[[DigitalFormat]].[[HoursMinutesSeparator]].
            // b. Append the Record { [[Type]]: "literal", [[Value]]: separator, [[Unit]]: empty } to result.
            sink.with_part(parts::LITERAL, |e| {
                e.write_str(self.fmt.digital.get().separator.as_ref())
            })?;
        }

        // 3. Let minutesStyle be durationFormat.[[MinutesStyle]].
        // 4. Assert: minutesStyle is "numeric" or minutesStyle is "2-digit".
        debug_assert!(
            self.fmt.options.minute == FieldStyle::Numeric
                || self.fmt.options.minute == FieldStyle::TwoDigit
        );

        // 5. Let nfOpts be OrdinaryObjectCreate(null).
        // 6. Let numberingSystem be durationFormat.[[NumberingSystem]].
        // 7. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).
        let mut fd = FixedDecimal::from(minute_val);

        // 8. If minutesStyle is "2-digit", then
        if self.fmt.options.minute == FieldStyle::TwoDigit {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumIntegerDigits", 2𝔽).
            fd.pad_start(2);
        }

        // 9. If signDisplayed is false, then
        if !*sign_displayed {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
            fd.apply_sign_display(SignDisplay::Never);
        }

        // 10. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).
        // 11. Let minutesParts be ! PartitionNumberPattern(nf, minutesValue).
        // 12. For each Record { [[Type]], [[Value]] } part of minutesParts, do
        // a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: "minute" } to result.

        let ffd = self.fmt.fdf.format(&fd);
        sink.with_part(parts::MINUTE, |e| ffd.write_to_parts(e))?;

        // 13. Return result.
        Ok(())
    }

    /// Section 1.1.7
    /// Computes the sum of all values in durationFormat units with "fractional" style,
    /// expressed as a fraction of the smallest unit of durationFormat that does not use "fractional" style.
    fn add_fractional_digits(&self, original: u64) -> FixedDecimal {
        // convert to u128 to prevent overflow
        let mut result: u128 = original.into();
        let mut exponent = 0;

        // 3. For each row of Table 2, except the header row, in table order, do
        // a. Let style be the value of durationFormat's internal slot whose name is the Style Slot value of the current row.
        for (style, value) in [
            (self.fmt.options.millisecond, self.duration.milliseconds),
            (self.fmt.options.microsecond, self.duration.microseconds),
            (self.fmt.options.nanosecond, self.duration.nanoseconds),
        ] {
            // b. If style is "fractional", then
            if style == FieldStyle::Fractional {
                // i. Assert: The Unit value of the current row is "milliseconds", "microseconds", or "nanoseconds".
                // ii. Let value be the value of duration's field whose name is the Value Field value of the current row.
                // iii. Set value to value / (10 ^ exponent).
                // iv. Set result to result + value.
                result *= u128::pow(10, 3);
                result += u128::from(value);
                // v. Set exponent to exponent + 3.
                exponent += 3;
            }
        }

        FixedDecimal::from(result).multiplied_pow10(-exponent)
    }

    /// Section 1.1.11
    /// Formats numeric seconds to sink. Requires seconds formatting style to be either Numeric or TwoDigit.
    fn format_numeric_seconds<V: PartsWrite + ?Sized>(
        &self,
        mut second_val: FixedDecimal,
        sign_displayed: &mut bool,
        minute_formatted: bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Let secondsStyle be durationFormat.[[SecondsStyle]].
        // 2. Assert: secondsStyle is "numeric" or secondsStyle is "2-digit".
        debug_assert!(matches!(
            self.fmt.options.second,
            FieldStyle::Numeric | FieldStyle::TwoDigit
        ));

        // 3. Let result be a new empty List.

        // 4. If minutesDisplayed is true, then
        if minute_formatted {
            // a. Let separator be durationFormat.[[DigitalFormat]].[[MinutesSecondsSeparator]].
            // b. Append the Record { [[Type]]: "literal", [[Value]]: separator, [[Unit]]: empty } to result.
            sink.with_part(parts::LITERAL, |e| {
                e.write_str(self.fmt.digital.get().separator.as_ref())
            })?;
        }

        // 5. Let nfOpts be OrdinaryObjectCreate(null).
        // 6. Let numberingSystem be durationFormat.[[NumberingSystem]].
        // 7. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).

        // 8. If secondsStyle is "2-digit", then
        if self.fmt.options.second == FieldStyle::TwoDigit {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumIntegerDigits", 2𝔽).
            second_val.pad_start(2);
        }

        // 9. If signDisplayed is false, then
        if !*sign_displayed {
            // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
            second_val.apply_sign_display(SignDisplay::Never);
        }

        // 10. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).

        match self.fmt.options.fractional_digits {
            // 11. If durationFormat.[[FractionalDigits]] is undefined, then
            FractionalDigits::ShowAll => {
                // a. Let maximumFractionDigits be 9𝔽.
                // b. Let minimumFractionDigits be +0𝔽.
                second_val.trunc(-9);
                second_val.pad_end(-1);
            }
            // 12. Else,
            FractionalDigits::Fixed(i) => {
                let i: i16 = i.into();
                // a. Let maximumFractionDigits be durationFormat.[[FractionalDigits]].
                second_val.trunc(-i);
                // b. Let minimumFractionDigits be durationFormat.[[FractionalDigits]].
                second_val.pad_end(-i);
            } // 13. Perform ! CreateDataPropertyOrThrow(nfOpts, "maximumFractionDigits", maximumFractionDigits).
              // 14. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumFractionDigits", minimumFractionDigits).
              // 15. Perform ! CreateDataPropertyOrThrow(nfOpts, "roundingMode", "trunc").
        }

        // 16. Let secondsParts be ! PartitionNumberPattern(nf, secondsValue).
        let ffd = self.fmt.fdf.format(&second_val);
        // 17. For each Record { [[Type]], [[Value]] } part of secondsParts, do
        // a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: "second" } to result
        sink.with_part(parts::SECOND, |e| ffd.write_to_parts(e))?;

        // 18. Return result.
        Ok(())
    }

    /// Section 1.1.12
    /// Formats the parts of duration that use Numeric or TwoDigit style to sink.
    fn format_numeric_units<V: PartsWrite + ?Sized>(
        &self,
        first_numeric_unit: Unit,
        sign_displayed: &mut bool,
        sink: &mut V,
    ) -> core::fmt::Result {
        // 1. Assert: firstNumericUnit is "hours", "minutes", or "seconds".
        debug_assert!(matches!(
            first_numeric_unit,
            Unit::Hour | Unit::Minute | Unit::Second
        ));

        // 3. Let hoursValue be duration.[[Hours]].
        // 4. Let hoursDisplay be durationFormat.[[HoursDisplay]].
        // 5. Let minutesValue be duration.[[Minutes]].
        // 6. Let minutesDisplay be durationFormat.[[MinutesDisplay]].
        // 7. Let secondsValue be duration.[[Seconds]].

        let second_value = // 8. If duration.[[Milliseconds]] is not 0 or duration.[[Microseconds]] is not 0 or duration.[[Nanoseconds]] is not 0, then
			if self.duration.milliseconds != 0
				|| self.duration.microseconds != 0
				|| self.duration.nanoseconds != 0
			{
				// a. Set secondsValue to secondsValue + AddFractionalDigits(durationFormat, duration).
				self.add_fractional_digits(self.duration.seconds)
			} else {
				FixedDecimal::from(self.duration.seconds)
			};

        // 9. Let secondsDisplay be durationFormat.[[SecondsDisplay]].
        // 10. Let hoursFormatted be false.
        let mut hours_formatted = false;

        // 11. If firstNumericUnit is "hours", then
        if first_numeric_unit == Unit::Hour {
            // a. If hoursValue is not 0 or hoursDisplay is not "auto", then
            if self.duration.hours != 0 || self.fmt.options.hour_visibility != FieldDisplay::Auto {
                // i. Set hoursFormatted to true.
                hours_formatted = true;
            }
        }

        // 12. If secondsValue is not 0 or secondsDisplay is not "auto", then
        // a. Let secondsFormatted be true.
        // 13. Else,
        // a. Let secondsFormatted be false.
        let seconds_formatted =
            self.duration.seconds != 0 || self.fmt.options.second_visibility != FieldDisplay::Auto;

        // 14. Let minutesFormatted be false.
        let mut minutes_formatted = false;

        // 15. If firstNumericUnit is "hours" or firstNumericUnit is "minutes", then
        if first_numeric_unit == Unit::Hour || first_numeric_unit == Unit::Minute {
            // a. If hoursFormatted is true and secondsFormatted is true, then
            // b. Else if minutesValue is not 0 or minutesDisplay is not "auto", then
            if (hours_formatted && seconds_formatted)
                || (self.duration.minutes != 0
                    || self.fmt.options.minute_visibility != FieldDisplay::Auto)
            {
                // i. Set minutesFormatted to true.
                minutes_formatted = true;
            }
        }

        // 16. If hoursFormatted is true, then
        if hours_formatted {
            // a. Append FormatNumericHours(durationFormat, hoursValue, signDisplayed) to result.
            self.format_numeric_hours(self.duration.hours, sign_displayed, sink)?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }
        // 17. If minutesFormatted is true, then
        if minutes_formatted {
            // a. Append FormatNumericMinutes(durationFormat, minutesValue, hoursFormatted, signDisplayed) to numericPartsList.
            self.format_numeric_minutes(
                self.duration.minutes,
                sign_displayed,
                hours_formatted,
                sink,
            )?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }

        // 18. If secondsFormatted is true, then
        if seconds_formatted {
            // a. Append FormatNumericSeconds(durationFormat, secondsValue, minutesFormatted, signDisplayed) to numericPartsList.
            self.format_numeric_seconds(second_value, sign_displayed, minutes_formatted, sink)?;
            // b. Set signDisplayed to false.
            *sign_displayed = false;
        }
        // 19. Return numericPartsList.
        Ok(())
    }

    /// Section 1.1.8
    /// Returns true if the next smallest unit uses the "fractional" style.
    fn next_unit_fractional(&self, unit: Unit) -> bool {
        // 1. Assert: unit is "seconds", "milliseconds", or "microseconds".
        debug_assert!(matches!(
            unit,
            Unit::Second | Unit::Millisecond | Unit::Microsecond
        ));
        // 2. If unit is "seconds" and durationFormat.[[MillisecondsStyle]] is "fractional", return true.
        // 3. Else if unit is "milliseconds" and durationFormat.[[MicrosecondsStyle]] is "fractional", return true.
        // 4. Else if unit is "microseconds" and durationFormat.[[NanosecondsStyle]] is "fractional", return true.
        // 5. Return false.

        match unit {
            Unit::Second if self.fmt.options.second == FieldStyle::Fractional => true,
            Unit::Millisecond if self.fmt.options.millisecond == FieldStyle::Fractional => true,
            Unit::Microsecond if self.fmt.options.microsecond == FieldStyle::Fractional => true,
            _ => false,
        }
    }

    fn partition_duration_format_pattern<V: PartsWrite + ?Sized>(
        &self,
        sink: &mut V,
    ) -> core::fmt::Result {
        // let mut result = Vec::new();
        // 2. Let signDisplayed be true.
        let mut sign_displayed = true;
        // 3. Let numericUnitFound be false.
        let mut numeric_unit_found = false;

        // 4.  While numericUnitFound is false, repeat for each row in Table 2 in table order, except the header row:
        for ((unit, style, display), value) in self
            .fmt
            .options
            .iter_units()
            .into_iter()
            .zip(self.duration.iter_units())
        {
            if numeric_unit_found {
                break;
            }
            // a. Let value be the value of duration's field whose name is the Value Field value of the current row.
            // b. Let style be the value of durationFormat's internal slot whose name is the Style Slot value of the current row.
            // c. Let display be the value of durationFormat's internal slot whose name is the Display Slot value of the current row.
            // d. Let unit be the Unit value of the current row.
            // e. If style is "numeric" or "2-digit", then
            if style == FieldStyle::Numeric || style == FieldStyle::TwoDigit {
                // i. Append FormatNumericUnits(durationFormat, duration, unit, signDisplayed) to result.
                self.format_numeric_units(unit, &mut sign_displayed, sink)?;
                // ii. Set numericUnitFound to true.
                numeric_unit_found = true;
            }
            // f. Else,
            else {
                // i. Let nfOpts be OrdinaryObjectCreate(null).
                // ii. If unit is "seconds", "milliseconds", or "microseconds", then
                if matches!(unit, Unit::Second | Unit::Millisecond | Unit::Microsecond) {
                    // 1. If NextUnitFractional(durationFormat, unit) is true, then
                    if self.next_unit_fractional(unit) {
                        // a. Set value to value + AddFractionalDigits(durationFormat, duration).
                        let mut value = self.add_fractional_digits(value);
                        match self.fmt.options.fractional_digits {
                            // b. If durationFormat.[[FractionalDigits]] is undefined, then
                            FractionalDigits::ShowAll => {
                                // i. Let maximumFractionDigits be 9𝔽.
                                value.trunc(-9);
                                // ii. Let minimumFractionDigits be +0𝔽.
                                value.pad_end(0);
                            }
                            // c. Else,
                            FractionalDigits::Fixed(i) => {
                                let i: i16 = i.into();
                                // i. Let maximumFractionDigits be durationFormat.[[FractionalDigits]].
                                value.trunc(-i);
                                // ii. Let minimumFractionDigits be durationFormat.[[FractionalDigits]].
                                value.pad_end(-i);
                            }
                        } // d. Perform ! CreateDataPropertyOrThrow(nfOpts, "maximumFractionDigits", maximumFractionDigits).
                          // e. Perform ! CreateDataPropertyOrThrow(nfOpts, "minimumFractionDigits", minimumFractionDigits).
                          // f. Perform ! CreateDataPropertyOrThrow(nfOpts, "roundingMode", "trunc").

                        // g. Set numericUnitFound to true.
                        numeric_unit_found = true;
                    }
                }
                //  iii. If value is not 0 or display is not "auto", then
                if value != 0 || display != FieldDisplay::Auto {
                    let mut fd = FixedDecimal::from(value);
                    // 1. Let numberingSystem be durationFormat.[[NumberingSystem]].
                    // 2. Perform ! CreateDataPropertyOrThrow(nfOpts, "numberingSystem", numberingSystem).
                    // 3. If signDisplayed is true, then
                    if sign_displayed {
                        // a. Set signDisplayed to false.
                        sign_displayed = false;
                        // b. If value is 0 and DurationSign(duration.[[Years]], duration.[[Months]], duration.[[Weeks]], duration.[[Days]], duration.[[Hours]], duration.[[Minutes]], duration.[[Seconds]], duration.[[Milliseconds]], duration.[[Microseconds]], duration.[[Nanoseconds]]) is -1, then
                        if value == 0 && self.duration.sign == DurationSign::Negative {
                            // i. Set value to negative-zero.
                        }
                    }
                    // 4. Else,
                    else {
                        // a. Perform ! CreateDataPropertyOrThrow(nfOpts, "signDisplay", "never").
                        fd.apply_sign_display(SignDisplay::Never);
                    }
                    // 5. Let numberFormatUnit be the NumberFormat Unit value of the current row.
                    // 6. Perform ! CreateDataPropertyOrThrow(nfOpts, "style", "unit").
                    // 7. Perform ! CreateDataPropertyOrThrow(nfOpts, "unit", numberFormatUnit).
                    // 8. Perform ! CreateDataPropertyOrThrow(nfOpts, "unitDisplay", style).
                    // 9. Let nf be ! Construct(%NumberFormat%, « durationFormat.[[Locale]], nfOpts »).
                    // 10. Let parts be ! PartitionNumberPattern(nf, value).
                    // 11. Let list be a new empty List.
                    // 12. For each Record { [[Type]], [[Value]] } part of parts, do
                    // a. Append the Record { [[Type]]: part.[[Type]], [[Value]]: part.[[Value]], [[Unit]]: numberFormatUnit } to list.
                    // 13. Append list to result.
                }
            }
        }

        // 5. Return ListFormatParts(durationFormat, result).
        Ok(())
    }
}

impl<'a> Writeable for FormattedDuration<'a> {
    fn write_to_parts<V: PartsWrite + ?Sized>(&self, sink: &mut V) -> core::fmt::Result {
        self.partition_duration_format_pattern(sink)
    }
}

impl<'a> core::fmt::Display for FormattedDuration<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.write_to(f)
    }
}

#[cfg(test)]
mod tests {
    use icu_locale::locale;

    use super::*;
    use crate::duration::{formatter::ValidatedDurationFormatterOptions, DurationSign};

    #[test]
    fn test_digital_formatter() {
        let duration = Duration {
            sign: DurationSign::Positive,
            years: 0,
            months: 0,
            weeks: 0,
            days: 0,
            hours: 12,
            minutes: 1,
            seconds: 32,
            milliseconds: 130,
            microseconds: 0,
            nanoseconds: 0,
        };

        let options = DurationFormatterOptions {
            hour: Some(HourStyle::Numeric),
            minute: Some(MinuteStyle::Numeric),
            second: Some(SecondStyle::Numeric),
            ..Default::default()
        };

        let options = ValidatedDurationFormatterOptions::validate(options).unwrap();

        let formatter = DurationFormatter::try_new(&locale!("en").into(), options).unwrap();

        let formatted = FormattedDuration {
            fmt: &formatter,
            duration: &duration,
        };

        assert_eq!(formatted.write_to_string().into_owned(), "12:01:32.13");
    }
}
