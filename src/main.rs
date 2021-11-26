use std::collections::HashMap;
use std::fs::read_dir;
use std::io::Read;
use std::path::PathBuf;
use tokio::spawn;

#[tokio::main]
async fn main() {
    let files = read_dir("./here").unwrap();
    let mut handles = Vec::new();

    let mut category = String::new();
    println!("Input category");
    std::io::stdin().read_line(&mut category).unwrap();

    for file in files {
        let file = file.unwrap();
        handles.push(spawn(upload_one(file.path(), category.clone())));
    }

    for h in handles {
        h.await.unwrap();
    }
}

async fn upload_one(path: PathBuf, category: String) {
    let mut file = std::fs::File::open(&path).unwrap();
    let mut buf = Vec::new();

    file.read_to_end(&mut buf).unwrap();

    let mut json = HashMap::new();
    json.insert("category", category);
    json.insert(
        "image_type",
        path.as_path()
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    );
    json.insert("image_buffer", base64::encode(buf));

    let client = reqwest::Client::new();
    let resp = client
        .post("http://127.0.0.1:8089/upload_image")
        .json(&json)
        .send()
        .await
        .unwrap();

    println!("{:#?}", resp);
}

#[tokio::test]
async fn upload_test() {
    upload_one(PathBuf::from("./here/sample.bmp"), "대개찜".to_string()).await;
}
