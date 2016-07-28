/**
 * 折半查找
 */
pub fn binary_search(haystack: &[isize], needle:isize) -> isize {
    let mut low = 0;
    let mut high = haystack.len() as isize - 1;
    if high > 0 {
        while low <= high {
            let mid = (high - low) / 2 as isize + low;
            let md = haystack[mid as usize];
            if md > needle {
                high = mid - 1;
            } else if md < needle {
                low = mid + 1;
            } else {
                return mid;
            }
        }
    }
    return -1;
}
