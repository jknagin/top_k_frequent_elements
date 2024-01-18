use std::collections::HashMap;
use std::hash::Hash;

struct FrequencyCount<'a, T: Eq + Hash>(HashMap<&'a T, usize>);

fn get_frequency_count<T: Eq + Hash>(data: &[T]) -> FrequencyCount<T> {
    FrequencyCount(data.iter().fold(HashMap::new(), |mut acc, x| {
        let count = acc.entry(x).or_default();
        *count += 1;
        acc
    }))
}

struct Buckets<'a, T: Eq + Hash>(Vec<Vec<&'a T>>);

fn bucket_by_frequency<'a, T: Eq + Hash>(
    frequency_count: &FrequencyCount<'a, T>,
) -> Buckets<'a, T> {
    Buckets(frequency_count.0.iter().fold(
        vec![vec![]; frequency_count.0.len()],
        |mut acc, (item, count)| {
            acc[*count - 1].push(item);
            acc
        },
    ))
}

fn top_k_buckets<'a, T: Eq + Hash>(buckets: &Buckets<'a, T>, k: usize) -> Vec<&'a T> {
    let ret = buckets
        .0
        .iter()
        .rev()
        .fold(Vec::<&'a T>::new(), |mut acc, bucket| {
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

pub fn top_k_most_frequent<T: Eq + Hash>(data: &[T], k: usize) -> Vec<&T> {
    let frequency_count = get_frequency_count(data);
    let buckets = bucket_by_frequency(&frequency_count);
    top_k_buckets(&buckets, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(
            vec![&10, &20, &30],
            top_k_most_frequent(&[10, 10, 10, 20, 20, 30], 3)
        );
    }

    #[test]
    fn test_k_equals_n() {
        assert_eq!(
            vec![&1, &2, &3],
            top_k_most_frequent(&[1, 1, 1, 2, 2, 3], 6)
        );
    }

    #[test]
    fn test_k_greater_than_n() {
        assert_eq!(
            vec![&1, &2, &3],
            top_k_most_frequent(&[1, 1, 1, 2, 2, 3], 7)
        );
    }

    #[test]
    fn test_not_all_elements_are_in_top_k_most_frequent() {
        assert_eq!(
            vec![&1, &2, &3],
            top_k_most_frequent(&[3, 3, 1, 1, 1, 1, 2, 2, 2, 6, 1234], 3)
        )
    }
}
