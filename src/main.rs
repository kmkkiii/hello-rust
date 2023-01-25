// main関数が最初に実行される
fn main() {
    println!("Hello, world!");

    // 変数宣言=束縛
    // 型推論あり
    // immutable
    let x: i32 = 100;

    // mutをつけて束縛すると変更できる
    let mut y = 50;
    y = 300;

    // 文字列は&strとStringがある
    let str_slice: &str = "world"; // 原則変更不可
    let _string: String = String::from(str_slice); // 可変にするためString型へ
    let _string_format: String = format!("Hello, {}", str_slice);

    // 配列(固定長配列)
    let mut array: [i32; 3] = [1, 2, 3];
    array[0] = 10; // mutなので変更できる
    // array.push(10); // mutでも要素の追加はできない

    // ベクタ(可変長配列)
    let mut vec: Vec<i32> = vec![1, 2, 3]; // vec![...]は配列ライクに書けるようにするマクロ
    vec[0] = 10; // mutなので変更できる
    vec.push(10); // mutなので要素も追加できる
}
