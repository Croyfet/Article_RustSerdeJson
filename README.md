# Rust serde_json

## About

`Rust` でJSONを扱う `serde_json` の基本的なスクリプトをまとめた。

## Environment

Linux (WSL) or Docker

## How to use

0. Dockerを使う (使わない場合はスキップ)

	Docker関連のファイルは `/docker/` に入っているが `make` にコマンドをまとめてある

	1. 起動時
		```sh
		make
		make up
		```
	2. Shellに入る
		```sh
		make exec
		```
	3. 停止&削除
		```sh
		make down
		```

1. サンプル

	ここでは `00_serde_json` を実行する。

	1. `/apps/00_serde_json` に移動

		```sh
		cd ./apps/00_serde_json
		```

		Dockerの場合は `/apps/` に最初にいるので

		```sh
		cd ./00_serde_json/
		```

	2. cargo

		```sh
		cargo run
		```