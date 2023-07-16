use crate::uc::AnalogValue;

pub trait AnalogValueMean {
    #[must_use]
    fn mean(self) -> AnalogValue;
}

impl<'a, T> AnalogValueMean for T
where
    T: IntoIterator<Item = &'a AnalogValue>,
{
    fn mean(self) -> AnalogValue {
        let mut total_value: u32 = 0;
        let mut count: u32 = 0;

        for val in self.into_iter() {
            total_value += val.value() as u32;
            count += 1;
        }

        if count == 0 {
            AnalogValue::ZERO
        } else {
            AnalogValue::new(((total_value + count / 2) / count) as u16)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::uc::AnalogValue;
    use crate::uc_utils::AnalogValueMean;

    #[test]
    fn analog_value_mean() {
        let vals = get_analog_values(vec! []);
        assert_eq!(vals.mean(), AnalogValue::new(0));

        let vals = get_analog_values(vec! [2000]);
        assert_eq!(vals.mean(), AnalogValue::new(2000));

        let vals = get_analog_values(vec! [1500, 1600, 1700]);
        assert_eq!(vals.mean(), AnalogValue::new(1600));

        let vals = get_analog_values(vec! [1500, 1501, 1500]);
        assert_eq!(vals.mean(), AnalogValue::new(1500));

        let vals = get_analog_values(vec! [1500, 1501, 1501]);
        assert_eq!(vals.mean(), AnalogValue::new(1501));

        let vals = get_analog_values(vec! [1501, 1501, 1501]);
        assert_eq!(vals.mean(), AnalogValue::new(1501));

        let vals = get_analog_values(vec! [u16::MAX, u16::MAX, u16::MAX, u16::MAX, u16::MAX]);
        assert_eq!(vals.mean(), AnalogValue::new(u16::MAX));
    }

    fn get_analog_values(values: Vec<u16>) -> Vec<AnalogValue> {
        values.into_iter().map(|x| x.into()).collect()
    }
}
