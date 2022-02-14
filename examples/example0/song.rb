require 'jungru'

j = ::Jungru::Piece.new

# should return -1.0 <= signal <= 1.0
# rust 実装的にはビルダーパターンかなー
j.instrument :my_instrument do |i|
  i.signal do |note, time, out, state|
    #out.signal saw(440.0, time)
    #  .lp(1000)
    #  .adsr(0.1, 0.4, 0.8, time)
    #  .sus(0.1)
  end
end

# もちろん何書いてもいい
#j.instrument :my_instrument do |i, t, n|
#  i.init do |i, n|
#    i.state.write [0.1, 0.2, 0.3, 0.4, 0.5]
#  end
#  
#  i.signal do |i, n, t|
#    array = i.state.read
#
#    i.out array[t % 5]
#
#    array[t % 5] = random(0...1)
#
#    i.state.write array
#  end
#end

j.track :hat, :my_instrument do |t|
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
end


#j.master do |m|
#  m.compose "
#    abab
#  "
#end
#
#j.track :hat0 do |t|
#end
#
#j.track do |t|
#  t.name = :hat0
#  t.inst :hat0
#  t.symbols do |k, i|
#    [{ vel: 100, tone: i }]
#  end
#  t.symbol :a, [{ vel: 60, tone: 0 }]
#
#  t.section do |s|
#    s.name = :a
#    s.division = 1.of 16.th
#    s.sheet "
#      .a.b .a.c .a.b .a.d
#    "
#  end
#
#  t.section do |s|
#    s.name = :b
#    s.division = 1.of 16.th
#    s.sheet "
#      .a.b ...c .a.b ...d
#    "
#  end
#end
#
#puts j.play.inspect