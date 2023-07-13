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
            AnalogValue::new((total_value / count) as u16)
        }
    }
}
