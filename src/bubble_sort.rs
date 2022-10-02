//Bubble sort
pub fn bubble_sort(arr: &mut [i32]) {
    let mut swapped = true;
    while swapped {
        //No swap means array is sorted
        swapped = false;
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true
            }
        }
    }
}
//Optimized bubble sort
pub fn bubble_sort_optimized(arr: &mut [i32]) {
    let mut new_len: usize;
    let mut len = arr.len();
    loop {
        new_len = 0;
        for i in 1..len {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                new_len = i;
            }
        }
        if new_len == o {
            break;
        }
        len = new_len;
    }
}
