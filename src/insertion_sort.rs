mod binary_search;
pub fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.sawp(j - 1, j);
            j -= 1;
        }
    }
}

//Binary isertion sort
//Binary insertion sort is an insertionnsort variant that utilizes binary search to reduce comparisons in a normal inertion sort
pub fn binary_insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        let pos = arr[..i].binary_search(&val).unwrap_or_else(|pos| pos);
        //Swap all elements until specific position
        while j > pos {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
