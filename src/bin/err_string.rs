fn get_int_from_file() -> Result<i32, String> {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    //冗長な書き方
    //let num_str_result = std::fs::read_to_string(path).map_err(|e| e.to_string());
    //let num_str = match num_str_result {
    //    Ok(t) => t,
    //    Err(e) => return Err(e),
    //};

    num_str //最初はnum_strは&str型
        .trim() //文字列の前後の空白文字を削除する。型は&str型のまま
        .parse::<i32>() //&strをi32に変換する。結果はResult<i32, ParseIntError>型
        .map(|t| t * 2) //parse()の結果がOk(t)の場合、t*2を実行してOk(t*2)となる
        .map_err(|e| e.to_string()) //parse()の結果がErr(e)の場合、eの文字列表現（String型）を返す
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("{}", x),
        Err(e) => println!("{}", e),
    }
}
