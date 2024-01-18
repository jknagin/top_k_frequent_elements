use std::collections::HashMap;
use std::hash::Hash;

pub fn top_k_most_frequent<T: PartialEq + Eq + Hash>(data: &[T], k: usize) -> Vec<&T> {
    let frequency_count = data.iter().fold(HashMap::<&T, usize>::new(), |mut acc, x| {
        let count = acc.entry(x).or_default();
        *count += 1;
        acc
    });
    let buckets: Vec<Vec<&T>> =
        frequency_count
            .iter()
            .fold(vec![vec![]; k], |mut acc, (item, count)| {
                acc[*count - 1].push(item);
                acc
            });
    let ret: Vec<&T> = buckets
        .iter()
        .rev()
        .fold(Vec::<&T>::new(), |mut acc, bucket| {
            if !bucket.is_empty() && acc.len() < k {
                for element in bucket {
                    acc.push(element);
                    if acc.len() == k {
                        break;
                    }
                }
            }
            acc
        });
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(
            vec![&1, &2, &3],
            top_k_most_frequent(&[1, 1, 1, 2, 2, 3], 3)
        );
    }

    #[test]
    fn test_k_equals_n() {
        assert_eq!(
            vec![&1, &2, &3],
            top_k_most_frequent(&[1, 1, 1, 2, 2, 3], 6)
        );

    }
}
