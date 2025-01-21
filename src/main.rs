use std::io::{self, Write};
use std::{env, fs, path::Path};
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("使用方法: {} <ディレクトリ名> <MDXファイル名>", args[0]);
        return;
    }
    let directory = &args[1];
    let filename = &args[2];
    let directory_path = format!("src/content/{}", directory);
    let output_path = format!("src/content/{}/{}.mdx", directory, filename);

    //ディレクトリが存在するか確認
    if Path::new(&directory_path).exists() {
        eprintln!(
            "ディレクトリ {} が既に存在します。MDXファイルを生成します",
            directory
        );
    } else {
        eprintln!(
            "ディレクトリ {} が存在しません。作成しますか (y/n) ",
            directory
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("入力の読み込みに失敗しました");
        if input.trim().to_lowercase() != "y" {
            eprintln!("処理を中止しました");
            return;
        }
        fs::create_dir_all(format!("src/content/{}", directory))
            .expect("ディレクトリの作成に失敗しました");
    }
    //現在の日付と時刻を取得
    let now = chrono::Local::now();
    let formatted_data = now.format("%Y-%m-%dT%H:%M:%S%.3f%:z").to_string();
    //MDXファイルの生成
    let mut file = fs::File::create(&output_path).expect("ファイルの作成に失敗しました");
    let mdx_template = format!(
        r#"---
title: 
category:
-
tags:
-
image:
publishDate: "{0}"
modifiedDate: "{0}"
---

import Tweet from "../../components/ui/Tweet.astro"
import Box from '../../components/ui/box.astro'
import Ahrefs from '../../components/mdx/Ahref.astro'
export const components = {{a: Ahrefs }}

"#,
        formatted_data
    );

    file.write_all(mdx_template.as_bytes())
        .expect("ファイルへの書き込みに失敗しました");
    println!("MDXファイルを生成しました: {}", output_path);
}
