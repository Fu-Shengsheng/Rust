use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // 使用zip方法创建一个元组的vector
    let scores_1: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // hashmap会拥有传入的作为key和value的String的所有权
    let field_name = String::from("Favirote color");
    let field_value = String::from("cyan");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 再次使用移入所有权的String，会报错
    // println!("{},{}", field_name, field_value);

    // 使用get方法访问hashmap中的值
    let team_name_1 = String::from("Blue");
    let score_1 = scores.get(&team_name_1);
    // get 返回 Option<V>，结果会被装进Some
    // 如果hashmap中没有找到key，则返回None

    // 可以使用for循环对hash进行遍历
    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    // 覆盖一个值
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // 只在键没有对应值时插入
    // 使用entry,返回Entry，表示可能存在，
    // Entry的or_insert方法在键对应的值存在时返回这个值的Entry，不存在则将参数作为新值插入并返回修改过的Entry
    scores.entry(String::from("Yellow")).or_insert(90);
    scores.entry(String::from("Orange")).or_insert(77);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map_t = HashMap::new();
    for word in text.split_whitespace() {
        // 根据旧值更新一个值，or_insert返回值的可变引用
        let count = map_t.entry(word).or_insert(0);
        // 解引用，然后赋值
        *count += 1;
    }
    println!("{:?}", map_t);
}
