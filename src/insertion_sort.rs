pub fn insertion_sort<'main>(n: &'main mut [i8],txt: &str) -> &'main mut [i8]{
    for x in 0..n.len(){
        for y in 0..n.len(){
            if n[x]<n[y]{
                n.swap(x,y);
            }
        }
    }
    println!("insertion_sort_{txt} : {:?}",n);
    n
}
fn _main(){
    let _array_result = insertion_sort(&mut [5,3,4,1,2],"array"); // '_' prefix is to mark unused or temp;
    let _vect_result = insertion_sort(&mut vec![15,33,4,21,92],"vector");
    println!("{}","=".repeat(50));
}