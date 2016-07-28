/**
 * 冒泡排序
 */
pub fn bubble_sort<T: Ord>(vlist: &mut[T]){
    let mut n = vlist.len();
    let mut swapped = true;

    while swapped {
        swapped = false;
        for i in 1..n {
            if vlist[i-1] > vlist[i] {
                vlist.swap(i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}
