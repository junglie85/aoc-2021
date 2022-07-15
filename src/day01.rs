pub fn calculate_increase_windowed(measurements: Vec<usize>, window_size: usize) -> usize {
    let mut increase = 0;
    let mut last_measurement = None;

    for m in measurements[..].windows(window_size) {
        let window_sum = m.iter().sum::<usize>();
        if let Some(last_sum) = last_measurement {
            if window_sum > last_sum {
                increase += 1;
            }
        }
        last_measurement = Some(window_sum);
    }

    increase
}

pub fn calculate_increase(measurements: Vec<usize>) -> usize {
    calculate_increase_windowed(measurements, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_measurements_has_no_increase() {
        let measurements = Vec::new();

        let increase = calculate_increase(measurements);

        assert_eq!(0, increase);
    }

    #[test]
    fn one_measurement_has_no_increase() {
        let measurements = vec![199];

        let increase = calculate_increase(measurements);

        assert_eq!(0, increase);
    }

    #[test]
    fn increase_from_measurement_1_to_2_is_an_increase() {
        let measurements = vec![199, 200];

        let increase = calculate_increase(measurements);

        assert_eq!(1, increase);
    }

    #[test]
    fn no_change_from_measurement_1_to_2_is_not_an_increase() {
        let measurements = vec![199, 199];

        let increase = calculate_increase(measurements);

        assert_eq!(0, increase);
    }

    #[test]
    fn decrease_from_measurement_1_to_2_is_not_an_increase() {
        let measurements = vec![201, 200];

        let increase = calculate_increase(measurements);

        assert_eq!(0, increase);
    }

    #[test]
    fn given_list_of_sample_data_calculates_increase_of_7() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let increase = calculate_increase(measurements);

        assert_eq!(7, increase);
    }

    #[test]
    fn given_list_of_sample_data_with_window_of_3_calculates_increase_of_5() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let increase = calculate_increase_windowed(measurements, 3);

        assert_eq!(5, increase);
    }
}
