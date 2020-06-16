pub fn to_celsius<T>(kelvin: T) -> T
    where T: num_traits::Float + num_traits::FromPrimitive
{
    kelvin - T::from_f64(273.15).unwrap()
}

#[cfg(test)]
mod test {
    const EPSILON: f64 = 1e-6;

    #[test]
    fn it_converts_to_celcius() {

        let samples = vec![
            (173.15, -100.0),
            (273.15, 0.0),
            (373.15, 100.0),
        ];

        for (input, expected) in samples {
            let actual = super::to_celsius::<f64>(input);
            assert_eq!((actual - expected).abs() < EPSILON, true);
        }
    }
}
