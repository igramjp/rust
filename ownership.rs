struct Rectangle {
    name: String,
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(w: i32, h: i32) -> Rectangle {
        Rectangle {
            name: "長方形".to_string(),
            width: w,
            height: h,
        }
    }
}

struct RectangleBorrow {
    width: i32,
    height: i32,
}

fn compute_area(rect: &RectangleBorrow) {
    let area = rect.width * rect.height;
    println!("長方形の面積 = {}", area);
}

fn compute_side(rect: &RectangleBorrow) {
    let side = 2 * (rect.width + rect.height);
    println!("長方形の辺 = {}", side);
}

struct MyString {
    mystr: String,
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("オブジェクトをドロップしました");
    }
}

struct RefRectangle<'a> {
    width: &'a i32,
    height: &'a i32,
}

impl<'a> RefRectangle<'a> {
    fn get_longer_edge(&self) -> &'a i32 {
        if self.width >= self.height {
            self.width
        } else {
            self.height
        }
    }
}

static mut GPTR: &i32 = &0;
static GVAL: i32 = 10;

fn main() {
    func();

    // 所有構造
    let rect = Rectangle::new(10, 20);
    println!(
        "name:{}, width:{}, height:{}",
        rect.name, rect.width, rect.height
    );

    // 所有構造は木構造
    let mut rectvec = Vec::new();

    rectvec.push(Rectangle::new(10, 20));
    rectvec.push(Rectangle::new(100, 50));
    rectvec.push(Rectangle::new(30, 90));

    for rect in &rectvec {
        println!(
            "name:{}, width:{}, height:{}",
            rect.name, rect.width, rect.height
        );
    }

    // 所有権の移動
    let name1 = String::from("Alice");
    let name2 = name1;

    println!("My name is {}.", name2);

    // 所有権の移動によるエラー
    // println!("My name is {}.", name1);

    // 関数使用時の所有権の移動
    println!("begin function");
    introduce(name2);
    println!("end function");

    // 関数使用時の所有権移動によるエラー
    // println!("{}", name2);

    // コピー型
    let x = 10;
    let y = 30;

    // 変数xとyはコピーされる
    incr(x, y);

    // 変数を参照
    println!("fn main: x = {}, y = {}", x, y);

    // 参照の借用
    let name3 = String::from("Alice");
    referencesr_and_borrowing(&name3); // 参照の借用(ポインタ型の変数に&を付ける)
    println!("{}", name3);

    // 借用が必要な例
    let rect2 = RectangleBorrow {
        width: 10,
        height: 20,
    };
    compute_area(&rect2); // 参照の借用
    compute_side(&rect2); // 参照の借用

    // 所有権を移動したときのオブジェクトのドロップ
    let name4 = MyString {
        mystr: String::from("Alice"),
    };
    introduce_and_drop(name4);
    println!("main関数の実行が終了しました");

    // 借用したときのオブジェクトのトロップ
    let name5 = MyString {
        mystr: String::from("Alice"),
    };
    introduce_and_borrowing_and_drop(&name5);
    println!("main関数の実行が終了しました");

    // 上記を共有参照の借用と呼ぶ
    // 借りた側がオブジェクトを参照することができ，かつ変更することができる仕組みを「可変参照の借用」と呼ぶ
    let mut msg4 = String::from("Hi");
    println!("before: msg4 = {}", msg4);

    change_msg(&mut msg4);
    println!("after: msg44 = {}", msg4);

    // 可変参照の借用を用いた関数例
    // フィボナッチ数列の計算
    let mut fib: Vec<i32> = vec![0, 1];

    // フィボナッチ数列に8要素分追加
    let mut cnt = 0;
    while cnt < 8 {
        add_element(&mut fib);
        cnt += 1;
    }

    // 結果を表示
    println!("fib = {:?}", fib);

    // コピー型の可変参照の借用
    drop(x);
    drop(y);

    let x = 1;
    incr_copy(x);
    println!("main関数: x = {}", x);

    let mut y = 1;
    incr_by_ref(&mut y);
    println!("main関数: y = {}", y);

    // 可変借用を用いたアルゴリズムの例
    // 挿入ソート
    let mut arr: [i32; 8] = [10, 3, 19, 20, 5, 4, 99, 1];

    println!("before sorting: {:?}", arr);

    insertion_sort(&mut arr);

    println!("after sorting: {:?}", arr);

    // 共有参照と可変参照
    // 借用と生存期間
    // 悪い例
    /*
    let ptr: &i32;
    {
        let val: i32 = 10;
        ptr = &val;
    }
    println!("*ptr = {}", *ptr);
    */
    // 修正したプログラム
    let val: i32 = 10;
    let ptr: &i32;

    ptr = &val; // 参照の借用

    println!("*ptr = {}", *ptr);

    // 生存期間の宣言の必要性
    // 悪い例
    drop(x);
    drop(y);

    /*
    let x = 10;
    let y = 20;

    let z: &i32 = bad_ref_max(&x, &y);
    println!("大きいほうの値 = {}", z);
    */

    // 修正したプログラム
    // 生存期間の宣言
    let x = 10;
    let y = 20;

    let z: &i32 = ref_max(&x, &y);

    println!("大きいほうの値 = {}", z);

    // 構造体メンバの生存期間パラメータ
    let width: i32 = 10;
    let height: i32 = 20;
    let ref_rect = RefRectangle {
        width: &width,
        height: &height,
    };

    println!("width = {}, height = {}", ref_rect.width, ref_rect.height);

    // 構造体メソッドにおける生存期間パラメータ
    println!("長いほうの辺 = {}", ref_rect.get_longer_edge());

    // staticな生存期間
    // ポインタ型の静的変数
    set_ref(&GVAL);

    unsafe {
        println!("*GPTR = {}", GPTR);
    }
}

fn set_ref(ptr: &'static i32) {
    unsafe {
        GPTR = ptr;
    }
}

fn ref_max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if *x >= *y {
        &x
    } else {
        &y
    }
}

/*
fn bad_ref_max(x: &i32, y: &i32) -> &i32 {
    if *x >= *y {
        &x
    } else {
        &y
    }
}
*/

// 挿入ソート
fn insertion_sort(unsorted: &mut [i32]) {
    for i in 1..unsorted.len() {
        let tmp = unsorted[i];
        if unsorted[i - 1] > tmp {
            let mut j = i;
            while j > 0 && unsorted[j - 1] > tmp {
                unsorted[j] = unsorted[j - 1];
                j -= 1;
            }
            unsorted[j] = tmp;
        }
    }
}

fn incr_copy(mut x: i32) {
    x += 1;
    println!("incr_copy関数: x = {}", x);
}

fn incr_by_ref(y: &mut i32) {
    *y += 1;
    println!("incr_by_ref関数: y = {}", *y);
}

// fib[n] = fib[n-2] + fib[n-1]
fn add_element(fib: &mut Vec<i32>) {
    let len = fib.len();
    let x = fib[len - 2];
    let y = fib[len - 1];
    fib.push(x + y);
}

fn change_msg(msg: &mut String) {
    msg.push_str(", how are you?");
}

fn introduce_and_borrowing_and_drop(myname: &MyString) {
    println!("My name is {}.", myname.mystr);
    drop(myname); // ドロップできないがエラーは出ない(name5がドロップされるのはmain関数の処理がすべて終了してから)
}

fn introduce_and_drop(myname: MyString) {
    println!("My name is {}.", myname.mystr);
    drop(myname);
}

fn referencesr_and_borrowing(myname: &String) {
    println!("My name is {}", myname);
}

fn incr(mut x: i32, mut y: i32) {
    x += 1;
    y += 1;
    println!("fn incr: x = {}, y = {}", x, y);
}

fn introduce(myname: String) {
    println!("My name is {}.", myname);
}

fn func() {
    let s1 = String::from("Alice");
    let s2 = String::from("Bob");

    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // 変数s1を解放
    drop(s1);

    let s1 = String::from("Chris");
    println!("s1 = {}", s1);
}
