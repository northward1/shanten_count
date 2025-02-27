# shanten_count

## テスト

[シャンテン数計算 - #define int ll](https://wistery-k.hatenadiary.org/entry/20130206/1360168063) のサンプルを利用する。

```bash
curl https://gist.githubusercontent.com/wistery-k/4723533/raw/2d77d9cbc1168ed1946bd276e110d1478bd62619/input%201 -o input.txt
curl https://gist.githubusercontent.com/wistery-k/4723571/raw/67d9a88000196bfb3d5d11eac22b632901a423af/output1 -o answer.txt

cat input.txt | cargo run --release --bin run_testcase > out.txt
diff answer.txt out.txt
```
