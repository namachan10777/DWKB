# Design note

## STM32L412RBTx

* HSIでDFU可能
* [stm32-usbd](https://github.com/stm32-rs/stm32-usbd)がサポート
* 安価

### VDD12

* 内部でのLDOをバイパスするためのもの。今回は1.10 V出力自体扱わないのでNo connectionに
