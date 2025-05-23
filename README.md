# gc-log-analyzer

🚀 Rust製の Java GC ログ解析＆可視化ツール（G1GC対応）

このツールは、JavaのG1GCログファイルを解析し、以下の情報を視覚化・CSV出力・サマリ集計します：

- 🟩 Eden / Survivor / Old / Humongous 領域の使用量推移（Before/After）
- ⏱ GCによるPause時間（STW）の時系列変化
- 📊 GCイベント種別ごとの発生件数サマリ

---

## 🧪 サンプル実行

```bash
cargo run -- \
  --input sample/sample_gc.log \
  --plot output/sample.png \
  --mode combined \
  --csv output/sample.csv \
  --summary
```

出力：
- 📈 `output/sample.png` にグラフ保存（指定モードに応じて）
- 📄 `output/sample.csv` にGCイベント一覧をCSV出力
- 📊 GCイベント種別の件数をコンソールに表示

---

## 📦 インストール

```bash
cargo install --path .
```

その後、以下のように実行可能：

```bash
gca --input sample/sample_gc.log --mode heap --plot out.png
```

---

## 📋 コマンドオプション一覧

```bash
gca --help
```

```
Usage: gca [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>     GC log file path
  -p, --plot <PLOT>       Output PNG file [default: output.png]
  -m, --mode <MODE>       Rendering mode: heap, pause, combined [default: combined]
      --csv <CSV>         CSV output destination (optional)
      --summary           Display number of GC types
  -h, --help              Print help
  -V, --version           Print version
```

---

## 📈 描画モードの説明

| モード     | 説明                                              |
|------------|---------------------------------------------------|
| `heap`     | ヒープ領域（Eden / Old / Humongousなど）のBefore/After推移を描画 |
| `pause`    | GCによるSTW(Pause)時間の時系列推移を描画                     |
| `combined` | 上記2つを1枚のグラフにオーバーレイ（相関を確認したいとき）         |

---

## 📝 ライセンス

MIT License

---

## ✨ TODO（今後の拡張）

- [ ] ZGCやParallel GCへの対応
- [ ] `--start-time`, `--end-time` による時刻範囲フィルタ
- [ ] `--summary-table` でターミナルに表形式出力
- [ ] `--highlight` でPause時間が閾値超のイベントを強調表示
