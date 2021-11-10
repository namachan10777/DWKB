# Design note

## STM32L412RBTx

* HSIでDFU可能
* [stm32-usbd](https://github.com/stm32-rs/stm32-usbd)がサポート
* 安価

### VDD12

* 内部でのLDOをバイパスするためのもの。今回は1.10 V出力自体扱わないのでNo connectionに

## TLV758P

* 出力電圧は$3.3 + V_f$なので、$3.3 + V_f = V_{FB} \left( 1 + \frac{R_1}{R_2}\right)$で、式変形して$\frac{R_1}{R_2} = \frac{3.3 + V_f}{V_{FB}} - 1$
  * $V_{FB} = 0.55\ \mathrm{V}$で${V_f = 0.65\ \mathrm{V}}$なので、${\frac{R_1}{R_2} = 6.18}$
  * $R_1 + R_2 \leq \frac{V_{OUT}}{I_{FB} \cdot 100}$であり、$V_{OUT} = 3.3 + V_{FB} = 3.95$ で $I_{FB} = 10\ \mathrm{nA}$ なので、$R_1 + R_2 \leq 395\ \mathrm{M\Omega}$
  * $R_1 = 68\ \mathrm{k\Omega}$、$R_2 = 11\ \mathrm{k\Omega}$
