require 'jungru'
require 'debug'

j = ::Jungru::Piece.new

j.instrument :inst0 do |i|
  i.init do |i|
  end
  
  i.before_each_note do |i, n|
  end

  i.signal do |i, n, l, t|
    if t < 2.5 * l
      i.out Math.sin(n[:f] * 2.0 * Math::PI * t) * n[:v]
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

j.gen :wav, 'out.wav'