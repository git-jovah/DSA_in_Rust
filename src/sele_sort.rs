pub fn selection_sort<'array>(n: &'array mut [i16],text: &str) -> &'array mut [i16]{ //takes any size of arrays, slice and vectors
    //'<name> is a lifetime for parameters so rust will know which parameter is returning
    let mut sm;
    for x in 0..n.len(){
        sm = n[x];
        let mut i = 0;
        for y in x..n.len(){
            if n[y]<=sm{
                sm=n[y];
                i = y;
            }
        }
        if n[x]>sm{
            n.swap(i,x);
        }
    }
    println!("selection_sort_{text} : {:?}",n);
    n // for serious ownership, function return type is necessary

}

fn _exp_swap(mut n:[i8; 10]) -> [i8;10]{     //takes and returns an array with size 10,type i8 {it is a private function}
    for x in 0..n.len(){
        for y in 0..n.len(){
            if n[x]<n[y]{
                n.swap(y,x);
            }
        }
    }
    println!("fixed_size_array_swap(_exp_swap) : {:?}",n);
    n
}

pub fn _main(){
    let mut arr = [3,5,62,26,65,3,1,36,9];
    selection_sort(&mut arr,"array");

    selection_sort(&mut vec![9,2,4,6,82,21,63,65],"vector");

    let sorted_arr = selection_sort(&mut arr[1..7],"sliced_array");
    println!("sorted_arr : {:?}",sorted_arr);
    _exp_swap([2,5,1,6,8,5,3,98,22,26]);

    let mut vect = vec![9,2,5,37,2,3,1];
    vect.sort(); //returns empty vect, changes occur in the original data
    println!("sorted_vect_using_inbuilt_function : {:?}",vect);
}