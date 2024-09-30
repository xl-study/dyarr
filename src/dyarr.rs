use crate::errors;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Dyarr<T> {
    data: Box<[T]>,
    dimensions: Box<[usize]>,
}

impl<T> Dyarr<T> {
    pub fn from_raw_parts(
        data: Box<[T]>,
        dimensions: Box<[usize]>,
    ) -> Result<Dyarr<T>, errors::IndexError> {
        if (*dimensions).into_iter().fold(1, |acc, ele| acc * ele)
            == data.len()
        {
            Ok(Dyarr { data, dimensions })
        } else {
            Err(errors::IndexError {
                reason: "Data length not matching dimensions.".to_string(),
            })
        }
    }

    pub fn raw(self) -> Box<[T]> {
        self.data
    }

    pub fn raw_ref(&self) -> &Box<[T]> {
        &self.data
    }

    pub fn raw_mut(&mut self) -> &mut Box<[T]> {
        &mut self.data
    }

    pub fn dim(&self) -> &Box<[usize]> {
        &self.dimensions
    }

    fn offset_of_valid_indices(
        &self,
        indices: &[usize],
    ) -> Result<usize, errors::IndexError> {
        if indices.len() != self.dimensions.len() {
            return Err(errors::IndexError {
                reason: "Bad length of indices.".to_string(),
            });
        }
        let mut unit_len = 1;
        let mut res = 0;
        for (&len, &index) in
            self.dimensions.iter().rev().zip(indices.iter().rev())
        {
            if index >= len {
                return Err(errors::IndexError {
                    reason: format!(
                        "Index {:?} out of bound [0, {:?})",
                        index, len
                    ),
                });
            }
            res += index * unit_len;
            unit_len *= len;
        }
        Ok(res)
    }

    pub fn offset(
        &self,
        indices: &[isize],
    ) -> Result<isize, errors::IndexError> {
        let mut unit_len = 1;
        let mut res = 0;
        for (&len, &index) in
            self.dimensions.iter().rev().zip(indices.iter().rev())
        {
            let Ok(len) = isize::try_from(len) else {
                return Err(errors::IndexError {
                    reason: format!(
                        "Dimension length {:?} is too big to process.",
                        len
                    ),
                });
            };
            if index >= len || index <= -len {
                return Err(errors::IndexError {
                    reason: format!(
                        "Index {:?} should be in range [{:?}, {:?}]",
                        index, -len, len
                    ),
                });
            }
            res += index * unit_len;
            unit_len *= len;
        }
        Ok(res)
    }
}

impl<T: Clone> Dyarr<T> {
    pub fn new(init_val: T, dimensions: &[usize]) -> Dyarr<T> {
        let dimensions = dimensions.to_owned().into_boxed_slice();
        Dyarr {
            data: vec![
                init_val;
                dimensions.iter().fold(1, |acc, ele| acc * ele)
            ]
            .into_boxed_slice(),
            dimensions,
        }
    }
}

impl<T, const D: usize> Index<[usize; D]> for Dyarr<T> {
    type Output = T;
    fn index(&self, index: [usize; D]) -> &Self::Output {
        &self[&index as &[usize]]
    }
}

impl<T, const D: usize> IndexMut<[usize; D]> for Dyarr<T> {
    fn index_mut(&mut self, index: [usize; D]) -> &mut T {
        &mut self[&index as &[usize]]
    }
}

impl<T> Index<&[usize]> for Dyarr<T> {
    type Output = T;
    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.raw_ref()[self.offset_of_valid_indices(index).unwrap()]
    }
}

impl<T> IndexMut<&[usize]> for Dyarr<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut T {
        let offset = self.offset_of_valid_indices(index).unwrap();
        &mut self.raw_mut()[offset]
    }
}

impl<T> Into<Box<[T]>> for Dyarr<T> {
    fn into(self) -> Box<[T]> {
        self.raw()
    }
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let some_arr = Dyarr::new(0, &[2, 3, 5]);
        assert_eq!(some_arr.data.len(), 30);
    }

    #[test]
    fn test_offset_of_valid() {
        let arr = Dyarr::new(0, &[3, 4, 5]);
        assert_eq!(arr.offset_of_valid_indices(&[0, 0, 0]).unwrap(), 0);
        assert_eq!(arr.offset_of_valid_indices(&[2, 3, 4]).unwrap(), 59);
        assert_eq!(arr.offset_of_valid_indices(&[2, 1, 3]).unwrap(), 48);
    }

    #[test]
    fn test_calc_offset() {
        let arr = Dyarr::new(0, &[3, 4, 5]);
        assert_eq!(arr.offset(&[0, 0, 0]).unwrap(), 0);
        assert_eq!(arr.offset(&[2, 3, 4]).unwrap(), 59);
        assert_eq!(arr.offset(&[2, 1, 3]).unwrap(), 48);
        assert_eq!(arr.offset(&[2, -1, -3]).unwrap(), 32);
        assert_eq!(arr.offset(&[-1, -1, -3]).unwrap(), -28);
        assert_eq!(arr.offset(&[-1, -3]).unwrap(), -8);
        assert_eq!(arr.offset(&[]).unwrap(), 0);
        arr.offset(&[0, 0, 5]).unwrap_err();
        arr.offset(&[2, -4, 4]).unwrap_err();
    }

    #[test]
    fn test_indexing() {
        let mut arr = Dyarr::new(0, &[3, 4, 5]);
        arr[[2, 3, 4]] = 1;
        assert_eq!(arr[[2, 3, 3]], 0);
        assert_eq!(arr[[2, 3, 4]], 1);
    }

    #[test]
    #[should_panic]
    fn test_get_offset_check_indices_len() {
        let arr = Dyarr::new(0, &[3, 4, 5]);
        arr.offset_of_valid_indices(&[0]).unwrap();
    }

    #[test]
    fn test_get_offset_check_indices_bound() {
        let arr = Dyarr::new(0, &[3, 4, 5]);
        arr.offset_of_valid_indices(&[1, 2, 2]).unwrap();
        arr.offset_of_valid_indices(&[3, 4, 5]).unwrap_err();
        arr.offset_of_valid_indices(&[0, 6, 2]).unwrap_err();
    }

    #[test]
    fn test_from_raw() {
        let data = vec![2, 3, 5, 7, 11, 13];
        let dimensions = vec![2, 3];
        let arr = Dyarr::from_raw_parts(
            data.clone().into(),
            dimensions.clone().into(),
        )
        .unwrap();
        assert_eq!(arr[[0, 0]], 2);
        assert_eq!(arr[[1, 2]], 13);
        Dyarr::from_raw_parts(data.clone().into(), vec![3, 2].into())
            .unwrap();
        Dyarr::from_raw_parts(data.clone().into(), vec![1, 1, 6].into())
            .unwrap();
        Dyarr::from_raw_parts(data.clone().into(), vec![2, 4].into())
            .unwrap_err();
        Dyarr::from_raw_parts(data.clone().into(), vec![].into())
            .unwrap_err();
        Dyarr::from_raw_parts(data.clone().into(), vec![1, 1].into())
            .unwrap_err();
        Dyarr::from_raw_parts(data.clone().into(), vec![0].into())
            .unwrap_err();
    }
}
