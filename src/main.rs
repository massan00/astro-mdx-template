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
        r##"---
title: 
category:
-
tags:
-
image: "../../assets/images/dummy.jpg"
publishDate: "{0}"
modifiedDate: "{0}"
---

import Tweet from "../../components/ui/Tweet.astro"
import Box from '../../components/ui/box.astro'
import Matchtable from '../../components/ui/Matchtable.astro'
import Ahrefs from '../../components/mdx/Ahref.astro'
import H2 from '../../components/mdx/H2.astro'
import ArticleSummary from '../../components/ui/ArticleSummary.astro'
import Imagemdx from '../../components/ui/Imagemdx.astro'
import CircleList from '../../components/ui/CircleList.astro'
import Aflink from '../../components/ui/aflink.astro'
import Dazncomp from '../../components/datacomponents/DaznComparison.astro'
import FaqWrapper from '../../components/base/FaqWrapper.astro'
import {{ DAZN }} from '../../components/daznarticle/index.js'
export const components = {{a: Ahrefs}}
	


	
<ArticleSummary items={{[{{text: "DAZN公式の料金プラン", url:"#daznダゾーンの月額料金と年間プランの比較"}},{{text: "DAZNを1ヶ月オトクに使う方法"}}]}} headings={{getHeadings()}} />

<Toc headings={{getHeadings()}} />

<div class="mt-8 p-8 border-2">
<CircleList>
<li>DAZN スタンダード(STANDARD)</li>
<li>DAZN ベースボール(BASEBALL)</li>
<li>DAZN グローバル(GLOBAL)</li>
<li>DAZN フリーミアム(FREEMIUM)</li>
</CircleList>
</div>


<div class="overflow-x-auto mt-8">
  <table class="table-auto border-collapse border border-gray-300 w-full text-sm">
    <thead>
      <tr class="bg-gray-100">
        <th class="border border-gray-300 px-4 py-2 text-left">項目</th>
        <th class="border border-gray-300 px-4 py-2 text-left">支払い</th>
        <th class="border border-gray-300 px-4 py-2 text-left">価格</th>
      </tr>
    </thead>
    <tbody>
      <tr class="hover:bg-gray-50">
        <td class="border border-gray-300 px-4 py-2">月間プラン</td>
        <td class="border border-gray-300 px-4 py-2">1ヶ月単位</td>
        <td class="border border-gray-300 px-4 py-2">980円/月</td>
      </tr>
    </tbody>
  </table>
</div>

<FaqWrapper 
  faqs = {{[
    {{ question: "DAZNスタンダープランの料金は?", answer: "DAZN公式で加入すると4,200円(税込)です。" }},
    {{ question: "DAZNの980円プランは何が見れる?", answer: "DAZN 980円プランはDAZNグローバルプランです。総合格闘技（MMA）、ボクシング、トライアスロン、チェス、レスリングが視聴できます。" }},
    {{ question: "DAZNの3ヶ月500円はどんなプラン?", answer: "DAZNがキャンペーンで行っていた、年間プラン契約で最初の3ヶ月500円で視聴できるプランです。ただ年間契約になるため途中解約はできません。4ヶ月目以降は通常の月額料金になります。" }}
	]}}
/>



"##,
        formatted_data
    );

    file.write_all(mdx_template.as_bytes())
        .expect("ファイルへの書き込みに失敗しました");
    println!("MDXファイルを生成しました: {}", output_path);
}
