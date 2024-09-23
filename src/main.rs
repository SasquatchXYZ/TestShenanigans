mod chapter_20;
mod chapter_21;
mod chapter_22;

fn main() {
    make_a_function!(print_it, 5, 5, 6, I);
    print_it();
    make_a_function!(say_its_nice, this, is, really, nice);
    say_its_nice();
    
    // let x = 6;
    // let my_vec = vec![7, 8, 9];
    // check!(x, 6);
    // check!(my_vec, vec![7, 8, 9]);
    // check!(x, 10);
    // 
    // print_anything!(ththdoetd, rcofe);
    // print_anything!();
    // print_anything!(87676oehq75onth, ntohe, 987987o, 097);
}
