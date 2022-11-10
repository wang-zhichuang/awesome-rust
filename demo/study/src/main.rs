fn main() {
    //不可变的东西可不希望被别人修改了！
    let mut s = String::from("看看帅气能否传递！");
    //这个方法是我的邻居，先给他用一下我的东西。
    get_owner(&mut s);
    //他应该会还回来……
    println!("{}",s);
}

fn get_owner(s:&mut String){
    s.push_str("传递了！");
    s.clear();
    s.push_str("x");
}//借用完了就给人家！
