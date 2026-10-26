use crate::number::{dimension, unit_functions};

dimension! {
    /// A CSS `<time>` with its unit.
    ///
    /// The helpers [`s`] and [`ms`] create times. No property in this crate takes a time, but a
    /// custom property can hold one for a stylesheet to use:
    ///
    /// ```rust
    /// use typed_css::{Time, custom_property, ms};
    ///
    /// // A stylesheet can use `transition-duration: var(--fade-duration)`.
    /// custom_property!(FADE_DURATION: Time = "--fade-duration");
    /// assert_eq!(FADE_DURATION.declare(ms(150)).to_string(), "--fade-duration:150ms;");
    /// ```
    ///
    /// [CSS Values Level 4 time units](https://www.w3.org/TR/css-values-4/#time)
    pub struct Time(TimeUnit), "a time";
    S => s, try_s, "s";
    Ms => ms, try_ms, "ms";
}

unit_functions! {
    Time;
    /// Create a time in seconds.
    s, try_s;
    /// Create a time in milliseconds.
    ms, try_ms;
}
