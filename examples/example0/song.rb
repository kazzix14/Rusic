require 'jungru'

j = ::Jungru::Piece.new

j.track :hat do |t|
  t.symbol :a, vel: 0.5, tone: 0
  t.symbol :b, vel: 0.5, tone: 1
  t.instrument :my_instrument

  t.section :a do |s|
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