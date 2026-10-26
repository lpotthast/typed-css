use crate::number::{dimension, unit_functions};

dimension! {
    /// A CSS `<angle>` with its unit.
    ///
    /// The helpers [`deg`], [`rad`], [`turn`], and [`grad`] create angles.
    ///
    /// [CSS Values Level 4 angle units](https://www.w3.org/TR/css-values-4/#angles)
    pub struct Angle(AngleUnit), "an angle";
    /// A full circle is `360deg`.
    Deg => deg, try_deg, "deg";
    /// A full circle is `2π rad`.
    Rad => rad, try_rad, "rad";
    /// A full circle is `1turn`.
    Turn => turn, try_turn, "turn";
    /// A full circle is `400grad`.
    Grad => grad, try_grad, "grad";
}

impl Angle {
    /// Return the angle in degrees, whatever its unit.
    pub(crate) fn degrees(self) -> f64 {
        let value = self.value();
        match self.unit {
            AngleUnit::Deg => value,
            AngleUnit::Rad => value.to_degrees(),
            AngleUnit::Turn => value * 360.0,
            AngleUnit::Grad => value * 0.9,
        }
    }
}

unit_functions! {
    Angle;
    /// Create an angle in degrees.
    deg, try_deg;
    /// Create an angle in radians.
    rad, try_rad;
    /// Create an angle in turns.
    turn, try_turn;
    /// Create an angle in gradians.
    grad, try_grad;
}
