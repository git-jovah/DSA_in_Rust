pub fn bubble_sort(n: &mut [i8],text: &str){ // it is a public function
    for x in 0..n.len(){
        for y in 0..n.len()-x-1{
            if n[y]>n[y+1]{
                n.swap(y,y+1);
            }
        }
    }
    println!("bubble_sort_{text} : {:?}",n);
}
fn infi_num(){
    for x in 0..10000u64{
        print!("{}",x);
        if x==999u64{
            print!("----hello----");
        }
    }
}

pub fn main(){
    bubble_sort(&mut [5,3,8,4,2],"array");
    bubble_sort(&mut vec![3,7,14,99,43,23,5],"vector");
    // infi_num();
}