欲しい機能

曲のエクスポート
書きながら再生 > キリのいいタイミングで新しいのに変更できる
デッキを用意して, クロスフェードとかもできる
好きなタイミングでトラックの抜き差しもできる

play, 現在のコードがコンパイルされて, 音になる.　それをメモリにおいて, 繰り返し再生する
新しいのに切り替える時, 現在のコードがコンパイルされて, 音になる. それを今流れているところとは別の場所に置く. 指定された切り替え方法に従って, クロスフェードや, カットインをする

e.g.
play when current end. cross fade 1 loop. cross fade shape is linear.
play when current end. cutin.

jungruはruby interpreterをもつ.
gemを使ったプロジェクトを作らなくても, 本体だけ渡せばいい感じにやってくれる
```bash
> jungru new song1
/peaces/song1.rb

> jungru play song1.rb
compiling... done
playing... 0:01

> jungru play song2.rb --after current-end --cross-fade 4bar --cross-fade-shape linear
compiling... done
waiting for song1 to finish
playing... 0:01

> jungru play song1.rb -mute hat0 -mute kick --after current-end --cross-fade 4bar --cross-fade-shape linear
```
 

コンパイラ: コード -> 音 をするシステム
プレイヤ: 再生をするシステム
の二つが必要.
プレイヤはコンパイラを使う
完成すると, プレイヤとテキストエディタがくっついたものになる？

とりあえず, コンパイラを作ろう. とりあえずwavがはければいい.

したのやつらは実行されると, 音を生成して終了する

peace/song0/tracks/tracks.rb
```ruby
master do
  composite len: 16.bar
  "
    4bar(a0 a1)
    4bar(a0 a1)
    4bar(a0 a1)
    4bar(a0 a1)
  "
end

track :hat0, inst: :hat0 do
  assign_each do |k, i|
    [{vel: 100, tone: i}]
  end

  assign :a, [{vel: 100, tone: 0}]
  
  section :a0, 16.th, len: 1.bar,
  "
    .a.b .a.c .a.b .a.d
    .a.b ...c .a.b ...d
  "

  section :a1, 16.th, len: 1.bar,
  "
    .a.b .a.c .a.b .a.d
    .a.b ...c .a.b ...d
  "
  
  automate do |time|
    effect :lp, freq: time.bar
  end

  effect :lp, freq: 1200
  effect :hp, freq: 200
end
```

peace/song0/instruments/instruments.rb
```ruby
# ここはRustでもいいかなと思ってる.
class Hat0 < Jungle::Instrument
  def play(note)

    func = Sampler.load('hoge.wav').to_fn
    
    func = ->(t) { 1.sec.ramp_down(:linear) * Osc.new(1.sec).sin(t * note[:frequency]) }

    {
      signal_func: func,
      offset: 0.sec,
    }
  end
end
```

```ruby
def sin(t)
  Sin.new(t)
  ->(len) { fast_sin.curry(t: t) }
end

class Osc
  def initialize(len)
    @len = len
  end

  def sin(t)
    if alive?
      RustFn::sin(t)
    else
      nil
    end
  end
  
  def alive?
    t < @len
  end
end
```

多分, rubyで書いた設定をrustで読みたいみたいな感じ

rev2
```ruby
j = ::Jungru.new

j.master do |master|
  master.sections 'a0 a1 b0 b1'
end

j.track do |track|
  track.name :hat0
  track.inst 'hat0'
  track.symbols do |k, i|
    [{ vel: 100, tone: i }]
  end
  track.symbol :a, [{ vel: 100, tone: 0 }]
  track.section do |section|
    section.name a0
    section.division 1.of 16.th
    section.sheet "
      .a.b .a.c .a.b .a.d
      .a.b ...c .a.b ...d
    "
  end
end
```