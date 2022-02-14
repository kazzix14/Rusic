require 'jungru'
require 'debug'

j = ::Jungru::Piece.new

j.instrument :inst0 do |i|
  i.signal do |i, n, l, t|
    if t < 2.5 * l
      i.out Math.sin(n[:f] * 2.0 * Math::PI * t) * n[:v]
    else
      i.out nil
    end
  end
end

j.instrument :kick do |i|
  i.signal do |i, n, l, t|
    if t < l
      i.out Math.sin((100.0 * (1 - t/l)) * t * 2.0 * Math::PI) * (1 - t/l)
    else
      i.out nil
    end
  end
end

j.meta do |m|
  m.bpm 120.0
  m.composite \
  '
    a
  '
end

j.track :track1, :inst0 do |t|
  t.symbol :a, {f: 220.0, v: 0.1}
  t.symbol :b, {f: 330.0, v: 0.1}
  t.symbol :c, {f: 440.0, v: 0.1}
  t.symbol :d, {f: 550.0, v: 0.1}

  t.section :a do |s|
    s.division 1, 4
    s.sheet \
    "
      aabb ccdd
      abcd abcd
    "
  end
end

j.track :track2, :kick do |t|
  t.symbol :a, {}

  t.section :a do |s|
    s.division 1, 4
    s.sheet \
    "
      aaaa aaaa
      aaaa aaaa
    "
  end
end

j.gen #:wav, 'out.wav'