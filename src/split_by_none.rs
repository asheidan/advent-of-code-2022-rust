

pub trait SplitByNone<T>
{
    fn split_by_none(self) -> Vec<Vec<T>>;
}

impl<'a, T, I> SplitByNone<T> for I
where
    I: Iterator<Item = Option<T>>,
{
    fn split_by_none(self) -> Vec<Vec<T>> {
        self.fold(vec![Vec::new()], |mut acc, maybe_value| match maybe_value {
            Some(value) => {
                acc.last_mut().unwrap().push(value);
                acc
            }
            None => {
                acc.push(Vec::new());
                acc
            }
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element_should_result_in_vec_in_vec() {
        // Given
        let input: Vec<Option<i32>> = vec![Some(1)];

        // When
        let result: Vec<Vec<i32>> = input.into_iter().split_by_none();

        // Then
        assert_eq!(vec![vec![1]], result);
    }

    #[test]
    fn elements_separated_by_none_should_be_in_different_vec() {
        // Given
        let input: Vec<Option<i32>> = vec![Some(1), None, Some(2)];

        // When
        let result: Vec<Vec<i32>> = input.into_iter().split_by_none();

        // Then
        assert_eq!(vec![vec![1], vec![2]], result);
    }
}
