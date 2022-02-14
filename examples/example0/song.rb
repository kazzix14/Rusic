require 'jungru'

j = ::Jungru::Piece.new

# should return -1.0 <= signal <= 1.0
# rust 実装的にはビルダーパターンかなー
j.instrument :inst0 do |i|
  i.init do |store|
    puts 'gero'
    #i.state.write [0.1, 0.2, 0.3, 0.4, 0.5]
  end
  
  i.before_each_note do |note, offset, store|
    puts 'unko'
    puts note
    puts offset
    puts store 
    store = 'neko'
  end

  i.signal do |note, time, out, store|
    #out = 1.0
    #out.signal saw(440.0, time)
    #  .lp(1000)
    #  .adsr(0.1, 0.4, 0.8, time)
    #  .sus(0.1)
  end
end

# もちろん何書いてもいい
j.instrument :inst1 do |i|
  i.init do |store|
    #i.state.write [0.1, 0.2, 0.3, 0.4, 0.5]
  end
  
  i.before_each_note do |note, offset, store|
    offset = 0.1;
  end
  
  i.signal do |note, time, out, store|
    #array = i.state.read

    #i.out array[t % 5]

    #array[t % 5] = random(0...1)

    #i.state.write array
  end
end

j.meta do |m|
  m.bpm 162.0
  m.composite \
  '
    a b a b
  '
end

j.track :hat, :inst0 do |t|
  t.symbol :a, vel: 0.5, tone: 0
  t.symbol :b, vel: 0.5, tone: 1

  t.section :a do |s|
    # rust 側で instrument が実装するnoteに変換する?
    s.symbol :a, vel: 0.5, tone: 2
    s.division 1, 16
    s.sheet \
    "
      aaaa bbbb
      aaaa bbbb
    "
  end

  t.section :b do |s|
    # rust 側で instrument が実装するnoteに変換する?
    s.symbol :a, vel: 0.5, tone: 2
    s.division 1, 16
    s.sheet \
    "
      aaaa bbbb
      aaaa bbbb
    "
  end
end

j.gen #:wav, 'out.wav'